#![allow(unused)]
use std::collections::{HashMap, hash_map::Iter};

use drowsed_math::{Transform, TransformMatrix};
use yum_mocha::camera::Camera;

use super::object::BasicObject;

pub struct Scene<T: TransformMatrix<f32>> {
    objects: HashMap<i128, BasicObject<T>>,
    pub current_camera: usize,
    cameras: Vec<Camera>
}

impl<T: TransformMatrix<f32>> Scene<T> {
    pub fn new(cameras: Vec<Camera>) -> Self {
        Self { objects: HashMap::new(), current_camera: 0, cameras }
    }
    pub fn push_object(&mut self, id: i128, object: BasicObject<T>) {
        self.objects.insert(id, object);
    }
    pub fn objects(&self) -> Iter<i128, BasicObject<T>> {
        self.objects.iter()
    }
    pub fn get_camera(&self) -> &Camera {
        &self.cameras[self.current_camera]
    }
    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.cameras[self.current_camera]
    }
    pub fn get_object_by_id(&self, id: i128) -> Option<&BasicObject<T>> {
        self.objects.get(&id)
    }
    pub fn get_object_by_id_mut(&mut self, id: i128) -> Option<&mut BasicObject<T>> {
        self.objects.get_mut(&id)
    }
}