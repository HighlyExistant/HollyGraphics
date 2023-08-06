use drowsed_math::{FVec3, TransformQuaternion3D, Triangle, FVec2, Transform2D};

use crate::{solid::physics::rigidbody::RigidBody3D, flat::GJKColliderFlat, collider::Collider};

mod solid;
mod flat;
pub mod collider;
pub mod bodies;

fn main() {
    let t0 = Transform2D {
        translation: FVec2::new(0.4, 7.5),
        ..Default::default()
    };
    // let x = RigidBody3D::new(1.0);
    let poly1 = GJKColliderFlat::new(vec![FVec2::new(1.0, 3.0), FVec2::new(2.0, 6.0), FVec2::new(6.0, 2.0)]);
    let poly2 = GJKColliderFlat::new(vec![FVec2::new(-10.0, 10.0), FVec2::new(10.0, -10.0), FVec2::new(12.0, 10.0), FVec2::new(-10.0, -10.0)]);

    // println!("{:?}", poly2.collision(&Transform2D::default(), &poly1, &t0));
}
