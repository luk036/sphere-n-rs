use lds_rs::lds::{Circle, VdCorput};
// use std::sync::{Mutex, MutexGuard};
// use std::collections::HashMap;

enum CylindVariant {
    For2(Box<Circle>),
    ForN(Box<NCylind>),
}

/// Generate N-Sphere using cylindrical coordinate method */
pub struct NCylind {
    vdc: VdCorput,
    c_gen: CylindVariant,
}

/// Generate N-Sphere using cylindrical coordinate method */
///
/// # Examples
///
/// ```
/// use sphere_n_rs::NCylind;
/// use approx_eq::assert_approx_eq;
///
/// let mut cgen = NCylind::new(5, &[2, 3, 5, 7, 11, 13]);
/// cgen.reseed(0);
/// for _i in 0..10 {
///     println!("{:?}", cgen.pop_vec());
/// }
/// let res = cgen.pop_vec();
///
/// assert_approx_eq!(res[1], 0.032662755534715766);
/// ```
impl NCylind {
    pub fn new(n: usize, base: &[usize]) -> Self {
        assert!(n >= 2);
        let c_gen = if n == 2 {
            CylindVariant::For2(Box::<Circle>::new(Circle::new(base[1])))
        } else {
            CylindVariant::ForN(Box::<NCylind>::new(NCylind::new(n - 1, &base[1..])))
        };
        NCylind {
            vdc: VdCorput::new(base[0]),
            c_gen,
        }
    }

    /**
     * @brief
     *
     * @return `Vec<f64>`
     */
    pub fn pop_vec(&mut self) -> Vec<f64> {
        let cosphi = 2.0 * self.vdc.pop() - 1.0; // map to [-1, 1];
        let sinphi = (1.0 - cosphi * cosphi).sqrt();

        // ???
        let mut res = match &mut self.c_gen {
            CylindVariant::For2(gen_2) => gen_2.pop().to_vec(),
            CylindVariant::ForN(gen_n) => gen_n.pop_vec(),
        };
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(cosphi);
        res
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        match &mut self.c_gen {
            // SphereVariant::ForS2(gen_2) => { X },
            CylindVariant::For2(gen_2) => gen_2.reseed(seed),
            CylindVariant::ForN(gen_n) => gen_n.reseed(seed),
        }
    }
}

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

/// Generate N-Sphere using cylindrical coordinate method */
///
/// # Examples
///
/// ```
/// use sphere_n_rs::CylindN;
/// use sphere_n_rs::CylindGen;
/// use approx_eq::assert_approx_eq;
///
/// let mut cgen = CylindN::new(5, &[2, 3, 5, 7, 11, 13]);
/// cgen.reseed(0);
/// for _i in 0..10 {
///     println!("{:?}", cgen.pop_vec());
/// }
/// let res = cgen.pop_vec();
///
/// assert_approx_eq!(res[1], 0.032662755534715766);
/// ```
impl CylindN {
    /**
     * @brief Construct a new cylin n::cylin n object
     *
     */
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
