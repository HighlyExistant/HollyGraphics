use std::{rc::Rc, borrow::BorrowMut, println};

use crate::{holly_types::model, buffer::{allocator, self}};
use super::vertex;
use ash::vk;

pub struct Batch< T: vertex::Vertex, I: model::Index> {
    pub vertex: buffer::raw::Buffer<T>,
    pub index: buffer::raw::Buffer<I>,
}

#[derive(Default)]
pub struct BatchRenderer<'a, T: vertex::Vertex, I: model::Index> {
    models: Vec<&'a mut dyn model::Model<T, I>>
}

impl<'a, T: vertex::Vertex + std::fmt::Debug, I: model::Index + std::fmt::Debug> BatchRenderer<'a, T, I>  {
    pub fn push(&mut self, model: &'a mut dyn model::Model<T, I>) {
        self.models.push(model);
    }
    pub fn create(&mut self, allocator: &mut allocator::BufferAllocator) -> Batch::<T, I> {
        let current_offset: I = I::zero();
        let mut dimesion1_vertex: Vec<T> = vec![];
        let mut dimesion1_indices: Vec<I> = vec![];
        for i in 0..self.models.len() {
            // Unravel vertices into 1 dimensional vertex
            let reference = &mut self.models[i];
            let vertices = reference.vertices();
            for j in 0..vertices.len() {
                let vertex = vertices[j];
                dimesion1_vertex.push(vertex);
            }

            let indices: Vec<I> = reference.indices();

            // Unravel Indices into 1 dimensional offseted vertex
            for j in 0..indices.len() {
                let index = indices[j];
                dimesion1_indices.push(index + current_offset);
            }
            let casted = unsafe { (&current_offset as *const I as *mut I as *mut usize).as_mut().unwrap() };
            *casted += dimesion1_indices.len();
        }
        println!("dimesion1_vertex: {:?}", dimesion1_vertex);
        println!("dimesion1_indices: {:?}", dimesion1_indices);

        let vertex_buffer = buffer::raw::Buffer::<T>::from_vec(allocator, 
            vk::BufferUsageFlags::VERTEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            dimesion1_vertex
        );
        let index_buffer = buffer::raw::Buffer::<I>::from_vec(allocator, 
            vk::BufferUsageFlags::INDEX_BUFFER, 
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            dimesion1_indices
        );

        Batch::<T, I> { vertex: vertex_buffer, index: index_buffer }
    }
}