use drowsed_math::{Transform, TransformMatrix};

pub struct BasicObject<T: Transform + drowsed_math::TransformMatrix<f32>> {
    pub transform: T,
}
impl<T: Transform + drowsed_math::TransformMatrix<f32>> BasicObject<T> {
    pub fn transform(&self) -> T {
        self.transform
    }
}
impl<T: Transform + drowsed_math::TransformMatrix<f32>> BasicObject<T> {
    pub fn new(t: T) -> Self {
        Self { transform: t }
    }
}