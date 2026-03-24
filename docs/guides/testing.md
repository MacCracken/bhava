# Testing Guide

## Test Categories

- **Unit tests**: colocated in modules via `#[cfg(test)]`
- **Integration tests**: `tests/integration.rs` — cross-module behavior
- **Benchmarks**: `benches/benchmarks.rs` — criterion performance tracking

## Module Breakdown

| Module | Tests | Notes |
|--------|-------|-------|
| error | 12 | All 9 error variants, Result alias, Send+Sync |
| traits | 100 | 15 trait kinds/levels, 4 groups, behaviors, profile ops, cosine similarity, OCEAN, markdown, mutation, blend, entropy |
| mood | 126 | Emotions, decay/blend, triggers, history, classify, baseline derivation, compound effects, tone guides, contagion, damping, memory, Plutchik |
| archetype | 42 | Layers, archetypes, validation, templates, crew composition, merge |
| sentiment | 52 | Positive/negative/neutral/mixed, negation, intensity, configurable lexicons, sentence-level |
| presets | 16 | All 5 presets validated with 15 traits, identity content |
| spirit | 13 | Passions, inspirations, pains, active count, prompt composition, serde |
| relationship | 27 | Graph CRUD, interactions, decay, allies/rivals, averages, serde |
| appraisal | 15 | OCC emotions, builder pattern, mood delta, attribution, affinity |
| stress | 11 | Tick, recovery, burnout, regulation effectiveness, personality-derived |
| regulation | 9 | Suppress/reappraise/distract, effectiveness, suppression gap, serde |
| growth | 12 | Emotion-to-pressure, decay, threshold, trait shifting, limits |
| monitor | 15 | Feed, flush, streaming simulation, apply, summary, reset, custom config |
| rhythm | 25 | Ultradian/seasonal/biorhythm modulate, zero-period guards, phase offsets, apply_rhythms |
| energy | 22 | Banister tick, fitness/fatigue decay, performance sigmoid, supercompensation, personality |
| circadian | 19 | Alertness bounds, peak/trough, post-lunch dip, chronotype shifts, UTC offset, personality |
| flow | 28 | State machine lifecycle, conditions, build/break/refractory/re-entry, modifiers, personality |
| eq | 23 | 4-branch scores, overall weighted, bonuses, levels, personality-derived, prompt |
| display_rules | 22 | All 5 rule types, felt unchanged, self-mask, negative factor, presets, distortion |
| microexpr | 14 | Leak detection, stress modulation, personality susceptibility, leak vector |
| affective | 13 | Complexity, granularity, inertia, variability, autocorrelation edge cases |
| proximity | 22 | 3 falloff functions, negative distance/radius, system evaluate, serde |
| reasoning | 11 | All 5 strategies, trait scoring, description, prompt composition |
| salience | 17 | Urgency/importance scoring, levels, memory salience, filter, weighted recall |
| actr | 19 | Base-level, recency, rehearse, Hebbian links, spread activation, capacity, orphan cleanup |
| preference | 20 | EMA update, alpha decay, convergence, eviction, bias, personality |
| active_hours | 16 | Window ranges, midnight wrap, timezone, schedules, zero-width |
| ai | 16 | System prompt composition, sentiment feedback, metadata, outcomes |
| storage | 12 | SQLite CRUD for all types, snapshots, full-agent persistence |
| integration | 35 | Cross-module: preset-to-prompt, sentiment-to-mood, triggers-to-history, templates-to-validation, crew, relationships, monitor, store |
| doc | 1 | SentimentMonitor doc test |
| **Total** | **785** | 749 unit + 35 integration + 1 doc |

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

Criterion benchmarks across 27 groups:

| Group | Count | What it measures |
|-------|-------|------------------|
| traits | 3 | behavior_lookup, level_name, level_from_numeric |
| personality | 9 | compose_prompt, instructions, active_traits, distance, compatibility, blend, group_average, mutate_toward, group_compatibility |
| mood | 23 | stimulate, intensity, blend, decay, dominant, nudge, deviation, apply_decay, classify, trigger, snapshot, influence, history, deviation_trend, baseline, mood_prompt, tone_guide, action_tendency, contagion, compound_emotions, damped_step, memory_recall, adaptive_baseline |
| appraisal | 2 | positive_event, complex_appraisal |
| ocean | 3 | to_ocean, from_ocean, entropy |
| sentiment | 8 | positive, negative, neutral, mixed, keyword_dense, negation, intensifiers, sentences |
| archetype | 7 | preamble, identity_2, identity_5, validate, template_apply, crew_3, merge |
| presets | 3 | get_preset, list_presets, preset_full_prompt |
| spirit | 1 | compose_prompt |
| relationship | 2 | record_interaction, decay_10 |
| markdown | 2 | to_markdown, from_markdown |
| ai | 5 | compose_system_prompt, minimal, sentiment_feedback, metadata, outcome |
| monitor | 3 | feed_sentence, feed_and_apply, streaming_10_tokens |
| serde | 6 | personality/mood/emotional_state serialize + deserialize |
| rhythm | 4 | ultradian/seasonal/biorhythm modulate, apply_all |
| energy | 3 | tick_exertion, performance sigmoid, exertion_from_mood |
| circadian | 3 | alertness, mood_modulation, decay_rate_modifier |
| flow | 2 | tick, check_conditions |
| eq | 2 | overall, compose_prompt |
| display_rules | 2 | apply_professional, apply_celebration |
| microexpr | 1 | detect |
| affective | 3 | snapshot_complexity, snapshot_granularity, compute_metrics_50 |
| proximity | 1 | evaluate_20_rules |
| reasoning | 2 | select_strategy, all_scores |
| salience | 1 | classify |
| actr | 2 | rehearse_100, retrieve_above |
| preference | 2 | record_100, top_preferences |

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
