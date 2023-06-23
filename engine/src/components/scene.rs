use std::collections::{HashMap, hash_map::Iter};

use drowsed_math::linear::TransformQuaternion3D;
use yum_mocha::camera::Camera;

use super::object::BasicObject;

pub struct Scene {
    objects: HashMap<i128, BasicObject<TransformQuaternion3D>>,
    pub current_camera: usize,
    cameras: Vec<Camera>
}

impl Scene {
    pub fn new(cameras: Vec<Camera>) -> Self {
        Self { objects: HashMap::new(), current_camera: 0, cameras }
    }
    pub fn push_object(&mut self, id: i128, object: BasicObject<TransformQuaternion3D>) {
        self.objects.insert(id, object);
    }
    pub fn objects(&self) -> Iter<i128, BasicObject<TransformQuaternion3D>> {
        self.objects.iter()
    }
    pub fn get_camera(&self) -> &Camera {
        &self.cameras[self.current_camera]
    }
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.cameras[self.current_camera]
    }
    pub fn get_object_by_id(&self, id: i128) -> Option<&BasicObject<TransformQuaternion3D>> {
        self.objects.get(&id)
    }
    pub fn get_object_by_id_mut(&mut self, id: i128) -> Option<&mut BasicObject<TransformQuaternion3D>> {
        self.objects.get_mut(&id)
    }
}