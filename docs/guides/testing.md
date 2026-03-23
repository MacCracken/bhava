# Testing Guide

## Test Categories

- **Unit tests**: colocated in modules via `#[cfg(test)]`
- **Integration tests**: `tests/integration.rs` — cross-module behavior
- **Benchmarks**: `benches/benchmarks.rs` — criterion performance tracking

## Module Breakdown

| Module | Tests | Notes |
|--------|-------|-------|
| error | 11 | All error variants, Result alias, Send+Sync |
| traits | 53 | 15 trait kinds/levels, 4 groups, behaviors, profile ops, cosine similarity, markdown, mutation, blend |
| mood | 47 | Emotions, decay/blend, triggers, history, classify, baseline derivation, compound effects, tone guides |
| archetype | 33 | Layers, archetypes, validation, templates, crew composition, merge |
| sentiment | 25 | Positive/negative/neutral/mixed, negation, intensity, configurable lexicons, sentence-level |
| presets | 16 | All 5 presets validated with 15 traits, identity content |
| spirit | 13 | Passions, inspirations, pains, active count, prompt composition, serde |
| relationship | 22 | Graph CRUD, interactions, decay, allies/rivals, averages, serde |
| ai | 17 | System prompt composition, sentiment feedback, metadata, outcomes |
| integration | 33 | Cross-module: preset→prompt, sentiment→mood, triggers→history, templates→validation, crew, relationships |
| **Total** | **386** | |

## Running Tests

```bash
# Core tests (default features)
cargo test

# All tests including presets and ai
cargo test --all-features

# Single module
cargo test --all-features -- traits::tests

# Full CI check
make check    # fmt + clippy + test + audit
```

## Coverage

Target: 90% project, 75% patch (configured in `codecov.yml`).

```bash
make coverage    # generates HTML report in coverage/
```

## Benchmarks

63 criterion benchmarks across 12 groups:

| Group | Count | What it measures |
|-------|-------|------------------|
| traits | 3 | behavior_lookup, level_name, level_from_numeric |
| personality | 10 | compose_prompt, instructions, active_traits, distance, compatibility, blend, group_average, mutate, group_compatibility |
| mood | 17 | stimulate, intensity, blend, decay, dominant, nudge, deviation, apply_decay, classify, trigger, snapshot, influence, history, baseline, mood_prompt, tone_guide |
| sentiment | 8 | positive, negative, neutral, mixed, keyword_dense, negation, intensifiers, sentences |
| archetype | 7 | preamble, identity_2, identity_5, validate, template_apply, crew_3, merge |
| presets | 3 | get_preset, list_presets, preset_full_prompt |
| spirit | 1 | compose_prompt |
| relationship | 2 | record_interaction, decay_10 |
| markdown | 2 | to_markdown, from_markdown |
| ai | 5 | compose_system_prompt, minimal, sentiment_feedback, metadata, outcome |
| serde | 6 | personality/mood/emotional_state serialize + deserialize |

```bash
# Run benchmarks with history tracking
make bench    # or ./scripts/bench-history.sh

# Results
# - bench-history.csv: append-mode CSV with timestamp, commit, benchmark, ns
# - BENCHMARKS.md: auto-generated 3-point trend table
```

## Testing Patterns

1. **Exhaustive enum coverage**: tests iterate `TraitKind::ALL`, `Emotion::ALL`, `IdentityLayer::ALL` to verify every variant
2. **Boundary values**: decay factors 0.0/1.0/5.0 (clamped), mood values beyond [-1.0, 1.0] (clamped), negative elapsed time (no-op)
3. **Serde roundtrip**: every serializable type has a serialize→deserialize test
4. **Determinism**: `behavioral_instructions()` and `compose_prompt()` produce identical output across runs (fixed iteration order)
5. **Cross-module integration**: sentiment results feed mood stimulation, presets generate valid prompts, distance metrics are symmetric
