use drowsed_math::{Vector3, Vector4, Number, SignedNumber, FVec3, FVec4, FMat4, Matrix4, Simplex, Transform, TransformQuaternion3D, Vector, SquareMatrix, TransformMatrix};

use crate::collider::{CollisionInfo, Collider, ColliderLayout};
use drowsed_math::EuclideanGeometry;
/// # furthest_point
/// *this function only works if the object is a convext polygon*
///
/// given a direction vector and a vector of points
/// return the furthest value.
fn furthest_point<T: SignedNumber>(direction: Vector3<T>, vertices: &Vec<Vector3<T>>, transform: &Matrix4<T>) -> Vector3<T>{
    
    let mut max_distance = -T::max_value();
    let mut max = Vector3::<T>::from(T::zero());

    for vertex in vertices.clone() {
        let point = Vector3::<T>::from(Vector4::<T>::new(vertex.x, vertex.y, vertex.z, T::one()) * *transform);
        let distance = point.dot(&direction);
        if distance > max_distance {
            max_distance = distance;
            max = point;
        }
    }
    max
}

fn minkowski_support<T: SignedNumber>(direction: Vector3<T>, vertices1: &Vec<Vector3<T>>, vertices2: &Vec<Vector3<T>>, transform1: &Matrix4<T>, transform2: &Matrix4<T>) -> Vector3<T>  {
    let b = furthest_point(-direction, vertices2, transform2);
    let a = furthest_point(direction, vertices1, transform1);
    a - b
}
pub fn same_direction<T: Number>(
	direction: &Vector3<T>,
	ao: &Vector3<T>) -> bool
{
	return direction.dot(ao) > T::zero();
}
fn dimension2<T: SignedNumber>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    let a = points.points[0];
    let b = points.points[1];

    let ab = b-a;
    let ao = -a;
    if same_direction(&ab, &ao) {
        *direction = ab.cross(ao).cross(ab);
    } else {
        points.initialize(vec![a]);
        *direction = ao;
    }
    false
}
pub(crate) fn dimension3<T: SignedNumber>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    let a = points.points[0];
	let b = points.points[1];
	let c = points.points[2];

	let ab = b - a;
	let ac = c - a;
	let ao = -a;
    
    let abc = ab.cross(ac);

    if same_direction(&abc.cross(ac), &ao) {
        if same_direction(&ac, &ao) {
			points.initialize(vec![a, c]);
			*direction = ac.cross(ao).cross(ac);
		}
		else {
            points.initialize(vec![a, b]);
			return dimension2(points, direction);
		}
    }
    else {
        if same_direction(&ab.cross(abc), &ao) {
			points.initialize(vec![a, b]);
			return dimension2(points, direction);
        } else {
            if same_direction(&abc, &ao) {
                *direction = abc;
            }
            else {
                points.initialize(vec![a, c, b]);
                *direction = -abc;
            }
        }
    }
    false
}
fn dimension4<T: SignedNumber>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    let a = points.points[0];
	let b = points.points[1];
	let c = points.points[2];
	let d = points.points[3];

	let ab = b - a;
	let ac = c - a;
	let ad = d - a;
	let ao = -a;
 
	let abc = ab.cross(ac);
	let acd = ac.cross(ad);
	let adb = ad.cross(ab);
 
	if same_direction(&abc, &ao) {
        points.initialize(vec![a, b, c]);
		return dimension3(points, direction);
	}
		
	if same_direction(&acd, &ao) {
        points.initialize(vec![a, c, d]);
		return dimension3(points, direction);
	}
 
	if same_direction(&adb, &ao) {
        points.initialize(vec![a, d, b]);
		return dimension3(points, direction);
	}

	return true;
}


fn next_simplex<T: SignedNumber>(points: &mut Simplex<Vector3<T>, 4>, direction: &mut Vector3<T>) -> bool {
    match points.size {
        2 => dimension2(points, direction),
        3 => dimension3(points, direction),
        4 => dimension4(points, direction),
        _ => false
    }
}
fn get_face_normals(polytope: &Vec<FVec3>, faces: &Vec<u32>) -> (Vec<FVec4>, usize) {
    let mut normals = Vec::<FVec4>::new();
    let mut min_triangle = 0;
    let mut min_distance = f32::MAX;
    
    let mut i = 0;
    while i < faces.len() {
        let a = polytope[faces[i] as usize];
        let b = polytope[faces[i + 1] as usize];
        let c = polytope[faces[i + 2] as usize];

        let mut normal = (b - a).cross(c - a).normalize();
        let mut distance = normal.dot(&a);

        if distance < 0.0 {
            normal = normal * -1.0;
            distance = distance * -1.0;
        }

        normals.push(FVec4::new(normal.x, normal.y, normal.z, distance));

        if distance < min_distance {
            min_triangle = i / 3;
            min_distance = distance;
        }

        i += 3;
    }
    return (normals, min_triangle)
}
fn add_if_unique_edge(unique_edges: &mut Vec<(u32, u32)>, faces: &Vec<u32>, a: usize, b: usize) {
    let mut reverse = 0;
    for (i, edge) in unique_edges.iter().enumerate() {
        reverse = i;
        if faces[b] == edge.0 && faces[a] == edge.1 {
            break;
        }
    }
    if !unique_edges.is_empty() && reverse != (usize::wrapping_sub(unique_edges.len(), 1))  {
        (unique_edges).remove(reverse);
    }
    else {
        unique_edges.push((faces[a], faces[b]));
    }
}
const MAX_EPA_ITER: usize = 16;
fn epa(simplex: Simplex<FVec3, 4>, vertices1: &Vec<Vector3<f32>>, vertices2: &Vec<Vector3<f32>>, transform1: &FMat4, transform2: &FMat4) -> Option<CollisionInfo<FVec3>> {
    let mut polytope = Vec::<FVec3>::new();
    for i in 0..simplex.size {
        polytope.push(simplex.points[i]);
    }
    let mut faces = vec![
        0, 1, 2,
        0, 3, 1,
        0, 2, 3,
        1, 3, 2
    ];

    let (mut normals, mut min_face) = get_face_normals(&polytope, &faces);

    let mut min_normal = FVec3::from(0.0);
    let mut min_distance = f32::MAX;
    let mut inc = 0;

    while min_distance == f32::MAX {
        min_normal = normals[min_face].xyz();
        min_distance = normals[min_face].w;
        let support = minkowski_support(min_normal, &vertices1, &vertices2, &transform1, &transform2);
        let distance = min_normal.dot(&support);

        if f32::abs(distance - min_distance) > 0.001 {
            min_distance = f32::MAX;

            let mut unique_edges = Vec::<(u32, u32)>::new();
            if inc > MAX_EPA_ITER {
                break;
            }
            inc += 1;
            let mut i = 0;
            while i < normals.len() {
                if same_direction(&normals[i].xyz(), &support) {
                    let f = i * 3;

                    add_if_unique_edge(&mut unique_edges, &faces, f, f + 1);
                    add_if_unique_edge(&mut unique_edges, &faces, f + 1, f + 2);
                    add_if_unique_edge(&mut unique_edges, &faces, f + 2, f);
                    faces[f + 2] = faces[faces.len() - 1]; 
                    faces.pop();
                    faces[f + 1] = faces[faces.len() - 1]; 
                    faces.pop();
                    faces[f] = faces[faces.len() - 1]; 
                    faces.pop();
                    normals[i] = normals[normals.len() - 1];
                    normals.pop();

                    i = usize::wrapping_sub(i, 1);
                }
                i = usize::wrapping_add(i, 1);
            }

            let mut new_faces = Vec::<u32>::new();
            for (index1, index2) in unique_edges {
                new_faces.push(index1);
                new_faces.push(index2);
                new_faces.push(polytope.len() as u32);
            }
            polytope.push(support);

            let (new_normals, new_min_face) = get_face_normals(&polytope, &new_faces);

            let mut old_min_distance = f32::MAX;
            for i in 0..normals.len() {
                if normals[i].w < old_min_distance {
                    old_min_distance = normals[i].w;
                    min_face = i;
                }
            }
            if !new_normals.is_empty() {
                if new_normals[new_min_face].w < old_min_distance {
                    min_face = new_min_face + normals.len();
                }
            }
            for new in new_faces {
                faces.push(new);
            }
            for new in new_normals {
                normals.push(new);
            }
        }
    }
    Some(CollisionInfo { normal: min_normal, depth: min_distance + 0.001 })
}

pub fn gjk(vertices1: &Vec<Vector3<f32>>, vertices2: &Vec<Vector3<f32>>, transform1: &FMat4, transform2: &FMat4) -> Option<CollisionInfo<FVec3>> {
    let mut point = minkowski_support(FVec3::new(1.0, 0.0, 0.0), vertices1, vertices2, transform1, transform2);
    
    let mut simplex = Simplex::<FVec3, 4>::new();
    simplex.push(point);

    let mut direction = -point;
    loop {
        point = minkowski_support(direction, vertices1, vertices2, transform1, transform2);
        
        if point.dot(&direction) <= 0.0 {
            return None;
        }
        simplex.push(point);

        if next_simplex(&mut simplex, &mut direction) {
            let collision = epa(simplex, &vertices1, &vertices2, transform1, transform2);
            return collision;
        }
    }
}

pub struct GJKColliderSolid {
    pub vertices: Vec<FVec3>,
}
impl GJKColliderSolid {
    pub fn new(vertices: Vec<FVec3>) -> Self {
        GJKColliderSolid { vertices }
    }
}

impl Collider for GJKColliderSolid {
    fn collision(&self, transform1: &TransformQuaternion3D, collider: &dyn Collider<TransformComponent = TransformQuaternion3D, ColliderLayoutVertex = FVec3>, transform2: &TransformQuaternion3D) -> Option<CollisionInfo<FVec3>> {
        let mut mat1 = FMat4::identity();
        transform1.apply_matrix4(&mut mat1);
        let mut mat2 = FMat4::identity();
        transform2.apply_matrix4(&mut mat2);
        match collider.layout() {
            ColliderLayout::Vertices(vertices) => {
                gjk(&self.vertices, vertices, &mat1, &mat2)
            }
            _ => {
                panic!("Layout not supported for GJKCollider")
            }
        }
    }
    fn layout(&self) -> ColliderLayout<FVec3> {
        ColliderLayout::Vertices(&self.vertices)
    }
    type ColliderLayoutVertex = FVec3;
    type TransformComponent = TransformQuaternion3D;
}