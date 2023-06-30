use drowsed_math::linear::{FVec3, Transform, TransformQuaternion3D};

use super::point::gjk;
#[derive(Debug, Default)]
pub struct CollisionInfo {
    pub normal: FVec3,
    pub depth: f32,
}
pub trait Collider3D {
    fn collision(&self, transform1: &TransformQuaternion3D, collider: &dyn Collider3D, transform2: &TransformQuaternion3D) -> Option<CollisionInfo>;
    fn collided(&self) -> bool;
    fn vertices(&self) -> &Vec<FVec3>;
}
pub struct GJKCollisions {
    pub vertices: Vec<FVec3>,
    pub colliding: bool,
}
impl GJKCollisions {
    pub fn new(vertices: Vec<FVec3>) -> Self {
        GJKCollisions { vertices, colliding: false }
    }
}
impl Collider3D for GJKCollisions {
    fn collision(&self, transform1: &TransformQuaternion3D, collider: &dyn Collider3D, transform2: &TransformQuaternion3D) -> Option<CollisionInfo> {
        let mat1 = transform1.matrix4();
        let mat2 = transform2.matrix4();
        gjk(&self.vertices, collider.vertices(), &mat1, &mat2)
    }
    fn vertices(&self) -> &Vec<FVec3> {
        &self.vertices
    }
    fn collided(&self) -> bool {
        self.colliding
    }
}