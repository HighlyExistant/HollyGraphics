use drowsed_math::linear::vector::Vector;
use num_traits::Zero;

pub struct Simplex<T: Vector + Zero, const N: usize> {
    pub points: [T; N],
    pub size: usize,
}

impl<T: Vector + Zero, const N: usize> Simplex<T, N> {
    pub fn new() -> Self {
        Self { points: [T::zero(); N], size: 0 }
    }
    pub fn push(&mut self, point: T) {
        for i in (1..N).rev() {
            self.points[i] = self.points[i - 1];
        }
        self.points[0] = point;
        self.size = std::cmp::min(self.size + 1, N);
    }
    pub fn initialize(&mut self, list: Vec<T>) {
        for (i, v) in list.iter().enumerate() {
            self.points[i] = *v;
        }
        self.size = list.len();
    }
}