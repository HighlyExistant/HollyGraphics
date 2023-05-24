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
    pub fn to_rotation_right(&self) -> Self {
        Self { x: -self.y, y: self.x }
    }
    pub fn to_rotation_left(&self) -> Self {
        Self { x: self.y, y: -self.x }
    }
}
impl std::ops::Add for FVec2{
    fn add(self, rhs: Self) -> Self::Output {
        FVec2::new(self.x + rhs.x, self.y + rhs.x)
    }
    type Output = FVec2;
}
impl std::ops::Sub for FVec2{
    fn sub(self, rhs: Self) -> Self::Output {
        FVec2::new(self.x - rhs.x, self.y - rhs.x)
    }
    type Output = FVec2;
}
impl std::ops::Mul for FVec2{
    fn mul(self, rhs: Self) -> Self::Output {
        FVec2::new(self.x * rhs.x, self.y * rhs.x)
    }
    type Output = FVec2;
}
impl std::ops::Div for FVec2{
    fn div(self, rhs: Self) -> Self::Output {
        FVec2::new(self.x / rhs.x, self.y / rhs.x)
    }
    type Output = FVec2;
}