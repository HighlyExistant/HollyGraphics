pub struct Matrix<T, const NI: usize, const NJ: usize> {
    pub(crate) mat: [[T; NI]; NJ]
}
impl<T, const NI: usize, const NJ: usize> Matrix<T, NI, NJ> {
}