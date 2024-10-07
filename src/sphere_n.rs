use interp::interp;
use lazy_static::lazy_static;
use lds_rs::lds::{Sphere, VdCorput};
use ndarray::Array1;
use std::f64::consts::PI;
// use std::sync::{Mutex, MutexGuard};
// use std::collections::HashMap;
use cached::proc_macro::cached;

lazy_static! {
    static ref X: Array1<f64> = Array1::linspace(0.0, PI, 300);
}

// lazy_static! {
//     static ref CACHE_ODD: Mutex<HashMap<usize, Array1<f64>>> = Mutex::new(HashMap::new());
//     static ref CACHE_EVEN: Mutex<HashMap<usize, Array1<f64>>> = Mutex::new(HashMap::new());
// }

/// The struct `Gl` in Rust contains three arrays of type `f64` representing `x`, `neg_cosine`, and
/// `sine`.
///
/// Properties:
///
/// * `x`: The `x` property in the `Gl` struct appears to be an array of floating-point numbers (`f64`).
/// It seems to represent some kind of data related to the struct.
/// * `neg_cosine`: The `neg_cosine` property in the `Gl` struct seems to be an array of floating-point
/// numbers (`f64`). It likely stores the negative cosine values for some calculations or processing
/// within the struct.
/// * `sine`: The `sine` property in the `Gl` struct is an `Array1<f64>` type, which likely represents
/// an array of floating-point numbers (f64) storing the sine values.
struct Gl {
    x: Array1<f64>,
    neg_cosine: Array1<f64>,
    sine: Array1<f64>,
    f2: Array1<f64>,
}

lazy_static! {
    static ref GL: Gl = Gl {
        x: X.clone(),
        neg_cosine: -X.mapv(f64::cos),
        sine: X.mapv(f64::sin),
        f2: X.mapv(|x| (x - x.cos() * x.sin()) / 2.0),
    };
}

#[cached]
fn get_tp_odd(n: u32) -> Array1<f64> {
    if n == 1 {
        GL.neg_cosine.clone() // Adjusted to call static method, assuming its existence
    } else {
        let tp_minus_2 = get_tp_odd(n - 2);

        (((n - 1) as f64) * &tp_minus_2
            + &GL.neg_cosine * &GL.sine.mapv(|x| x.powi((n - 1) as i32)))
            / (n as f64)
    }
}

#[cached]
fn get_tp_even(n: u32) -> Array1<f64> {
    if n == 0 {
        GL.x.clone() // Adjusted to call static method, assuming its existence
    } else {
        let tp_minus_2 = get_tp_even(n - 2);

        (((n - 1) as f64) * &tp_minus_2
            + &GL.neg_cosine * &GL.sine.mapv(|x| x.powi((n - 1) as i32)))
            / (n as f64)
    }
}

fn get_tp(n: u32) -> Array1<f64> {
    if n % 2 == 0 {
        get_tp_even(n)
    } else {
        get_tp_odd(n)
    }
}

/// The `SphereGen` trait in Rust defines a set of methods that need to be implemented by types that
/// want to be considered as generators for spheres. Here's a breakdown of the methods defined in the
/// `SphereGen` trait:
pub trait SphereGen {
    // fn new(base: &[usize]) -> Self;
    fn pop_vec(&mut self) -> Vec<f64>;
    fn reseed(&mut self, seed: usize);
    fn get_tp(&self) -> &Array1<f64>;
}

/// The `Sphere3` struct in Rust contains fields for VdCorput, Sphere, and an `Array1<f64>`.
///
/// Properties:
///
/// * `vdc`: The `vdc` property in the `Sphere3` struct is of type `VdCorput`.
/// * `sphere2`: The `sphere2` property in the `Sphere3` struct is of type `Sphere`. It seems to be a
/// reference to another struct named `Sphere`.
/// * `tp`: The `tp` property in the `Sphere3` struct is of type `Array1<f64>`, which is an array of
/// floating-point numbers with one dimension.
pub struct Sphere3 {
    vdc: VdCorput,
    sphere2: Sphere,
    tp: Array1<f64>,
}

impl Sphere3 {
    /// The function `new` constructs a new `Sphere3` object with specified parameters.
    ///
    /// Arguments:
    ///
    /// * `base`: The `base` parameter is an array of `usize` values that contains information needed to
    /// initialize a `Sphere3` object. It is used to create a new `Sphere3` object by passing specific
    /// values to initialize its internal components such as `VdCorput` and `Sphere`.
    ///
    /// Returns:
    ///
    /// A new `Sphere3` object is being returned from the `new` function.
    pub fn new(base: &[usize]) -> Self {
        Sphere3 {
            vdc: VdCorput::new(base[0]),
            sphere2: Sphere::new(&base[1..3]),
            // tp: 0.5 * (X.mapv(|x| x) - SINE.mapv(|x| x) + NEG_COSINE.mapv(|x| x)),
            tp: 0.5 * (&GL.x + &GL.sine * &GL.neg_cosine),
        }
    }

    /// The `pop` function in Rust calculates values based on input data and returns a 4-element array.
    ///
    /// Returns:
    ///
    /// The function `pop` returns an array of 4 `f64` values. The first three values are calculated
    /// based on some operations involving popping values from `self.vdc` and `self.sphere2`, and the
    /// last value is the cosine of the interpolated value `xi`. The array returned contains the values
    /// `[sinxi * s0, sinxi * s1, sinxi * s
    pub fn pop(&mut self) -> [f64; 4] {
        let ti = PI * self.vdc.pop(); // map to [0, pi];
        let xi = interp(&GL.f2.to_vec(), &X.to_vec(), ti);
        let cosxi = xi.cos();
        let sinxi = xi.sin();
        let [s0, s1, s2] = self.sphere2.pop();
        [sinxi * s0, sinxi * s1, sinxi * s2, cosxi]
    }
}

/// Generate Sphere-3 Low-discrepency sequence
///
/// # Examples
///
/// ```
/// use sphere_n_rs::Sphere3;
/// use sphere_n_rs::SphereGen;
/// use approx_eq::assert_approx_eq;
///
/// let mut sgen = Sphere3::new(&[2, 3, 5]);
/// sgen.reseed(10);
/// for _i in 0..10 {
///     println!("{:?}", sgen.pop());
/// }
/// let res = sgen.pop();
///
/// assert_approx_eq!(res[1], -0.24135188409032363);
/// ```
impl SphereGen for Sphere3 {
    #[inline]
    fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.sphere2.reseed(seed);
    }

    #[inline]
    fn pop_vec(&mut self) -> Vec<f64> {
        self.pop().to_vec()
    }

    #[inline]
    fn get_tp(&self) -> &Array1<f64> {
        &self.tp
    }
}

/// The `NSphere` struct represents a generator for Sphere-N Low-discrepency sequence.
///
/// Properties:
///
/// * `vdc`: The `vdc` property seems to be of type `VdCorput`, which is likely used for generating
/// Low-discrepency sequences. Low-discrepency sequences are deterministic sequences that are used for quasi-random
/// sampling. The `VdCorput` struct probably implements the Van der Corput sequence generation
/// algorithm.
/// * `s_gen`: The `s_gen` property in the `NSphere` struct is a Box containing a trait object that
/// implements the `SphereGen` trait. This allows for dynamic dispatch and the ability to store
/// different types that implement the `SphereGen` trait in the `NSphere` struct.
/// * `tp`: The `tp` property in the `NSphere` struct is of type `Array1<f64>`. It is used to store some
/// data related to the sphere generation process.
pub struct NSphere {
    vdc: VdCorput,
    s_gen: Box<dyn SphereGen>,
    n: u32,
    tp: Array1<f64>,
}

impl NSphere {
    /// The function `new` in Rust initializes a NSphere struct with specific parameters based on the
    /// input size and base array.
    ///
    /// Arguments:
    ///
    /// * `n`: The parameter `n` represents the dimensionality of the sphere being generated.
    /// * `base`: The `base` parameter is a slice of `usize` values that contains the base values used
    /// for generating the NSphere. The function `new` takes two parameters: `n`, which is the dimension
    /// of the NSphere, and `base`, which is a slice containing the base values needed for
    ///
    /// Returns:
    ///
    /// The `new` function returns an instance of the `NSphere` struct.
    pub fn new(n: u32, base: &[usize]) -> Self {
        assert!(n >= 3);
        let (s_gen, tp_minus2): (Box<dyn SphereGen>, Array1<f64>) = if n == 3 {
            (Box::new(Sphere3::new(&base[1..4])), GL.neg_cosine.clone())
        } else {
            let s_minus1 = NSphere::new(n - 1, &base[1..]);
            let ssn_minus2 = s_minus1.get_tp_minus1().clone();
            (Box::new(NSphere::new(n - 1, &base[1..])), ssn_minus2)
        };
        let tp = (((n - 1) as f64) * tp_minus2
            + &GL.neg_cosine * &GL.sine.mapv(|x| x.powi((n - 1) as i32)))
            / n as f64;
        NSphere {
            vdc: VdCorput::new(base[0]),
            s_gen,
            n,
            tp,
        }
    }

    /// The function `get_tp_minus1` returns a reference to an `Array1<f64>` obtained from calling the
    /// `get_tp` method on the `s_gen` field.
    ///
    /// Returns:
    ///
    /// The `get_tp_minus1` function is returning a reference to an `Array1<f64>` which is obtained by
    /// calling the `get_tp` method on the `s_gen` field of the struct or object that the function is
    /// defined on.
    #[inline]
    pub fn get_tp_minus1(&self) -> &Array1<f64> {
        self.s_gen.get_tp()
    }
}

/// Generate N-Sphere Low-discrepency sequence
///
/// # Examples
///
/// ```
/// use sphere_n_rs::NSphere;
/// use sphere_n_rs::SphereGen;
/// use approx_eq::assert_approx_eq;
///
/// let mut sgen = NSphere::new(3, &[2, 3, 5, 7]);
/// sgen.reseed(0);
/// let res = sgen.pop_vec();
///
/// assert_approx_eq!(res[0], 0.4809684718990214);
/// ```
impl SphereGen for NSphere {
    #[allow(dead_code)]
    fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        self.s_gen.reseed(seed);
    }

    fn get_tp(&self) -> &Array1<f64> {
        &self.tp
    }

    fn pop_vec(&mut self) -> Vec<f64> {
        let vd = self.vdc.pop();
        let tp = get_tp(self.n);
        let ti = tp[0] + (tp[tp.len() - 1] - tp[0]) * vd; // map to [t0, tm-1];
        let xi = interp(&tp.to_vec(), &X.to_vec(), ti);
        let sinphi = xi.sin();
        let mut res = self.s_gen.pop_vec();
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(xi.cos());
        res
    }
}

enum SphereVariant {
    // ForS2(Box<Sphere>),
    ForS3(Box<Sphere3>),
    ForSn(Box<SphereN>),
}

/// Generate N-Sphere Low-discrepency sequence
///
/// # Examples
///
/// ```
/// use sphere_n_rs::SphereN;
/// use approx_eq::assert_approx_eq;
///
/// let mut sgen = SphereN::new(3, &[2, 3, 5, 7]);
/// sgen.reseed(0);
/// let res = sgen.pop_vec();
///
/// assert_approx_eq!(res[0], 0.4809684718990214);
/// ```
pub struct SphereN {
    vdc: VdCorput,
    s_gen: SphereVariant,
    tp: Array1<f64>,
}

// static IntSinPowerTable sp {};
impl SphereN {
    pub fn new(n: usize, base: &[usize]) -> Self {
        assert!(n >= 3);
        let (s_gen, tp_minus2) = match n {
            3 => (
                SphereVariant::ForS3(Box::<Sphere3>::new(Sphere3::new(&base[1..4]))),
                GL.neg_cosine.clone(),
            ),
            _ => {
                let s_minus1 = SphereN::new(n - 1, &base[1..]);
                let ssn_minus2 = s_minus1.get_tp_minus1().clone();
                (
                    SphereVariant::ForSn(Box::<SphereN>::new(s_minus1)),
                    ssn_minus2,
                )
            }
        };
        let tp = (((n - 1) as f64) * tp_minus2
            + &GL.neg_cosine * &GL.sine.mapv(|x| x.powi((n - 1) as i32)))
            / n as f64;

        SphereN {
            vdc: VdCorput::new(base[0]),
            s_gen,
            tp,
        }
    }

    pub fn get_tp(&self) -> &Array1<f64> {
        &self.tp
    }

    pub fn get_tp_minus1(&self) -> &Array1<f64> {
        match &self.s_gen {
            // SphereVariant::ForS2(gen_2) => { X },
            SphereVariant::ForS3(gen_3) => gen_3.get_tp(),
            SphereVariant::ForSn(gen_n) => gen_n.get_tp(),
        }
    }

    pub fn pop_vec(&mut self) -> Vec<f64> {
        let vd = self.vdc.pop();
        let ti = self.tp[0] + (self.tp[self.tp.len() - 1] - self.tp[0]) * vd; // map to [t0, tm-1];
        let xi = interp(&self.tp.to_vec(), &X.to_vec(), ti);
        let sinphi = xi.sin();
        let mut res = match &mut self.s_gen {
            SphereVariant::ForS3(gen_3) => gen_3.pop().to_vec(),
            SphereVariant::ForSn(gen_n) => gen_n.pop_vec(),
        };
        for xi in res.iter_mut() {
            *xi *= sinphi;
        }
        res.push(xi.cos());
        res
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc.reseed(seed);
        match &mut self.s_gen {
            // SphereVariant::ForS2(gen_2) => { X },
            SphereVariant::ForS3(gen_3) => gen_3.reseed(seed),
            SphereVariant::ForSn(gen_n) => gen_n.reseed(seed),
        }
    }
}
