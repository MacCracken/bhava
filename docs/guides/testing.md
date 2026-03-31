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
| belief | 50 | Schema theory, 4 belief kinds, conviction, decay, coherence, SelfModel, WorldModel, InsightEvent, shadow beliefs |
| belief_emotion | 20 | Personal/social classification, emotion-to-belief mapping, suppression-aware creation |
| intuition | 25 | Signal synthesis, 5 knowing layers, convergence algorithm, shadow signals, active_layer |
| aesthetic | 20 | 5 dimensions, mere-exposure, crystallization, trait pressure, mood shift |
| compat | 46 | Jantu bridge: threat response, stress, instincts, contagion, habituation, genome, signals |
| psychology | 28 | Bodh bridge: affect conversion, classify, appraisal, regulation, psychometrics, ACT-R, Yerkes-Dodson, attribution |
| sociology | 25 | Sangha bridge: Hatfield contagion, propagation, clustering, Dunbar, conformity, loafing, groupthink, Shapley |
| integration | 35 | Cross-module: preset-to-prompt, sentiment-to-mood, triggers-to-history, templates-to-validation, crew, relationships, monitor, store |
| doc | 42 | Doc-tests across compat (16), psychology (14), sociology (12) |
| **Total** | **1051** | 974 unit + 35 integration + 42 doc |

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

Criterion benchmarks across 34 groups:

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
| belief | 4 | coherence, beliefs_of_kind, add_source_memory, decay |
| belief_emotion | 4 | classify_emotion, apply_emotion_100, shadow_beliefs_query, decay_with_shadow |
| intuition | 4 | synthesize_5_tags, synthesize_20_tags, profile_from_personality, active_layer |
| aesthetic | 5 | record_exposure, record_exposure_50, crystallize_beliefs, aesthetic_trait_pressure, aesthetic_mood_shift |
| psychology | 4 | affect_from_mood, classify_mood, base_level_activation, yerkes_dodson_performance |
| sociology | 4 | hatfield_contagion_10, shapley_4_players, clustering_coefficient, groupthink_risk |

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
