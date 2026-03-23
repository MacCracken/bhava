# ADR-003: Cause-Tagged Decay for Emotional State

**Status:** Accepted
**Date:** 2026-03-23

## Context

Exponential decay uniformly returns all emotion dimensions toward baseline over time. In reality, emotions persist while their cause is active — a threat keeps you afraid, a deadline keeps you stressed.

The FAtiMA model (FearNot! Affective Mind Architecture) solves this with cause-tagged decay: emotions linked to active causes resist decay.

## Decision

Add `active_causes: Vec<ActiveCause>` to `EmotionalState`. Each cause specifies which emotion dimensions it sustains. The `apply_decay()` method now skips emotions that have active causes.

```rust
state.add_active_cause("deadline", vec![Emotion::Frustration, Emotion::Arousal]);
// ... frustration and arousal won't decay while "deadline" is active
state.resolve_cause("deadline"); // now they decay normally
```

## Consequences

- **Behavioral realism**: Agents maintain contextually appropriate emotions while causes persist
- **Consumer control**: Caller decides when causes are resolved — bhava doesn't need to understand game/agent semantics
- **Backward compatible**: Default is no active causes (same decay behavior as before)
- **Serde**: `active_causes` defaults to empty via `#[serde(default)]` for backward-compatible deserialization
