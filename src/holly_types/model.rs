use crate::holly_types::vertex::Vertex;
use num_traits;
pub trait Index: num_traits::Num + core::clone::Clone + num_traits::AsPrimitive<u8> + num_traits::AsPrimitive<u16> + num_traits::AsPrimitive<u32> + num_traits::AsPrimitive<usize> {}

impl Index for u8 {}
impl Index for u16 {}
impl Index for u32 {}

pub trait Model<T, I> {
    fn vertices(&mut self) -> Vec<T>
        where T: Vertex;
    fn indices(&mut self) -> Vec<I>
        where I: Index;
}