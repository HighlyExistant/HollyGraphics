use std::{collections::HashMap, cell::{Cell, RefCell}, rc::Rc};

use drowsed_math::{linear::Transform, Vector};
use mofongo::collider::{Collider, CollisionInfo};
use yum_mocha::vk_obj::rendering::mesh::{VulkanIndexable, Vertex};

use crate::components::{rendering::rendersystem::RenderSystem, collisions::collision_system::CollisionSystem, physics::{physics_system::PhysicsSystem, self}, scene::Scene, self, object::BasicObject};

use super::scene_manager::SceneManager;

pub struct SystemManager<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform> {
    pub scene_manager: SceneManager<T>,
    pub rendering: RenderSystem<V, I>,
    pub collisions: CollisionSystem<T, E>,
    pub physics: PhysicsSystem,
}

impl<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform> SystemManager<V, E, I, T> {
    pub fn new() -> Self {
        let collisions = components::collisions::collision_system::CollisionSystem::new();
        let rendering = RenderSystem::<V, I>::default();
        let physics = physics::physics_system::PhysicsSystem::new();
        Self { rendering, collisions, physics, scene_manager: SceneManager::new() }
    }
    pub fn is_object_present(&self, id: &i128) -> bool {
        self.scene_manager.objects.contains_key(id)
    }
    pub fn get_object(&self, id: &i128) -> Option<&BasicObject<T>> {
        self.scene_manager.objects.get(id)
    }
    pub fn get_object_mut(&mut self, id: &i128) -> Option<&mut BasicObject<T>> {
        self.scene_manager.objects.get_mut(id)
    }
    pub fn get_rigidbody(&self, id: &i128) -> Option<&physics::rigidbody::RigidBody> {
        self.physics.get_rigidbody_by_id(*id)
    }
    pub fn get_rigidbody_mut(&mut self, id: &i128) -> Option<&mut physics::rigidbody::RigidBody> {
        self.physics.get_rigidbody_by_id_mut(*id)
    }
    pub fn get_collider(&self, id: &i128) -> Option<&(Rc<RefCell<dyn Collider<ColliderLayoutVertex = E, TransformComponent = T>>>, Cell<Option<CollisionInfo<E>>>)> {
        self.collisions.get_collider_by_id(*id)
    }
}