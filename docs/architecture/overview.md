# Architecture

## Module Map

```
bhava
├── traits        — PersonalityProfile, TraitKind (15), TraitLevel (5), 4 groups, cosine similarity  [feature: traits]
├── mood          — MoodVector (6D), EmotionalState, decay, triggers, history, baseline derivation   [feature: mood]
├── archetype     — IdentityLayer (5), IdentityContent, templates, validation, crew composition      [feature: archetype]
├── sentiment     — SentimentResult, negation, intensity modifiers, sentence-level analysis           [feature: sentiment]
├── presets       — 5 built-in personalities (BlueShirtGuy, T.Ron, Friday, Oracle, Scout)            [feature: presets]
├── spirit        — Spirit (passions, inspirations, pains) with prompt composition                    [feature: archetype]
├── relationship  — RelationshipGraph, affinity, trust, interaction tracking, decay                   [feature: mood]
├── appraisal     — OCC appraisal model, 12 emotions, goal-aware generation                          [feature: mood]
├── stress        — Allostatic load / burnout (McEwen), regulation effectiveness gating              [feature: mood]
├── regulation    — Suppress/reappraise/distract, felt vs expressed mood split                       [feature: mood]
├── growth        — Experience-driven personality evolution via trait pressure                        [feature: mood+traits]
├── monitor       — SentimentMonitor for streaming text with mood feedback                           [feature: sentiment]
├── rhythm        — Ultradian (BRAC), seasonal (SAD), biorhythm (NPC individuation)                  [feature: mood]
├── energy        — Banister fitness-fatigue, depletable resource, performance sigmoid               [feature: mood]
├── circadian     — Dual-cosine 24h alertness, chronotype, post-lunch dip                            [feature: mood]
├── flow          — Csikszentmihalyi flow state machine with hysteresis                              [feature: mood]
├── eq            — Mayer-Salovey EQ (perception, facilitation, understanding, management)           [feature: mood]
├── display_rules — Matsumoto cultural display rules (amplify, mask, neutralize, qualify)             [feature: mood]
├── microexpr     — Ekman micro-expression detection during suppression                              [feature: mood]
├── affective     — Affective metrics: complexity, granularity, inertia, variability                 [feature: mood]
├── salience      — Damasio somatic marker urgency/importance scoring                                [feature: mood]
├── actr          — ACT-R activation math with Hebbian associative links                             [feature: mood]
├── proximity     — Location-based mood triggers with falloff functions                              [feature: mood]
├── reasoning     — Personality-driven reasoning strategy selection (5 strategies)                    [feature: mood+traits]
├── preference    — Adaptive preference learning via EMA with personality bias                       [feature: mood]
├── active_hours  — Time-of-day activation scheduling with timezone support                          [feature: mood]
├── ai            — System prompt composition, sentiment feedback, agent metadata                     [feature: ai]
├── store         — BhavaStore trait for pluggable persistence backends                              [all core features]
├── storage       — SqliteStore implementation of BhavaStore                                         [feature: sqlite]
└── error         — BhavaError (9 variants, #[non_exhaustive])                                      [always]
```

## Feature Flags

| Feature | Default | Dependencies | Description |
|---------|---------|--------------|-------------|
| `traits` | yes | — | 15-dimension personality spectrums with behavioral instructions |
| `mood` | yes | — | Emotional state vectors with decay, triggers, history, baselines |
| `archetype` | yes | — | Identity hierarchy, templates, validation, spirit, crew composition |
| `sentiment` | yes | mood | Sentiment analysis with negation, intensity, configurable lexicons |
| `presets` | no | traits, archetype | Built-in personality templates (BlueShirtGuy, T.Ron, Friday, Oracle, Scout) |
| `ai` | no | traits, mood, archetype, sentiment, reqwest, tokio, serde_json | Prompt composition, sentiment feedback, metadata |
| `sqlite` | no | traits, mood, archetype, sentiment, rusqlite, serde_json | SQLite persistence |
| `full` | — | all of the above | Enable everything |

## Design Principles

- **Deterministic output**: Prompt composition iterates traits in fixed `TraitKind::ALL` order via `[TraitLevel; 15]` array
- **Cosine similarity**: Compatibility scoring measures behavioral pattern direction, not magnitude
- **`#[must_use]`**: Pure functions annotated to prevent accidental value drops
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
text → lowercase → single-pass whitespace iteration
  → check negation (not, never, hardly...) → flip next word
  → check intensity modifier (very, extremely, slightly...) → scale next word
  → match against 5 lexicons (positive, negative, trust, curiosity, frustration)
  → compute valence (pos_score - neg_score) / word_count, clamped [-1.0, 1.0]
  → SentimentResult { valence, confidence, emotions, matched_keywords }
```

### Trait-to-Mood Baseline

```
PersonalityProfile (15 traits)
  → per-trait valence/arousal modifiers (TRAIT_VALUE_MODIFIERS table)
  → average across all traits
  → apply compound effects (7 emergent combos: playful, nurturing, mentoring, ...)
  → MoodVector { joy: valence, arousal: arousal }
```

### AI Integration (full pipeline)

```
compose_system_prompt(profile, identity, mood, spirit)
  → compose_identity_prompt()     — archetype preamble + layer content
  → profile.compose_prompt()      — trait behavioral instructions
  → compose_mood_prompt()         — current mood state + tone guide
  → spirit text                   — passions, inspirations, pains
  → single String for InferenceRequest.system

Response → apply_sentiment_feedback(text, state, scale)
  → sentiment::analyze()          — valence + emotions
  → stimulate emotional state     — scaled feedback into mood vector
```

### Live Sentiment Monitoring

```
SentimentMonitor::new(scale)
  → feed(chunk)                     — buffer text, analyze at sentence boundaries
  → feed_and_apply(chunk, state)    — feed + apply results to EmotionalState
  → flush()                         — analyze remaining buffered text
  → summary()                       — positive/negative/neutral counts + average valence
```

### Persistence

```
BhavaStore (trait)
  ├── save_profile / load_profile / delete_profile / list_profile_ids
  ├── save_emotional_state / load_emotional_state
  ├── save_mood_history / load_mood_history
  ├── append_snapshot / load_snapshots
  ├── save_relationships / load_relationships
  └── save_spirit / load_spirit

SqliteStore implements BhavaStore (feature: sqlite)
Custom backends: implement BhavaStore for Postgres, Redis, etc.
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `serde` | Serialization for all public types |
| `thiserror` | Error derive macros |
| `chrono` | Timestamp tracking for mood decay |
| `reqwest` | HTTP client (ai feature only) |
| `tokio` | Async runtime (ai feature only) |
| `serde_json` | JSON handling (ai + sqlite features) |
| `rusqlite` | SQLite database (sqlite feature only) |

## Consumers

- **SecureYeoman** — agent personalities (T.Ron, Friday, etc.)
- **joshua** — NPC emotional states and personality-driven behavior
- **agnosai** — crew member personality differentiation and mood-driven temperature
- **hoosh** — response sentiment analysis
