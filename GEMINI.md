# Gemini Code Understanding

## Project Overview

This project is a Rust library named `sphere-n-rs` for generating low-discrepancy sequences of points on the surface of n-dimensional spheres. Low-discrepancy sequences are useful in applications requiring uniformly distributed points, such as Monte Carlo simulations, numerical integration, and computer graphics.

The library provides two main approaches for point generation:

1.  **Recursive Sphere Method:** Implemented in `src/sphere_n.rs`, this method constructs higher-dimensional spheres by recursively building upon lower-dimensional ones, starting from a base 3D sphere.
2.  **Cylindrical Coordinate Method:** Implemented in `src/cylind_n.rs`, this method uses a recursive cylindrical coordinate-based approach to generate points on the sphere.

The core logic relies on the `lds-rs` crate for generating the underlying low-discrepancy sequences (specifically, the Van der Corput sequence) and `ndarray` for numerical computations. The library also uses `cached` and `lazy_static` to optimize performance by caching and reusing calculated values.

## Building and Running

### Building

The project is a standard Rust library and can be built using Cargo:

```bash
cargo build
```

### Running Tests

The project includes a suite of tests to ensure the correctness of the generation algorithms. To run the tests, use the following command:

```bash
cargo test
```

## Development Conventions

*   **Code Style:** The code follows standard Rust conventions and formatting.
*   **Modularity:** The core logic is well-structured into modules (`sphere_n` and `cylind_n`), each with a clear responsibility.
*   **Traits:** The library makes good use of traits (`SphereGen`, `CylindGen`) to define common interfaces for the generators, promoting code reuse and extensibility.
*   **Testing:** The project has a strong emphasis on testing, with a comprehensive test suite in `src/lib.rs` that covers various dimensions and edge cases. The `approx_eq` crate is used for floating-point comparisons.
*   **Dependencies:** The project uses a minimal set of well-established crates from the Rust ecosystem.
*   **Documentation:** The `README.md` file provides a good high-level overview of the project, and the code includes doc comments explaining the purpose of structs and functions.
