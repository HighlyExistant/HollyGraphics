use drowsed_math::{Vector, Number, Transform, EuclideanGeometry, TransformMatrix};
use num_traits::MulAddAssign;

pub trait RigidBody {
    type SpatialVector: Vector + EuclideanGeometry;
    type Mass: Number;
    type DeltaTime: Number + MulAddAssign<Self::Mass>;
    type Transformation: TransformMatrix<f32>;
    fn velocity(&self) -> Self::SpatialVector;
    fn angular_velocity(&self) -> Self::SpatialVector;
    fn mass(&self) -> Self::Mass;
    fn apply_force(&mut self, force: Self::SpatialVector, pos: Self::SpatialVector);
    fn apply_torque(&mut self, torque: <Self::SpatialVector as EuclideanGeometry>::CrossProduct);
    fn step(&mut self, deltatime: Self::DeltaTime, gravity: Self::SpatialVector, transform: &Self::Transformation) -> Self::Transformation;
}