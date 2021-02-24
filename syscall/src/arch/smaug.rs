use super::error::{Error, Result, ENOSYS};


pub unsafe fn syscall_0(_a: usize) -> Result<usize> {
    Err(Error::new(ENOSYS))
}

pub unsafe fn syscall_1(_a: usize, _b: usize) -> Result<usize> {
    Err(Error::new(ENOSYS))
}
pub unsafe fn syscall_2(_a: usize, _b: usize, _c: usize) -> Result<usize> {
    Err(Error::new(ENOSYS))
}
pub unsafe fn syscall_3(_a: usize, _b: usize, _c: usize, _d: usize) -> Result<usize> {
    Err(Error::new(ENOSYS))
}
pub unsafe fn syscall_4(_a: usize, _b: usize, _c: usize, _d: usize, _e: usize) -> Result<usize> {
    Err(Error::new(ENOSYS))
}

pub unsafe fn syscall_5(
    _a: usize,
    _b: usize,
    _c: usize,
    _d: usize,
    _e: usize,
    _f: usize,
) -> Result<usize> {
    Err(Error::new(ENOSYS))
}
