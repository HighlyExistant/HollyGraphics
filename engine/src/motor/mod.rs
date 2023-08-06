use ash::Entry;
use drowsed_math::{Transform, Vector, TransformQuaternion3D, FVec3, TransformMatrix};
use yum_mocha::{vk_obj::{rendering::mesh::{Vertex, VulkanIndexable}, device::WindowOption}, model::vertex::GlobalDebugVertex};

use crate::components::scene::Scene;

use self::system_manager::SystemManagerInfo;

pub trait SchonMotorBase {
    type VertexType: Vertex;
    type VectorType: Vector;
    type VulkanIndexType: VulkanIndexable;
    type UniversalTransformType: TransformMatrix<f32>;
    type RigidBodyType: mofongo::bodies::RigidBody<Transformation = Self::UniversalTransformType, SpatialVector = Self::VectorType>;
}
pub struct SolidMotor;
impl SchonMotorBase for SolidMotor {
    type VertexType = GlobalDebugVertex;
    type VectorType = FVec3;
    type VulkanIndexType = u32;
    type UniversalTransformType = TransformQuaternion3D;
    type RigidBodyType = mofongo::solid::physics::rigidbody::RigidBody3D;
}

pub mod system_manager;
pub mod device_manager;
pub mod scene_manager;
/// Im trying to learn german which is why I named it
/// **SchonMotor** meaning Beaatiful Motor.
pub struct SchonMotor<Base: SchonMotorBase> {
// <V: Vertex, E: Vector, I: VulkanIndexable, T: Transform, R: mofongo::bodies::RigidBody<Transformation = T>> {
    pub device_manager: device_manager::DeviceManager,
    pub system_manager: system_manager::SystemManager<Base::VertexType, Base::VectorType, Base::VulkanIndexType, Base::UniversalTransformType, Base::RigidBodyType>,
}

impl<Base: SchonMotorBase> SchonMotor<Base> {
    pub fn new(entry: &Entry, window: WindowOption, info: &SystemManagerInfo<Base::VectorType>) -> Self {
        let device_manager = device_manager::DeviceManager::new(entry, window);
        let system_manager = system_manager::SystemManager::new(info);
        Self { system_manager, device_manager }
    }
    pub fn push_scene(&mut self, scene: Scene<Base::UniversalTransformType>) {
        self.system_manager.scene_manager.push(scene);
    }
}
