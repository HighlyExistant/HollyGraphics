use crate::lin_alg::f32::{FVec2, FMat2};
pub struct Transform2D {
    pub translation: FVec2,
    pub scale: FVec2,
    pub rotation: f32,
}

impl Transform2D {
    #[inline]
    fn new(translation: FVec2, scale: FVec2, rotation: f32) -> Self {
        Self { translation, scale, rotation }
    }
    pub fn mat2(&self) -> FMat2 {
        let sin_rot = self.rotation.sin();
        let cos_rot = self.rotation.cos();
        let mat_rot = FMat2::from_fvec2(FVec2::new(cos_rot, sin_rot), FVec2::new(sin_rot, cos_rot));
        
        let mat_scale = FMat2::from_fvec2(FVec2::new(self.scale.x, 0.0), FVec2::new(0.0, self.scale.y));
        return mat_rot * mat_scale;
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        let translation = FVec2::ZERO;
        let scale = FVec2::ONE;
        let rotation = 0.0;
        Self { translation, scale, rotation }
    }
}