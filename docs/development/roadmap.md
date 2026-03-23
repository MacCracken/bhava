# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon), game logic (joshua), desktop integration (aethersafha).

## v1.0

### Refactoring

- Split `mood.rs` (2,700 lines) into submodules: core, triggers, history, plutchik, damping, memory, contagion, baseline
- Split `traits.rs` (2,160 lines) into submodules: core, groups, ocean, serialization

### Tier 1 — High Value

- **Mood-congruent memory** — recall biased by current mood; sad agents dwell on sad memories, creating feedback loops (Bower 1981)
- **Stress / allostatic load** — chronic accumulated emotional load with burnout thresholds, distinct from acute mood (McEwen 1998)
- **Emotion regulation** — suppress, reappraise, distract strategies; felt vs expressed mood split; personality-driven defaults (Gross 1998)
- **Experience-driven personality growth** — traits evolve from accumulated event pressure, not predetermined targets; appraisal events generate trait pressure

### Tier 2 — Good Value

- **Emotional intelligence (EQ)** — perception, facilitation, understanding, management scores; modulates contagion, regulation, and appraisal effectiveness (Mayer-Salovey model)
- **Cultural display rules** — expression amplification, de-amplification, masking, neutralization by cultural context (Matsumoto framework)
- **Micro-expressions / emotional tells** — involuntary leaks of true emotion when suppressing; consumed by animation/dialogue systems (Ekman)
- **Affective computing metrics** — emotional complexity, granularity, inertia, forecasting accuracy, regulation effectiveness
