use drowsed_math::linear::Transform;

pub struct BasicObject<T: Transform> {
    pub transform: T,
}
impl<T: Transform> BasicObject<T> {
    pub fn transform(&self) -> T {
        self.transform
    }
}
impl<T: Transform> BasicObject<T> {
    pub fn new(t: T) -> Self {
        Self { transform: t }
    }
}