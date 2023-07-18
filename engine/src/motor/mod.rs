use ash::Entry;
use drowsed_math::{linear::Transform, Vector};
use yum_mocha::vk_obj::{rendering::mesh::{Vertex, VulkanIndexable}, device::WindowOption};

use crate::components::scene::Scene;

pub mod system_manager;
pub mod device_manager;
pub mod scene_manager;
/// Im trying to learn german which is why I named it
/// **SchonMotor** meaning Beaatiful Motor.
pub struct SchonMotor<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform> {
    pub device_manager: device_manager::DeviceManager,
    pub system_manager: system_manager::SystemManager<V, E, I, T>,
}

impl<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform> SchonMotor<V, E, I, T> {
    pub fn new(entry: &Entry, window: WindowOption) -> Self {
        let device_manager = device_manager::DeviceManager::new(entry, window);
        let system_manager = system_manager::SystemManager::new();
        Self { system_manager, device_manager }
    }
    pub fn push_scene(&mut self, scene: Scene<T>) {
        self.system_manager.scene_manager.push(scene);
    }
}
