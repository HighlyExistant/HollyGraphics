// use drowsed_math::{FMat2, FVec2, Transform2D, EuclideanGeometry, Vector, Transform};

// use crate::bodies::RigidBody;

// pub struct RigidBody2D {
//     pub velocity: FVec2,
//     pub angular_velocity: FVec2,
//     pub mass: f32,
//     net_force: FVec2,
//     pub net_torque: f32,
//     pub inertia: FMat2
// }

// impl RigidBody for RigidBody2D {
//     type DeltaTime = f32;
//     type Mass = f32;
//     type SpatialVector = FVec2;
//     type Transformation = Transform2D;
//     fn angular_velocity(&self) -> Self::SpatialVector {
//         self.angular_velocity
//     }
//     fn velocity(&self) -> Self::SpatialVector {
//         self.velocity
//     }
//     fn apply_force(&mut self, force: Self::SpatialVector, pos: Self::SpatialVector) {
//         self.net_force += force;
//         let cross = pos.cross(force);
//         self.apply_torque(cross);
//     }
//     fn apply_torque(&mut self, torque: f32) {
//         self.net_torque += torque;
//     }
//     fn mass(&self) -> Self::Mass {
//         self.mass
//     }
//     fn step(&mut self, deltatime: f32, gravity: Self::SpatialVector, transform: &Self::Transformation) -> Self::Transformation {
//         // apply gravity
//         self.net_force += gravity * self.mass;
//         self.velocity += self.net_force / self.mass * deltatime;

//         // get angular velocity 
//         self.angular_velocity += self.net_torque * deltatime;

//         let vec = if self.angular_velocity == 0.0 {
//             FVec2::new(0.0, 0.0)
//         } else {
//             self.angular_velocity.normalize()
//         };
//         let rotation = self.angular_velocity.length() * deltatime;
        
//         // end
//         self.net_force = FVec2::from(0.0);
//         self.net_torque = 0.0;
        
//         transform.rotate(rotation).translate(self.velocity * deltatime)
//         // transform
//     }
// }