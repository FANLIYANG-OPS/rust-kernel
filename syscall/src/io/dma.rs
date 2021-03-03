use crate::PartialAllocStrategy;
use crate::PhysallocFlags;
use crate::Result;
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
