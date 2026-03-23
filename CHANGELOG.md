# Changelog

## [0.5.0] - 2026-03-22

### Added
- archetype: `ValidationRules` with `required_layers`, `max_layer_length`, `min_layer_length`; `ValidationRules::strict()` preset (Soul+Spirit, 10-2000 chars)
- archetype: `IdentityContent::validate()` and `is_valid()` — returns typed `ValidationError` variants (MissingRequired, TooLong, TooShort)
- archetype: `IdentityContent::clear()` — remove content for a layer
- archetype: `IdentityContent::merge()` — combine two identities with separator for overlapping layers
- archetype: 4 archetype templates: `template_assistant()` (2 layers), `template_expert()` (3), `template_creative()` (4), `template_guardian()` (5)
- archetype: `ArchetypeTemplate` with `apply()` to produce `IdentityContent`; `list_templates()`, `get_template()`
- archetype: `CrewMember` struct and `compose_crew_prompt()` for multi-agent identity composition
- 22 new tests for v0.5 features (298 total)
- 3 new benchmarks: validate, template_apply, crew_prompt_3 (48 total)

## [0.4.0] - 2026-03-22

### Added
- sentiment: Negation handling — "not", "no", "never", "neither", "nor", "hardly", "barely" flip the next keyword's sentiment
- sentiment: Intensity modifiers — "very" (1.5x), "extremely" (2.0x), "really" (1.4x), "slightly" (0.3x), etc. scale keyword contribution
- sentiment: `SentimentConfig` for configurable lexicons — `extra_positive`, `extra_negative`, `extra_trust`, `extra_curiosity`, `extra_frustration` extend built-in lists
- sentiment: `analyze_with_config()` — analyze with custom lexicon configuration
- sentiment: `analyze_sentences()` / `analyze_sentences_with_config()` — per-sentence analysis with `DocumentResult` containing aggregate + per-sentence `SentenceResult`
- sentiment: `SentenceResult`, `DocumentResult` types with serde support
- 27 new tests for v0.4 features (276 total)
- 3 new benchmarks: negation, intensifiers, sentences_3 (45 total)

### Changed
- sentiment: `analyze()` now handles negation and intensity modifiers (backwards-compatible — same results for text without negators/modifiers)

### Optimized
- traits: `PersonalityProfile` internal storage changed from `HashMap<TraitKind, TraitLevel>` to `[TraitLevel; 11]` fixed array — 4-30x faster across all profile operations, zero heap allocation for trait data
- traits: Added `TraitKind::index()` and `TraitKind::COUNT` for O(1) array-indexed access
- mood: `MoodHistory` changed from `Vec` to `VecDeque` for O(1) ring buffer operations
- mood: `apply_trigger()` batches nudges and updates timestamp once instead of per-response

## [0.3.0] - 2026-03-22

### Added
- mood: `MoodState` enum (12 named states: Calm, Content, Euphoric, Melancholy, Agitated, Assertive, Overwhelmed, Trusting, Guarded, Curious, Disengaged, Frustrated) with Display and serde
- mood: `EmotionalState::classify()` — derives named mood state from the mood vector
- mood: `MoodTrigger` — stimulus-response mapping with builder pattern (`new().respond().respond()`)
- mood: 4 built-in triggers: `trigger_praised()`, `trigger_criticized()`, `trigger_surprised()`, `trigger_threatened()`
- mood: `EmotionalState::apply_trigger()` — apply all trigger responses at once
- mood: `MoodSnapshot` — point-in-time mood capture with state classification and deviation
- mood: `EmotionalState::snapshot()` — capture current state
- mood: `MoodHistory` — ring buffer for trend analysis with `record()`, `average_deviation()`, `latest_state()`, `state_distribution()`, `deviation_trend()`
- mood: `mood_trait_influence()` — compute trait-level modifier from current mood (feature-gated on `traits`)
- 30 new tests for v0.3 features (228 total)
- 4 new benchmarks: classify, apply_trigger, snapshot, mood_trait_influence (40 total)

## [0.2.0] - 2026-03-22

### Added
- traits: `TraitGroup` enum (Social, Cognitive, Behavioral) with `TraitKind::group()`, `TraitGroup::traits()`, Display, serde
- traits: `PersonalityProfile::set_group()` — bulk-set all traits in a group to the same level
- traits: `PersonalityProfile::group_average()` — average normalized value for a trait group (-1.0 to 1.0)
- traits: `PersonalityProfile::compatibility()` — compatibility score between two profiles (0.0 to 1.0)
- traits: `PersonalityProfile::group_compatibility()` — compatibility restricted to a specific trait group
- traits: `PersonalityProfile::blend()` — weighted average merge of two profiles with level snapping
- traits: `PersonalityProfile::mutate_toward()` — gradual personality shift toward a target at configurable rate
- traits: `TraitLevel::from_normalized()` — snap a float (-1.0..=1.0) to the nearest trait level
- 29 new tests for v0.2 features (198 total)
- 4 new benchmarks: compatibility, blend, group_average, mutate_toward (36 total)

## [0.1.0] - 2026-03-22

### Added
- traits: 11 personality dimensions (formality, humor, verbosity, directness, warmth, empathy, patience, confidence, creativity, risk_tolerance, curiosity) with 5 graduated levels each, behavioral instruction mapping, PersonalityProfile with prompt composition and distance metrics
- mood: PAD-extended emotional state vectors (joy, arousal, dominance, trust, interest, frustration) with time-based exponential decay toward configurable baseline, stimulate/nudge/blend operations, MoodVector with Default impl
- archetype: "In Our Image" identity hierarchy (Soul/Spirit/Brain/Body/Heart) with cosmological preamble, IdentityContent with layer-specific prompt composition
- sentiment: keyword-based sentiment analysis with valence scoring, emotion detection (trust, curiosity, frustration), confidence estimation
- presets: 5 built-in personality templates (BlueShirtGuy, T.Ron, Friday, Oracle, Scout)
- ai: daimon/hoosh client integration (feature-gated), DaimonClient returns Result instead of panicking
- error: BhavaError with #[non_exhaustive], Network variant (ai feature)

### Fixed
- Deterministic prompt output: behavioral_instructions() and compose_prompt() now iterate in fixed TraitKind::ALL order instead of non-deterministic HashMap order
- Dead multi-word phrases removed from frustration lexicon ("doesn't work", "can't", "give up" never matched single-word scan); replaced with matchable single-word entries

### Optimized
- compose_preamble/compose_identity_prompt use write!/writeln! instead of format!() temp allocations (70% faster)
- compose_prompt pre-allocates String capacity based on instruction count (33% faster)
- list_presets returns &'static [&str] instead of allocating Vec per call (94% faster)

### Removed
- Unused dependencies: uuid, tracing (neither was referenced in any source file)

### Infrastructure
- CI pipeline: 8-job GitHub Actions (check, security, deny, test, msrv, coverage, benchmarks, doc)
- Release pipeline: version verification, crates.io publish, GitHub Release
- 169 tests across 8 modules (152 unit + 17 integration)
- 32 criterion benchmarks across 7 groups with CSV history tracking
- Supply-chain security: cargo-deny with license allowlist, wildcard ban, unknown registry denial
- Documentation: architecture overview, roadmap, threat model, testing guide
- CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, codecov.yml
