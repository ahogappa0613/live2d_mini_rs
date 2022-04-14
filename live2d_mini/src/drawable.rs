use std::{ffi::CStr, os::raw::c_char};

use crate::constant_flag::*;
use crate::dynamic_flag::*;
use crate::model::*;
use crate::vector2::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Live2DDrawable<'a> {
    id: &'a *const c_char,
    constant_flag: &'a Live2DConstantFlag,
    dynamic_flag: &'a Live2DDynamicFlag,
    texture_index: &'a i32,
    draw_order: &'a i32,
    render_order: &'a i32,
    opacitiy: &'a f32,
    mask_count: &'a i32,
    vertex_count: &'a i32,
    index_count: &'a i32,

    masks: &'a *const i32,
    indices: &'a *const u16,
    vertex_positions: &'a *const Live2DVector2,
    vertex_uvs: &'a *const Live2DVector2,
}

impl<'a> Live2DDrawable<'a> {
    #[inline]
    pub fn id(&self) -> &str {
        unsafe { CStr::from_ptr(*self.id).to_str().expect("id error") }
    }

    #[inline]
    pub fn constant_flag(&self) -> &Live2DConstantFlag {
        self.constant_flag
    }

    #[inline]
    pub fn dynamic_flag(&self) -> &Live2DDynamicFlag {
        self.dynamic_flag
    }

    #[inline]
    pub fn texture_index(&self) -> &i32 {
        self.texture_index
    }

    #[inline]
    pub fn draw_order(&self) -> &i32 {
        self.draw_order
    }

    #[inline]
    pub fn render_order(&self) -> &i32 {
        self.render_order
    }

    #[inline]
    pub fn opacitiy(&self) -> &f32 {
        self.opacitiy
    }

    #[inline]
    pub fn mask_count(&self) -> &i32 {
        self.mask_count
    }

    #[inline]
    pub fn vertex_count(&self) -> &i32 {
        self.vertex_count
    }

    #[inline]
    pub fn index_count(&self) -> &i32 {
        self.index_count
    }

    #[inline]
    pub fn masks(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(*self.masks, *self.mask_count() as usize) }
    }

    #[inline]
    pub fn indices(&self) -> Option<&[u16]> {
        if *self.index_count() == 0 {
            return None;
        }

        unsafe {
            Some(std::slice::from_raw_parts(
                *self.indices,
                *self.index_count() as usize,
            ))
        }
    }

    #[inline]
    pub fn vertex_positions(&self) -> &[Live2DVector2] {
        unsafe { std::slice::from_raw_parts(*self.vertex_positions, *self.vertex_count() as usize) }
    }

    #[inline]
    pub fn vertex_uvs(&self) -> &[Live2DVector2] {
        unsafe { std::slice::from_raw_parts(*self.vertex_uvs, *self.vertex_count() as usize) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Live2DDrawableIter<'a> {
    pub(crate) pos: usize,
    pub(crate) len: usize,

    pub(crate) inner: &'a Live2DModel,
}

impl<'a> Iterator for Live2DDrawableIter<'a> {
    type Item = Live2DDrawable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            return None;
        } else {
            let pos = self.pos;

            self.pos += 1;

            unsafe {
                Some(Live2DDrawable {
                    id: self.inner.csm_get_drawable_ids().get_unchecked(pos),
                    constant_flag: self
                        .inner
                        .csm_get_drawable_constant_flags()
                        .get_unchecked(pos),
                    texture_index: self
                        .inner
                        .csm_get_drawable_texture_indices()
                        .get_unchecked(pos),
                    draw_order: self.inner.csm_get_drawable_draw_orders().get_unchecked(pos),
                    opacitiy: self.inner.csm_get_drawable_opacities().get_unchecked(pos),
                    mask_count: self.inner.csm_get_drawable_mask_counts().get_unchecked(pos),
                    vertex_count: self
                        .inner
                        .csm_get_drawable_vertex_counts()
                        .get_unchecked(pos),
                    index_count: self
                        .inner
                        .csm_get_drawable_index_counts()
                        .get_unchecked(pos),
                    render_order: self
                        .inner
                        .csm_get_drawable_render_orders()
                        .get_unchecked(pos),
                    dynamic_flag: self
                        .inner
                        .csm_get_drawable_dynamic_flags()
                        .get_unchecked(pos),

                    masks: self.inner.csm_get_drawable_masks().get_unchecked(pos),
                    indices: self.inner.csm_get_drawable_indices().get_unchecked(pos),
                    vertex_positions: self
                        .inner
                        .csm_get_drawable_vertex_positions()
                        .get_unchecked(pos),
                    vertex_uvs: self.inner.csm_get_drawable_vertex_uvs().get_unchecked(pos),
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Live2DSortedDrawableIter<'a> {
    pub(crate) sorted_indices: Vec<usize>,

    pub(crate) inner: &'a Live2DModel,
}

impl<'a> Iterator for Live2DSortedDrawableIter<'a> {
    type Item = Live2DDrawable<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.sorted_indices.pop() {
            Some(index) => unsafe {
                Some(Live2DDrawable {
                    id: self.inner.csm_get_drawable_ids().get_unchecked(index),
                    constant_flag: self
                        .inner
                        .csm_get_drawable_constant_flags()
                        .get_unchecked(index),
                    texture_index: self
                        .inner
                        .csm_get_drawable_texture_indices()
                        .get_unchecked(index),
                    draw_order: self
                        .inner
                        .csm_get_drawable_draw_orders()
                        .get_unchecked(index),
                    opacitiy: self.inner.csm_get_drawable_opacities().get_unchecked(index),
                    mask_count: self
                        .inner
                        .csm_get_drawable_mask_counts()
                        .get_unchecked(index),
                    vertex_count: self
                        .inner
                        .csm_get_drawable_vertex_counts()
                        .get_unchecked(index),
                    index_count: self
                        .inner
                        .csm_get_drawable_index_counts()
                        .get_unchecked(index),
                    render_order: self
                        .inner
                        .csm_get_drawable_render_orders()
                        .get_unchecked(index),
                    dynamic_flag: self
                        .inner
                        .csm_get_drawable_dynamic_flags()
                        .get_unchecked(index),

                    masks: self.inner.csm_get_drawable_masks().get_unchecked(index),
                    indices: self.inner.csm_get_drawable_indices().get_unchecked(index),
                    vertex_positions: self
                        .inner
                        .csm_get_drawable_vertex_positions()
                        .get_unchecked(index),
                    vertex_uvs: self
                        .inner
                        .csm_get_drawable_vertex_uvs()
                        .get_unchecked(index),
                })
            },
            None => return None,
        }
    }
}
