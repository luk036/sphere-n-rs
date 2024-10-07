pub mod cylind_n;
pub mod sphere_n;

pub use crate::cylind_n::{CylindGen, CylindN, NCylind};
pub use crate::sphere_n::{NSphere, Sphere3, SphereGen, SphereN};

#[cfg(test)]
mod tests {
    use super::cylind_n::*;
    use super::sphere_n::*;
    use approx_eq::assert_approx_eq;

    use lds_rs::lds::PRIME_TABLE;

    #[test]
    fn test_sphere3() {
        let base: [usize; 5] = [2, 3, 5, 7, 11];

        let mut sgen = Sphere3::new(&base);
        sgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", sgen.pop());
        }
        let res = sgen.pop();
        assert_approx_eq!(res[1], -0.24135188409032363);
    }

    #[test]
    fn test_cylin_n() {
        let mut cgen = CylindN::new(5, &PRIME_TABLE);
        cgen.reseed(0);
        for _i in 0..10 {
            println!("{:?}", cgen.pop_vec());
        }
        let res = cgen.pop_vec();
        assert_approx_eq!(res[1], 0.032662755534715766);
    }

    #[test]
    fn test_cylind_n() {
        let mut cgen = CylindN::new(5, &PRIME_TABLE);
        cgen.reseed(0);
        for _i in 0..10 {
            println!("{:?}", cgen.pop_vec());
        }
        let res = cgen.pop_vec();
        assert_approx_eq!(res[1], 0.032662755534715766);
    }

    #[test]
    fn test_sphere_n() {
        let mut sgen = SphereN::new(3, &PRIME_TABLE);
        sgen.reseed(0);
        let res = sgen.pop_vec();
        assert_approx_eq!(res[0], 0.4809684718990214);
    }

    // #[test]
    // fn test_n_sphere() {
    //     let mut sgen = NSphere::new(4, &PRIME_TABLE);
    //     sgen.reseed(0);
    //     for _i in 0..10 {
    //         println!("{:?}", sgen.pop_vec());
    //     }
    //     let res = sgen.pop_vec();
    //     assert_approx_eq!(res[0], 0.006903401092767657);
    // }
}
