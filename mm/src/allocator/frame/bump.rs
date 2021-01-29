use core::marker::PhantomData;

use crate::{FrameAllocator, FrameCount, FrameUsage, MemoryArea, PhysicalAddress};

pub struct BumpAllocator<A> {
    areas: &'static [MemoryArea],
    // 偏移量
    offset: usize,
    phantom: PhantomData<A>,
}
