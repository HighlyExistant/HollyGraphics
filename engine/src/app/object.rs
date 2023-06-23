use drowsed_math::{linear::{TransformQuaternion3D, FVec3}, complex::quaternion::Quaternion};
use yum_mocha::{vk_obj::{self, buffer}, model::vertex::GlobalDebugVertex, app::models::{Mesh3D, FromFBX}};

pub struct BasicObject3D {
    pub mesh: Mesh3D<GlobalDebugVertex>,
    pub transform: TransformQuaternion3D,
    pub script: Box<dyn CodeComponent>,
}

impl BasicObject3D {
    pub fn get_buffers(&self, device: std::sync::Arc<vk_obj::device::Device>) -> (buffer::raw::Buffer<GlobalDebugVertex>, buffer::raw::Buffer<u32>) {
        self.mesh.create(device)
    }
}
impl FromFBX for BasicObject3D {
    fn from_fbx(filepath: &str) -> Vec<Self>
            where Self: Sized {
        let meshes = Mesh3D::<GlobalDebugVertex>::from_fbx(filepath);

        meshes.iter().map(|mesh|{
            Self {
                mesh: mesh.clone(),
                transform: TransformQuaternion3D { translation: FVec3::default(), scale: FVec3::from(1.0), rotation: Quaternion::<f32>::from_euler(FVec3::default()) }
            }
        }).collect()
    }
}