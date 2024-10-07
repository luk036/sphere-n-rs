use interp::interp;
use lazy_static::lazy_static;
use lds_rs::lds::{Circle, Sphere, VdCorput};
use ndarray::Array1;

const PI: f64 = std::f64::consts::PI;

pub trait CylindGen {
    // fn new(base: &[usize]) -> Self;
    fn pop_vec(&mut self) -> Vec<f64>;
    fn reseed(&mut self, seed: usize);
}

impl CylindGen for Circle {
    fn pop_vec(&mut self) -> Vec<f64> {
        self.pop().to_vec()
    }

    fn reseed(&mut self, seed: usize) {
        self.reseed(seed);
    }
}

/** Generate using cylindrical coordinate method */
pub struct CylindN {
    vdc: VdCorput,
    c_gen: Box<dyn CylindGen>,
}

impl CylindN {
    #[allow(dead_code)]
    pub fn new(n: usize, base: &[usize]) -> Self {
        assert!(n >= 2);
        let c_gen: Box<dyn CylindGen> = if n == 2 {
            Box::new(Circle::new(base[1]))
        } else {
            Box::new(CylindN::new(n - 1, &base[1..]))
        };
        CylindN {
            vdc: VdCorput::new(base[0]),
            c_gen,
        }
    }
}

impl CylindGen for CylindN {
    fn pop_vec(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();
        let mut res = self.c_gen.pop_vec();
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(cosphi);
        res
    }

    fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.c_gen.reseed(seed);
    }
}
