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
        transform = Self::translate(transform, self.translation);
        // let rotation = FMat4::from(self.rotation);

        transform = Self::rotate(transform, self.rotation.y, FVec3::new(0.0, 1.0, 0.0));
        transform = Self::rotate(transform, self.rotation.x, FVec3::new(1.0, 0.0, 0.0));
        transform = Self::rotate(transform, self.rotation.z, FVec3::new(0.0, 0.0, 1.0));
        let scale = FMat4::from_scale(self.scale);

        transform = transform * scale;
        // println!("matrix: {:?}", transform);
        transform
    }
    
    pub fn rotate(m: FMat4, angle: f32, v: FVec3) -> FMat4 {
        let a = angle;
		let c = a.cos();
		let s = a.sin();

		let axis = v.normalize();
		let temp: FVec3 = (axis * (1.0 - c) );

		let mut rotate = FMat4::default();
		rotate.x.x = c + temp.x * axis.x;
		rotate.x.y = temp.x * axis.y + s * axis.z;
		rotate.x.z = temp.x * axis.z - s * axis.y;

		rotate.y.x = temp.y * axis.x - s * axis.z;
		rotate.y.y = c + temp.y * axis.y;
		rotate.y.z = temp.y * axis.z + s * axis.x;

		rotate.z.x = temp.z * axis.x + s * axis.y;
		rotate.z.y = temp.z * axis.y - s * axis.x;
		rotate.z.z = c + temp.z * axis.z;

		let mut result = FMat4::default();
		result.x = m.x * rotate.x.x + m.y * rotate.x.y + m.z * rotate.x.z;
		result.y = m.x * rotate.y.x + m.y * rotate.y.y + m.z * rotate.y.z;
		result.z = m.x * rotate.z.x + m.y * rotate.z.y + m.z * rotate.z.z;
		result.w = m.w;
        result
    }
    pub fn translate(m: FMat4, v: FVec3) -> FMat4 {
        let mut result = m;
		result.w = m.x * v.x + m.y * v.y + m.z * v.z + m.w;
        result
    }
}
// impl Default for Transform3D {
//     fn default() -> Self {
//         let translation = FVec3 { x: 0.0, y: 0.0, z: 0.0, };
//         let scale = FVec3 { x: 1.0, y: 1.0, z: 1.0, };
//         let rotation = FQuaternion::new(0.0, 0.0, 0.0, 0.0);
//         Self { translation, scale, rotation }
//     }
// }