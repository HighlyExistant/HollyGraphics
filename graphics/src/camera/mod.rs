use drowsed_math::linear::FMat4;

pub struct Camera {
    pub projection: FMat4,
}
impl Default for Camera{
    fn default() -> Self {
        let projection = FMat4::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        Self { projection: projection }
    }
}
impl Camera {
    pub fn new(mat: FMat4) -> Self{
        Self { projection: mat }
    }
    pub fn set_orthographic_projection(&mut self, left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) {
        self.projection = FMat4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);;
        self.projection.x.x = 2.0 / (right - left);
        self.projection.y.y = 2.0 / (bottom - top);
        self.projection.z.z = 1.0 / (far - near);
        self.projection.w.x = -(right + left) / (right - left);
        self.projection.w.y = -(bottom + top) / (bottom - top);
        self.projection.w.z = -near / (far - near);
    }
    pub fn set_perspective_projection(&mut self, fovy: f32, aspect: f32, near: f32, far: f32) {
        let tan_half_fovy = f32::tan(fovy / 2.0);
        self.projection = FMat4::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        self.projection.x.x = 1.0 / (aspect * tan_half_fovy);
        self.projection.y.y = 1.0 / (tan_half_fovy);
        self.projection.z.z = far / (far - near);
        self.projection.z.w = 1.0;
        self.projection.w.z = -(far * near) / (far - near);
    }
}