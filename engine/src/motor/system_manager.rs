#![allow(unused)]
use std::{cell::{Cell, RefCell}, rc::Rc};

use ash::vk;
use drowsed_math::{Transform, Vector};
use mofongo::collider::{Collider, CollisionInfo};
use yum_mocha::vk_obj::{rendering::mesh::{VulkanIndexable, Vertex}, device::LogicalDevice};

use crate::components::{rendering::rendersystem::RenderSystem, collisions::collision_system::CollisionSystem, physics::{physics_system::PhysicsSystem, self}, self, object::BasicObject};

use super::scene_manager::SceneManager;
pub struct SystemManagerInfo<V: Vector> {
    pub global_gravity: V,
}
pub struct SystemManager<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform, R: mofongo::bodies::RigidBody<Transformation = T>> {
    pub scene_manager: SceneManager<T>,
    pub rendering: RenderSystem<V, I, T>,
    pub collisions: CollisionSystem<T, E>,
    pub physics: PhysicsSystem<R>,
}

impl<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform, R: mofongo::bodies::RigidBody<Transformation = T>> SystemManager<V, E, I, T, R> {
    pub fn new(info: &SystemManagerInfo<R::SpatialVector>) -> Self {
        let collisions = components::collisions::collision_system::CollisionSystem::new();
        let rendering = RenderSystem::<V, I, T>::default();
        let physics = physics::physics_system::PhysicsSystem::new(info.global_gravity);
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
    pub fn get_rigidbody(&self, id: &i128) -> Option<&R> {
        self.physics.get_rigidbody_by_id(*id)
    }
    pub fn get_rigidbody_mut(&mut self, id: &i128) -> Option<&mut R> {
        self.physics.get_rigidbody_by_id_mut(*id)
    }
    pub fn get_collider(&self, id: &i128) -> Option<&(Rc<RefCell<dyn Collider<ColliderLayoutVertex = E, TransformComponent = T>>>, Cell<Option<CollisionInfo<E>>>)> {
        self.collisions.get_collider_by_id(*id)
    }
    pub fn render_collisions(&mut self, device: std::sync::Arc<LogicalDevice>, scenemanager: &SceneManager<T>) {
        self.collisions.render(device, scenemanager)
    }
    pub fn render_graphics(&mut self, device: std::sync::Arc<LogicalDevice>, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, scenemanager: &SceneManager<T>) {
        self.rendering.render(device, command_buffer, layout, scenemanager)
    }
    pub fn render_physics(&mut self, device: std::sync::Arc<LogicalDevice>, deltatime: R::DeltaTime, scenemanager: &mut SceneManager<T>) {
        self.physics.render(device, deltatime, scenemanager)
    }
    pub fn render_all(&mut self, device: std::sync::Arc<LogicalDevice>, deltatime: R::DeltaTime, command_buffer: vk::CommandBuffer, layout: vk::PipelineLayout, scenemanager: &mut SceneManager<T>) {
        self.render_collisions(device.clone(), scenemanager);
        self.render_graphics(device.clone(), command_buffer, layout, scenemanager);
        self.render_physics(device.clone(), deltatime, scenemanager);
    }
}