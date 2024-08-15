use std::{alloc::{alloc, dealloc, Layout}, marker::PhantomData};

use crate::buffer::Buffer;

pub struct CoherentAlloc<'ca> {
    layout: Layout,
    free: usize,
    block: *mut u8,
    _marker: PhantomData<&'ca ()>,
}
impl CoherentAlloc<'ca> {
    pub fn new(align: usize, size: usize) -> Self {
        let layout = Layout::array::<u8>(size).unwrap().align_to(align).unwrap();
        let block = unsafe {
            alloc(layout)
        };

        Self {
            layout,
            free: layout.size(),
            block,
            _marker: PhantomData,
        }
    }
    pub fn alloc(&mut self, size: usize) -> Buffer<'ca> {
        Buffer::Allocated { len: (), vec: () }
    }
}
impl Drop for CoherentAlloc {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.block, self.layout);
        }
    }
}
