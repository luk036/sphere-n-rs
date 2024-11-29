# ⚽ sphere-n-rs

[![Crates.io](https://img.shields.io/crates/v/sphere-n-rs.svg)](https://crates.io/crates/sphere-n-rs)
[![Docs.rs](https://docs.rs/sphere-n-rs/badge.svg)](https://docs.rs/sphere-n-rs)
[![CI](https://github.com/luk036/sphere-n-rs/workflows/CI/badge.svg)](https://github.com/luk036/sphere-n-rs/actions)
[![codecov](https://codecov.io/gh/luk036/sphere-n-rs/graph/badge.svg?token=2naWVCVeCb)](https://codecov.io/gh/luk036/sphere-n-rs)

> Low Discrepancy Sequence for S_n in Rust

This library implements a generator for the generation of low-discrepancy sequences on n-dimensional spheres. Low-discrepancy sequences are utilized for the generation of points that are distributed uniformly across a given space. This technique is of significant value in a number of fields, including computer graphics, numerical integration, and Monte Carlo simulations.

The principal objective of this library is to facilitate the generation of points on the surface of spheres of varying dimensions, including three-dimensional and higher-dimensional spheres. The input required is the dimension of the sphere (n) and a set of base numbers to be used for the underlying sequence generation. The output is a series of vectors, with each vector representing a point on the surface of the n-dimensional sphere.

The library achieves this through a combination of mathematical calculations and recursive structures. The library utilizes a number of fundamental components, including:

1. The VdCorput sequence generator produces a sequence of numbers that are evenly distributed between 0 and 1.
2. Subsequently, the aforementioned numerical data is mapped onto the surface of a sphere through the use of interpolation functions.
3. The SphereGen module represents an abstract base class that defines the common interface for all sphere generators.
4. The recursive structures, namely Sphere3 and NSphere, facilitate the construction of higher-dimensional spheres from their lower-dimensional counterparts.

The primary logic flow begins with the construction of a SphereN object, which utilizes either a Sphere3 (for three-dimensional applications) or a recursive process to generate lower-dimensional spheres for higher dimensions. In the generation of points, the VdCorput sequence is employed to obtain a fundamental number, which is then subjected to a series of transformations involving the sine, cosine, and interpolation functions. This mapping is performed in order to place the fundamental number onto the surface of the sphere.

A salient feature of the library is its incorporation of caching (via the @cached attribute) and lazy initialization (through lazy_static) to augment performance by storing and reusing calculated values.
Moreover, the library provides traits and structures that facilitate the adaptable utilization of the sphere generators. The SphereGen trait establishes a common interface for disparate sphere generators, whereas the NSphere and SphereN structures implement the actual generation logic.

In conclusion, this library provides a sophisticated yet flexible approach to generating evenly distributed points on high-dimensional spheres, which can be advantageous in numerous scientific and computational applications.

## 🛠️ Installation

### 📦 Cargo

- Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install sphere-n-rs`

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
