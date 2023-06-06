use drowsed_math::{linear::{FVec2, FMat2, vector::Vector3, FVec3, FMat4}, complex::quaternion::Quaternion};


#[derive(Clone, Copy)]
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
        let mat_rot = FMat2::from_vec(FVec2::new(cos_rot, sin_rot), FVec2::new(-sin_rot, cos_rot));
        
        let mat_scale = FMat2::from_vec(FVec2::new(self.scale.x, 0.0), FVec2::new(0.0, self.scale.y));
        return mat_rot * mat_scale;
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        let translation = FVec2::new(0.0, 0.0);
        let scale = FVec2::new(1.0, 1.0);
        let rotation = 0.0;
        Self { translation, scale, rotation }
    }
}
#[derive(Clone, Copy)]
pub struct Transform3D {
    pub translation: FVec3,
    pub scale: FVec3,
    pub rotation: Vector3<f32>,
}
impl Transform3D {
    #[inline]
    fn new(translation: FVec3, scale: FVec3, rotation: Vector3<f32>) -> Self {
        Self { translation, scale, rotation }
    }
    pub fn mat4(&self) -> FMat4 {
        // let mut transform = FMat4::from_translation(self.translation);
        let mut transform = FMat4::default();
        transform.x.x = 1.0;
        transform.y.y = 1.0;
        transform.z.z = 1.0;
        transform.w.w = 1.0;
        transform = drowsed_math::linear::translate(&transform, self.translation);
        // let rotation = FMat4::from(self.rotation);

        transform = drowsed_math::linear::rotate(&transform, self.rotation.y, FVec3::new(0.0, 1.0, 0.0));
        transform = drowsed_math::linear::rotate(&transform, self.rotation.x, FVec3::new(1.0, 0.0, 0.0));
        transform = drowsed_math::linear::rotate(&transform, self.rotation.z, FVec3::new(0.0, 0.0, 1.0));
        let scale = FMat4::from_scale(self.scale);

        transform = transform * scale;
        transform
    }
    
}

#[derive(Clone, Copy)]
pub struct TransformQuaternion3D {
    pub translation: FVec3,
    pub scale: FVec3,
    pub rotation: Quaternion<f32>,
}
impl TransformQuaternion3D {
    #[inline]
    fn new(translation: FVec3, scale: FVec3, rotation: Quaternion<f32>) -> Self {
        Self { translation, scale, rotation }
    }
    pub fn mat4(&self) -> FMat4 {
        // let mut transform = FMat4::from_translation(self.translation);
        let mut transform = FMat4::default();
        transform.x.x = 1.0;
        transform.y.y = 1.0;
        transform.z.z = 1.0;
        transform.w.w = 1.0;
        transform = drowsed_math::linear::translate(&transform, self.translation);

        let rotation = FMat4::from(self.rotation);

        transform = transform * rotation;

        let scale = FMat4::from_scale(self.scale);

        transform = transform * scale;
        transform
    }
}