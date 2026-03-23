# Testing Guide

## Test Categories

- **Unit tests**: colocated in modules via `#[cfg(test)]`
- **Integration tests**: `tests/integration.rs` — cross-module behavior
- **Benchmarks**: `benches/benchmarks.rs` — criterion performance tracking

## Module Breakdown

| Module | Tests | Notes |
|--------|-------|-------|
| error | 11 | All 7 error variants, Result alias, Send+Sync |
| traits | 34 | All trait kinds/levels, behavior text, profile operations, serde, distance |
| mood | 34 | All emotions, decay/blend edge cases, EmotionalState lifecycle, serde |
| archetype | 21 | All layers, cosmic archetypes, prompt composition, serde |
| sentiment | 24 | Positive/negative/neutral/mixed, emotion detection, confidence, serde |
| presets | 16 | All 5 presets validated, trait configs, identity content |
| ai | 8 | Config defaults, serde, client construction |
| integration | 17 | Cross-module: preset→prompt, sentiment→mood, distance, decay, serde |
| **Total** | **165+** | |

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

32 criterion benchmarks across 7 groups:

| Group | Count | What it measures |
|-------|-------|------------------|
| traits | 3 | behavior_lookup, level_name, level_from_numeric |
| personality | 4 | compose_prompt, behavioral_instructions, active_traits, distance |
| mood | 8 | stimulate, intensity, blend, decay, dominant_emotion, nudge, deviation, apply_decay |
| sentiment | 5 | positive_short, negative_medium, neutral_long, mixed_emotions, keyword_dense |
| archetype | 3 | compose_preamble, compose_identity_2_layers, compose_identity_5_layers |
| presets | 3 | get_preset, list_presets, preset_full_prompt |
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
