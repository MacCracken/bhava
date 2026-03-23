# Bhava

> **Bhava** (Sanskrit: भाव — emotion, feeling, state of being) — emotion and personality engine for AGNOS

[![CI](https://github.com/MacCracken/bhava/actions/workflows/ci.yml/badge.svg)](https://github.com/MacCracken/bhava/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/bhava.svg)](https://crates.io/crates/bhava)
[![docs.rs](https://docs.rs/bhava/badge.svg)](https://docs.rs/bhava)

Shared personality and emotional state system for AI agents, game NPCs, and any entity that needs expressive behavior. Extracted from SecureYeoman's soul/brain architecture.

**15-trait personalities, PAD mood vectors, cosine similarity, sentiment analysis, identity archetypes, relationship graphs** — zero `unsafe`, 3 core deps, 417 tests.

## Installation

```toml
[dependencies]
bhava = "0.22"
```

Default features: `traits`, `mood`, `archetype`, `sentiment`.

Optional: `presets`, `ai`, `sqlite`.

MSRV: **1.89** (Rust edition 2024).

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `traits` | yes | 15-dimension personality spectrums with behavioral instructions |
| `mood` | yes | PAD emotional state vectors with decay, triggers, history, baselines |
| `archetype` | yes | Identity hierarchy (Soul/Spirit/Brain/Body/Heart) with templates and validation |
| `sentiment` | yes | Keyword analysis with negation, intensity modifiers, sentence-level |
| `presets` | no | Built-in personalities (BlueShirtGuy, T.Ron, Friday, Oracle, Scout) |
| `ai` | no | Prompt composition, sentiment feedback, agent metadata |
| `sqlite` | no | SQLite persistence via `SqliteStore` |

## Quick Start

```rust
use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
use bhava::mood::{EmotionalState, Emotion};
use bhava::archetype::{compose_identity_prompt, IdentityContent, IdentityLayer};
use bhava::sentiment;

// Create a personality
let mut personality = PersonalityProfile::new("Guy");
personality.set_trait(TraitKind::Warmth, TraitLevel::Highest);
personality.set_trait(TraitKind::Humor, TraitLevel::High);
let prompt = personality.compose_prompt();

// Track emotional state
let mut state = EmotionalState::new();
state.stimulate(Emotion::Joy, 0.8);
state.stimulate(Emotion::Trust, 0.5);

// Analyze sentiment
let result = sentiment::analyze("This is wonderful work!");
assert!(result.is_positive());

// Compose identity prompt
let mut identity = IdentityContent::default();
identity.set(IdentityLayer::Soul, "You are an optimistic helper.");
let full_prompt = compose_identity_prompt(&identity);
```

## Modules

| Module | Description |
|--------|-------------|
| `traits` | 15 personality dimensions, 4 groups, cosine similarity, blending, mutation, markdown serialization |
| `mood` | 6D emotional vectors, decay, triggers, history, 12 named states, baseline derivation, tone guides |
| `archetype` | "In Our Image" hierarchy, validation, 4 templates, crew composition, identity merging |
| `sentiment` | Negation, intensity modifiers, configurable lexicons, sentence-level analysis |
| `spirit` | Passions, inspirations, pains — the animating force |
| `relationship` | Affinity, trust, interaction tracking, decay, allies/rivals |
| `presets` | BlueShirtGuy, T.Ron, Friday, Oracle, Scout |
| `monitor` | Live streaming sentiment with mood feedback |
| `ai` | System prompt composition, sentiment feedback loop, agent metadata |
| `store` | `BhavaStore` trait for pluggable persistence backends |
| `storage` | `SqliteStore` implementation |

## Consumers

- **SecureYeoman** — agent personalities (T.Ron, Friday, etc.)
- **joshua** — NPC emotional states and personality-driven behavior
- **agnosai** — crew member personality differentiation and mood-driven temperature
- **hoosh** — response sentiment analysis

## Documentation

- [Architecture Overview](docs/architecture/overview.md) — module map, data flows, design principles
- [Mathematical Reference](docs/architecture/math.md) — all algorithms and formulas
- [Usage Guide](docs/guides/usage.md) — patterns, philosophy, code examples
- [Testing Guide](docs/guides/testing.md) — 484 tests, 77 benchmarks, testing patterns
- [Threat Model](docs/development/threat-model.md) — attack surface, mitigations, privilege model
- [Dependency Watch](docs/development/dependency-watch.md) — dependency tracking and upgrade notes
- [ADRs](docs/adr/) — architectural decision records
- [Contributing](CONTRIBUTING.md) — workflow, code style, testing requirements
- [Security Policy](SECURITY.md) — reporting, supported versions

## License

GPL-3.0
