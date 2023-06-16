use crate::model::vertex::Vertex;
use num_traits;
pub trait Index: num_traits::Num + core::clone::Clone + num_traits::AsPrimitive<u8> + num_traits::AsPrimitive<u16> + num_traits::AsPrimitive<u32> + num_traits::AsPrimitive<usize> + core::clone::Clone {}

impl Index for u8 {}
impl Index for u16 {}
impl Index for u32 {}

pub trait Mesh<V: Vertex, I: Index> {
    fn vertices(&self) -> Vec<V>;
    fn indices(&self) -> Vec<I>;
}