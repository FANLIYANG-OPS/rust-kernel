use crate::{Arch, MemoryArea, PageEntry, PhysicalAddress, VirtualAddress, X8664Arch};
use core::{marker::PhantomData, mem, ptr};
use std::collections::BTreeMap;

struct Machine<A> {
    memory: Box<[u8]>,
    map: BTreeMap<VirtualAddress, PageEntry<A>>,
    table_addr: PhysicalAddress,
    phantom: PhantomData<A>,
}

impl<A: Arch> Machine<A> {
    fn new(memory_size: usize) -> Self {
        Self {
            memory: vec![0; memory_size].into_boxed_slice(),
            map: BTreeMap::new(),
            table_addr: PhysicalAddress::new(0),
            phantom: PhantomData,
        }
    }
    fn read_phys<T>(&self, phys: PhysicalAddress) -> T {
        let size = mem::size_of::<T>();
        if phys.add(size).data() <= self.memory.len() {
            unsafe { ptr::read(self.memory.as_ptr().add(phys.data()) as *const T) }
        } else {
            panic!(
                "read_phys: 0x{:X} size of 0x{:X} outside of memory",
                phys.data(),
                size
            );
        }
    }
    fn write_phys<T>(&mut self, phys: PhysicalAddress, value: T) {
        let size = mem::size_of::<T>();
        if phys.add(size).data() <= self.memory.len() {
            unsafe { ptr::write(self.memory.as_mut_ptr().add(phys.data()) as *mut T, value) };
        } else {
            panic!(
                "write_phys: 0x{:X} size 0x{:X} outside of memory",
                phys.data(),
                size
            );
        }
    }
    fn write_phys_bytes(&mut self, phys: PhysicalAddress, value: u8, count: usize) {
        if phys.add(count).data() <= self.memory.len() {
            unsafe {
                ptr::write_bytes(
                    self.memory.as_mut_ptr().add(phys.data()) as *mut u8,
                    value,
                    count,
                );
            }
        } else {
            panic!(
                "write_phys_bytes: 0x{:X} count 0x{:X} outside of memory",
                phys.data(),
                count
            );
        }
    }

    fn translate(&self, virt: VirtualAddress) -> Option<(PhysicalAddress, usize)> {
        let virt_data = virt.data();
        let page = virt_data & A::PAGE_ADDRESS_MASK;
        let offset = virt_data & A::PAGE_OFFSET_MASK;
        let entry = self.map.get(&VirtualAddress::new(page))?;
        Some((entry.address().add(offset), entry.flags()))
    }
}

#[derive(Clone, Copy)]
pub struct EmulateArch;

impl Arch for EmulateArch {
    const PAGE_SHIFT: usize = X8664Arch::PAGE_SHIFT;
    const PAGE_ENTRY_SHIFT: usize = X8664Arch::PAGE_ENTRY_SHIFT;
    const PAGE_LEVELS: usize = X8664Arch::PAGE_LEVELS;
    const ENTRY_ADDRESS_SHIFT: usize = X8664Arch::PAGE_ADDRESS_SHIFT;
    const ENTRY_FLAG_PRESENT: usize = X8664Arch::ENTRY_FLAG_PRESENT;
    const ENTRY_FLAG_WRITABLE: usize = X8664Arch::ENTRY_FLAG_WRITABLE;
    const ENTRY_FLAG_USER: usize = X8664Arch::ENTRY_FLAG_USER;
    const ENTRY_FLAG_HUGE: usize = X8664Arch::ENTRY_FLAG_HUGE;
    const ENTRY_FLAG_GLOBAL: usize = X8664Arch::ENTRY_FLAG_GLOBAL;
    const ENTRY_FLAG_NO_EXEC: usize = X8664Arch::ENTRY_FLAG_NO_EXEC;
    const PHYS_OFFSET: usize = X8664Arch::PHYS_OFFSET;

    unsafe fn init() -> &'static [MemoryArea] {
        todo!()
    }

    unsafe fn invalid_data(address: crate::VirtualAddress) {
        todo!()
    }

    unsafe fn table() -> crate::PhysicalAddress {
        todo!()
    }

    unsafe fn set_table(address: crate::PhysicalAddress) {
        todo!()
    }
}
