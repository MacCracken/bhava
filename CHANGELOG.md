# Changelog

## [0.22.3] - 2026-03-23

Initial release. Extracted from SecureYeoman's soul/brain architecture.

### Core Modules
- **traits**: 15-dimension personality system across 4 groups (Social, Cognitive, Behavioral, Professional) with 5 graduated levels each. Behavioral instruction mapping for LLM system prompts. Cosine similarity compatibility scoring, profile blending, gradual mutation, markdown serialization. Fixed-size `[TraitLevel; 15]` array for O(1) access.
- **mood**: PAD-extended 6D emotional state vectors (Joy, Arousal, Dominance, Trust, Interest, Frustration) with time-based exponential decay toward configurable baselines. Mood triggers (praised, criticized, surprised, threatened), ring-buffer history with trend analysis, 12 named mood states, trait-to-mood baseline derivation with 7 compound effects, mood tone guides for prompt injection.
- **archetype**: "In Our Image" identity hierarchy (Soul → Spirit → Brain → Body → Heart) with cosmological preamble. Layer validation with required fields and length bounds. 4 archetype templates (assistant, expert, creative, guardian). Multi-agent crew composition. Identity merging.
- **sentiment**: Keyword-based sentiment analysis with negation handling, intensity modifiers, configurable lexicons, and sentence-level analysis. Valence scoring, confidence estimation, emotion detection.
- **spirit**: Passions, inspirations, and pains — the animating force within an agent. Prompt-injectable markdown composition for the Spirit identity layer.
- **relationship**: Inter-entity relationship graph with affinity (-1.0 to 1.0), trust (0.0 to 1.0), interaction tracking, time-based decay toward neutral, allies/rivals queries.
- **presets**: 5 built-in personality templates matching SecureYeoman configurations (BlueShirtGuy, T.Ron, Friday, Oracle, Scout).
- **ai**: System prompt composition from personality + identity + mood + spirit. Sentiment feedback loop with configurable scale. Personality metadata export for agent registration. Interaction outcome mapping to mood triggers.
- **error**: `BhavaError` with 8 `#[non_exhaustive]` variants including feature-gated `Network`.

### Quality
- 386 tests (353 unit + 33 integration)
- 63 criterion benchmarks across 12 groups with CSV history tracking
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
