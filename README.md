# Bhava

> **Bhava** (Sanskrit: भाव — emotion, feeling, state of being) — emotion and personality engine for AGNOS

[![CI](https://github.com/MacCracken/bhava/actions/workflows/ci.yml/badge.svg)](https://github.com/MacCracken/bhava/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/bhava.svg)](https://crates.io/crates/bhava)
[![docs.rs](https://docs.rs/bhava/badge.svg)](https://docs.rs/bhava)

Shared personality and emotional state system for AI agents, game NPCs, and any entity that needs expressive behavior. Extracted from SecureYeoman's soul/brain architecture.

**15-trait personalities, PAD mood vectors, cosine similarity, sentiment analysis, identity archetypes, relationship graphs, energy/circadian/flow systems, EQ, cultural display rules, ACT-R activation, preference learning, belief system, intuition engine, aesthetic attribution, psychology math bridge (bodh), sociology math bridge (sangha), physiology bridge (sharira), microbiology bridge (jivanu)** — zero `unsafe`, 4 core deps, 1117 tests.

## Installation

```toml
[dependencies]
bhava = "1.0"
```

Default features: `traits`, `mood`, `archetype`, `sentiment`.

Optional: `presets`, `ai`, `sqlite`, `instinct`, `psychology`, `sociology`, `tracing`.

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
| `instinct` | no | Jantu creature behavior bridge (15 functions) |
| `psychology` | no | Bodh psychology math bridge (14 functions) |
| `sociology` | no | Sangha sociology math bridge (12 functions) |
| `physiology` | no | Sharira body/biomechanics bridge (12 functions) |
| `microbiology` | no | Jivanu microbial/immune bridge (10 functions) |
| `tracing` | no | Structured observability via `tracing::instrument` |
| `full` | no | All features enabled |

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
| `appraisal` | OCC appraisal model — goal-aware emotion generation |
| `stress` | Allostatic load / burnout modeling (McEwen) |
| `regulation` | Emotion regulation: suppress, reappraise, distract (Gross) |
| `growth` | Experience-driven personality evolution via trait pressure |
| `rhythm` | Biological rhythms: ultradian, seasonal, biorhythm cycles |
| `energy` | Depletable energy with Banister fitness-fatigue model |
| `circadian` | 24-hour alertness cycle with chronotype (Borbely) |
| `flow` | Flow state detection with hysteresis (Csikszentmihalyi) |
| `eq` | Emotional intelligence — Mayer-Salovey four-branch model |
| `display_rules` | Cultural display rules — Matsumoto framework |
| `microexpr` | Micro-expression detection during suppression (Ekman) |
| `affective` | Affective computing metrics: complexity, granularity, inertia |
| `proximity` | Spatial proximity triggers for location-based mood effects |
| `reasoning` | Personality-driven reasoning strategy selection |
| `active_hours` | Time-of-day personality activation scheduling |
| `salience` | Somatic marker urgency/importance scoring (Damasio) |
| `actr` | ACT-R frequency x recency memory activation with Hebbian boost |
| `preference` | Adaptive preference learning from interaction outcomes |
| `belief` | Belief system — memories crystallize into beliefs, beliefs form self-concept |
| `intuition` | Subconscious pattern integration — gut feelings from converging subsystems |
| `aesthetic` | Aesthetic attribution — repeated exposure crystallizes into beliefs and trait pressure |
| `compat` | Jantu creature behavior bridge (15 functions, feature: `instinct`) |
| `psychology` | Bodh psychology math bridge — affect, memory, cognition, psychometrics (14 functions, feature: `psychology`) |
| `sociology` | Sangha sociology math bridge — contagion, networks, influence, coalitions (12 functions, feature: `sociology`) |
| `physiology` | Sharira body bridge — fatigue, pain, balance, exertion, gait, morphology, heart rate (12 functions, feature: `physiology`) |
| `microbiology` | Jivanu immune bridge — sickness behavior, immune drain, contagion avoidance, pharmacology (10 functions, feature: `microbiology`) |

## Dependency Stack

```
┌─────────────────────────────────────────────────┐
│               CONSUMERS                          │
│  SecureYeoman / joshua / agnosai / hoosh         │
└──────────────────────┬──────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────┐
│                      BHAVA                            │
│  37 modules: traits, mood, belief, intuition...      │
├──────────────────────────────────────────────────────┤
│                   BRIDGE LAYER                        │
│  compat     psychology  sociology  physiology  micro  │
│  (jantu)    (bodh)      (sangha)   (sharira)  (jivanu)│
│  15 fn      14 fn       12 fn      12 fn      10 fn   │
└──┬──────────┬──────────┬──────────┬──────────┬───────┘
   │          │          │          │          │
┌──▼───┐ ┌───▼────┐ ┌───▼────┐ ┌───▼────┐ ┌──▼─────┐
│JANTU │ │ BODH   │ │SANGHA  │ │SHARIRA │ │JIVANU  │
│animal│ │ psych  │ │ socio  │ │ body   │ │ micro  │
└──────┘ └────────┘ └────────┘ └────────┘ └────────┘
```

## Consumers

- **SecureYeoman** — agent personalities (T.Ron, Friday, etc.)
- **joshua** — NPC emotional states and personality-driven behavior
- **agnosai** — crew member personality differentiation and mood-driven temperature
- **hoosh** — response sentiment analysis

## Documentation

- [Architecture Overview](docs/architecture/overview.md) — module map, data flows, design principles
- [Mathematical Reference](docs/architecture/math.md) — all algorithms and formulas
- [Usage Guide](docs/guides/usage.md) — patterns, philosophy, code examples
- [Testing Guide](docs/guides/testing.md) — 1117 tests, testing patterns
- [Roadmap](docs/development/roadmap.md) — v1.0 status, future features
- [Threat Model](docs/development/threat-model.md) — attack surface, mitigations, privilege model
- [Dependency Watch](docs/development/dependency-watch.md) — dependency tracking and upgrade notes
- [ADRs](docs/adr/) — architectural decision records
- [Contributing](CONTRIBUTING.md) — workflow, code style, testing requirements
- [Security Policy](SECURITY.md) — reporting, supported versions

## License

GPL-3.0
