pub mod cylind_n;
pub mod sphere_n;

pub use crate::cylind_n::{CylindGen, CylindN};
pub use crate::sphere_n::{Sphere3, SphereGen, SphereN};

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
            sgen.pop();
        }
        let res = sgen.pop();
        assert_approx_eq!(res[0], -0.4797401941417802);
        assert_approx_eq!(res[1], 0.5799062768626047);
        assert_approx_eq!(res[2], -0.609958496658014);
        assert_approx_eq!(res[3], -0.24800945251109802);
    }

    #[test]
    fn test_cylind_n() {
        let mut cgen = CylindN::new(5, &PRIME_TABLE);
        cgen.reseed(0);
        for _i in 0..10 {
            cgen.pop_vec();
        }
        let res = cgen.pop_vec();
        assert_approx_eq!(res[0], 0.6284456558612375);
        assert_approx_eq!(res[1], 0.032662755534715766);
        assert_approx_eq!(res[2], 0.11758502909407893);
        assert_approx_eq!(res[3], -0.31367724756166177);
        assert_approx_eq!(res[4], 0.3180323054739925);
        assert_approx_eq!(res[5], 0.625);
    }

    #[test]
    fn test_sphere_n() {
        let mut sgen = SphereN::new(5, &PRIME_TABLE);
        sgen.reseed(0);
        let res = sgen.pop_vec();
        assert_approx_eq!(res[0], 0.361367435929826);
        assert_approx_eq!(res[1], 0.18966026119111232);
        assert_approx_eq!(res[2], -0.5807571031092044);
        assert_approx_eq!(res[3], 0.531081205910399);
        assert_approx_eq!(res[4], 0.4171022266758069);
        assert_approx_eq!(res[5], 0.20035526944272675);
        assert_approx_eq!(res[6], 6.123233995736766e-17);
    }

    #[test]
    fn test_reseed() {
        let mut sgen = SphereN::new(5, &PRIME_TABLE);
        sgen.reseed(0);
        let res1 = sgen.pop_vec();
        sgen.reseed(0);
        let res2 = sgen.pop_vec();
        assert_eq!(res1, res2);
    }

    #[test]
    fn test_normalized() {
        let mut sgen = SphereN::new(5, &PRIME_TABLE);
        sgen.reseed(0);
        let res = sgen.pop_vec();
        let norm_sq: f64 = res.iter().map(|&x| x * x).sum();
        assert_approx_eq!(norm_sq, 1.0);
    }

    #[test]
    fn test_sphere_n_3d() {
        let mut sgen = SphereN::new(3, &PRIME_TABLE);
        sgen.reseed(0);
        let res = sgen.pop_vec();
        assert_approx_eq!(res[0], 0.4809684718990214);
        assert_approx_eq!(res[1], 0.6031153874276115);
        assert_approx_eq!(res[2], -0.5785601510223212);
        assert_approx_eq!(res[3], 0.2649326520763179);
    }

    #[test]
    fn test_cylind_n_2d() {
        let mut cgen = CylindN::new(2, &PRIME_TABLE);
        cgen.reseed(0);
        let res = cgen.pop_vec();
        assert_approx_eq!(res[0], -0.5);
        assert_approx_eq!(res[1], 0.8660254037844387);
    }
}
