use std::f64::consts::{PI, FRAC_PI_2};
use std::cmp::Ordering;

struct VdCorput {
    base: u32,
    index: u32,
}

impl VdCorput {
    fn new(base: u32) -> Self {
        VdCorput {
            base,
            index: 0,
        }
    }

    fn pop(&mut self) -> f64 {
        let mut n = self.index;
        let mut result = 0.0;
        let mut denom = 1.0;
        while n > 0 {
            denom *= self.base as f64;
            result += (n % self.base) as f64 / denom;
            n /= self.base;
        }
        self.index += 1;
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.index = seed;
    }
}

struct Circle {
    base: u32,
    vdc: VdCorput,
    c_gen: Option<Box<Circle>>,
}

impl Circle {
    fn new(base: u32) -> Self {
        Circle {
            base,
            vdc: VdCorput::new(base),
            c_gen: None,
        }
    }

    fn pop(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0;
        let sinphi = (1.0 - cosphi * cosphi).sqrt();
        let mut result = vec![sinphi];
        if let Some(ref mut c_gen) = self.c_gen {
            result.extend(c_gen.pop());
        }
        result.push(cosphi);
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
        if let Some(ref mut c_gen) = self.c_gen {
            c_gen.reseed(seed);
        }
    }
}

struct CylinN {
    base: Vec<u32>,
    vdc: VdCorput,
    c_gen: Circle,
}

impl CylinN {
    fn new(base: Vec<u32>) -> Self {
        let n = base.len();
        CylinN {
            base,
            vdc: VdCorput::new(base[0]),
            c_gen: if n == 1 {
                Circle::new(base[1])
            } else {
                CylinN::new(base[1..].to_vec())
            },
        }
    }

    fn pop(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0;
        let sinphi = (1.0 - cosphi * cosphi).sqrt();
        let mut result = self.c_gen.pop();
        result.iter_mut().for_each(|xi| *xi *= sinphi);
        result.push(cosphi);
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
        self.c_gen.reseed(seed);
    }
}

fn get_tp(n: usize) -> Vec<f64> {
    let mut tp = vec![0.0; n];
    if n == 0 {
        return tp;
    }
    if n == 1 {
        for (i, x) in tp.iter_mut().enumerate() {
            *x = -(-PI + i as f64 * 2.0 * PI / 299.0).cos();
        }
        return tp;
    }
    let tp_minus2 = get_tp(n - 2);
    for (i, x) in tp.iter_mut().enumerate() {
        *x = ((n - 1) as f64 * tp_minus2[i] + (-PI + i as f64 * 2.0 * PI / 299.0).cos() * (-(-PI + i as f64 * 2.0 * PI / 299.0).sin()).powi(n as i32 - 1)) / n as f64;
    }
    tp
}

struct Sphere {
    base: Vec<u32>,
    vdc: VdCorput,
    sphere2: Sphere2,
}

impl Sphere {
    fn new(base: Vec<u32>) -> Self {
        Sphere {
            base,
            vdc: VdCorput::new(base[0]),
            sphere2: Sphere2::new(base[1..].to_vec()),
        }
    }

    fn pop(&mut self) -> Vec<f64> {
        let ti = FRAC_PI_2 * self.vdc.pop();
        let xi = interpolate(ti, &get_tp(2), &X);
        let cosxi = xi.cos();
        let sinxi = xi.sin();
        let mut result = self.sphere2.pop();
        result.iter_mut().for_each(|s| *s *= sinxi);
        result.push(cosxi);
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
        self.sphere2.reseed(seed);
    }
}

struct Sphere2 {
    base: Vec<u32>,
    vdc: VdCorput,
}

impl Sphere2 {
    fn new(base: Vec<u32>) -> Self {
        Sphere2 {
            base,
            vdc: VdCorput::new(base[0]),
        }
    }

    fn pop(&mut self) -> Vec<f64> {
        let mut result = vec![0.0; self.base.len()];
        for (i, s) in result.iter_mut().enumerate() {
            *s = self.vdc.pop();
        }
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
    }
}

struct SphereN {
    base: Vec<u32>,
    vdc: VdCorput,
    s_gen: Sphere,
    n: usize,
    range: f64,
}

impl SphereN {
    fn new(n: usize, base: Vec<u32>) -> Self {
        SphereN {
            base,
            vdc: VdCorput::new(base[0]),
            s_gen: if n == 2 {
                Sphere::new(base[1..].to_vec())
            } else {
                SphereN::new(n - 1, base[1..].to_vec())
            },
            n,
            range: get_tp(n).last().unwrap() - get_tp(n).first().unwrap(),
        }
    }

    fn pop(&mut self) -> Vec<f64> {
        let vd = self.vdc.pop();
        let tp = get_tp(self.n);
        let ti = tp.first().unwrap() + self.range * vd;
        let xi = interpolate(ti, &tp, &X);
        let sinphi = xi.sin();
        let mut result = self.s_gen.pop();
        result.iter_mut().for_each(|xi| *xi *= sinphi);
        result.push(xi.cos());
        result
    }

    fn reseed(&mut self, seed: u32) {
        self.vdc.reseed(seed);
        self.s_gen.reseed(seed);
    }
}

fn interpolate(x: f64, xp: &[f64], fp: &[f64]) -> f64 {
    match xp.binary_search_by(|&v| v.partial_cmp(&x).unwrap_or(Ordering::Less)) {
        Ok(i) => fp[i],
        Err(i) if i > 0 => {
            let x0 = xp[i - 1];
            let x1 = xp[i];
            let f0 = fp[i - 1];
            let f1 = fp[i];
            f0 + (f1 - f0) * (x - x0) / (x1 - x0)
        }
        _ => unreachable!(),
    }
}

const X: [f64; 300] = {
    let mut x = [0.0; 300];
    for (i, xi) in x.iter_mut().enumerate() {
        *xi = -PI + i as f64 * 2.0 * PI / 299.0;
    }
    x
};

fn main() {
    let mut cgen = CylinN::new(vec![2, 3, 5, 7]);
    cgen.reseed(0);
    for _ in 0..1 {
        println!("{:?}", cgen.pop());
    }

    let mut sgen = SphereN::new(3, vec![2, 3, 5]);
    sgen.reseed(0);
    for _ in 0..1 {
        println!("{:?}", sgen.pop());
    }
}
