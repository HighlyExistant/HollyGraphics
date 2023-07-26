// use drowsed_math::{linear::{FVec3, Transform, TransformQuaternion3D, FMat3}, complex::quaternion::Quaternion, Vector};
// #[derive(Clone, Copy)]
// pub struct RigidBody {
//     pub velocity: FVec3,
//     pub angular_velocity: FVec3,
//     pub force: FVec3,
//     pub net_force: FVec3,
//     pub net_torque: FVec3,
//     pub mass: f32,
//     pub inertia: FMat3
// }

// impl RigidBody {
//     pub fn new(mass: f32) -> Self{
//         Self { velocity: FVec3::from(0.0), angular_velocity: FVec3::from(0.0), force: FVec3::from(0.0), net_force: FVec3::from(0.0), net_torque: FVec3::from(0.0), mass, inertia: FMat3::identity(1.0) }
//     }
//     pub fn apply_force(&mut self, force: FVec3, pos: FVec3) {
//         self.net_force += force;
//         self.apply_torque(pos.cross(force));
//     }
//     fn apply_torque(&mut self, torque: FVec3) {
//         self.net_torque += torque;
//     }
//     pub fn step<T: Transform>(&mut self, deltatime: f32, gravity: FVec3, transform: &TransformQuaternion3D) -> TransformQuaternion3D {
//         // apply gravity
//         self.net_force += gravity * self.mass;
//         self.velocity += self.net_force / self.mass * deltatime;

//         // get angular velocity 
//         self.angular_velocity += self.net_torque * deltatime * self.inertia;

//         let vec = if self.angular_velocity == 0.0 {
//             FVec3::new(0.0, 0.0, 1.0)
//         } else {
//             self.angular_velocity.normalize()
//         };
//         let rotation = Quaternion::<f32>::angle_axis(
//             self.angular_velocity.length() * deltatime, 
//             vec);

//         // end
//         self.net_force = FVec3::from(0.0);
//         self.net_torque = FVec3::from(0.0);
        
//         transform.rotate(rotation).translate(self.velocity * deltatime)
//         // transform
//     }
// }