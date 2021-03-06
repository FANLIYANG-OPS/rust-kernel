use core::marker::PhantomData;

use crate::Io;

#[derive(Copy, Clone)]
pub struct Pio<T> {
    port: u16,
    value: PhantomData<T>,
}

impl<T> Pio<T> {
    pub const fn new(port: u16) -> Self {
        Pio::<T> {
            port: port,
            value: PhantomData,
        }
    }
}

impl Io for Pio<u8> {
    type Value = u8;
    #[inline(always)]
    fn read(&self) -> u8 {
        let value: u8;
        unsafe {
            llvm_asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }
    #[inline(always)]
    fn write(&mut self, value: u8) {
        unsafe {
            llvm_asm!("out $1,$0"::"{al}"(value),"{dx}"(self.port):"memory":"intel","volatile");
        }
    }
}

impl Io for Pio<u16> {
    type Value = u16;
    #[inline(always)]
    fn read(&self) -> u16 {
        let value: u16;
        unsafe {
            llvm_asm!("in $0, $1" : "={ax}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u16) {
        unsafe {
            llvm_asm!("out $1,$0"::"{ax}"(value),"{dx}"(self.port):"memory":"intel","volatile");
        }
    }
}

impl Io for Pio<u32> {
    type Value = u32;
    #[inline(always)]
    fn read(&self) -> u32 {
        let value: u32;
        unsafe {
            llvm_asm!("in $0, $1" : "={eax}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    #[inline(always)]
    fn write(&mut self, value: u32) {
        unsafe {
            llvm_asm!("out $1,$0"::"{eax}"(value),"{dx}"(self.port):"memory":"intel","volatile");
        }
    }
    
}


