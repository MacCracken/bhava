# Changelog

## [0.1.0] - 2026-03-22

### Added
- traits: 11 personality dimensions (formality, humor, verbosity, directness, warmth, empathy, patience, confidence, creativity, risk_tolerance, curiosity) with 5 graduated levels each, behavioral instruction mapping, PersonalityProfile with prompt composition and distance metrics
- mood: PAD-extended emotional state vectors (joy, arousal, dominance, trust, interest, frustration) with time-based exponential decay toward configurable baseline, stimulate/nudge/blend operations
- archetype: "In Our Image" identity hierarchy (Soul/Spirit/Brain/Body/Heart) with cosmological preamble, IdentityContent with layer-specific prompt composition
- sentiment: keyword-based sentiment analysis with valence scoring, emotion detection (trust, curiosity, frustration), confidence estimation
- presets: 5 built-in personality templates (BlueShirtGuy, T.Ron, Friday, Oracle, Scout)
- ai: daimon/hoosh client integration (feature-gated)
- error: BhavaError with #[non_exhaustive]
