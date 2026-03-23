# ADR-001: Fixed Array Over HashMap for Trait Storage

**Status:** Accepted
**Date:** 2026-03-23

## Context

`PersonalityProfile` stores 15 trait levels. The original implementation used `HashMap<TraitKind, TraitLevel>`, which incurs heap allocation, hashing overhead on every get/set, and non-deterministic iteration order.

## Decision

Replace `HashMap` with `[TraitLevel; TraitKind::COUNT]` — a fixed-size array indexed by `TraitKind::index()`.

Custom serde (via `trait_array_serde` module) converts to/from HashMap format in JSON for human readability.

## Consequences

- **Performance**: 4–30x faster across all profile operations (distance: 269→9ns, blend: 849→82ns)
- **Memory**: ~200+ bytes heap → 15 bytes stack
- **Determinism**: Iteration follows `TraitKind::ALL` order, not hash order
- **Trade-off**: `personality_serialize` is ~1.7x slower due to array→HashMap conversion, but serialization is a cold path
