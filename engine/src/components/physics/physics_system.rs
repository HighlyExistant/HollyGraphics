use std::{collections::HashMap, sync::Arc};

use drowsed_math::linear::FVec3;
use yum_mocha::vk_obj;

use crate::components::scene::Scene;

use super::rigidbody::{RigidBody, self};

pub struct PhysicsSystem {
    global_gravity: FVec3,
    rigidbodies: HashMap<i128, RigidBody>
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self { global_gravity: FVec3::new(0.0, 9.81, 0.0), rigidbodies: HashMap::new() }
    }
    pub fn set_gravity(&mut self, global_gravity: FVec3) {
        self.global_gravity = global_gravity;
    }
    pub fn push(&mut self, id: i128, rigidbody: RigidBody) {
        self.rigidbodies.insert(id, rigidbody);
    }
    pub fn render_all(&mut self, device: Arc<vk_obj::device::Device>, deltatime: f32, scene: &mut Scene) {
        for (id, rigidbody) in &mut self.rigidbodies {
            let object = scene.get_object_by_id_mut(*id).unwrap();
            object.transform = rigidbody.step(deltatime, self.global_gravity, &object.transform);
        }
    }
    pub fn get_rigidbody_by_id(&self, id: i128) -> Option<&RigidBody> {
        self.rigidbodies.get(&id)
    }
    pub fn get_rigidbody_by_id_mut(&mut self, id: i128) -> Option<&mut RigidBody> {
        self.rigidbodies.get_mut(&id)
    }
}