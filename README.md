# 🏐 sphere-n-rs

[![Crates.io](https://img.shields.io/crates/v/sphere-n-rs.svg)](https://crates.io/crates/sphere-n-rs)
[![Docs.rs](https://docs.rs/sphere-n-rs/badge.svg)](https://docs.rs/sphere-n-rs)
[![CI](https://github.com/luk036/sphere-n-rs/workflows/CI/badge.svg)](https://github.com/luk036/sphere-n-rs/actions)
[![codecov](https://codecov.io/gh/luk036/sphere-n-rs/graph/badge.svg?token=2naWVCVeCb)](https://codecov.io/gh/luk036/sphere-n-rs)

> Low Discrepancy Sequence for S_n in Rust

This repository implements a generator for creating low-discrepancy sequences on n-dimensional spheres. Low-discrepancy sequences are used to generate points that are evenly distributed across a space, which is useful in various fields like computer graphics, numerical integration, and Monte Carlo simulations.

The main purpose of this repository is to provide a way to generate points on the surface of spheres of different dimensions (3D and higher). It takes as input the dimension of the sphere (n) and a set of base numbers used for the underlying sequence generation. The output is a series of vectors, where each vector represents a point on the surface of the n-dimensional sphere.

The repository achieves this through a combination of mathematical calculations and recursive structures. It uses several key components:

1. The VdCorput sequence generator, which produces evenly distributed numbers between 0 and 1.
2. Interpolation functions to map these numbers onto the surface of a sphere.
3. Recursive structures (Sphere3 and NSphere) to build up from lower dimensions to higher ones.

The main logic flow starts with the creation of a SphereN object, which internally uses either a Sphere3 (for 3D) or recursively creates lower-dimensional spheres for higher dimensions. When generating points, it uses the VdCorput sequence to get a base number, then applies various transformations involving sine, cosine, and interpolation to map this onto the sphere's surface.

An important aspect of the repository is its use of caching (with the @cached attribute) and lazy initialization (with lazy_static) to improve performance by storing and reusing calculated values.

The repository also provides traits and structures to allow for flexible use of the sphere generators. The SphereGen trait defines a common interface for different sphere generators, while the NSphere and SphereN structures implement the actual generation logic.

Overall, this repository provides a sophisticated yet flexible way to generate evenly distributed points on high-dimensional spheres, which can be valuable in many scientific and computational applications.

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
