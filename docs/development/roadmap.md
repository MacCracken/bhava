# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon/agnosai), game logic (joshua), desktop integration (aethersafha), voice/audio (dhvani/shruti), policy enforcement (OPA/intent).

## v1.0

### Rhythms & Cycles

- **Energy / fatigue system** — depletable/renewable resource distinct from stress; gates flow state, modulates regulation effectiveness, cognitive performance sigmoid (Banister fitness-fatigue model)
- **Circadian rhythm** — 24-hour alertness/mood cycle with chronotype (morning/evening person); dual-cosine model with post-lunch dip; modulates baseline, decay rate, energy recovery
- **Flow state detection** — threshold detector over mood dimensions (moderate arousal + high interest + low frustration + dominance); builds slowly, breaks instantly; performance bonus + fatigue reduction
- **Ultradian rhythm** — 90-120 minute focus/rest cycles; modulates interest and arousal; interacts with circadian multiplicatively
- **Seasonal rhythm** — long-period mood variation mapped to simulation seasons; sensitivity parameter for SAD-like effects
- **Biorhythm cycles** — multiple overlapping sine waves at incommensurate periods for NPC individuation; deterministic but complex variation

### Tier 2 — Behavioral Depth

- **Emotional intelligence (EQ)** — perception, facilitation, understanding, management scores (Mayer-Salovey model)
- **Cultural display rules** — expression amplification, de-amplification, masking by cultural context (Matsumoto framework)
- **Micro-expressions / emotional tells** — involuntary leaks of true emotion when suppressing (Ekman)
- **Affective computing metrics** — emotional complexity, granularity, inertia, forecasting accuracy

### Tier 3 — SY Feature Parity

- **Salience classification** — somatic marker urgency/importance scoring (Damasio model)
- **ACT-R activation math** — frequency × recency memory activation with Hebbian boost
- **Spatial proximity triggers** — location-based mood effects via proximity rules
- **Reasoning strategy selection** — personality-driven reasoning mode
- **Preference learning** — adaptive feedback patterns from conversation history
- **Active hours** — time-of-day personality activation with scheduling
