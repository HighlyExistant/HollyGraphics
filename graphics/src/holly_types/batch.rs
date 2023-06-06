#![allow(unused)]
use std::{marker::PhantomData, cell::RefCell, fmt::{Debug, Arguments}};

use crate::{holly_types::model, buffer::{allocator, self}, device};
use super::vertex;
use ash::vk;

pub struct Batch<'a, T: vertex::Vertex, I: model::Index> {
    pub vertex: buffer::raw::Buffer<T>,
    pub index: buffer::raw::Buffer<I>,
    pub index_count: Option<u32>, // Possibly no indices provided
    pub constants: Option<&'a [u8]> // Possibly no constants provided
}
impl<'a, T: vertex::Vertex, I: model::Index> Batch<'a, T, I> {
    fn render(device: device::Device, cmd_buffer: vk::CommandBuffer) {
        
    }
}
pub struct BatchRenderer<'a, T: vertex::Vertex, I: model::Index, M: model::Mesh<T, I>> {
    pub models: Vec<*const M>,
    pub constants: Option<&'a [u8]>,
    phantom_t: PhantomData<T>,
    phantom_i: PhantomData<I>,
}

impl<'a, T: vertex::Vertex + std::fmt::Debug, I: model::Index + std::fmt::Debug, M: model::Mesh<T, I>> BatchRenderer<'a, T, I, M>  {
    pub fn push(&mut self, model: *const M) {
        self.models.push(model);
    }
    pub fn push_constants(&mut self, constants: Option<&'a [u8]>) {
        self.constants = constants;
    }
    pub fn create(&mut self, allocator: &mut allocator::BufferAllocator) -> Batch::<T, I> {
        let current_offset: I = I::zero();
        let mut dimesion1_vertex: Vec<T> = vec![];
        let mut dimesion1_indices: Vec<I> = vec![];
        for i in 0..self.models.len() {
            // Unravel vertices into 1 dimensional vertex
            let mut reference = unsafe { (self.models[i]) };
            let vertices = unsafe { (*reference).vertices() };
            for j in 0..vertices.len() {
                let vertex = vertices[j];
                dimesion1_vertex.push(vertex);
            }

            let indices: Vec<I> = unsafe { (*reference).indices() };
            
            // Unravel Indices into 1 dimensional offseted vertex
            for j in 0..indices.len() {
                let index = indices[j];
                dimesion1_indices.push(index + current_offset);
            }
            let casted = unsafe { (&current_offset as *const I as *mut I as *mut usize).as_mut().unwrap() };
            *casted += dimesion1_indices.len();
        }
        println!("dimesion1_vertex: {:#?}\n Length: {}", dimesion1_vertex, dimesion1_vertex.len());
        // println!("dimesion1_indices: {:?}", dimesion1_indices);
        let empty = dimesion1_indices.is_empty();
        let index_count = if (empty) {
            None
        } else {
            Some(dimesion1_indices.len() as u32)
        };

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

        Batch::<T, I> { vertex: vertex_buffer, index: index_buffer, index_count, constants: self.constants }
    }
}

impl<'a, T: vertex::Vertex, I: model::Index, M: model::Mesh<T, I>> Default for BatchRenderer<'a, T, I, M> {
    fn default() -> Self {
        Self { models: vec![], constants: None, phantom_t: PhantomData, phantom_i: PhantomData }
    }
}