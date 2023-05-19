#![allow(unused)]
// This file is heavily inspired by bitshifer's glam-rs library https://github.com/bitshifter/glam-rs/
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct FVec2 {
    pub x: f32,
    pub y: f32,
}
impl FVec2 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub const ZERO: Self = Self::set_all(0.0);
    pub const ONE: Self = Self::set_all(1.0);
    pub const NEG_ONE: Self = Self::set_all(-1.0);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new( 0.0, 1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0);
    pub const NEG_Y: Self = Self::new( 0.0, -1.0);
    #[inline]
    pub const fn set_all(xy: f32) -> Self {
        
        Self { x: xy, y: xy }
    }
    
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}
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