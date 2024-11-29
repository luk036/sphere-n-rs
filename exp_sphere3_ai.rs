use ndarray::{Array1, Array2, Axis};
use qhull_sys::{qh_new_qhull, qh_triangulate, qh_qh};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::f64::consts::PI;

fn discrep_2(k: &Array2<i32>, x: &Array2<f64>) -> f64 {
    let (nsimplex, n) = k.dim();
    let mut maxq = 0.0;
    let mut minq = 1000.0;
    for k_row in k.rows() {
        let p: Vec<Array1<f64>> = k_row.iter().map(|&i| x.row(i as usize).to_owned()).collect();
        for i in 0..n-1 {
            for j in i+1..n {
                let dot = p[i].dot(&p[j]);
                let q = 1.0 - dot * dot;
                maxq = maxq.max(q);
                minq = minq.min(q);
            }
        }
    }
    (maxq.sqrt().asin() - minq.sqrt().asin()).abs()
}

fn sample_spherical(npoints: usize, ndim: usize) -> Array2<f64> {
    let mut rng = rand::thread_rng();
    let mut vec = Array2::from_shape_fn((npoints, ndim), |_| rng.sample(Standard));
    for mut row in vec.rows_mut() {
        let norm = row.dot(&row).sqrt();
        row.mapv_inplace(|x| x / norm);
    }
    vec
}

fn dispersion(triples: &Array2<f64>) -> f64 {
    let (npoints, ndim) = triples.dim();
    let mut points: Vec<f64> = triples.iter().cloned().collect();

    let mut hull = qh_new_qhull(
        ndim as i32,
        npoints as i32,
        points.as_mut_ptr(),
        0,
        std::ptr::null(),
        std::ptr::null_mut(),
    );

    qh_triangulate();

    let facet_list = unsafe { (*qh_qh()).facet_list };
    let mut triangles = Vec::new();

    let mut facet = facet_list;
    while !facet.is_null() {
        let vertices = unsafe { (*facet).vertices };
        let mut vertex_indices = Vec::new();
        let mut vertex = unsafe { (*vertices).e.next };
        while vertex != vertices {
            let index = unsafe { (*((*vertex).p as *const qhull_sys::vertexT)).id };
            vertex_indices.push(index);
            vertex = unsafe { (*vertex).next };
        }
        triangles.push(vertex_indices);
        facet = unsafe { (*facet).next };
    }

    let k = Array2::from_shape_fn((triangles.len(), ndim), |(i, j)| triangles[i][j] as i32);
    discrep_2(&k, triples)
}

struct Sphere3Hopf {
    // Implement Sphere3Hopf struct
}

impl Sphere3Hopf {
    fn new(_: Vec<i32>) -> Self {
        // Implement Sphere3Hopf::new
        Self {}
    }

    fn pop(&self) -> Array1<f64> {
        // Implement Sphere3Hopf::pop
        Array1::zeros(4)
    }
}

struct Sphere3 {
    // Implement Sphere3 struct
}

impl Sphere3 {
    fn new(_: Vec<i32>) -> Self {
        // Implement Sphere3::new
        Self {}
    }

    fn pop(&self) -> Array1<f64> {
        // Implement Sphere3::pop
        Array1::zeros(4)
    }
}

fn main() {
    let npoints = 2001;
    let ndim = 4;
    let triples_r = sample_spherical(npoints, ndim);
    let sphopfgen = Sphere3Hopf::new(vec![2, 3, 5]);
    let spgen = Sphere3::new(vec![2, 3, 5]);
    let triples_h = Array2::from_shape_fn((npoints, ndim), |_| sphopfgen.pop()[0]);
    let triples_s = Array2::from_shape_fn((npoints, ndim), |_| spgen.pop()[0]);

    let x: Vec<usize> = (100..npoints).step_by(100).collect();
    let mut res_r = Vec::new();
    let mut res_h = Vec::new();
    let mut res_s = Vec::new();

    for &i in &x {
        res_r.push(dispersion(&triples_r.slice(s![0..i, ..])));
        res_h.push(dispersion(&triples_h.slice(s![0..i, ..])));
        res_s.push(dispersion(&triples_s.slice(s![0..i, ..])));
    }

    // Plotting functionality is not implemented in this Rust version
    println!("x: {:?}", x);
    println!("res_r: {:?}", res_r);
    println!("res_h: {:?}", res_h);
    println!("res_s: {:?}", res_s);
}
