use super::arch::*;
use super::error::Result;
use super::flag::*;
use super::number::*;

pub unsafe fn physmap(physical_address: usize, size: usize, flags: PhysmapFlags) -> Result<usize> {
    syscall_3(SYS_PHYSMAP, physical_address, size, flags.bits())
}

pub unsafe fn physalloc_0(size: usize) -> Result<usize> {
    syscall_1(SYS_PHYSALLOC, size)
}

pub unsafe fn physalloc_2(size: usize, flags: usize) -> Result<usize> {
    let mut ret = 1usize;
    physalloc_3(size, flags, &mut ret)
}

pub unsafe fn physalloc_3(size: usize, flags: usize, min: &mut usize) -> Result<usize> {
    syscall_3(SYS_PHYSALLOC3, size, flags, min as *mut usize as usize)
}

pub unsafe fn physfree(physical_address: usize, size: usize) -> Result<usize> {
    syscall_2(SYS_PHYSFREE, physical_address, size)
}
