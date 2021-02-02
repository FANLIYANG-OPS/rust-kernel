use core::{marker::PhantomData, mem};

use crate::{Arch, VirtualAddress};

#[must_use = "the page table must be flushed"]
pub struct PageFlush<A> {
    virt: VirtualAddress,
    phontom: PhantomData<A>,
}

impl<A: Arch> PageFlush<A> {
    pub fn new(virt: VirtualAddress) -> Self {
        Self {
            virt,
            phontom: PhantomData,
        }
    }
    pub fn flush(self) {
        unsafe {
            A::invalid_data(self.virt);
        }
    }
    pub unsafe fn ignore(self) {
        mem::forget(self);
    }
}
#[must_use = "the page table must be flushed"]
pub struct PageFlushAll<A> {
    phontom: PhantomData<A>,
}

impl<A: Arch> PageFlushAll<A> {
    pub fn new() -> Self {
        Self {
            phontom: PhantomData,
        }
    }
    pub fn consume(&self, flush: PageFlush<A>) {
        unsafe {
            flush.ignore();
        }
    }
    pub fn flush(self) {
        unsafe {
            A::invalid_data_all();
        }
    }
    pub unsafe fn ignore(self) {
        mem::forget(self)
    }
}
