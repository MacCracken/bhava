# Changelog

## [Unreleased] — v1.6.0

Environmental reactivity — the physical world pressing on emotion.

### Added

- **`environment` module** — environmental reactivity system (feature: `mood`)
  - `Environment` struct — 8 physical parameters (temperature, humidity, pressure, light, noise, wind, air quality, altitude) + `WeatherCondition` enum
  - `WeatherCondition` enum — Clear, Overcast, Fog, Rain, Snow, Storm (`#[non_exhaustive]`, serde, Display)
  - `EnvironmentalEffect` struct — 9 multipliers/offsets for energy drain/recovery, stress accumulation, alertness, flow disruption, mood baselines (joy, arousal, trust), salience range
  - `environmental_modifiers()` — pure function mapping Environment + optional PersonalityProfile → EnvironmentalEffect
    - Temperature: heat stress (>35°C apparent) increases energy drain/stress, cold (<0°C) increases drain
    - Humidity × heat compound: accelerated stress above 30°C/60% RH
    - Barometric pressure: low pressure → anxiety nudge (arousal↑, trust↓), sensitivity-amplified
    - Light: <100 lux → drowsiness (alertness↓), >10k lux → alertness boost
    - Noise: >70 dB → stress + flow disruption, >55 dB sustained → flow impairment
    - Wind: >10 m/s → energy drain from exposure
    - Air quality: AQI >150 → reduced recovery, elevated stress
    - Altitude: >2500m → energy drain, reduced recovery (O₂)
    - Weather conditions: Clear → joy nudge, Fog → salience reduction, Rain → personality-dependent calm/anxiety, Storm → stress + arousal + flow disruption
  - `apply_environment()` — convenience function mutating EnergyState, StressState, MoodVector directly
  - Personality modulation via 4 trait axes:
    - High Patience → heat/noise tolerance (dampens stress accumulation)
    - High Sensitivity (inverse Patience + Empathy) → weather-reactive, barometric anxiety
    - High Resilience (Confidence) → all environmental stress dampened (0.7–1.3× factor)
    - High Curiosity → rain/fog as interesting not stressful
  - `Environment::heat_index()` — Rothfusz/NWS regression for apparent temperature above 27°C
  - `Environment::wind_chill()` — Environment Canada/NWS formula below 10°C
  - `Environment::apparent_temperature()` — unified heat index / wind chill
  - 6 factory presets: `comfortable_indoor`, `hot_summer_day`, `cold_winter_night`, `storm`, `office`, `forest`
  - All multipliers clamped to sane ranges (energy drain 0.5–3.0×, recovery 0.3–1.5×, etc.)
  - 26 unit tests + 4 doc tests covering all environmental factors, edge cases, serde roundtrips
  - 6 criterion benchmarks: modifiers_indoor (~14 ns), modifiers_storm (~28 ns), apply_environment (~22 ns), heat_index (~3 ns), wind_chill (~6 ns)
  - Zero new dependencies — uses existing mood, energy, stress, traits modules

## [1.4.0] - 2026-03-30

Sharira physiology + Jivanu microbiology bridge modules — the body presses on emotion.

### Added

- **`physiology` feature** — optional sharira body/biomechanics integration via `physiology` module
  - 12 bridge functions connecting sharira's body state to bhava's emotion/personality systems
  - Fatigue capacity → mood (irritability, reduced joy, despondency)
  - Fatigue capacity → energy drain multiplier
  - Joint constraint violation → stress input (sigmoid), pain intensity (logarithmic saturation)
  - Stability margin → anxiety mood shift (balance confidence / falling panic)
  - Muscle activation level → energy exertion rate (quadratic)
  - Body mass → basal metabolic rate via Kleiber's law (sharira::bridge)
  - Morphology mass factor → dominance/confidence bias
  - Gait speed → physiological arousal (sigmoid), gait type → emotional valence
  - Allometric heart rate → baseline arousal (log scale)
- **`microbiology` feature** — optional jivanu microbial/immune system integration via `microbiology` module
  - 10 bridge functions connecting jivanu's biological state to bhava's emotion systems
  - Infected fraction → sickness behavior mood (cytokine-driven: fatigue, anhedonia, withdrawal)
  - SEIR exposed + infected → normalized severity
  - Recovered fraction → mood restoration boost
  - Infected fraction → immune energy drain multiplier (1.0–3.0)
  - R0 (beta, gamma) → social withdrawal pressure via jivanu::epidemiology
  - Vaccination coverage + R0 → trust/safety feeling via herd immunity threshold
  - Growth rate fraction → metabolic energy efficiency
  - Cardinal temperature model → thermal discomfort/stress via jivanu::growth
  - Emax pharmacological model → cognitive effect via jivanu::metabolism
  - Drug concentration / EC50 → sedation level

### Changed

- `full` feature flag now includes `physiology` and `microbiology`

### Stats

- 1117 tests (1019 unit + 35 integration + 63 doc) — up from 1051
- 37 modules (up from 35)
- Zero `unwrap()`/`panic!()`/`unsafe` in library code
- Zero clippy warnings

## [1.3.0] - 2026-03-30

Bodh psychology math + Sangha sociology math bridge modules — backing existing bhava systems with validated computational models from sibling AGNOS crates.

### Added

- **`psychology` feature** — optional bodh psychology math integration via `psychology` module
  - 14 bridge functions connecting bodh's validated psychology formulas to bhava's emotion/personality systems
  - Affect ↔ MoodVector conversion, circumplex emotion classification (Ekman's 6 basic emotions)
  - Scherer stimulus evaluation check (SEC) appraisal enrichment — OCC → Scherer → Affect pipeline
  - Gross regulation meta-analytic effectiveness coefficients (Suppress: 0.30, Reappraise: 0.85, Distract: 0.45)
  - Big Five → OceanScores mapping, Cronbach's alpha trait reliability measurement
  - ACT-R base-level activation and softmax retrieval probability (Anderson's equations)
  - Yerkes-Dodson arousal-performance inverted-U curve
  - Mood-congruent memory retrieval bias, Kelley covariation attribution model
- **`sociology` feature** — optional sangha sociology math integration via `sociology` module
  - 12 bridge functions connecting sangha's computational sociology to bhava's social/group systems
  - Hatfield emotional contagion model — network-based emotional mimicry propagation
  - Linear mood diffusion with neutral-decay, epidemic threshold computation (largest eigenvalue)
  - Network clustering coefficient, Dunbar intimacy layers (5/15/50/150)
  - Asch conformity pressure model, social proof adoption weight
  - Ringelmann social loafing (effort loss in groups), Janis groupthink risk assessment
  - Wisdom of crowds aggregation (mean/median/trimmed), Shapley value fair allocation
- **benchmarks** — 4 new benchmark groups: psychology (affect conversion, ACT-R, Yerkes-Dodson), sociology (Hatfield contagion, Shapley, clustering, groupthink)

### Changed

- `full` feature flag now includes `psychology` and `sociology`

### Stats

- 1051 tests (974 unit + 35 integration + 42 doc) — up from 972
- 134 criterion benchmarks across 34 groups (up from 126 across 32)
- 35 modules (up from 33)
- Zero `unwrap()`/`panic!()`/`unsafe` in library code
- Zero clippy warnings

## [1.2.0] - 2026-03-27

Aesthetic attribution, crate-wide tracing, and performance hardening.

### Breaking

- **belief** — `beliefs_of_kind()` now returns `impl Iterator<Item = &Belief>` instead of `Vec<&Belief>`. Migration: callers using `.len()` should use `.count()`; callers that need random access should `.collect::<Vec<_>>()` at the call site.
- **belief** — `Belief.source_memories` field type changed from `Vec<String>` to `VecDeque<String>`. Migration: replace `.as_slice()` with `.make_contiguous()`, or iterate instead of indexing.
- **intuition** — `synthesize_intuition()` now takes an additional `&AestheticSignals` parameter before `&IntuitionProfile`. Migration: pass `&AestheticSignals::default()` to preserve existing behavior.

### Added

- **aesthetic** — new module: aesthetic attribution via repeated exposure to art, music, beauty
  - `AestheticDimension` enum: Beauty, Harmony, Sublimity, Meaning, Novelty
  - `AestheticProfile` — per-dimension preference learning with mere-exposure effect (Zajonc)
  - `crystallize_beliefs()` — aesthetic preferences above threshold become world/self beliefs
  - `aesthetic_trait_pressure()` — sustained exposure creates Creativity, Curiosity, Empathy pressure
  - `aesthetic_mood_shift()` — maps aesthetic dimensions to Joy, Interest, Trust, Arousal
  - `aesthetic_intuition_signal()` — feeds aesthetic sensitivity into intuition synthesis
- **intuition** — `SignalSource::AestheticSensitivity` variant for aesthetic-driven intuition signals
- **tracing** — new `tracing` feature flag with `tracing` 0.1 optional dependency
  - ~160 public functions instrumented with `#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]`
  - Zero overhead when feature is disabled (compile-time elimination)
  - Spans auto-named from function signatures for structured observability

### Performance

- **belief** — `coherence()`: O(n^2) pairwise scan replaced with O(n) single-pass grouping by kind
- **belief** — `beliefs_of_kind()`: returns lazy iterator instead of collecting into Vec (zero allocation)
- **belief** — `add_source_memory()`: `Vec::remove(0)` O(n) shift replaced with `VecDeque::pop_front()` O(1)

### Changed

- `tracing` added to `full` feature flag

### Stats

- 972 tests (921 unit + 35 integration + 16 doc)
- 126 criterion benchmarks across 32 groups
- 33 modules
- Zero `unwrap()`/`panic!()`/`unsafe` in library code
- Zero clippy warnings

## [1.1.1] - 2026-03-26

Jantu 1.0.0 creature behavior integration — 15 bridge functions connecting animal instincts to human personality.

### Added

- **`instinct` feature** — optional jantu creature behavior integration via `compat` module
  - Jantu dependency on crates.io (`version = "1"`, `default-features = false`, no_std compatible)

#### Core Bridges
- **compat** — `mood_from_threat_response()` — maps fight/flight/freeze/fawn to PAD mood dimensions
- **compat** — `load_from_stress()` — converts jantu's two-tier stress model to bhava allostatic load input
- **compat** — `mood_shift_from_instinct()` — maps jantu instinct urgency to emotion dimensions
- **compat** — `dominance_from_rank()` — feeds jantu hierarchy position into bhava dominance
- **compat** — `instinct_layer_score()` — provides Layer 1 (Instinct) score for the intuition system

#### Contagion & Social
- **compat** — `mood_from_contagion()` — maps jantu emotional contagion pressure (Fear/Aggression/Calm/Excitement) to bhava mood shift
- **compat** — `trust_from_cohesion()` — converts jantu group cohesion to bhava trust delta for relationship system
- **compat** — `mood_from_territorial()` — maps territorial aggression response to dominance/frustration mood shift

#### Learning & Memory
- **compat** — `reactivity_from_habituation()` — converts jantu habituation response multiplier to emotional reactivity scalar (0.0–2.0)
- **compat** — `actr_seed_from_memory()` — extracts valence/strength from jantu memory traces for bhava ACT-R activation system

#### Environment & Body
- **compat** — `stress_from_landscape()` — converts jantu perceived risk (landscape of fear) to bhava stress input with quadratic nonlinearity
- **compat** — `energy_drain_from_drives()` — maps active jantu instinct drives to bhava energy exertion (rest drive reduces drain)
- **compat** — `alertness_from_activity()` — bridges jantu circadian activity level to bhava alertness via smoothstep

#### Genetics → Personality
- **compat** — `TraitSeeds` struct — 7 personality trait seed values derived from animal behavioral genome
- **compat** — `trait_seeds_from_genome()` — maps jantu's 5-axis behavioral genome (aggression, boldness, sociability, activity, exploration) to 7 bhava personality dimensions (warmth, empathy, patience, confidence, curiosity, risk_tolerance, directness)

#### Signals
- **compat** — `mood_from_signal()` — converts received jantu signals (Alarm, MatingCall, Submission, Threat, Contact, FoodCall, etc.) to mood shifts with honesty-modulated intensity

### Stats

- 936 tests (885 unit + 35 integration + 16 doc) — up from 875
- 46 tests in `compat` module, 16 doc-tests
- 15 bridge functions
- Zero clippy warnings
- No breaking changes — patch release

## [1.1.0] - 2026-03-25

Belief system, intuition engine, personal/social emotion classification, and shadow beliefs.

### Added

#### Belief Module (`belief`)
- **belief** — Schema theory (Beck, Piaget) for belief formation from emotional patterns
  - `BeliefKind` — four categories: SelfBelief ("I am..."), WorldBelief ("The world is..."), OtherBelief ("X is..."), UniversalBelief ("Everything is...")
  - `Belief` — individual belief with conviction (asymptotic growth), evidence tracking, source memory provenance, suppression depth
  - `BeliefSystem` — capacity-bounded belief store with reinforcement, challenge, decay, and coherence scoring
  - `SelfModel` — emergent bottom-up self-concept derived from self-beliefs, mapped to trait dimensions
  - `WorldModel` — emergent worldview tracking trust (safe vs hostile) and meaning (purposeful vs random) axes
  - `InsightEvent` — detects moments when self-knowledge and world-knowledge resonate ("as above, so below")
  - Understanding chain: `self_understanding()` -> `cosmic_understanding()` -> `check_insight()`
  - Integration: `belief_trait_pressure()`, `appraisal_bias()`, `identity_alignment()`

#### Personal vs Social Emotions & Shadow Beliefs (`belief`)
- `EmotionCategory` — classifies OCC emotions as Personal (about events) or Social (about standards/relationships)
- `classify_emotion()`, `emotion_to_belief_kind()`, `emotion_to_belief_tag()` — maps emotions to belief formation
- `apply_emotion_to_beliefs()` — main integration: emotion + suppression -> belief with appropriate kind/tag/conviction
- `Belief.suppression_depth` — tracks how deeply suppressed a belief is (0.0 conscious to 1.0 fully shadow)
- `reinforce_or_create_with_suppression()` — creates beliefs with suppression tracking, blends on reinforcement
- `shadow_beliefs()` — extracts shadow beliefs for intuition synthesis
- Shadow beliefs decay at half rate — what you deny persists longer

#### Intuition Module (`intuition`)
- **intuition** — Subconscious pattern integration: gut feelings from converging subsystems
  - `SignalSource` — which subsystem contributed (MemoryActivation, SomaticMarker, MicroExpressionLeak, EmotionalComplexity, PerceptualSensitivity)
  - `KnowingLayer` — five layers of knowing: Instinct (scaffold), Conditioning, Belief, Intuition, Insight
  - `LayerCharacteristics` — speed, accuracy, explainability per layer
  - `IntuitiveSignal` — a gut feeling with tag, valence, strength, sources, and confidence gap
  - `IntuitionProfile` — entity's intuitive capacity (sensitivity, integration_depth, trust_in_intuition), derivable from personality
  - Composable input types: `ActivationSignals`, `SalienceSignals`, `MicroExpressionSignals`, `AffectiveSignals`, `PerceptionSignals`
  - `synthesize_intuition()` — core algorithm: 1 source = noise, 2 = coincidence, 3+ = intuition (geometric mean convergence)
  - `active_layer()` — determines which knowing layer is currently dominant
  - `shadow_belief_signals()` — converts shadow beliefs into intuition inputs
  - `should_override_reasoning()` — when gut feelings should override conscious analysis

### Quality
- 875 tests (839 unit + 35 integration + 1 doc) — up from 785
- 119 criterion benchmarks across 30 groups — up from 105 across 27
- Zero `unwrap()`/`panic!()` in library code
- Zero `unsafe` code
- Full cleanliness pass: fmt, clippy, audit, deny, doc all clean

## [1.0.0] - 2026-03-24

First stable release. API surface locked under semver. 30 modules, 785 tests, 105 benchmarks across 27 groups.

### API Stability
- All public types committed to semver compatibility
- `#[non_exhaustive]` on all public enums for forward-compatible extension
- `#[must_use]` on all pure functions

### Fixed (from v1.0 audit)
- Eliminated `unwrap()` in `EmotionalMemoryBank::record()` eviction — now handles NaN gracefully via `unwrap_or(Ordering::Equal)`
- Eliminated `unwrap()` in `action_tendency()` — now returns `ActionTendency::Neutral` on empty candidates
- Added `PartialEq` to `OceanScores` and `DisplayRule` for comparison support
- Added `Serialize`/`Deserialize` to `SentimentMonitor` for streaming state persistence
- Added `#[inline]` + `#[must_use]` to `TraitLevel::numeric()` and `TraitLevel::normalized()` (cascading 30-60% speedup on OCEAN/personality paths)
- Added `#[must_use]` to `MoodHistory::state_distribution()`, `deviation_trend()`, `average_deviation()`, `len()`, `is_empty()`
- Completed truncated test `test_amplifier_neurotic_amplifies_negative`

### Quality
- 785 tests (749 unit + 35 integration + 1 doc)
- 105 criterion benchmarks across 27 groups
- Zero `unwrap()`/`panic!()` in library code
- Zero `unsafe` code
- Full P(-1) scaffold hardening pass: fmt, clippy, audit, deny all clean

## [0.24.3] - 2026-03-24

14 new modules completing the v1.0 roadmap. Mood test suite extracted to dedicated file. P(-1) scaffold hardening pass.

### New Modules

#### Rhythms & Cycles
- **rhythm**: Ultradian (90-120 min BRAC), seasonal (SAD sensitivity), and biorhythm (incommensurate sine waves for NPC individuation) cycles. `apply_rhythms()` convenience composer. Division-by-zero guards on all periods.
- **energy**: Depletable energy resource with Banister fitness-fatigue impulse-response model. Cognitive performance sigmoid. Gates flow state entry and regulation effectiveness. Personality-derived recovery/drain rates.
- **circadian**: Dual-cosine 24-hour alertness cycle (Borbely two-process) with post-lunch dip. 5 chronotypes (EarlyBird to NightOwl). Modulates baseline mood, decay rate, and energy recovery. Personality-derived chronotype.
- **flow**: Csikszentmihalyi flow state detector with 4-phase state machine (Inactive → Building → Active → Disrupted). 6 threshold conditions (interest, frustration, arousal band, dominance, energy, alertness). Builds slowly, breaks instantly. Performance bonus (1.1-1.3×), energy drain reduction (0.5×), stress shielding (0.3×).

#### Behavioral Depth
- **eq**: Mayer-Salovey four-branch emotional intelligence (Perception, Facilitation, Understanding, Management). Hierarchically weighted overall score. Bonus multipliers for micro-expression detection, regulation effectiveness, stress recovery, contagion resistance, appraisal accuracy. Personality-derived baseline.
- **display_rules**: Matsumoto cultural display rules framework. 5 rule types (Amplify, DeAmplify, Mask, Neutralize, Qualify). Applies to regulation's felt/expressed split. `cultural_distortion()` metric. 5 preset contexts (professional, formal, celebration, mourning, adversarial).
- **microexpr**: Ekman micro-expression detection during emotional suppression. Leak intensity scales with suppression gap × felt intensity. Stress-modulated and personality-modulated variants. Summary leak vector for NPC renderers.
- **affective**: Affective computing metrics from mood history: emotional complexity (active emotion count), granularity (Shannon entropy), inertia (lag-1 autocorrelation), variability (mood-to-mood distance). Stack-allocated computation.

#### SY Feature Parity
- **salience**: Damasio somatic marker urgency/importance scoring. Geometric mean magnitude. 4-level classification (Background/Notable/Significant/Critical). Salience-weighted memory recall.
- **actr**: ACT-R base-level activation (ln(n) - d×ln(L)), recency bonus, Hebbian associative links with asymptotic strengthening. Capacity-bounded store with lowest-activation eviction and orphan link cleanup.
- **proximity**: Location-based mood triggers with 3 falloff functions (Step, Linear, Exponential). ProximitySystem evaluates rules against entity positions. Multi-location batch evaluation.
- **reasoning**: Personality-driven reasoning strategy selection (Analytical, Intuitive, Empathetic, Systematic, Creative). Trait-scored with prompt injection.
- **preference**: Adaptive preference learning via exponential moving average with decreasing learning rate. Personality-biased (Warmth → positive gain, Skepticism → negative gain). Capacity-bounded with weakest-valence eviction.
- **active_hours**: Time-of-day activation scheduling with timezone offset. Midnight-wrapping windows. Factory presets (default 9-5, night owl, early bird, always-on).

### Changed
- Extracted mood test suite from `mood/mod.rs` into dedicated `mood/tests.rs` (1288 lines)

### Fixed
- Truncated test function `test_amplifier_neurotic_amplifies_negative` — completed with full neurotic vs calm profile assertions

### Quality
- 785 tests (749 unit + 35 integration + 1 doc)
- 105 criterion benchmarks across 27 groups
- 3 audit rounds: division-by-zero guards, unbounded growth fixes, orphan cleanup, hardcoded constant elimination, missing `#[inline]`/Serde/edge-case tests
- P(-1) scaffold hardening: all cleanliness checks pass (fmt, clippy, audit, deny)

## [0.23.3] - 2026-03-23

Initial release. Extracted from SecureYeoman's soul/brain architecture.

### Core Modules
- **traits**: 15-dimension personality system across 4 groups (Social, Cognitive, Behavioral, Professional) with 5 graduated levels each. Behavioral instruction mapping for LLM system prompts. Cosine similarity compatibility scoring, profile blending, gradual mutation, markdown serialization. Fixed-size `[TraitLevel; 15]` array for O(1) access.
- **mood**: PAD-extended 6D emotional state vectors (Joy, Arousal, Dominance, Trust, Interest, Frustration) with time-based exponential decay toward configurable baselines. Mood triggers (praised, criticized, surprised, threatened), ring-buffer history with trend analysis, 12 named mood states, trait-to-mood baseline derivation with 7 compound effects, mood tone guides for prompt injection.
- **archetype**: "In Our Image" identity hierarchy (Soul → Spirit → Brain → Body → Heart) with cosmological preamble. Layer validation with required fields and length bounds. 4 archetype templates (assistant, expert, creative, guardian). Multi-agent crew composition. Identity merging.
- **sentiment**: Keyword-based sentiment analysis with negation handling, intensity modifiers, configurable lexicons, and sentence-level analysis. Valence scoring, confidence estimation, emotion detection.
- **spirit**: Passions, inspirations, and pains — the animating force within an agent. Prompt-injectable markdown composition for the Spirit identity layer.
- **relationship**: Inter-entity relationship graph with affinity (-1.0 to 1.0), trust (0.0 to 1.0), interaction tracking, time-based decay toward neutral, allies/rivals queries.
- **presets**: 5 built-in personality templates matching SecureYeoman configurations (BlueShirtGuy, T.Ron, Friday, Oracle, Scout).
- **monitor**: Live sentiment monitoring for streaming text. Buffers chunks, analyzes at sentence boundaries, feeds back into emotional state. Running summaries with positive/negative/neutral counts.
- **ai**: System prompt composition from personality + identity + mood + spirit. Sentiment feedback loop with configurable scale. Personality metadata export for agent registration. Interaction outcome mapping to mood triggers.
- **store**: `BhavaStore` trait — abstract persistence interface for pluggable backends (SQLite, Postgres, Redis, etc.).
- **storage**: `SqliteStore` — SQLite implementation of `BhavaStore` with tables for profiles, emotional states, mood history, snapshots, relationships, and spirit data.
- **error**: `BhavaError` with 9 `#[non_exhaustive]` variants including feature-gated `Network` and `Storage`.

### Quality
- 417 tests (381 unit + 35 integration + 1 doc)
- 66 criterion benchmarks across 13 groups with CSV history tracking
- `#[must_use]` on 37 pure functions
- `# Errors` doc sections on Result-returning functions
- Zero `unsafe` code
- 3 core dependencies (serde, thiserror, chrono)

### Infrastructure
- CI pipeline: 8-job GitHub Actions (check, security, deny, test, msrv, coverage, benchmarks, doc)
- Release pipeline: version verification, crates.io publish, GitHub Release
- Supply-chain security: cargo-deny with license allowlist, wildcard ban, unknown registry denial
- Documentation: architecture overview, threat model, testing guide, roadmap
- CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, codecov.yml
