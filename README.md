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

## Consumers

- **SecureYeoman** — agent personalities (T.Ron, Friday, etc.)
- **joshua** — NPC emotional states and personality-driven behavior
- **agnosai** — crew member personality differentiation
- **Any daimon agent** — consistent personality framework

## License

GPL-3.0
