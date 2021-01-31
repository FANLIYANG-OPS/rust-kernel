use crate::PhysicalAddress;

pub use self::buddy::*;
pub use self::bump::*;
mod buddy;
mod bump;

/// 页框大小
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct FrameCount(usize);

impl FrameCount {
    pub fn new(count: usize) -> Self {
        Self(count)
    }
    pub fn data(&self) -> usize {
        self.0
    }
}

/// 页框分配情况
pub struct FrameUsage {
    used: FrameCount,
    total: FrameCount,
}

impl FrameUsage {
    pub fn new(used: FrameCount, total: FrameCount) -> Self {
        Self { used, total }
    }
    pub fn used(&self) -> FrameCount {
        self.used
    }
    pub fn tatal(&self) -> FrameCount {
        self.total
    }

    pub fn free(&self) -> FrameCount {
        FrameCount(self.total.0 - self.used.0)
    }
}

/// 页帧分配器 trait
pub trait FrameAllocator {
    /// 分配物理内存
    unsafe fn allocate(&mut self, count: FrameCount) -> Option<PhysicalAddress>;
    /// 释放已经分配的物理内存
    unsafe fn free(&mut self, address: PhysicalAddress, count: FrameCount);
    /// 分配一块物理内存地址
    unsafe fn allocate_one(&mut self) -> Option<PhysicalAddress> {
        self.allocate(FrameCount::new(1))
    }
    /// 释放一块物理内存地址
    unsafe fn free_one(&mut self, address: PhysicalAddress) {
        self.free(address, FrameCount::new(1));
    }
    /// 内存分配情况
    unsafe fn usage(&self) -> FrameUsage;
}

