use core::ptr;

use crate::{MemoryArea, PhysicalAddress, VirtualAddress};

pub trait Arch: Clone + Copy {
    /// 页帧号 16进制的内存地址通常有两部分组成： 高位 + 低位
    /// 其中高位表示页号，低位表示偏移量，将低位偏移量去之不用，高位页号移动到右端，得到的结果为页帧号
    const PAGE_SHIFT: usize;
    const PAGE_ENTRY_SHIFT: usize;
    const PAGE_LEVELS: usize;
    const ENTRY_ADDRESS_SHIFT: usize;
    const ENTRY_FLAG_PRESENT: usize;
    const ENTRY_FLAG_WRITABLE: usize;
    const ENTRY_FLAG_USER: usize;
    const ENTRY_FLAG_HUGE: usize;
    const ENTRY_FLAG_GLOBAL: usize;
    const ENTRY_FLAG_NO_EXEC: usize;
    const PHYS_OFFSET: usize;
    const PAGE_SIZE: usize = 1 << Self::PAGE_SHIFT;
    const PAGE_OFFSET_MASK: usize = Self::PAGE_SIZE - 1;
    const PAGE_ADDRESS_SHIFT: usize = Self::PAGE_LEVELS * Self::PAGE_ENTRY_SHIFT + Self::PAGE_SHIFT;
    const PAGE_ADDRESS_SIZE: usize = 1 << Self::PAGE_ADDRESS_SHIFT;
    const PAGE_ADDRESS_MASK: usize = Self::PAGE_ADDRESS_SIZE - Self::PAGE_SIZE;
    const PAGE_ENTRY_SIZE: usize = 1 << (Self::PAGE_SHIFT - Self::PAGE_ENTRY_SHIFT);
    const PAGE_ENTRIES: usize = 1 << Self::PAGE_ENTRY_SHIFT;
    const PAGE_ENTRY_MASK: usize = Self::PAGE_ENTRIES - 1;
    const PAGE_NEGATIVE_MASK: usize = !(Self::PAGE_ADDRESS_SIZE - 1);
    const ENTRY_ADDRESS_SIZE: usize = 1 << Self::ENTRY_ADDRESS_SHIFT;
    const ENTRY_ADDRESS_MASK: usize = Self::ENTRY_ADDRESS_SIZE - Self::PAGE_SIZE;
    const ENTRY_FLAGS_MASK: usize = !Self::ENTRY_ADDRESS_MASK;
    unsafe fn init() -> &'static [MemoryArea];
    #[inline(always)]
    unsafe fn read<T>(address: VirtualAddress) -> T {
        ptr::read(address.data() as *const T)
    }
    #[inline(always)]
    unsafe fn write<T>(address: VirtualAddress, value: T) {
        ptr::write(address.data() as *mut T, value);
    }
    #[inline(always)]
    unsafe fn write_bytes(address: VirtualAddress, value: u8, count: usize) {
        ptr::write_bytes(address.data() as *mut u8, value, count)
    }
    unsafe fn invalid_data(address: VirtualAddress);
    unsafe fn table() -> PhysicalAddress;
    unsafe fn set_table(address: PhysicalAddress);
    #[inline(always)]
    unsafe fn invalid_data_all() {
        Self::set_table(Self::table());
    }
    #[inline(always)]
    unsafe fn phys_to_virt(phys: PhysicalAddress) -> VirtualAddress {
        VirtualAddress::new(phys.data() + Self::PHYS_OFFSET)
    }
}
