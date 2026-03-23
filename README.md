# Bhava

> **Bhava** (Sanskrit: भाव — emotion, feeling, state of being) — emotion and personality engine for AGNOS

Shared personality and emotional state system for AI agents, game NPCs, and any entity that needs expressive behavior. Extracted from SecureYeoman's soul/brain architecture.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `traits` | yes | 15-dimension personality spectrums with behavioral instructions |
| `mood` | yes | PAD emotional state vectors with decay, triggers, history, baselines |
| `archetype` | yes | Identity hierarchy (Soul/Spirit/Brain/Body/Heart) with templates and validation |
| `sentiment` | yes | Keyword-based analysis with negation, intensity modifiers, sentence-level |
| `presets` | no | Built-in personalities (BlueShirtGuy, T.Ron, Friday, Oracle, Scout) |
| `ai` | no | Prompt composition, sentiment feedback, agent metadata (activates all core features) |

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

15 personality dimensions across 4 groups (Social, Cognitive, Behavioral, Professional) with 5 graduated levels each (Lowest → Low → Balanced → High → Highest). Each trait/level maps to behavioral instructions for LLM system prompts. Supports cosine similarity compatibility scoring, profile blending, gradual mutation, and portable markdown serialization.

### mood

6-dimensional emotional state vectors based on the PAD model (Pleasure-Arousal-Dominance) extended with Trust, Interest, and Frustration. Supports time-based exponential decay toward a configurable baseline, mood triggers, history ring buffer with trend analysis, 12 named mood states, trait-to-mood baseline derivation with compound effects, and mood tone guides for prompt injection.

### archetype

The "In Our Image" identity hierarchy: Soul → Spirit → Brain → Body → Heart. Each layer flows from the one above. Includes layer validation, 4 archetype templates (assistant, expert, creative, guardian), multi-agent crew composition, and identity merging.

### sentiment

Keyword-based sentiment analysis with negation handling ("not good" → negative), intensity modifiers ("very good" → stronger), configurable lexicons, and sentence-level analysis. Classifies text with valence scoring, confidence estimation, and emotion detection.

### spirit

The animating force within an agent — passions (what drives you), inspirations (what illuminates your path), and pains (what grounds your empathy). Composes into prompt-injectable markdown for the Spirit identity layer.

### relationship

Inter-entity relationship tracking with affinity (-1.0 to 1.0), trust (0.0 to 1.0), interaction counting, and time-based decay toward neutral. Supports allies/rivals queries and aggregate metrics.

### presets

5 built-in personality templates matching SecureYeoman configurations:

| Preset | Style |
|--------|-------|
| `blue-shirt-guy` | Eternally optimistic, warm, creative, curious |
| `t-ron` | Security watchdog, blunt, risk-averse, skeptical, meticulous |
| `friday` | Professional assistant, formal, concise, autonomous, precise |
| `oracle` | Wise advisor, patient, curious, socratic, skeptical |
| `scout` | Energetic explorer, bold, creative, autonomous |

### ai

Integration with agnosai and hoosh for personality-aware AI:
- `compose_system_prompt()` — build full system prompts from personality + identity + mood + spirit
- `apply_sentiment_feedback()` — analyze AI responses and feed back into emotional state
- `build_personality_metadata()` — export structured metadata for agent registration

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
