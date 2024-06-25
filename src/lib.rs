pub mod sphere_n;

pub use crate::sphere_n::{CylinN, CylindN, NSphere, Sphere3, SphereN};
pub use crate::sphere_n::{Cylind, SphereGen}; // Traits

#[cfg(test)]
mod tests {
    use super::sphere_n::*;
    use approx_eq::assert_approx_eq;

    use lds_rs::lds::PRIME_TABLE;
    use lds_rs::lds::{Circle, Halton, Sphere, Sphere3Hopf, VdCorput};
    use lds_rs::lds_n::HaltonN;

    #[test]
    fn test_vdcorput() {
        let mut vgen = VdCorput::new(2);
        vgen.reseed(10);
        for _i in 0..10 {
            println!("{}", vgen.pop());
        }
        let res = vgen.pop();
        assert_approx_eq!(res, 0.65625);
    }

    #[test]
    fn test_circle() {
        let mut cgen = Circle::new(2);
        cgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", cgen.pop());
        }
        let res = cgen.pop();
        assert_approx_eq!(res[0], -0.8314696123025452);
    }

    #[test]
    fn test_halton() {
        let mut hgen = Halton::new(2, 3);
        hgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", hgen.pop());
        }
        let res = hgen.pop();
        assert_approx_eq!(res[0], 0.65625);
    }

    #[test]
    fn test_sphere() {
        let base: [usize; 5] = [2, 3, 5, 7, 11];

        let mut sgen = Sphere::new(&base);
        sgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", sgen.pop());
        }
        let res = sgen.pop();
        assert_approx_eq!(res[0], 0.8722297870746605);
    }

    #[test]
    fn test_sphere3hopf() {
        let base: [usize; 5] = [2, 3, 5, 7, 11];

        let mut s3fgen = Sphere3Hopf::new(&base);
        s3fgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", s3fgen.pop());
        }
        let res = s3fgen.pop();
        assert_approx_eq!(res[0], 0.23764785962349413);
    }

    #[test]
    fn test_sphere3() {
        let base: [usize; 5] = [2, 3, 5, 7, 11];

        let mut sgen = Sphere3::new(&base);
        sgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", sgen.pop());
        }
        let res = sgen.pop();
        assert_approx_eq!(res[0], 0.5799062768626047);
    }

    #[test]
    fn test_halton_n() {
        let mut hgen = HaltonN::new(5, &PRIME_TABLE);
        hgen.reseed(10);
        for _i in 0..10 {
            println!("{:?}", hgen.pop_vec());
        }
        let res = hgen.pop_vec();
        assert_approx_eq!(res[0], 0.65625);
    }

    #[test]
    fn test_cylin_n() {
        let mut cgen = CylinN::new(5, &PRIME_TABLE);
        cgen.reseed(0);
        for _i in 0..10 {
            println!("{:?}", cgen.pop_vec());
        }
        let res = cgen.pop_vec();
        assert_approx_eq!(res[0], 0.032662755534715766);
    }

    #[test]
    fn test_cylind_n() {
        let mut cgen = CylindN::new(5, &PRIME_TABLE);
        cgen.reseed(0);
        for _i in 0..10 {
            println!("{:?}", cgen.pop_vec());
        }
        let res = cgen.pop_vec();
        assert_approx_eq!(res[0], 0.032662755534715766);
    }

    #[test]
    fn test_sphere_n() {
        let mut sgen = SphereN::new(4, &PRIME_TABLE);
        sgen.reseed(0);
        for _i in 0..10 {
            println!("{:?}", sgen.pop_vec());
        }
        let res = sgen.pop_vec();
        assert_approx_eq!(res[0], 0.006903401092767657);
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
