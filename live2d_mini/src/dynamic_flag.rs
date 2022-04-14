#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Live2DDynamicFlag(live2d_mini_sys::csmFlags);

impl Live2DDynamicFlag {
    #[inline]
    pub fn is_csm_is_visible(&self) -> bool {
        if (self.0 as u32 & live2d_mini_sys::csmIsVisible) == live2d_mini_sys::csmIsVisible {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_csm_visibility_did_change(&self) -> bool {
        if (self.0 as u32 & live2d_mini_sys::csmVisibilityDidChange)
            == live2d_mini_sys::csmVisibilityDidChange
        {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_csm_opacity_did_change(&self) -> bool {
        if (self.0 as u32 & live2d_mini_sys::csmOpacityDidChange)
            == live2d_mini_sys::csmOpacityDidChange
        {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_csm_draw_order_did_change(&self) -> bool {
        if (self.0 as u32 & live2d_mini_sys::csmDrawOrderDidChange)
            == live2d_mini_sys::csmDrawOrderDidChange
        {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn is_csm_vertex_positions_did_change(&self) -> bool {
        if (self.0 as u32 & live2d_mini_sys::csmVertexPositionsDidChange)
            == live2d_mini_sys::csmVertexPositionsDidChange
        {
            true
        } else {
            false
        }
    }
}
