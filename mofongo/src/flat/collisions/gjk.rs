use core::num;

use drowsed_math::{Vector2, Matrix3, SignedNumber, Vector, Vector3, FMat3, FVec3, FVec2, EuclideanGeometry, Simplex, Transform2D, Transform};

use crate::collider::{CollisionInfo, Collider, ColliderLayout};

fn furthest_point<T: SignedNumber>(direction: Vector2<T>, vertices: &Vec<Vector2<T>>, transform: &Matrix3<T>) -> Vector2<T> {
    let mut maxdot = T::min_value();
    let mut best = Vector2::from(T::zero());
    for vertex in vertices {
        let point = (Vector3::from(*vertex) * *transform).xy();
        let dot = point.dot(&direction);
        if dot > maxdot {
            maxdot = dot;
            best = point;
        }
    }
    best
}

fn supportidx<T: SignedNumber>(direction: Vector2<T>, vertices: &Vec<Vector2<T>>, transform: &Matrix3<T>) -> usize {
    let mut maxdot = T::min_value();
    let mut idx = 0;
    for (i, vertex) in vertices.iter().enumerate() {
        let point = (Vector3::from(*vertex) * *transform).xy();
        let dot = point.dot(&direction);
        if dot > maxdot {
            maxdot = dot;
            idx = i;
        }
    }
    idx
}

fn support<T: SignedNumber>(direction: Vector2<T>, vertices1: &Vec<Vector2<T>>, vertices2: &Vec<Vector2<T>>, transform1: &Matrix3<T>, transform2: &Matrix3<T>) -> Vector2<T> {
    let b = furthest_point(-direction, vertices2, transform2);
    let a = furthest_point(direction, vertices1, transform1);
    a - b
}


fn closest_edge(simplex: &Simplex<FVec2, 3>) -> (f32, usize, FVec2, FVec2, FVec2) {
    let number_of_points = 3;
    let mut min_distance = f32::INFINITY;
    let (mut closest_distance, mut idx, mut edgevertex1, mut edgevertex2, mut normal) = (f32::INFINITY, 0, FVec2::from(0.0), FVec2::from(0.0), FVec2::from(0.0));
    for i in 0..number_of_points {
        let p = simplex.points[i];
        let q = simplex.points[((i + 1) % number_of_points) as usize];
        let e = q - p;
        let crossep = e.cross(p);
        let n = FVec2::new(-e.y * crossep, e.x * crossep);
        let dist = n.dot(&p);
        if dist < min_distance {
            min_distance = dist;
            (closest_distance, idx, edgevertex1, edgevertex2, normal) = (dist, i, p, q, n);
        }
    }
    return (closest_distance, idx, edgevertex1, edgevertex2, normal);
}

fn epa(vertices1: &Vec<Vector2<f32>>, vertices2: &Vec<Vector2<f32>>, transform1: &FMat3, transform2: &FMat3, simplex: &Simplex<FVec2, 3>) -> Option<CollisionInfo<FVec2>> {
    // let mut polytope = *simplex;
    // const MAX_EPA_ITER: usize = 6;
    // for i in 0..MAX_EPA_ITER {
    //     let (dist, i, p, q, n) = closest_edge(&polytope);
    //     let r = support(n, vertices1, vertices2, transform1, transform2);
    //     if (n.dot(&r) - dist).abs() < 0.001 {
    //         let collision_info = CollisionInfo {
    //             normal: n,
    //             depth: dist
    //         };
    //         return Some(collision_info);
    //     }
    //     let mut vec = polytope.points.to_vec();
    //     vec.insert(i + 1, r);
    //     polytope.initialize(vec);
    // }

    let mut min_index = 0;
    let mut min_distance = f32::INFINITY;
    let mut min_normal = FVec2::from(0.0);
    
    let mut polytope = simplex.to_vec();
    while min_distance == f32::INFINITY {
        for i in 0..polytope.len() {
            let j = (i+1) % polytope.len();
            let vertexi = polytope[i];
            let vertexj = polytope[j];

            let ij = vertexj - vertexi;
            let mut normal = FVec2::new(ij.y, -ij.x).normalize();
            let mut distance = normal.dot(&vertexi);
            if distance < 0.0 {
                distance = -distance;
                normal = -normal;
            }
            if distance < min_distance {
                min_distance = distance;
                min_normal = normal;
                min_index = j;
            }
        }
        let support = support(min_normal, vertices1, vertices2, transform1, transform2);
        let s_distance = min_normal.dot(&support);

        if (s_distance - min_distance).abs() > 0.001 {
            min_distance = f32::INFINITY;
            polytope.insert(min_index, support);
        }
    }
    let collision_info = CollisionInfo {
        normal: min_normal * (min_distance + 0.001),
        depth: min_distance,
    };
    return Some(collision_info);
}

fn gjk(vertices1: &Vec<Vector2<f32>>, vertices2: &Vec<Vector2<f32>>, transform1: &FMat3, transform2: &FMat3) -> Option<Simplex<FVec2, 3>> {
    let mut a = support(FVec2::new(1.0, 1.0), vertices1, vertices2, transform1, transform2);
    let mut v = -a;
    let mut b = support(v, vertices1, vertices2, transform1, transform2);
    if b.dot(&v) <= 0.0 {
        return None;
    }

    let ab = b - a;
    let crossab = ab.cross(-a);
    v = FVec2::new(-ab.y * crossab, ab.x * crossab);
    loop {
        let c = support(v, vertices1, vertices2, transform1, transform2);
        if c.dot(&v) <= 0.0 {
            return None;
        }
        let c0 = -c;
        let cb = b - c;
        let ca = a - c;

        let crosscacb = ca.cross(cb);
        let crosscbca = cb.cross(ca);

        let cbperp = FVec2::new(-cb.y * crosscacb, cb.x * crosscacb);
        let caperp = FVec2::new(-ca.y * crosscbca, ca.x * crosscbca);
        if caperp.dot(&c0) > 0.0 {
            b = c;
            v = caperp;
        } else if cbperp.dot(&c0) > 0.0 {
            a = c;
            v = cbperp;
        } else {
            return Some(Simplex::from_slice(&[a, b, c]));
        }
    }
}

pub fn collision_gjk(vertices1: &Vec<Vector2<f32>>, vertices2: &Vec<Vector2<f32>>, transform1: &FMat3, transform2: &FMat3) -> Option<CollisionInfo<FVec2>> {
    if let Some(simplex) = gjk(vertices1, vertices2, transform1, transform2) {
        epa(vertices1, vertices2, transform1, transform2, &simplex)
    } else {
        None
    }
}

pub struct GJKColliderFlat {
    pub vertices: Vec<FVec2>,
}
impl GJKColliderFlat {
    pub fn new(vertices: Vec<FVec2>) -> Self {
        GJKColliderFlat { vertices }
    }
}


// impl Collider for GJKColliderFlat {
//     fn collision(&self, transform1: &Transform2D, collider: &dyn Collider<TransformComponent = Transform2D, ColliderLayoutVertex = FVec2>, transform2: &Transform2D) -> Option<CollisionInfo<FVec2>> {
//         let mat1 = FMat3::
//         transform1.apply_matrix3();
//         let mat2 = transform2.matrix3();
//         match collider.layout() {
//             ColliderLayout::Vertices(vertices) => {
//                 collision_gjk(&self.vertices, vertices, &mat1, &mat2)
//             }
//             _ => {
//                 panic!("Layout not supported for GJKCollider")
//             }
//         }
//     }
//     fn layout(&self) -> ColliderLayout<FVec2> {
//         ColliderLayout::Vertices(&self.vertices)
//     }
//     type ColliderLayoutVertex = FVec2;
//     type TransformComponent = Transform2D;
// }