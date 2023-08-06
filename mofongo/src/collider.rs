use drowsed_math::{Transform, Triangle, Vector};

pub enum ColliderLayout<'a, V: Vector> {
    Vertices(&'a Vec<V>),
    IndexedVertices(&'a Vec<V>, &'a Vec<u32>),
    Triangles(&'a Vec<Triangle<V>>),
    IndexedTriangles(&'a Vec<Triangle<V>>, &'a Vec<u32>),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CollisionInfo<V: Vector> {
    pub normal: V,
    pub depth: f32,
}

pub trait Collider {
    fn collision(&self, 
        transform1: &Self::TransformComponent, 
        collider: &dyn Collider<TransformComponent = Self::TransformComponent, 
        ColliderLayoutVertex = Self::ColliderLayoutVertex>, 
        transform2: &Self::TransformComponent
    ) -> Option<CollisionInfo<Self::ColliderLayoutVertex>>;
    fn layout(&self) -> ColliderLayout<Self::ColliderLayoutVertex>;
    type ColliderLayoutVertex: Vector;
    type TransformComponent: Transform;
}