# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon/agnosai), game logic (joshua), desktop integration (aethersafha), voice/audio (dhvani/shruti), policy enforcement (OPA/intent).

## v1.0

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

### Tier 3 — SY Feature Parity

- **Salience classification** — somatic marker urgency/importance scoring: urgency, error signal, user frustration, success, curiosity; composite weighted score (Damasio model, from SY brain/salience.ts)
- **ACT-R activation math** — frequency × recency memory activation with Hebbian boost and salience weighting; improves emotional memory recall (from SY brain/activation.ts)
- **Spatial proximity triggers** — location-based mood effects via proximity rules (enter/leave radius/zone); extends relationship + contagion systems (from SY simulation/spatial-engine.ts)
- **Reasoning strategy selection** — personality-driven reasoning mode (chain_of_thought, reflexion, tree_of_thought, etc.); maps personality traits to preferred reasoning approach (from SY soul/strategy-storage.ts)
- **Preference learning** — adaptive feedback patterns from conversation history; response length, code preference, style detection; feeds into experience-driven growth (from SY brain/preference-learner.ts)
- **Active hours** — time-of-day personality activation with day-of-week scheduling and timezone support (from SY soul types)
