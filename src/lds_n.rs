extern crate lazy_static;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::{Mutex};

lazy_static! {
    static ref CACHE_ODD: Mutex<HashMap<i32, Vec<f64>>> = Mutex::new(HashMap::new());
    static ref CACHE_EVEN: Mutex<HashMap<i32, Vec<f64>>> = Mutex::new(HashMap::new());
}

const HALF_PI: f64 = PI / 2.0;
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
        // Assuming there's a way to retrieve X statically or this needs adjustment
        unimplemented!("Static retrieval of 'X' vector")
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

// Note: The above assumes static methods exist to fetch neg_cosine and sine vectors.
// Adjustments are needed depending on how you want to manage global state in Rust.
