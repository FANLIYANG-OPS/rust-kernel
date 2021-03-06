use crate::error::*;
use crate::flag::*;
use core::cmp;
use core::convert::TryFrom;

pub fn calc_seek_offset_usize(
    cur_offset: usize,
    pos: isize,
    whence: usize,
    buf_len: usize,
) -> Result<isize> {
    let cur_offset = isize::try_from(cur_offset).or_else(|_| Err(Error::new(EOVERFLOW)))?;
    let buf_len = isize::try_from(buf_len).or_else(|_| Err(Error::new(EOVERFLOW)))?;
    calc_seek_offset_isize(cur_offset, pos, whence, buf_len)
}

pub fn calc_seek_offset_isize(
    cur_offset: isize,
    pos: isize,
    whence: usize,
    buf_len: isize,
) -> Result<isize> {
    let new_offset = match whence {
        SEEK_CUR => pos.checked_add(cur_offset),
        SEEK_END => pos.checked_add(buf_len),
        SEEK_SET => Some(pos),
        _ => None,
    };
    match new_offset {
        Some(new_offset) if new_offset < 0 => Err(Error::new(EINVAL)),
        Some(new_offset) => Ok(cmp::min(new_offset, buf_len)),
        None => Err(Error::new(EOVERFLOW)),
    }
}
