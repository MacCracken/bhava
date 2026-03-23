# Architecture

## Module Map

```
bhava
├── traits       — PersonalityProfile, TraitKind (11), TraitLevel (5), behavioral instructions   [feature: traits]
├── mood         — MoodVector (6D), EmotionalState, time-based exponential decay                  [feature: mood]
├── archetype    — CosmicArchetype, IdentityLayer (5), IdentityContent, prompt composition        [feature: archetype]
├── sentiment    — SentimentResult, keyword-based valence/emotion detection                       [feature: sentiment]
├── presets      — 5 built-in personalities (BlueShirtGuy, T.Ron, Friday, Oracle, Scout)          [feature: presets]
├── ai           — DaimonClient, HooshConfig (daimon/hoosh integration)                           [feature: ai]
└── error        — BhavaError (7 variants, #[non_exhaustive])                                    [always]
```

## Feature Flags

| Feature | Default | Dependencies | Description |
|---------|---------|--------------|-------------|
| `traits` | yes | — | Personality trait spectrums and behavioral instruction mapping |
| `mood` | yes | — | Emotional state vectors with time-based decay |
| `archetype` | yes | — | Identity hierarchy and prompt composition |
| `sentiment` | yes | — | Keyword-based sentiment analysis |
| `presets` | no | traits, archetype | Built-in personality templates |
| `ai` | no | reqwest, tokio, serde_json | Daimon/hoosh network integration |
| `full` | — | all of the above | Enable everything |

## Design Principles

- **Deterministic output**: Prompt composition iterates traits in fixed `TraitKind::ALL` order, not HashMap iteration order
- **Zero network I/O in core**: All core modules are pure computation; network deps are behind the `ai` feature flag
- **Clamped values**: Mood dimensions are always clamped to [-1.0, 1.0]; decay factors to [0.0, 1.0]
- **Minimal allocations**: Prompt builders use `write!`/`writeln!` directly into Strings; static slices where possible
- **Serde everywhere**: All public types are serializable for persistence and network transport
- **Non-exhaustive enums**: `BhavaError`, `TraitKind`, `Emotion`, `IdentityLayer` are `#[non_exhaustive]` for forward compatibility

## Data Flow

### Personality → Prompt

```
TraitKind × TraitLevel → trait_behavior() → behavioral instruction text
PersonalityProfile.compose_prompt() → iterates ALL traits in order → "## Personality\n- instruction\n..."
```

### Emotion Lifecycle

```
EmotionalState::new() → neutral baseline
  → stimulate(Emotion, intensity) → nudge mood vector (clamped)
  → apply_decay(now) → exponential decay toward baseline over time
  → deviation() → distance from baseline (Euclidean)
```

### Identity Composition

```
compose_preamble() → cosmological "In Our Image" text + 5 layer descriptions
compose_identity_prompt(IdentityContent) → preamble + populated layer sections (### Soul, ### Spirit, ...)
```

### Sentiment Analysis

```
text → lowercase → split whitespace → trim punctuation
  → match against 5 lexicons (positive, negative, trust, curiosity, frustration)
  → compute valence (pos - neg) / word_count, clamped [-1.0, 1.0]
  → compute confidence from keyword density
  → SentimentResult { valence, confidence, emotions, matched_keywords }
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `serde` | Serialization for all public types |
| `thiserror` | Error derive macros |
| `chrono` | Timestamp tracking for mood decay |
| `reqwest` | HTTP client (ai feature only) |
| `tokio` | Async runtime (ai feature only) |
| `serde_json` | JSON handling (ai feature only) |

## Consumers

- **SecureYeoman** — agent personalities (T.Ron, Friday, etc.)
- **joshua** — NPC emotional states and personality-driven behavior
- **agnosai** — crew member personality differentiation
- **Any daimon agent** — consistent personality framework via presets
