use std::alloc::{dealloc, Layout};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Live2DAddress {
    pub(crate) ptr: *mut u8,
    pub(crate) layout: Layout,
}

impl Drop for Live2DAddress {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr, self.layout) };
    }
}
