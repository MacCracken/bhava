# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon), game logic (joshua), desktop integration (aethersafha).

## v0.2 — Trait Expansion (done)

- Trait groups (social, cognitive, behavioral) for bulk operations
- Trait compatibility scoring between two profiles
- Personality blending (merge two profiles with weights)
- Trait mutation (gradual personality shift over time)

## v0.3 — Mood Enhancements (done)

- Mood triggers (stimulus → response mapping)
- Mood history (ring buffer of past states for trend analysis)
- Mood influence on trait expression (e.g., high frustration amplifies directness)
- Named emotional states (calm, agitated, euphoric) from mood vector thresholds

## v0.4 — Sentiment Depth (done)

- Negation handling ("not good" → negative)
- Intensity modifiers ("very good" → stronger positive)
- Configurable lexicons (add domain-specific keywords)
- Sentence-level analysis (per-sentence valence, not just document-level)

## v0.5 — Archetype Expansion (done)

- Layer validation (required fields, content length bounds)
- Archetype templates (predefined layer structures for common patterns)
- Multi-agent identity composition (crew dynamics)

## v1.0 — Stable API

- Stable public API for PersonalityProfile, EmotionalState, SentimentResult, IdentityContent
- Comprehensive documentation on docs.rs
- No breaking changes within 1.x
- Published to crates.io

## AI Integration (ai feature)

- Agent registration with personality metadata
- Mood-aware prompt injection (dynamic system prompt sections based on current state)
- Sentiment feedback loop (analyze agent output → adjust mood)
