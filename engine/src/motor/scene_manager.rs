#![allow(unused)]
use std::collections::HashMap;

use drowsed_math::Transform;

use crate::components::{scene::Scene, object::BasicObject};
/// The *SceneManager* contains the objects and scenes
/// that compose the application. Objects are a global listing
/// of all objects while scenes contain a grouping of certain objects
pub struct SceneManager<T: Transform> {
    pub(crate) objects: HashMap<i128, BasicObject<T>>,
    pub(crate) scenes: Vec<Scene<T>>,
    pub selected_scene: usize,
}

impl<T: Transform> SceneManager<T> {
    pub fn new() -> Self {
        Self { objects: HashMap::new(), scenes: vec![], selected_scene: 0 }
    }
    pub fn get_object_by_id(&self, id: i128) -> Option<&BasicObject<T>> {
        self.objects.get(&id)
    }
    pub fn get_object_by_id_mut(&mut self, id: i128) -> Option<&mut BasicObject<T>> {
        self.objects.get_mut(&id)
    }
    pub fn get_selected_scene(&self) -> &Scene<T> {
        &self.scenes[self.selected_scene]
    }
    pub fn get_selected_scene_mut(&mut self) -> &mut Scene<T> {
        &mut self.scenes[self.selected_scene]
    }
    pub fn push(&mut self, scene: Scene<T>) {
        self.scenes.push(scene);
    }
}