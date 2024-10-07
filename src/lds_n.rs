extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Mutex;
use lds_rs::lds::{Circle, Sphere, VdCorput};

lazy_static! {
    static ref CACHE_ODD: Mutex<HashMap<i32, Vec<f64>>> = Mutex::new(HashMap::new());
    static ref CACHE_EVEN: Mutex<HashMap<i32, Vec<f64>>> = Mutex::new(HashMap::new());
}

const N_POINTS: usize = 300;

pub struct Globals {
    pub x: Vec<f64>,
    pub neg_cosine: Vec<f64>,
    pub sine: Vec<f64>,
}

impl Globals {
    fn new() -> Self {
        let mut x = vec![0.0; N_POINTS];
        let mut neg_cosine = vec![0.0; N_POINTS];
        let mut sine = vec![0.0; N_POINTS];

        for i in 0..N_POINTS {
            let x_val = (i as f64) * PI / (N_POINTS as f64 - 1.0);
            x[i] = x_val;
            neg_cosine[i] = -x_val.cos();
            sine[i] = x_val.sin();
        }

        Globals {
            x,
            neg_cosine,
            sine,
        }
    }
}

lazy_static! {
    static ref GL: Globals = Globals::new();
}

fn get_tp_odd(n: i32) -> Vec<f64> {
    let mut cache_odd = CACHE_ODD.lock().unwrap();
    if let Some(result) = cache_odd.get(&n) {
        return result.clone();
    }

    let result = if n == 1 {
        GL.neg_cosine.clone() // Adjusted to call static method, assuming its existence
    } else {
        let tp_minus_2 = get_tp_odd(n - 2);
        let mut result = vec![0.0; N_POINTS];
        for i in 0..N_POINTS {
            result[i] = ((n - 1) as f64 * tp_minus_2[i]
                + GL.neg_cosine[i] * GL.sine[i].powf((n - 1) as f64))
                / n as f64;
        }
        result
    };

    cache_odd.insert(n, result);
    cache_odd.get(&n).unwrap().clone()
}

fn get_tp_even(n: i32) -> Vec<f64> {
    let mut cache_even = CACHE_EVEN.lock().unwrap();
    if let Some(result) = cache_even.get(&n) {
        return result.clone();
    }

    let result = if n == 0 {
        GL.x.clone() // Adjusted to call static method, assuming its existence
    } else {
        let tp_minus_2 = get_tp_even(n - 2);
        let mut result = vec![0.0; N_POINTS];
        for i in 0..N_POINTS {
            result[i] = ((n - 1) as f64 * tp_minus_2[i]
                + GL.neg_cosine[i] * GL.sine[i].powf((n - 1) as f64))
                / n as f64;
        }
        result
    };

    cache_even.insert(n, result);
    cache_even.get(&n).unwrap().clone()
}

fn get_tp(n: i32) -> Vec<f64> {
    if n % 2 == 0 {
        get_tp_even(n)
    } else {
        get_tp_odd(n)
    }
}

pub struct Sphere3 {
    vdc: VdCorput,
    sphere2: Sphere,
}
pub struct SphereN {
    n: usize,
    vdc: VdCorput,
    s_gen: SphereVariant,
}
pub enum SphereVariant {
    Sphere3(Box<Sphere3>),
    SphereN(Box<SphereN>),
}
impl Sphere3 {
    pub fn new(base: &[usize]) -> Self {
        Sphere3 {
            vdc: VdCorput,
            sphere2: Sphere::new(&base[1..3]),
        }
    }
    pub fn pop(&self) -> [f64; 4] {
        let ti = 0.5 * PI * self.vdc.pop(); // Assuming implementation for vdc.pop()
        let tp = GL.get_tp(3).expect("Failed to get TP");
        let xi = interp(&GL.x, &tp, ti);
        let cosxi = xi.cos();
        let sinxi = xi.sin();
        let [s0, s1, s2] = self.sphere2.pop();
        [sinxi * s0, sinxi * s1, sinxi * s2, cosxi]
    }
}
impl SphereN {
    pub fn new(base: &[usize]) -> Self {
        let m = base.len();
        assert!(m >= 4, "Base size must be at least 4");
        let s_gen = if m == 4 {
            SphereVariant::Sphere3(Box::new(Sphere3::new(&base[1..])))
        } else {
            SphereVariant::SphereN(Box::new(SphereN::new(&base[1..])))
        };
        SphereN {
            n: m - 1,
            vdc: VdCorput,
            s_gen,
        }
    }
    pub fn pop(&self) -> Vec<f64> {
        let vd = self.vdc.pop(); // Assuming implementation for vdc.pop()
        let tp = GL.get_tp(self.n).expect("Failed to get TP");
        let ti = tp.first().unwrap() + (tp.last().unwrap() - tp.first().unwrap()) * vd;
        let xi = interp(&GL.x, &tp, ti);
        let sinphi = xi.sin();
        match &self.s_gen {
            SphereVariant::Sphere3(sphere3) => {
                let arr: [f64; 4] = sphere3.pop();
                arr.into_iter().map(|x| x * sinphi).chain(std::iter::once(xi.cos())).collect()
            },
            SphereVariant::SphereN(sphere_n) => {
                let mut res = sphere_n.pop();
                for elem in res.iter_mut() {
                    *elem *= sinphi;
                }
                res.push(xi.cos());
                res
            },
        }
    }
}

fn interp(x: &[f64], x_ref: &[f64], val: f64) -> f64 {
    // Implement the interp function according to Rust's conventions.
    // Placeholder for now.
    unimplemented!()
}

