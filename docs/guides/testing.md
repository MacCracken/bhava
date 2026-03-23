# Testing Guide

## Test Categories

- **Unit tests**: colocated in modules via `#[cfg(test)]`
- **Integration tests**: `tests/integration.rs` — cross-module behavior
- **Benchmarks**: `benches/benchmarks.rs` — criterion performance tracking

## Module Breakdown

| Module | Tests | Notes |
|--------|-------|-------|
| error | 12 | All 9 error variants, Result alias, Send+Sync |
| traits | 53 | 15 trait kinds/levels, 4 groups, behaviors, profile ops, cosine similarity, markdown, mutation, blend |
| mood | 47 | Emotions, decay/blend, triggers, history, classify, baseline derivation, compound effects, tone guides |
| archetype | 33 | Layers, archetypes, validation, templates, crew composition, merge |
| sentiment | 25 | Positive/negative/neutral/mixed, negation, intensity, configurable lexicons, sentence-level |
| presets | 16 | All 5 presets validated with 15 traits, identity content |
| spirit | 13 | Passions, inspirations, pains, active count, prompt composition, serde |
| relationship | 22 | Graph CRUD, interactions, decay, allies/rivals, averages, serde |
| monitor | 15 | Feed, flush, streaming simulation, apply, summary, reset, custom config |
| ai | 17 | System prompt composition, sentiment feedback, metadata, outcomes |
| storage | 14 | SQLite CRUD for all types, snapshots, full-agent persistence |
| integration | 35 | Cross-module: preset→prompt, sentiment→mood, triggers→history, templates→validation, crew, relationships, monitor, store |
| **Total** | **417** | |

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

66 criterion benchmarks across 13 groups:

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
| monitor | 3 | feed_sentence, feed_and_apply, streaming_10_tokens |
| serde | 6 | personality/mood/emotional_state serialize + deserialize |

```bash
# Run benchmarks with history tracking
make bench    # or ./scripts/bench-history.sh

# Results
# - bench-history.csv: append-mode CSV with timestamp, commit, benchmark, ns
# - BENCHMARKS.md: auto-generated 3-point trend table
```

## Testing Patterns

1. **Exhaustive enum coverage**: tests iterate `TraitKind::ALL`, `Emotion::ALL`, `IdentityLayer::ALL`, `TraitGroup::ALL` to verify every variant
2. **Boundary values**: decay factors 0.0/1.0/5.0 (clamped), mood values beyond [-1.0, 1.0] (clamped), negative elapsed time (no-op), zero-rate mutation (no-op)
3. **Serde roundtrip**: every serializable type has a serialize→deserialize test
4. **Determinism**: `behavioral_instructions()` and `compose_prompt()` produce identical output across runs (fixed `[TraitLevel; 15]` array, not HashMap)
5. **Cross-module integration**: sentiment→mood feedback, triggers→classify→history pipeline, presets→prompt generation, templates→validation, crew composition from presets, mood baseline from profiles
6. **Markdown roundtrip**: all 15 traits × 5 levels survive `to_markdown()` → `from_markdown()`, tested per-preset
7. **Sorted invariants**: `test_lexicons_sorted` verifies all sentiment lexicons remain alphabetically sorted
8. **Cosine similarity properties**: identical profiles → 1.0, opposite → 0.0, same-direction-different-magnitude → >0.9, orthogonal → ~0.5
9. **Compound effects**: warm+funny → playful boost verified against baseline-only derivation
10. **Feedback scaling**: sentiment feedback at scale 0.0 is a no-op, scale 1.0 > scale 0.5 in effect
