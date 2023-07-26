#![allow(unused)]
use std::{collections::HashMap, sync::Arc};

use yum_mocha::vk_obj::{self, device::ReplacingDevice};

use crate::motor::scene_manager::SceneManager;

pub struct PhysicsSystem<R: mofongo::bodies::RigidBody> {
    global_gravity: R::SpatialVector,
    rigidbodies: HashMap<i128, R>
}

impl<R: mofongo::bodies::RigidBody> PhysicsSystem<R> {
    pub fn new(gravity: R::SpatialVector) -> Self {
        Self { global_gravity: gravity, rigidbodies: HashMap::new() }
    }
    pub fn set_gravity(&mut self, global_gravity: R::SpatialVector) {
        self.global_gravity = global_gravity;
    }
    pub fn push(&mut self, id: i128, rigidbody: R) {
        self.rigidbodies.insert(id, rigidbody);
    }
    pub fn render(&mut self, device: Arc<ReplacingDevice>, deltatime: R::DeltaTime, scenemanager: &mut SceneManager<R::Transformation>) {
        let scene = scenemanager.get_selected_scene_mut();
        for (id, rigidbody) in &mut self.rigidbodies {
            let object = scene.get_object_by_id_mut(*id).unwrap();
            object.transform = rigidbody.step(deltatime, self.global_gravity, &object.transform);
        }
    }
    pub fn get_rigidbody_by_id(&self, id: i128) -> Option<&R> {
        self.rigidbodies.get(&id)
    }
    pub fn get_rigidbody_by_id_mut(&mut self, id: i128) -> Option<&mut R> {
        self.rigidbodies.get_mut(&id)
    }
}