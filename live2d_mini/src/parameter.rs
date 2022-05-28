use std::{ffi::CStr, os::raw::c_char};

use crate::model_resource::Live2DModelResource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Live2DParameter<'a> {
    id: &'a *const c_char,
    minimum_value: &'a f32,
    maximum_value: &'a f32,
    default_value: &'a f32,
}

impl<'a> Live2DParameter<'a> {
    #[inline]
    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(*self.id).to_str().expect("id error") }
    }

    #[inline]
    pub fn minimum_value(&self) -> &f32 {
        self.minimum_value
    }

    #[inline]
    pub fn maximum_value(&self) -> &f32 {
        self.maximum_value
    }

    #[inline]
    pub fn default_value(&self) -> &f32 {
        self.default_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Live2DParameterIter<'a> {
    pub(crate) pos: usize,
    pub(crate) len: usize,

    pub(crate) inner: &'a Live2DModelResource,
}

impl<'a> Iterator for Live2DParameterIter<'a> {
    type Item = Live2DParameter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        } else {
            self.pos += 1;

            unsafe {
                Some(Live2DParameter {
                    id: self
                        .inner
                        .csm_get_parameter_ids()
                        .get_unchecked(self.pos - 1),
                    minimum_value: self
                        .inner
                        .csm_get_parameter_minimum_values()
                        .get_unchecked(self.pos - 1),
                    maximum_value: self
                        .inner
                        .csm_get_parameter_maximum_values()
                        .get_unchecked(self.pos - 1),
                    default_value: self
                        .inner
                        .csm_get_parameter_default_values()
                        .get_unchecked(self.pos - 1),
                })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Live2DParameterMut<'a> {
    id: &'a *const c_char,
    pub minimum_value: &'a f32,
    pub maximum_value: &'a f32,
    pub default_value: &'a f32,
    pub value: &'a mut f32,
}

impl<'a> Live2DParameterMut<'a> {
    #[inline]
    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(*self.id).to_str().expect("id error") }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Live2DParameterIterMut<'a> {
    pub(crate) pos: usize,
    pub(crate) len: usize,

    pub(crate) inner: &'a Live2DModelResource,
}

impl<'a> Iterator for Live2DParameterIterMut<'a> {
    type Item = Live2DParameterMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        } else {
            self.pos += 1;

            unsafe {
                Some(Live2DParameterMut {
                    id: self
                        .inner
                        .csm_get_parameter_ids()
                        .get_unchecked(self.pos - 1),
                    minimum_value: self
                        .inner
                        .csm_get_parameter_minimum_values()
                        .get_unchecked(self.pos - 1),
                    maximum_value: self
                        .inner
                        .csm_get_parameter_maximum_values()
                        .get_unchecked(self.pos - 1),
                    default_value: self
                        .inner
                        .csm_get_parameter_default_values()
                        .get_unchecked(self.pos - 1),
                    value: self
                        .inner
                        .csm_get_parameter_values()
                        .get_unchecked_mut(self.pos - 1),
                })
            }
        }
    }
}
