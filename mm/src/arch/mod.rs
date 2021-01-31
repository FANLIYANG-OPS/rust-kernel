use core::ptr;

use crate::{MemoryArea, PhysicalAddress, VirtualAddress};

mod x86_64;
pub use self::x86_64::X8664Arch;
mod emulate;
pub use self::emulate::EmulateArch;

pub trait Arch: Clone + Copy {
    /// page最大长度 = 12 (x86中一般为12)
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
    /// page_size 页长 1 << 12 也就是 2^12 = 4096
    const PAGE_SIZE: usize = 1 << Self::PAGE_SHIFT;
    const PAGE_OFFSET_MASK: usize = Self::PAGE_SIZE - 1;
    /// 最大寻址地址，x64中一般为2^64，实际中用不到这么多，取2^48
    const PAGE_ADDRESS_SHIFT: usize = Self::PAGE_LEVELS * Self::PAGE_ENTRY_SHIFT + Self::PAGE_SHIFT;
    /// 可以用到的最大物理地址数量 2^48
    const PAGE_ADDRESS_SIZE: usize = 1 << Self::PAGE_ADDRESS_SHIFT;
    /// 一个page为2^12,总共有2^48,不知道减去page有什么用？ 2^48-2^12 = 2^36
    const PAGE_ADDRESS_MASK: usize = Self::PAGE_ADDRESS_SIZE - Self::PAGE_SIZE;
    /// 一个虚拟地址前3位表示page_number,后9位表示他的偏移量，2^(12-9)，算出page_number的最大值
    const PAGE_ENTRY_SIZE: usize = 1 << (Self::PAGE_SHIFT - Self::PAGE_ENTRY_SHIFT);
    /// page的最大偏移量 2^9
    const PAGE_ENTRIES: usize = 1 << Self::PAGE_ENTRY_SHIFT;
    /// page的最大容量
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
