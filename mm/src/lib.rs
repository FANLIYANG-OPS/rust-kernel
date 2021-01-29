#![cfg_attr(not(feature = "std"), no_std)]
#![feature(asm)]
#![feature(const_fn)]

pub use crate::allocator::*;
mod allocator;
pub use crate::arch::*;
mod arch;

pub const KILO_BYTE: usize = 1024;
pub const MEGA_BYTE: usize = KILO_BYTE * KILO_BYTE;
pub const GIGA_BYTE: usize = MEGA_BYTE * KILO_BYTE;
pub const TERA_BYTE: usize = GIGA_BYTE * KILO_BYTE;

/// 物理内存地址
/// 告诉编译器，想C一样进行内存布局
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    #[inline(always)]
    pub const fn new(address: usize) -> Self {
        Self(address)
    }
    #[inline(always)]
    pub fn data(&self) -> usize {
        self.0
    }
    #[inline(always)]
    pub fn add(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

/// 虚拟内存地址
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    #[inline(always)]
    pub const fn new(address: usize) -> Self {
        Self(address)
    }
    #[inline(always)]
    pub fn data(&self) -> usize {
        self.0
    }
    #[inline(always)]
    pub fn add(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

/// 存储块
#[derive(Clone, Copy, Debug)]
pub struct MemoryArea {
    pub base: PhysicalAddress,
    pub size: usize,
}
