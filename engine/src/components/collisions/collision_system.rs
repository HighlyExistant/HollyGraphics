use std::{collections::HashMap, sync::Arc, cell::{RefCell, Cell}, rc::Rc};

use drowsed_math::{TransformQuaternion3D, FVec3, Transform, Vector};
use mofongo::collider::{Collider, CollisionInfo};
use yum_mocha::vk_obj::{self, device::ReplacingDevice};

use crate::{components::scene::Scene, motor::scene_manager::SceneManager};

pub struct CollisionSystem<T: Transform, V: Vector> {
    colliders: HashMap<i128, (Rc<RefCell<(dyn Collider<TransformComponent = T, ColliderLayoutVertex = V>)>>, Cell<Option<CollisionInfo<V>>>)>
}

impl<T: Transform, V: Vector> CollisionSystem<T, V> {
    pub fn new() -> Self {
        Self { colliders: HashMap::new() }
    }
    pub fn push(&mut self, id: i128, collider: Rc<RefCell<dyn Collider<TransformComponent = T, ColliderLayoutVertex = V>>>) {
        self.colliders.insert(id, (collider, Cell::new(None)));
    }
    pub fn get_collider_by_id(&self, id: i128) -> Option<&(Rc<RefCell<dyn Collider<TransformComponent = T, ColliderLayoutVertex = V>>>, Cell<Option<CollisionInfo<V>>>)> {
        self.colliders.get(&id)
    }
    pub fn render(&mut self, device: Arc<ReplacingDevice>, scenemanager: &SceneManager<T>) {
        let scene = scenemanager.get_selected_scene();
        for (id, (collider, info)) in &self.colliders {
            let object = scene.get_object_by_id(*id).unwrap();
            let cell = collider.borrow();
            for (j_id, j_collider) in self.colliders.iter() {
                if *id == *j_id {
                    continue;
                }
                let object2 = scene.get_object_by_id(*j_id).unwrap();
                let j_cell = j_collider.0.borrow();
                let collision_info = cell.collision(&object.transform, &*j_cell, &object2.transform);
                info.set(collision_info);
            }
        }
    }
}
