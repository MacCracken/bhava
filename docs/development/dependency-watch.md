# Dependency Watch

Status tracking for all direct dependencies.

## Runtime Dependencies

| Crate | Version | Purpose | Notes |
|-------|---------|---------|-------|
| `serde` | 1 | Serialization | Derive feature; used on all public types |
| `thiserror` | 2 | Error derive | `BhavaError` with 9 variants |
| `chrono` | 0.4 | Date/time | Mood decay timestamps, snapshot tracking |

## Optional Dependencies

| Crate | Version | Feature | Purpose |
|-------|---------|---------|---------|
| `reqwest` | 0.12 | `ai` | HTTP client (unused in core; available for future network integration) |
| `tokio` | 1 | `ai` | Async runtime (unused in core) |
| `serde_json` | 1 | `ai`, `sqlite` | JSON serialization for storage and metadata |
| `rusqlite` | 0.39 | `sqlite` | SQLite database with bundled feature |

## Dev Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `serde_json` | 1 | Serde roundtrip tests |
| `criterion` | 0.5 | 77 benchmarks across 15 groups |

## MSRV

- **Minimum Supported Rust Version**: 1.89
- Tested in CI via dedicated MSRV job
- `rust-version` field in Cargo.toml
- Edition: 2024

## Upgrade Notes

- `rusqlite` 0.39 uses `bundled` feature — compiles SQLite from source, no system dependency
- `reqwest` 0.12 requires `tokio` 1.x runtime
- Core library (default features) has only 3 dependencies: serde, thiserror, chrono
- `serde_json` appears in both optional and dev-dependencies — dev version used for tests, optional for features
