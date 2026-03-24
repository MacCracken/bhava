# Changelog

## [1.0.0] - 2026-03-24

First stable release. API surface locked under semver. 30 modules, 785 tests, 105 benchmarks.

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
