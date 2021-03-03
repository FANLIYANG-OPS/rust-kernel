use super::arch::*;
use super::error::Result;
use super::number::*;

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
