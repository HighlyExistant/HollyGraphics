use std::{collections::HashMap, sync::Arc, cell::RefCell, rc::Rc};

use yum_mocha::vk_obj;

use crate::components::scene::Scene;

use super::collider::{Collider3D, self};

pub struct CollisionSystem {
    
    colliders: HashMap<i128, Rc<RefCell<dyn Collider3D>>>
}


impl CollisionSystem {
    pub fn new() -> Self {
        Self { colliders: HashMap::new() }
    }
    pub fn push(&mut self, id: i128, collider: Rc<RefCell<dyn Collider3D>>) {
        self.colliders.insert(id, collider);
    }
    pub fn get_collider_by_id(&self, id: i128) -> Option<&Rc<RefCell<dyn Collider3D>>> {
        self.colliders.get(&id)
    }
    pub fn render_all(&mut self, device: Arc<vk_obj::device::Device>, scene: &mut Scene) {
        for (id, collider) in &self.colliders {
            let object = scene.get_object_by_id(*id).unwrap();
            let cell = collider.borrow();
            for (j_id, j_collider) in &self.colliders {
                if *id == *j_id {
                    continue;
                }
                let object2 = scene.get_object_by_id(*j_id).unwrap();
                let j_cell = j_collider.borrow();
                if let Some(info) = cell.collision(&object.transform, &*j_cell, &object2.transform) {
                    println!("{:?}", info);
                }
            }
        }
    }
}
