use core::mem::MaybeUninit;
use core::ops::{BitAnd, BitOr, Not};
use core::ptr::{read_volatile, write_volatile};

use crate::Io;
#[repr(packed)]
pub struct Mmio<T> {
    value: MaybeUninit<T>,
}

impl<T> Mmio<T> {
    #[deprecated = "unsound because it's possible to read even though it's uninitialized"]
    pub fn new() -> Self {
        unsafe { Self::uninit() }
    }

    pub unsafe fn zeroed() -> Self {
        Self {
            value: MaybeUninit::zeroed(),
        }
    }

    pub unsafe fn uninit() -> Self {
        Self {
            value: MaybeUninit::uninit(),
        }
    }

    pub const fn from(value: T) -> Self {
        Self {
            value: MaybeUninit::new(value),
        }
    }
}

impl<T> Io for Mmio<T>
where
    T: Copy + PartialEq + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T>,
{
    type Value = T;

    fn read(&self) -> Self::Value {
        unsafe { read_volatile(self.value.as_ptr()) }
    }
    fn write(&mut self, value: Self::Value) {
        unsafe { write_volatile(self.value.as_mut_ptr(), value) }
    }
}
