# Bhava

> **Bhava** (Sanskrit: भाव — emotion, feeling, state of being) — emotion and personality engine for AGNOS

Shared personality and emotional state system for AI agents, game NPCs, and any entity that needs expressive behavior. Extracted from SecureYeoman's soul/brain architecture.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `traits` | yes | Personality trait spectrums (11 dimensions, 5 levels each) |
| `mood` | yes | Emotional state vectors (PAD model + extensions) with time-based decay |
| `archetype` | yes | Identity hierarchy (Soul/Spirit/Brain/Body/Heart) |
| `sentiment` | yes | Basic keyword-based sentiment analysis |
| `presets` | no | Built-in personality templates (BlueShirtGuy, T.Ron, Friday, etc.) |
| `ai` | no | Daimon/hoosh integration (network deps) |

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

### traits

11 personality dimensions with 5 graduated levels each (Lowest → Low → Balanced → High → Highest). Each trait/level combination maps to a behavioral instruction for LLM system prompts. Profiles support distance metrics and deterministic prompt composition.

### mood

6-dimensional emotional state vectors based on the PAD model (Pleasure-Arousal-Dominance) extended with Trust, Interest, and Frustration. Supports time-based exponential decay toward a configurable baseline, stimulus application, and vector blending.

### archetype

The "In Our Image" identity hierarchy: Soul → Spirit → Brain → Body → Heart. Each layer flows from the one above. Composes cosmological preamble and layer-specific content into system prompts.

### sentiment

Fast, local keyword-based sentiment analysis. Classifies text into positive/negative/neutral with valence scoring, confidence estimation, and emotion detection (trust, curiosity, frustration). Zero network I/O.

### presets

5 built-in personality templates ready for immediate use:

| Preset | Style |
|--------|-------|
| `blue-shirt-guy` | Eternally optimistic, warm, creative |
| `t-ron` | Security watchdog, blunt, risk-averse |
| `friday` | Professional assistant, formal, concise |
| `oracle` | Wise advisor, patient, curious |
| `scout` | Energetic explorer, bold, creative |

## Consumers

- **SecureYeoman** — agent personalities (T.Ron, Friday, etc.)
- **joshua** — NPC emotional states and personality-driven behavior
- **agnosai** — crew member personality differentiation
- **Any daimon agent** — consistent personality framework

## Documentation

- [Architecture Overview](docs/architecture/overview.md)
- [Roadmap](docs/development/roadmap.md)
- [Threat Model](docs/development/threat-model.md)
- [Testing Guide](docs/guides/testing.md)
- [Contributing](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)

## License

GPL-3.0
