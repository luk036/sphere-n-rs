use std::f64::consts::PI;
use std::cmp::Ordering;
use std::collections::HashMap;

fn neg_cosine(x: f64) -> f64 {
    -x.cos()
}

fn sine(x: f64) -> f64 {
    x.sin()
}

fn get_tp(n: i32) -> Vec<f64> {
    let x: Vec<f64> = (0..300).map(|i| i as f64 * PI / 299.0).collect();
    if n == 0 {
        return x;
    }
    if n == 1 {
        return x.iter().map(|&xi| neg_cosine(xi)).collect();
    }
    let tp_minus2 = get_tp(n - 2);
    let sine_n_minus_1: Vec<f64> = x.iter().map(|&xi| sine(xi).powi(n - 1)).collect();
    let numerator: Vec<f64> = tp_minus2.iter().zip(sine_n_minus_1.iter()).map(|(&tp, &sine_n_minus_1)| (n - 1) as f64 * tp + neg_cosine(x) * sine_n_minus_1).collect();
    numerator.iter().map(|&numerator| numerator / n as f64).collect()
}

struct Sphere3 {
    vdc: VdCorput,
    sphere2: Sphere,
}

impl Sphere3 {
    fn new(base: &[i32]) -> Self {
        Self {
            vdc: VdCorput::new(base[0]),
            sphere2: Sphere::new(&base[1..3]),
        }
    }

    fn pop(&mut self) -> Vec<f64> {
        let ti = PI / 2.0 * self.vdc.pop();
        let xi = get_tp(2).binary_search_by(|&tp| tp.partial_cmp(&ti).unwrap_or(Ordering::Equal)).unwrap_or_else(|x| x) as f64;
        let cosxi = xi.cos();
        let sinxi = xi.sin();
        self.sphere2.pop().iter().map(|&s| s * sinxi).chain(std::iter::once(cosxi)).collect()
    }
}

struct SphereN {
    vdc: VdCorput,
    s_gen: Box<dyn SphereTrait>,
    n: i32,
    range: f64,
}

impl SphereN {
    fn new(n: i32, base: &[i32]) -> Self {
        assert!(n >= 2);
        Self {
            vdc: VdCorput::new(base[0]),
            s_gen: if n == 2 {
                Box::new(Sphere::new(&base[1..3]))
            } else {
                Box::new(SphereN::new(n - 1, &base[1..]))
            },
            n,
            range: get_tp(n).last().unwrap() - get_tp(n).first().unwrap(),
        }
    }

    fn pop(&mut self) -> Vec<f64> {
        let vd = self.vdc.pop();
        let tp = get_tp(self.n);
        let ti = tp.first().unwrap() + self.range * vd;
        let xi = get_tp(self.n).binary_search_by(|&tp| tp.partial_cmp(&ti).unwrap_or(Ordering::Equal)).unwrap_or_else(|x| x) as f64;
        let sinphi = xi.sin();
        self.s_gen.pop().iter().map(|&xi| xi * sinphi).chain(std::iter::once(xi.cos())).collect()
    }
}

trait SphereTrait {
    fn pop(&mut self) -> Vec<f64>;
}

struct Sphere {
    x: f64,
    y: f64,
}

impl Sphere {
    fn new(base: &[i32]) -> Self {
        Self {
            x: base[0] as f64 / 30269.0,
            y: base[1] as f64 / 30307.0,
        }
    }
}

impl SphereTrait for Sphere {
    fn pop(&mut self) -> Vec<f64> {
        let x2 = self.x * 2.0;
        let y2 = self.y * 2.0;
        let s1 = (x2 - 1.0).powi(2) + (y2 - 1.0).powi(2);
        let s2 = (x2 + 1.0).powi(2) + (y2 + 1.0).powi(2);
        let s3 = (x2 - 1.0).powi(2) + (y2 + 1.0).powi(2);
        let s4 = (x2 + 1.0).powi(2) + (y2 - 1.0).powi(2);
        let min_s = [s1, s2, s3, s4].iter().cloned().fold(std::f64::INFINITY, f64::min);
        let (x, y) = if min_s == s1 {
            (self.x - 1.0, self.y - 1.0)
        } else if min_s == s2 {
            (self.x + 1.0, self.y + 1.0)
        } else if min_s == s3 {
            (self.x - 1.0, self.y + 1.0)
        } else {
            (self.x + 1.0, self.y - 1.0)
        };
        self.x = x;
        self.y = y;
        vec![x, y]
    }
}

struct VdCorput {
    base: i32,
    vdc: i32,
    inv_base: f64,
}

impl VdCorput {
    fn new(base: i32) -> Self {
        Self {
            base,
            vdc: 0,
            inv_base: 1.0 / base as f64,
        }
    }

    fn pop(&mut self) -> f64 {
        let mut r = 0.0;
        let mut i = 1;
        let mut vdc = self.vdc;
        while vdc > 0 {
            r += (vdc % self.base) as f64 * self.inv_base.powi(i);
            vdc /= self.base;
            i += 1;
        }
        self.vdc += 1;
        r
    }
}

