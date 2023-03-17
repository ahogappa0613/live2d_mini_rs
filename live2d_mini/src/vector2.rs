#[derive(Debug, Clone, Copy)]
pub struct Live2DVector2(pub(crate) live2d_mini_sys::csmVector2);
impl Live2DVector2 {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.X
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0.Y
    }
}
