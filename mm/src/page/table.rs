use core::marker::PhantomData;

use crate::{Arch, PageEntry, PhysicalAddress, VirtualAddress};

pub struct PageTable<A> {
    base: VirtualAddress,
    phys: PhysicalAddress,
    level: usize,
    phontom: PhantomData<A>,
}

impl<A: Arch> PageTable<A> {
    pub unsafe fn new(base: VirtualAddress, phys: PhysicalAddress, level: usize) -> Self {
        Self {
            base,
            phys,
            level,
            phontom: PhantomData,
        }
    }
    pub unsafe fn top() -> Self {
        Self::new(VirtualAddress::new(0), A::table(), A::PAGE_LEVELS - 1)
    }
    pub fn base(&self) -> VirtualAddress {
        self.base
    }
    pub fn phys(&self) -> PhysicalAddress {
        self.phys
    }
    pub fn level(&self) -> usize {
        self.level
    }
    pub unsafe fn virt(&self) -> VirtualAddress {
        A::phys_to_virt(self.phys)
    }
    pub fn entry_base(&self, i: usize) -> Option<VirtualAddress> {
        if i < A::PAGE_ENTRIES {
            let level_shift = self.level() * A::PAGE_ENTRY_SHIFT + A::PAGE_SHIFT;
            Some(self.base.add(i << level_shift))
        } else {
            None
        }
    }
    pub unsafe fn entry_virt(&self, i: usize) -> Option<VirtualAddress> {
        if i < A::PAGE_ENTRIES {
            Some(self.virt().add(i * A::PAGE_ENTRY_SIZE))
        } else {
            None
        }
    }
    pub unsafe fn entry(&self, i: usize) -> Option<PageEntry<A>> {
        let addr = self.entry_virt(i)?;
        Some(PageEntry::new(A::read::<usize>(addr)))
    }
    pub unsafe fn set_entry(&mut self, i: usize, entry: PageEntry<A>) -> Option<()> {
        let addr = self.entry_virt(i)?;
        A::write::<usize>(addr, entry.data());
        Some(())
    }

    pub unsafe fn index_of(&self, address: VirtualAddress) -> Option<usize> {
        let address = VirtualAddress::new(address.data() & A::PAGE_ADDRESS_MASK);
        let level_shift = self.level * A::PAGE_ENTRY_SHIFT + A::PAGE_SHIFT;
        let level_mask = (A::PAGE_ENTRIES << level_shift) - 1;
        if address >= self.base() && address <= self.base().add(level_mask) {
            Some((address.data() >> level_shift) & A::PAGE_ENTRY_MASK)
        } else {
            None
        }
    }

    pub unsafe fn next(&self, i: usize) -> Option<Self> {
        if self.level() > 0 {
            let entry = self.entry(i)?;
            if entry.present() {
                return Some(PageTable::new(
                    self.entry_base(i)?,
                    entry.address(),
                    self.level - 1,
                ));
            }
        }
        None
    }
}
