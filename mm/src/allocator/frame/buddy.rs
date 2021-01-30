use core::{marker::PhantomData, mem};

use alloc::alloc::realloc;

use crate::{
    allocator, arch, Arch, BumpAllocator, FrameAllocator, PhysicalAddress, VirtualAddress,
};

#[repr(transparent)]
struct BuddyUsage(u8);

#[repr(packed)]
struct BuddyEntry<A> {
    base: PhysicalAddress,
    size: usize,
    skip: usize,
    used: usize,
    phantom: PhantomData<A>,
}

impl<A> Clone for BuddyEntry<A> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            size: self.size,
            skip: self.skip,
            used: self.used,
            phantom: PhantomData,
        }
    }
}

impl<A: Arch> BuddyEntry<A> {
    fn empty() -> Self {
        Self {
            base: PhysicalAddress::new(0),
            size: 0,
            skip: 0,
            used: 0,
            phantom: PhantomData,
        }
    }
    #[inline(always)]
    fn pages(&self) -> usize {
        self.size >> A::PAGE_SHIFT
    }
    fn usage_pages(&self) -> usize {
        let bytes = self.pages() * mem::size_of::<BuddyUsage>();
        (bytes + A::PAGE_OFFSET_MASK) >> A::PAGE_SHIFT
    }
    unsafe fn usage_addr(&self, page: usize) -> Option<VirtualAddress> {
        if page < self.pages() {
            let phys = self.base.add(page * mem::size_of::<BuddyUsage>());
            Some(A::phys_to_virt(phys))
        } else {
            None
        }
    }
    unsafe fn usage(&self, page: usize) -> Option<BuddyUsage> {
        let addr = self.usage_addr(page)?;
        Some(A::read(addr))
    }
    unsafe fn set_usage(&self, page: usize, usage: BuddyUsage) -> Option<()> {
        let addr = self.usage_addr(page)?;
        Some(A::write(addr, usage))
    }
}

pub struct BuddyAllocator<A> {
    table_virt: VirtualAddress,
    phantom: PhantomData<A>,
}

impl<A: Arch> BuddyAllocator<A> {
    const BUDDY_ENTRIES: usize = A::PAGE_SIZE / mem::size_of::<BuddyEntry<A>>();
    pub unsafe fn new(mut bump_allocator: BumpAllocator<A>) -> Option<Self> {
        let table_phys = bump_allocator.allocate_one()?;
        let table_virt = A::phys_to_virt(table_phys);
        for i in 0..(A::PAGE_SIZE / mem::size_of::<BuddyEntry<A>>()) {
            let virt = table_virt.add(i * mem::size_of::<BuddyEntry<A>>());
            A::write(virt, BuddyEntry::<A>::empty());
        }
        let allocator = Self {
            table_virt,
            phantom: PhantomData,
        };
        let mut offset = bump_allocator.offset();
        for old_area in bump_allocator.areas().iter() {
            let mut area = old_area.clone();
            if offset >= area.size {
                offset -= area.size;
                continue;
            } else if offset > 0 {
                area.base = area.base.add(offset);
                area.size -= offset;
                offset = 0;
            }
            for i in 0..(A::PAGE_SIZE / mem::size_of::<BuddyEntry<A>>()) {
                let virt = table_virt.add(i * mem::size_of::<BuddyEntry<A>>());
                let mut entry = A::read::<BuddyEntry<A>>(virt);
                let inserted = if area.base.add(area.size) == entry.base {
                    entry.base = area.base;
                    entry.size = area.size;
                    true
                } else if area.base == entry.base.add(entry.size) {
                    entry.size += area.size;
                    true
                } else if entry.size == 0 {
                    entry.base = area.base;
                    entry.size = area.size;
                    true
                } else {
                    false
                };
                if inserted {
                    A::write(virt, entry);
                    break;
                }
            }
        }

        for i in 0..Self::BUDDY_ENTRIES {
            let virt = table_virt.add(i * mem::size_of::<BuddyEntry<A>>());
            let mut entry = A::read::<BuddyEntry<A>>(virt);
            let usage_pages = entry.usage_pages();
            if entry.pages() > usage_pages {
                let usage_start = entry.usage_addr(0)?;
                for page in 0..usage_pages {
                    A::write_bytes(usage_start.add(page << A::PAGE_SHIFT), 0, A::PAGE_SIZE);
                }
                for page in 0..usage_pages {
                    entry.set_usage(page, BuddyUsage(1))?;
                }
            }
            entry.skip = usage_pages;
            entry.used = usage_pages;
            A::write(virt, entry)
        }
        Some(allocator)
    }
}

impl<A: Arch> FrameAllocator for BuddyAllocator<A> {
    unsafe fn allocate(&mut self, count: crate::FrameCount) -> Option<PhysicalAddress> {
        todo!()
    }

    unsafe fn free(&mut self, address: PhysicalAddress, count: crate::FrameCount) {
        todo!()
    }

    unsafe fn usage(&self) -> crate::FrameUsage {
        todo!()
    }
}
