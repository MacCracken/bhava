# Contributing to Bhava

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Run `make check` to validate your changes
4. Open a pull request

## Prerequisites

- Rust stable (MSRV 1.89)
- `rustfmt` and `clippy` components (installed via `rust-toolchain.toml`)
- Optional: `cargo-audit`, `cargo-deny`, `cargo-llvm-cov`

## Makefile Targets

| Target | Description |
|--------|-------------|
| `make check` | Run fmt + clippy + test + audit |
| `make fmt` | Check formatting |
| `make clippy` | Lint with zero warnings |
| `make test` | Run all tests |
| `make bench` | Run benchmarks with history tracking |
| `make audit` | Security advisory check |
| `make deny` | Supply-chain checks |
| `make coverage` | Generate HTML coverage report |
| `make doc` | Build documentation (warnings as errors) |
| `make build` | Release build with all features |
| `make clean` | Remove build artifacts |

## Adding a Module

1. Create `src/module_name.rs` with doc comments
2. Add `pub mod module_name;` to `src/lib.rs` (feature-gated if applicable)
3. Re-export key types from `lib.rs`
4. Add unit tests in the module
5. Update the README feature table

## Code Style

- `cargo fmt` is mandatory
- `cargo clippy --all-features --all-targets -- -D warnings` must pass with zero warnings
- `///` doc comments on all public items
- `#[non_exhaustive]` on public enums
- `#[must_use]` on pure functions that return computed values
- No `unsafe` code
- No `println!` in library code

## Testing

- Unit tests colocated in modules via `#[cfg(test)]`
- Integration tests in `tests/integration.rs`
- Feature-gated tests with `#[cfg(feature = "...")]`
- Criterion benchmarks in `benches/benchmarks.rs`
- Run `./scripts/bench-history.sh` to record benchmark history
- Target: 90%+ line coverage

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
