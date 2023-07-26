use ash::Entry;
use drowsed_math::{Transform, Vector};
use yum_mocha::vk_obj::{rendering::mesh::{Vertex, VulkanIndexable}, device::WindowOption};

use crate::components::scene::Scene;

use self::system_manager::SystemManagerInfo;

pub mod system_manager;
pub mod device_manager;
pub mod scene_manager;
/// Im trying to learn german which is why I named it
/// **SchonMotor** meaning Beaatiful Motor.
pub struct SchonMotor<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform, R: mofongo::bodies::RigidBody<Transformation = T>> {
    pub device_manager: device_manager::DeviceManager,
    pub system_manager: system_manager::SystemManager<V, E, I, T, R>,
}

impl<V: Vertex, E: Vector, I: VulkanIndexable, T: Transform, R: mofongo::bodies::RigidBody<Transformation = T>> SchonMotor<V, E, I, T, R> {
    pub fn new(entry: &Entry, window: WindowOption, info: &SystemManagerInfo<R::SpatialVector>) -> Self {
        let device_manager = device_manager::DeviceManager::new(entry, window);
        let system_manager = system_manager::SystemManager::new(info);
        Self { system_manager, device_manager }
    }
    pub fn push_scene(&mut self, scene: Scene<T>) {
        self.system_manager.scene_manager.push(scene);
    }
}
