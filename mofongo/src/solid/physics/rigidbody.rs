use drowsed_math::{Transform, FVec3, FMat3, TransformQuaternion3D, complex::quaternion::Quaternion, Vector, EuclideanGeometry, SquareMatrix};

use crate::bodies::RigidBody;

pub struct RigidBody3D {
    pub velocity: FVec3,
    pub angular_velocity: FVec3,
    pub mass: f32,
    net_force: FVec3,
    pub net_torque: FVec3,
    pub inertia: FMat3
}

impl RigidBody for RigidBody3D {
    type DeltaTime = f32;
    type Mass = f32;
    type SpatialVector = FVec3;
    type Transformation = TransformQuaternion3D;
    fn angular_velocity(&self) -> Self::SpatialVector {
        self.angular_velocity
    }
    fn velocity(&self) -> Self::SpatialVector {
        self.velocity
    }
    fn apply_force(&mut self, force: Self::SpatialVector, pos: Self::SpatialVector) {
        self.net_force += force;
        self.apply_torque(pos.cross(force));
    }
    fn apply_torque(&mut self, torque: Self::SpatialVector) {
        self.net_torque += torque;
    }
    fn mass(&self) -> Self::Mass {
        self.mass
    }
    fn step(&mut self, deltatime: f32, gravity: Self::SpatialVector, transform: &Self::Transformation) -> Self::Transformation {
        // apply gravity
        self.net_force += gravity * self.mass;
        self.velocity += self.net_force / self.mass * deltatime;

        // get angular velocity 
        self.angular_velocity += self.net_torque * deltatime * self.inertia;

        let vec = if self.angular_velocity == 0.0 {
            FVec3::new(0.0, 0.0, 1.0)
        } else {
            self.angular_velocity.normalize()
        };
        let rotation = Quaternion::<f32>::angle_axis(
            self.angular_velocity.length() * deltatime, 
            vec);
        
        // end
        self.net_force = FVec3::from(0.0);
        self.net_torque = FVec3::from(0.0);
        let mut transform = *transform;
        transform.rotation = transform.rotation * rotation;
        transform.translate(&(self.velocity * deltatime))
        // transform
    }
}

impl RigidBody3D {
    pub fn new(mass: f32) -> Self {
        Self { 
            velocity: FVec3::from(0.0), 
            angular_velocity: FVec3::from(0.0), 
            net_force: FVec3::from(0.0), 
            net_torque: FVec3::from(0.0), 
            mass, 
            inertia: FMat3::identity() 
        }
    }
}