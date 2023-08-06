use drowsed_math::Vector;

pub struct RepRigidBody<V: Vector> {
    linear_velocity: V,
    rotational_velocity: V,
    mass: f32,
    density: f32,
    restitution: f32,
    area: f32,
}

