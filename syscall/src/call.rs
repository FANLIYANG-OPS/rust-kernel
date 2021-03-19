use crate::TimeSpec;

use super::arch::*;
use super::data::*;
use super::error::Result;
use super::flag::*;
use super::number::*;
use core::{mem, ptr};

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

pub fn f_chmod(fb: usize, mode: u16) -> Result<usize> {
    unsafe { syscall_2(SYS_FCHMOD, fb, mode as usize) }
}

pub fn f_chown(fd: usize, uid: u32, gid: u32) -> Result<usize> {
    unsafe { syscall_3(SYS_FCHOWN, fd, uid as usize, gid as usize) }
}

pub fn f_cntl(fd: usize, cmd: usize, arg: usize) -> Result<usize> {
    unsafe { syscall_3(SYS_FCNTL, fd, cmd, arg) }
}

pub fn f_exec(fd: usize, args: &[[usize; 2]], vars: &[[usize; 2]]) -> Result<usize> {
    unsafe {
        syscall_5(
            SYS_FEXEC,
            fd,
            args.as_ptr() as usize,
            args.len(),
            vars.as_ptr() as usize,
            vars.len(),
        )
    }
}

pub unsafe fn f_map(fd: usize, map: &Map) -> Result<usize> {
    syscall_3(
        SYS_FMAP,
        fd,
        map as *const Map as usize,
        mem::size_of::<Map>(),
    )
}

pub unsafe fn f_unmap(addr: usize, len: usize) -> Result<usize> {
    syscall_2(SYS_FUNMAP, addr, len)
}

pub fn f_path(fd: usize, buf: &mut [u8]) -> Result<usize> {
    unsafe { syscall_3(SYS_FPATH, fd, buf.as_mut_ptr() as usize, buf.len()) }
}

pub fn f_rename<T: AsRef<[u8]>>(fd: usize, path: T) -> Result<usize> {
    unsafe {
        syscall_3(
            SYS_FRENAME,
            fd,
            path.as_ref().as_ptr() as usize,
            path.as_ref().len(),
        )
    }
}

pub fn f_stat(fd: usize, stat: &mut Stat) -> Result<usize> {
    unsafe {
        syscall_3(
            SYS_FSTAT,
            fd,
            stat as *mut Stat as usize,
            mem::size_of::<Stat>(),
        )
    }
}

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

pub fn sched_yield() -> Result<usize> {
    unsafe { syscall_0(SYS_YIELD) }
}

pub fn write(fd: usize, buf: &[u8]) -> Result<usize> {
    unsafe { syscall_3(SYS_WRITE, fd, buf.as_ptr() as usize, buf.len()) }
}

pub fn wait_pid(pid: usize, status: &mut usize, options: WaitFlags) -> Result<usize> {
    unsafe {
        syscall_3(
            SYS_WAITPID,
            pid,
            status as *mut usize as usize,
            options.bits(),
        )
    }
}

pub unsafe fn virt_tophys(virtual_address: usize) -> Result<usize> {
    syscall_1(SYS_VIRTTOPHYS, virtual_address)
}

pub fn unlink<T: AsRef<[u8]>>(path: T) -> Result<usize> {
    unsafe {
        syscall_2(
            SYS_UNLINK,
            path.as_ref().as_ptr() as usize,
            path.as_ref().len(),
        )
    }
}

pub fn u_mask(mask: usize) -> Result<usize> {
    unsafe { syscall_1(SYS_UMASK, mask) }
}

pub fn sig_proc_mask(
    how: usize,
    set: Option<&[u64; 2]>,
    oldset: Option<&mut [u64; 2]>,
) -> Result<usize> {
    unsafe {
        syscall_3(
            SYS_SIGPROCMASK,
            how,
            set.map(|x| x as *const _).unwrap_or_else(ptr::null) as usize,
            oldset.map(|x| x as *mut _).unwrap_or_else(ptr::null_mut) as usize,
        )
    }
}

pub fn sigaction(
    sig: usize,
    act: Option<&SigAction>,
    oldact: Option<&mut SigAction>,
) -> Result<usize> {
    unsafe {
        syscall_4(
            SYS_SIGACTION,
            sig,
            act.map(|x| x as *const _).unwrap_or_else(ptr::null) as usize,
            oldact.map(|x| x as *mut _).unwrap_or_else(ptr::null_mut) as usize,
            restorer as usize,
        )
    }
}

pub fn set_reuid(ruid: usize, euid: usize) -> Result<usize> {
    unsafe { syscall_2(SYS_SETREUID, ruid, euid) }
}

pub fn set_rens(rns: usize, ens: usize) -> Result<usize> {
    unsafe { syscall_2(SYS_SETRENS, rns, ens) }
}

pub fn set_regid(rgid: usize, egid: usize) -> Result<usize> {
    unsafe { syscall_2(SYS_SETREGID, rgid, egid) }
}

pub fn set_pgid(pid: usize, pgid: usize) -> Result<usize> {
    unsafe { syscall_2(SYS_SETPGID, pid, pgid) }
}

pub fn rmdir<T: AsRef<[u8]>>(path: T) -> Result<usize> {
    unsafe {
        syscall_2(
            SYS_RMDIR,
            path.as_ref().as_ptr() as usize,
            path.as_ref().len(),
        )
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> Result<usize> {
    unsafe { syscall_3(SYS_READ, fd, buf.as_mut_ptr() as usize, buf.len()) }
}

pub fn pipe2(fds: &mut [usize; 2], flags: usize) -> Result<usize> {
    unsafe { syscall_2(SYS_PIPE2, fds.as_ptr() as usize, flags) }
}

pub fn open<T: AsRef<[u8]>>(path: T, flags: usize) -> Result<usize> {
    unsafe {
        syscall_3(
            SYS_OPEN,
            path.as_ref().as_ptr() as usize,
            path.as_ref().len(),
            flags,
        )
    }
}

pub fn nanosleep(req: &TimeSpec, rem: &mut TimeSpec) -> Result<usize> {
    unsafe {
        syscall_2(
            SYS_NANOSLEEP,
            req as *const TimeSpec as usize,
            rem as *mut TimeSpec as usize,
        )
    }
}

pub unsafe fn mprotect(addr: usize, size: usize, flags: MapFlags) -> Result<usize> {
    syscall_3(SYS_MPROTECT, addr, size, flags.bits())
}

