use crate::{Arch, MemoryArea, PageEntry, PhysicalAddress, VirtualAddress, X8664Arch, MEGA_BYTE};
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

    fn read<T>(&self, virt: VirtualAddress) -> T {
        let virt_data = virt.data();
        let size = mem::size_of::<T>();
        if (virt_data & A::PAGE_ADDRESS_MASK) != ((virt_data + (size - 1)) & A::PAGE_ADDRESS_MASK) {
            panic!(
                "read: 0x{:X} size 0x {:X} passes page boundary",
                virt_data, size,
            );
        }
        if let Some((phys, _flags)) = self.translate(virt) {
            self.read_phys(phys)
        } else {
            panic!("read: 0x{:X} size 0x{:X} not present", virt_data, size);
        }
    }

    fn write<T>(&mut self, virt: VirtualAddress, value: T) {
        let virt_data = virt.data();
        let size = mem::size_of::<T>();
        if (virt_data & A::PAGE_ADDRESS_MASK) != ((virt_data + (size - 1)) & A::PAGE_ADDRESS_MASK) {
            panic!(
                "read: 0x{:X} size 0x {:X} passes page boundary",
                virt_data, size,
            );
        }
        if let Some((phys, flags)) = self.translate(virt) {
            if flags & A::ENTRY_FLAG_WRITABLE != 0 {
                self.write_phys(phys, value);
            } else {
                panic!("write: 0x{:X} size 0x{:X} not writable", virt_data, size);
            }
        } else {
            panic!("write: 0x{:X} size 0x{:X} not present", virt_data, size);
        }
    }

    fn write_bytes(&mut self, virt: VirtualAddress, value: u8, count: usize) {
        let virt_data = virt.data();
        if (virt_data & A::PAGE_ADDRESS_MASK) != ((virt_data + (count - 1)) & A::PAGE_ADDRESS_MASK)
        {
            panic!(
                "write_bytes: 0x{:X} size 0x {:X} passes page boundary",
                virt_data, count,
            )
        }
        if let Some((phys, flags)) = self.translate(virt) {
            if flags & A::ENTRY_FLAG_WRITABLE != 0 {
                self.write_phys_bytes(phys, value, count);
            } else {
                panic!(
                    "write_bytes: 0x{:X} count 0x{:X} not writable",
                    virt_data, count
                );
            }
        } else {
            panic!(
                "write_bytes: 0x{:X} count 0x{:X} not present",
                virt_data, count
            );
        }
    }
    fn invalid_data(&mut self, _address: VirtualAddress) {
        unimplemented!("EmulateArch::invalid_data not implemented");
    }

    fn get_table(&self) -> PhysicalAddress {
        self.table_addr
    }
    fn set_table(&mut self, address: PhysicalAddress) {
        self.table_addr = address;
        self.invalid_data_all();
    }

    fn invalid_data_all(&mut self) {
        self.map.clear();
        let a4 = self.table_addr.data();
        for i4 in 0..A::PAGE_ENTRIES {
            let e3 = self.read_phys::<usize>(PhysicalAddress::new(a4 + i4 * A::PAGE_ENTRY_SIZE));
            let f3 = e3 & A::ENTRY_FLAGS_MASK;
            if f3 & A::ENTRY_FLAG_PRESENT == 0 {
                continue;
            }
            let a3 = e3 & A::ENTRY_ADDRESS_MASK;
            for i3 in 0..A::PAGE_ENTRIES {
                let e2 =
                    self.read_phys::<usize>(PhysicalAddress::new(a3 + i3 * A::PAGE_ENTRY_SIZE));
                let f2 = e2 & A::ENTRY_FLAGS_MASK;
                if f2 & A::ENTRY_FLAG_PRESENT == 0 {
                    continue;
                }
                let a2 = e2 & A::ENTRY_ADDRESS_MASK;
                for i2 in 0..A::PAGE_ENTRIES {
                    let e1 =
                        self.read_phys::<usize>(PhysicalAddress::new(a2 + i2 * A::PAGE_ENTRY_SIZE));
                    let f1 = e1 & A::ENTRY_FLAGS_MASK;
                    if f1 & A::ENTRY_FLAG_PRESENT == 0 {
                        continue;
                    }
                    let a1 = e1 & A::ENTRY_ADDRESS_MASK;
                    for i1 in 0..A::PAGE_ENTRIES {
                        let e = self
                            .read_phys::<usize>(PhysicalAddress::new(a1 + i1 * A::PAGE_ENTRY_SIZE));
                        let f = e & A::ENTRY_FLAGS_MASK;
                        if f & A::ENTRY_FLAG_PRESENT == 0 {
                            continue;
                        }
                        let page = (i4 << 39) | (i3 << 30) | (i2 << 21) | (i1 << 12);
                        self.map
                            .insert(VirtualAddress::new(page), PageEntry::new(e));
                    }
                }
            }
        }
    }
}

const MEMORY_SIZE: usize = 64 * MEGA_BYTE;
static MEMORY_AREAS: [MemoryArea; 2] = [
    MemoryArea {
        base: PhysicalAddress::new(EmulateArch::PAGE_SIZE * 4),
        size: MEMORY_SIZE / 2 - EmulateArch::PAGE_SIZE * 4,
    },
    MemoryArea {
        base: PhysicalAddress::new(MEMORY_SIZE / 2),
        size: MEMORY_SIZE / 2,
    },
];
static mut MACHINE: Option<Machine<EmulateArch>> = None;

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
        let mut machine = Machine::new(MEMORY_SIZE);
        let pm14 = 0;
        let pdp = pm14 + Self::PAGE_SIZE;
        let flags = Self::ENTRY_FLAG_WRITABLE | Self::ENTRY_FLAG_PRESENT;
        machine.write_phys::<usize>(
            PhysicalAddress::new(pm14 + 256 * Self::PAGE_ENTRY_SIZE),
            pdp | flags,
        );
        let pd = pdp + Self::PAGE_SIZE;
        machine.write_phys::<usize>(PhysicalAddress::new(pdp), pd | flags);
        let pt = pd + Self::PAGE_SIZE;
        machine.write_phys::<usize>(PhysicalAddress::new(pd), pt | flags);

        for i in 0..Self::PAGE_ENTRIES {
            let page = i * Self::PAGE_SIZE;
            machine.write_phys::<usize>(
                PhysicalAddress::new(pt + i * Self::PAGE_ENTRY_MASK),
                page | flags,
            )
        }
        MACHINE = Some(machine);
        EmulateArch::set_table(PhysicalAddress::new(pm14));
        &MEMORY_AREAS
    }

    unsafe fn read<T>(address: VirtualAddress) -> T {
        MACHINE.as_ref().unwrap().read(address)
    }

    unsafe fn write<T>(address: VirtualAddress, value: T) {
        MACHINE.as_mut().unwrap().write(address, value);
    }

    unsafe fn write_bytes(address: VirtualAddress, value: u8, count: usize) {
        MACHINE.as_mut().unwrap().write_bytes(address, value, count);
    }

    unsafe fn invalid_data_all() {
        MACHINE.as_mut().unwrap().invalid_data_all();
    }

    unsafe fn invalid_data(address: VirtualAddress) {
        MACHINE.as_mut().unwrap().invalid_data(address);
    }

    unsafe fn table() -> PhysicalAddress {
        MACHINE.as_mut().unwrap().get_table()
    }

    unsafe fn set_table(address: PhysicalAddress) {
        MACHINE.as_mut().unwrap().set_table(address);
    }
}
