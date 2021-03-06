use crate::PartialAllocStrategy;
use crate::PhysallocFlags;
use crate::Result;
use core::mem::{self, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::{ptr, slice};

#[derive(Debug)]
pub struct PhysBox {
    address: usize,
    size: usize,
}

impl PhysBox {
    pub fn new_partial_allocation(
        size: usize,
        flags: PhysallocFlags,
        strategy: Option<PartialAllocStrategy>,
        mut min: usize,
    ) -> Result<Self> {
        debug_assert!(!(flags.contains(PhysallocFlags::PARTIAL_ALLOC) && strategy.is_none()));
        let address = unsafe {
            crate::physalloc_3(
                size,
                flags.bits() | strategy.map(|s| s as usize).unwrap_or(0),
                &mut min,
            )?
        };
        Ok(Self { address, size: min })
    }

    pub unsafe fn from_raw_parts(address: usize, size: usize) -> Self {
        Self { address, size }
    }
    pub fn address(&self) -> usize {
        self.address
    }
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new_with_flags(size: usize, flags: PhysallocFlags) -> Result<Self> {
        assert!(flags.contains(PhysallocFlags::PARTIAL_ALLOC));
        let address = unsafe { crate::physalloc_2(size, flags.bits())? };
        Ok(Self { address, size })
    }

    pub fn new_in_32bit_space(size: usize) -> Result<Self> {
        Self::new_with_flags(size, PhysallocFlags::SPACE_32)
    }

    pub fn new(size: usize) -> Result<Self> {
        let address = unsafe { crate::physalloc_0(size)? };
        Ok(Self { address, size })
    }
}

impl Drop for PhysBox {
    fn drop(&mut self) {
        let _ = unsafe { crate::physfree(self.address, self.size) };
    }
}

pub struct Dma<T: ?Sized> {
    phys: PhysBox,
    virt: *mut T,
}

impl<T> Dma<T> {
    pub fn from_physbox_uninit(phys: PhysBox) -> Result<Dma<MaybeUninit<T>>> {
        let virt = unsafe { crate::physmap(phys.address, phys.size, crate::PHYSMAP_WRITE)? }
            as *mut MaybeUninit<T>;
        Ok(Dma { phys, virt })
    }
    pub fn from_physbox_zeroed(phys: PhysBox) -> Result<Dma<MaybeUninit<T>>> {
        let this = Self::from_physbox_uninit(phys)?;
        unsafe { ptr::write_bytes(this.virt as *mut MaybeUninit<u8>, 0, this.phys.size) }
        Ok(this)
    }
    pub fn from_physbox(phys: PhysBox, value: T) -> Result<Self> {
        let this = Self::from_physbox_uninit(phys)?;
        Ok(unsafe {
            ptr::write(this.virt, MaybeUninit::new(value));
            this.assume_init()
        })
    }
    pub fn new(value: T) -> Result<Self> {
        let phys = PhysBox::new(mem::size_of::<T>())?;
        Self::from_physbox(phys, value)
    }

    pub fn zerod() -> Result<Dma<MaybeUninit<T>>> {
        let phys = PhysBox::new(mem::size_of::<T>())?;
        Self::from_physbox_zeroed(phys)
    }
}

impl<T> Dma<MaybeUninit<T>> {
    pub unsafe fn assume_init(self) -> Dma<T> {
        let &Dma {
            phys: PhysBox { address, size },
            virt,
        } = &self;
        mem::forget(self);
        Dma {
            phys: PhysBox { address, size },
            virt: virt as *mut T,
        }
    }
}

impl<T: ?Sized> Dma<T> {
    pub fn physical(&self) -> usize {
        self.phys.address()
    }
    pub fn size(&self) -> usize {
        self.phys.size()
    }
    pub fn phys(&self) -> &PhysBox {
        &self.phys
    }
}

impl<T> Dma<[T]> {
    pub fn from_physbox_uninit_unsized(phys: PhysBox, len: usize) -> Result<Dma<[MaybeUninit<T>]>> {
        let max_len = phys.size() / mem::size_of::<T>();
        assert!(len <= max_len);
        let d = Dma {
            virt: unsafe {
                slice::from_raw_parts_mut(
                    crate::physmap(phys.address, phys.size, crate::PHYSMAP_WRITE)?
                        as *mut MaybeUninit<T>,
                    len,
                )
            },
            phys,
        };
        Ok(d)
    }

    pub fn from_physbox_zeroed_unsized(phys: PhysBox, len: usize) -> Result<Dma<[MaybeUninit<T>]>> {
        let this = Self::from_physbox_uninit_unsized(phys, len)?;
        unsafe { ptr::write_bytes(this.virt as *mut MaybeUninit<u8>, 0, this.phys.size()) }
        Ok(this)
    }

    pub unsafe fn zeroed_unsized(count: usize) -> Result<Self> {
        let phys = PhysBox::new(mem::size_of::<T>() * count)?;
        Ok(Self::from_physbox_zeroed_unsized(phys, count)?.assume_init())
    }
}

impl<T> Dma<[MaybeUninit<T>]> {
    pub unsafe fn assume_init(self) -> Dma<[T]> {
        let &Dma {
            phys: PhysBox { address, size },
            virt,
        } = &self;
        mem::forget(self);
        Dma {
            phys: PhysBox { address, size },
            virt: virt as *mut [T],
        }
    }
}

impl<T: ?Sized> Deref for Dma<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.virt }
    }
}

impl<T: ?Sized> DerefMut for Dma<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.virt }
    }
}

impl<T: ?Sized> Drop for Dma<T> {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.virt) }
        let _ = unsafe { crate::physunmap(self.virt as *mut u8 as usize) };
    }
}

