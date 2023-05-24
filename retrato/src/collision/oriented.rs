use std::cell::RefCell;

use puraexpr::linear::f32::FVec2;

use crate::{holly_types::transform::{self, Transform2D}};


pub struct Quadrilateral {
    pub a: FVec2,
    pub b: FVec2,
    pub c: FVec2,
    pub d: FVec2,
}
impl Quadrilateral {
    pub fn new(a: FVec2, b: FVec2, c: FVec2, d: FVec2) -> Self {
        Self { a, b, c, d }
    }
    pub fn rotated(transform: RefCell<transform::Transform2D>, area: &FVec2) -> Quadrilateral {
        let mut interior = transform.borrow_mut();
        let center_x = interior.translation.x + (area.x / 2.0);
        let center_y = interior.translation.y + (area.y / 2.0);
			
        let  top_left = Self::work_points(center_x, center_y, interior.translation.x, interior.translation.y, interior.rotation);
		let  top_right = Self::work_points(center_x, center_y, interior.translation.x + area.x, interior.translation.y, interior.rotation);
		let  bottom_left = Self::work_points(center_x, center_y, interior.translation.x, interior.translation.y + area.y, interior.rotation);
		let  bottom_right = Self::work_points(center_x, center_y, interior.translation.x + area.x, interior.translation.y + area.y, interior.rotation);

        Quadrilateral { a: top_left, b: top_right, c: bottom_left, d: bottom_right }
    }
    fn work_points(cx: f32,cy: f32, vx: f32 ,vy: f32, radians: f32) -> FVec2 {
		let dx = vx - cx;
		let dy = vy - cy;
        let distance = (dx * dx + dy * dy).sqrt();
        let original_angle = dy.atan2(dx);

        let  rotated_x = cx + distance * (original_angle + radians).cos();
        let  rotated_y = cy + distance * (original_angle + radians).sin();

        FVec2 { x: rotated_x, y: rotated_y }
    }
}
impl Default for Quadrilateral {
    fn default() -> Self {
        Self { a: FVec2::new(-1.0, 1.0), b: FVec2::ONE, c: FVec2::NEG_ONE, d: FVec2::new( 1.0, -1.0) }
    }
}
#[derive(Default)]
pub struct Polygon2D {
    pub vertices: Vec<FVec2>,
    pub edges: Vec<FVec2>
}
pub fn seperate_axis(poly_a: &Polygon2D, poly_b: &Polygon2D) -> bool {
    let mut perpendicular_line = FVec2::ZERO;
    let mut dot = 0.0;
    let mut perpendicular_stack: Vec<FVec2> = vec![];

    let (mut amin, mut amax, mut bmin, mut bmax) = (0.0, 0.0, 0.0, 0.0);
    
    for i in 0..poly_a.edges.len() {
        perpendicular_line = poly_a.edges[i].to_rotation_right();
        perpendicular_stack.push(perpendicular_line);
    }
    
    for i in 0..poly_b.edges.len() {
        perpendicular_line = poly_b.edges[i].to_rotation_right();
        perpendicular_stack.push(perpendicular_line);
    }
    for item in perpendicular_stack {
        amin = 0.0;
        amax = 0.0;
        bmin = 0.0;
        bmax = 0.0;
        for i in 0..poly_a.vertices.len() {
            dot = poly_a.vertices[i].dot(item);

            if (amax == 0.0 || dot > amax) {
                amax = dot;
            }
            if (amin == 0.0 || dot < amin){
                amin = dot;
            }
        }

        for i in 0..poly_b.vertices.len() {
			dot = poly_b.vertices[i].dot(item);

			if (bmax == 0.0 || dot > bmax) {
				bmax = dot;
            }
			if (bmin == 0.0 || dot < bmin){
				bmin = dot;
            }
        }
        if ((amin < bmax && amin > bmin) ||
            (bmin < amax && bmin > amin))
        {
            continue;
        }
        else { return false }
    }
    return true;
}
#[derive(Clone)]
pub struct OrientedSquareCollider {
    pub area: FVec2,
    pub transform: RefCell<transform::Transform2D>,
}
impl OrientedSquareCollider {
    pub fn new(area_x: f32, area_y: f32, transform: RefCell<transform::Transform2D>) -> Self {
        Self { area: FVec2 { x: area_x, y: area_y }, transform }
    }
    pub fn from_fvec2(area: FVec2, transform: RefCell<transform::Transform2D>) -> Self {
        Self { area, transform }
    }
    pub fn check_collision(&self, other: Self) -> bool {
        let quad_self = Quadrilateral::rotated(self.transform.clone(), &self.area);
        let quad_a = Quadrilateral::rotated(other.transform.clone(), &other.area);
    
        let mut poly_a = Polygon2D::default();
        let mut poly_b = Polygon2D::default();

        poly_a.vertices.push(quad_self.a);
        poly_a.vertices.push(quad_self.b);
        poly_a.vertices.push(quad_self.c);
        poly_a.vertices.push(quad_self.d);

        poly_a.edges.push(quad_self.b - quad_self.a);
        poly_a.edges.push(quad_self.c - quad_self.b);
        poly_a.edges.push(quad_self.d - quad_self.c);
        poly_a.edges.push(quad_self.a - quad_self.d);
        
        poly_b.vertices.push(quad_a.a);
        poly_b.vertices.push(quad_a.b);
        poly_b.vertices.push(quad_a.c);
        poly_b.vertices.push(quad_a.d);

        if seperate_axis(&poly_a, &poly_b) {
            return true;
        } else {
            let self_unwrapped = self.transform.borrow();
            let other_unwrapped = self.transform.borrow();
            if (self_unwrapped.rotation == 0.0 && self_unwrapped.rotation == 0.0) {
				if (!(
					self_unwrapped.translation.x > other_unwrapped.translation.x + other.area.x ||
					self_unwrapped.translation.x + self.area.x < other_unwrapped.translation.x ||
					self_unwrapped.translation.y> other_unwrapped.translation.y + other.area.y ||
					self_unwrapped.translation.y + self.area.y < other_unwrapped.translation.y)) {
                    return true;
				}
            }
        }
        false
    }
}