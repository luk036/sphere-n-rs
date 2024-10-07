use std::f64::consts::PI;
use std::sync::Mutex;
use cached::proc_macro::cached;
use ndarray::{Array1, Array};
use std::f64;
use std::sync::Arc;

const X: Array1<f64> = Array::linspace(0.0, PI, 300);
const NEG_COSINE: Array1<f64> = Array::from_iter(X.iter().map(|&x| -x.cos()));
const SINE: Array1<f64> = Array::from_iter(X.iter().map(|&x| x.sin()));

#[cached]
fn get_tp_odd(n: usize) -> Array1<f64> {
    if n == 1 {
        return NEG_COSINE.clone();
    }
    let tp_minus2 = get_tp_odd(n - 2);
    (tp_minus2 * (n - 1) as f64 + &NEG_COSINE * SINE.mapv(|x| x.powi((n - 1) as i32))) / n as f64
}

#[cached]
fn get_tp_even(n: usize) -> Array1<f64> {
    if n == 0 {
        return X.clone();
    }
    let tp_minus2 = get_tp_even(n - 2);
    (tp_minus2 * (n - 1) as f64 + &NEG_COSINE * SINE.mapv(|x| x.powi((n - 1) as i32))) / n as f64
}

fn get_tp(n: usize) -> Array1<f64> {
    if n % 2 == 0 {
        get_tp_even(n)
    } else {
        get_tp_odd(n)
    }
}

trait SphereGen {
    fn pop(&mut self) -> Vec<f64>;
    fn reseed(&mut self, seed: u64);
}

struct Sphere3 {
    vdc: VdCorput,
    sphere2: Sphere,
}

impl Sphere3 {
    fn new(base: Vec<usize>) -> Self {
        Sphere3 {
            vdc: VdCorput::new(base[0]),
            sphere2: Sphere::new(base[1..3].to_vec()),
        }
    }
}

impl SphereGen for Sphere3 {
    fn pop(&mut self) -> Vec<f64> {
        let ti = PI * self.vdc.pop();
        let xi = interp(ti, &get_tp_even(2), &X);
        let cosxi = xi.cos();
        let sinxi = xi.sin();
        let mut result = self.sphere2.pop().iter().map(|&s| sinxi * s).collect::<Vec<f64>>();
        result.push(cosxi);
        result
    }

    fn reseed(&mut self, seed: u64) {
        self.vdc.reseed(seed);
        self.sphere2.reseed(seed);
    }
}

struct SphereN {
    vdc: VdCorput,
    s_gen: Box<dyn SphereGen>,
    n: usize,
    range: f64,
}

impl SphereN {
    fn new(base: Vec<usize>) -> Self {
        let n = base.len() - 1;
        assert!(n >= 2);
        let s_gen: Box<dyn SphereGen> = if n == 2 {
            Box::new(Sphere::new(base[1..3].to_vec()))
        } else {
            Box::new(SphereN::new(base[1..].to_vec()))
        };
        let tp = get_tp(n);
        let range = tp[tp.len() - 1] - tp[0];
        SphereN {
            vdc: VdCorput::new(base[0]),
            s_gen,
            n,
            range,
        }
    }
}

impl SphereGen for SphereN {
    fn pop(&mut self) -> Vec<f64> {
        let vd = self.vdc.pop();
        let tp = get_tp(self.n);
        let ti = tp[0] + self.range * vd;
        let xi = interp(ti, &tp, &X);
        let sinphi = xi.sin();
        let mut result = self.s_gen.pop().iter().map(|&xi| xi * sinphi).collect::<Vec<f64>>();
        result.push(xi.cos());
        result
    }

    fn reseed(&mut self, seed: u64) {
        self.vdc.reseed(seed);
        self.s_gen.reseed(seed);
    }
}

fn interp(x: f64, xp: &Array1<f64>, fp: &Array1<f64>) -> f64 {
    let idx = xp.iter().position(|&v| v >= x).unwrap_or(xp.len() - 1);
    if idx == 0 {
        return fp[0];
    }
    if idx == xp.len() {
        return fp[xp.len() - 1];
    }
    let x0 = xp[idx - 1];
    let x1 = xp[idx];
    let y0 = fp[idx - 1];
    let y1 = fp[idx];
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

fn main() {
    // Initialize global arrays
    initialize_global_arrays();

    // Test code would go here
}

fn initialize_global_arrays() {
    // This function is a placeholder to initialize global arrays if needed
}
