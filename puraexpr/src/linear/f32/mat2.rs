#![allow(unused)]
pub use super::FVec2;
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FMat2 {
    pub x_axis: FVec2,
    pub y_axis: FVec2,
}

impl FMat2 {
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub const fn new(
        m00: f32,
        m01: f32,
        m10: f32,
        m11: f32,
    ) -> Self {
        Self {
            x_axis: FVec2::new(m00, m01),
            y_axis: FVec2::new(m10, m11),
        }
    }
    #[inline(always)]
    pub const fn from_fvec2(
        x_axis: FVec2,
        y_axis: FVec2
    ) -> Self{
        Self { x_axis, y_axis }
    } 
}
impl std::ops::Mul for FMat2 {
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
			self.x_axis.x * rhs.x_axis.x + self.y_axis.x * rhs.x_axis.y,
			self.x_axis.y * rhs.x_axis.x + self.y_axis.y * rhs.x_axis.y,
			self.x_axis.x * rhs.y_axis.x + self.y_axis.x * rhs.y_axis.y,
			self.x_axis.y * rhs.y_axis.x + self.y_axis.y * rhs.y_axis.y)
    }
    type Output = FMat2;
}

impl std::ops::Add for FMat2{
    fn add(self, rhs: Self) -> Self::Output {
        FMat2::from_fvec2(self.x_axis + rhs.x_axis, self.y_axis + rhs.x_axis)
    }
    type Output = FMat2;
}
impl std::ops::Sub for FMat2{
    fn sub(self, rhs: Self) -> Self::Output {
        FMat2::from_fvec2(self.x_axis - rhs.x_axis, self.y_axis - rhs.x_axis)
    }
    type Output = FMat2;
}