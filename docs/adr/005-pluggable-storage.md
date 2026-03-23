# ADR-005: Pluggable Storage via BhavaStore Trait

**Status:** Accepted
**Date:** 2026-03-23

## Context

Bhava needs persistence for emotional states, mood history, relationships, and personality profiles. Different consumers use different databases (SQLite for local agents, Postgres for servers, Redis for cache).

## Decision

Define a `BhavaStore` trait with methods for all persistable types. Provide `SqliteStore` as a built-in implementation behind the `sqlite` feature flag.

```rust
pub trait BhavaStore {
    fn save_profile(&self, id: &str, profile: &PersonalityProfile) -> Result<()>;
    fn load_profile(&self, id: &str) -> Option<PersonalityProfile>;
    // ... 14 methods total
}
```

Storage uses JSON serialization via serde — each table stores `(id TEXT, data TEXT)` pairs. This avoids schema migrations when types change.

## Consequences

- **Backend-agnostic**: Consumers implement `BhavaStore` for their database
- **Object-safe**: `dyn BhavaStore` works for runtime backend selection
- **SQLite built-in**: `SqliteStore` provides ready-to-use persistence with zero config
- **JSON storage**: Simple but not queryable at the field level — acceptable since bhava loads full objects
- **Trade-off**: No incremental updates (entire object serialized on save). Fine for the data sizes involved (<10KB per object)
