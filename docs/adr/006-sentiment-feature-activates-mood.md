# ADR-006: Sentiment Feature Activates Mood

**Status:** Accepted
**Date:** 2026-03-23

## Context

The `sentiment` module uses `crate::mood::Emotion` to classify detected emotions. This creates a compile-time dependency on the `mood` module.

Users enabling only `sentiment` without `mood` get a compilation error.

## Decision

The `sentiment` feature implicitly activates the `mood` feature in Cargo.toml:

```toml
sentiment = ["mood"]
```

This ensures `Emotion` is always available when `sentiment` is enabled.

## Consequences

- **No surprise**: Enabling `sentiment` always works
- **Slightly larger binary**: `mood` module compiled even if only sentiment is needed
- **Acceptable**: The `mood` module is small (~2700 lines including tests) and has no additional dependencies
