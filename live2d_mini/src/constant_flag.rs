#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Live2DConstantFlag(live2d_mini_sys::csmFlags);

impl Live2DConstantFlag {
    #[inline]
    pub fn is_csm_blend_additive(&self) -> bool {
        self.0 as u32 & live2d_mini_sys::csmBlendAdditive == live2d_mini_sys::csmBlendAdditive
    }

    #[inline]
    pub fn is_csm_blend_multiplicative(&self) -> bool {
        self.0 as u32 & live2d_mini_sys::csmBlendMultiplicative
            == live2d_mini_sys::csmBlendMultiplicative
    }

    /// trueは両面描画=カリングなし
    #[inline]
    pub fn is_csm_is_double_sided(&self) -> bool {
        self.0 as u32 & live2d_mini_sys::csmIsDoubleSided == live2d_mini_sys::csmIsDoubleSided
    }

    #[inline]
    pub fn is_csm_is_inverted_mask(&self) -> bool {
        self.0 as u32 & live2d_mini_sys::csmIsInvertedMask == live2d_mini_sys::csmIsInvertedMask
    }
}
