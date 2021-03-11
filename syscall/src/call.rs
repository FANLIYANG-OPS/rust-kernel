use crate::TimeSpec;

use super::arch::*;
use super::error::Result;
use super::flag::*;
use super::number::*;

extern "C" fn restorer() -> ! {
    sigreturn().unwrap();
    unreachable!();
}

pub fn chdir<T: AsRef<[u8]>>(path: T) -> Result<usize> {
    unsafe {
        syscall_2(
            SYS_CHDIR,
            path.as_ref().as_ptr() as usize,
            path.as_ref().len(),
        )
    }
}

#[deprecated(since = "0.1.55", note = "use fchmod instead")]
pub fn chmod<T: AsRef<[u8]>>(path: T, mode: usize) -> Result<usize> {
    unsafe {
        syscall_3(
            SYS_CHMOD,
            path.as_ref().as_ptr() as usize,
            path.as_ref().len(),
            mode,
        )
    }
}

pub unsafe fn clone(flags: CloneFlags) -> Result<usize> {
    syscall_1(SYS_CLONE, flags.bits())
}

pub fn close(fd: usize) -> Result<usize> {
    unsafe { syscall_1(SYS_CLOSE, fd) }
}

pub fn clock_gettime(clock: usize, tp: &mut TimeSpec) -> Result<usize> {
    unsafe { syscall_2(SYS_CLOCK_GETTIME, clock, tp as *mut TimeSpec as usize) }
}

pub fn dup(fb: usize, buf: &[u8]) -> Result<usize> {
    unsafe { syscall_3(SYS_DUP, fb, buf.as_ptr() as usize, buf.len()) }
}

pub fn dup_2(fb: usize, new_fb: usize, buf: &[u8]) -> Result<usize> {
    unsafe { syscall_4(SYS_DUP2, fb, new_fb, buf.as_ptr() as usize, buf.len()) }
}

pub fn exit(status: usize) -> Result<usize> {
    unsafe { syscall_1(SYS_EXIT, status) }
}

pub fn fchmod(fb: usize, mode: u16) -> Result<usize> {
    unsafe { syscall_2(SYS_FCHMOD, fb, mode as usize) }
}

pub fn fchown(fd: usize, uid: u32, gid: u32) -> Result<usize> {
    unsafe { syscall_3(SYS_FCHOWN, fd, uid as usize, gid as usize) }
}

pub fn fcntl(fd: usize, cmd: usize, arg: usize) -> Result<usize> {
    unsafe { syscall_3(SYS_FCNTL, fd, cmd, arg) }
}

// todo

pub fn sigreturn() -> Result<usize> {
    unsafe { syscall_0(SYS_SIGRETURN) }
}

pub unsafe fn physmap(physical_address: usize, size: usize, flags: PhysmapFlags) -> Result<usize> {
    syscall_3(SYS_PHYSMAP, physical_address, size, flags.bits())
}

pub unsafe fn physunmap(virtual_address: usize) -> Result<usize> {
    syscall_1(SYS_PHYSUNMAP, virtual_address)
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
