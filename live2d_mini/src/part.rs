use std::{ffi::CStr, os::raw::c_char};

use crate::model_resource::Live2DModelResource;

#[derive(Debug, PartialEq)]
pub struct Live2DPart<'a> {
    id: &'a *const c_char,
    opacitiy: &'a f32,
    parent_part_index: &'a i32,
}

impl<'a> Live2DPart<'a> {
    #[inline]
    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(*self.id).to_str().expect("id error") }
    }

    #[inline]
    pub fn opacitiy(&self) -> &f32 {
        self.opacitiy
    }

    #[inline]
    pub fn parent_part_index(&self) -> &i32 {
        self.parent_part_index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Live2DPartIter<'a> {
    pub(crate) pos: usize,
    pub(crate) len: usize,

    pub(crate) inner: &'a Live2DModelResource,
}

impl<'a> Iterator for Live2DPartIter<'a> {
    type Item = Live2DPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        } else {
            self.pos += 1;

            unsafe {
                Some(Live2DPart {
                    id: self.inner.csm_get_part_ids().get_unchecked(self.pos - 1),
                    opacitiy: self
                        .inner
                        .csm_get_part_opacities()
                        .get_unchecked(self.pos - 1),
                    parent_part_index: self
                        .inner
                        .csm_get_part_parent_part_indices()
                        .get_unchecked(self.pos - 1),
                })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Live2DPartMut<'a> {
    id: &'a *const c_char,
    pub opacitiy: &'a mut f32,
    pub parent_part_index: &'a i32,
}
impl<'a> Live2DPartMut<'a> {
    #[inline]
    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(*self.id).to_str().expect("id error") }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Live2DPartIterMut<'a> {
    pub(crate) pos: usize,
    pub(crate) len: usize,

    pub(crate) inner: &'a Live2DModelResource,
}

impl<'a> Iterator for Live2DPartIterMut<'a> {
    type Item = Live2DPartMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        } else {
            self.pos += 1;

            unsafe {
                Some(Live2DPartMut {
                    id: self.inner.csm_get_part_ids().get_unchecked(self.pos - 1),
                    opacitiy: self
                        .inner
                        .csm_get_part_opacities()
                        .get_unchecked_mut(self.pos - 1),
                    parent_part_index: self
                        .inner
                        .csm_get_part_parent_part_indices()
                        .get_unchecked(self.pos - 1),
                })
            }
        }
    }
}
