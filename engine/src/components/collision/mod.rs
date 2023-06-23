use drowsed_math::linear::{vector::Vector3, FVec3};
use yum_mocha::vk_obj::rendering::mesh::{Mesh, VulkanIndexable, Vertex};

use self::point::gjk;
mod point;
mod simplex;
pub trait Collider {
    fn check_collision(&self, collider: &dyn Collider) -> (bool, Option<(FVec3, f32)>);
    fn vertices(&self) -> Vec<FVec3>;
    fn indices(&self) -> Vec<u32>;
}

pub struct GJK {
    vertices: Vec<FVec3>,
    indices: Vec<u32>,
}

impl Collider for GJK {
    fn vertices(&self) -> Vec<FVec3> {
        self.vertices.clone()
    }
    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
    fn check_collision(&self, collider: &dyn Collider) -> (bool, Option<(FVec3, f32)>) {
        gjk(&self.vertices, &collider.vertices())
    }
}