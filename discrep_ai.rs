extern crate convex_hull;
extern crate nalgebra;
use assert_approx_eq::assert_approx_eq;
use nalgebra::{Point3, Vector3};
use sphere_n::{CylindN, SphereN};
use std::f64::consts::PI; // Assuming sphere_n crate is available

fn discrep_2<K: AsRef<[usize]>>(k_indices: &[K], points: &[Vector3<f64>]) -> f64 {
    let nsimplex = k_indices.len();
    let n = points.len();
    let mut maxq = 0.0;
    let mut minq = 1000.0;
    for k in 0..nsimplex {
        let indices = k_indices[k].as_ref();
        for i in 0..n - 1 {
            for j in i + 1..n {
                let dot = points[indices[i]].dot(&points[indices[j]]);
                let q = 1.0 - dot * dot;
                maxq = maxq.max(q);
                minq = minq.min(q);
            }
        }
    }
    (maxq.sqrt().asin() - minq.sqrt().asin()).abs()
}

fn run_lds<T: SphereGenerator>(spgen: &mut T) -> f64
where
    T::Output: Into<Vector3<f64>>,
{
    let npoints = 600;
    let mut triples = Vec::with_capacity(npoints);
    for _ in 0..npoints {
        triples.push(spgen.pop().into());
    }
    let hull = convex_hull::convex_hull_3d(&triples);
    let triangles = hull.unwrap_or(Vec::new()); // Handle potential None case
    discrep_2(&triangles, &triples)
}

#[test]
fn test_sphere_n() {
    let mut spgen = SphereN::new(vec![2, 3, 5, 7]);
    let measure = run_lds(&mut spgen);
    assert_approx_eq!(measure, 0.9125914, 1e-6);
}

#[test]
fn test_cylin_n() {
    let mut cygen = CylindN::new(vec![2, 3, 5, 7]);
    let measure = run_lds(&mut cygen);
    assert_approx_eq!(measure, 1.0505837105828988, 1e-6);
}
