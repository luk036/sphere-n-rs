# AGENTS.md - sphere-n-rs

Compact guidance for agents working in this repository.

## Project Type

Single Rust crate (not monorepo). Generates low-discrepancy sequences on n-dimensional spheres for Monte Carlo integration, computer graphics, and numerical methods.

## Developer Commands

```bash
# Run all tests
cargo test --all-features --workspace

# Run single test
cargo test test_sphere3

# Check formatting
cargo fmt --all --check

# Clippy lints
cargo clippy --all-targets --all-features --workspace

# Build documentation
cargo doc --no-deps --document-private-items --all-features --workspace --examples

# Benchmarks
cargo bench
```

CI runs: test → rustfmt → clippy → docs (all required to pass)

## Testing

Tests are **inline in `src/lib.rs`** (not in separate `tests/` directory). Uses `approx_eq::assert_approx_eq!` for floating-point assertions.

Run individual tests with `cargo test <test_name>`.

## Dependencies

- `ndarray` - array operations
- `lds-rs` - low-discrepancy sequences (VdCorput, Sphere, Circle)
- `interp` - interpolation
- `lazy_static` - static initialization

## Key Types

- `Sphere3` - 3D sphere generator (use `SphereGen` trait)
- `SphereN` - n-dimensional sphere generator
- `CylindN` - cylindrical coordinate method
- `SphereGen` trait - common interface for sphere generators

## Public API

`PRIME_TABLE` is re-exported from crate root for convenience.

## Build Requirements

- Rust edition 2021
- Uses stable Rust (tested on stable, beta, nightly in CI)