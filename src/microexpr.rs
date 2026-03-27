//! Micro-expressions — involuntary emotional leaks during suppression.
//!
//! When an entity suppresses emotions (via the regulation module), the true
//! feeling can "leak" through as a brief, involuntary micro-expression.
//! Based on Ekman's research on facial action coding and deception (1969, 2003).
//!
//! The leak probability increases with:
//! - Suppression gap (how much is being hidden)
//! - Emotional intensity (stronger feelings are harder to contain)
//! - Stress level (stressed agents leak more)
//!
//! The leak probability decreases with:
//! - Personality traits (high formality, high confidence = better poker face)

use serde::{Deserialize, Serialize};

use crate::mood::{Emotion, MoodVector};
use crate::regulation::RegulatedMood;

/// A detected micro-expression — a brief involuntary display of true emotion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroExpression {
    /// Which emotion leaked.
    pub emotion: Emotion,
    /// The true internal intensity.
    pub true_intensity: f32,
    /// How much actually showed through.
    pub expressed_intensity: f32,
    /// Duration of the leak in milliseconds (typically 50–500ms).
    pub duration_ms: u32,
}

impl MicroExpression {
    /// The leak ratio: how much of the true feeling was revealed.
    ///
    /// Returns 0.0 (nothing leaked) to 1.0 (full expression).
    #[must_use]
    pub fn leak_ratio(&self) -> f32 {
        if self.true_intensity.abs() < f32::EPSILON {
            return 0.0;
        }
        (self.expressed_intensity / self.true_intensity)
            .abs()
            .clamp(0.0, 1.0)
    }
}

/// Detect micro-expressions from a regulated mood state.
///
/// For each emotion where felt differs from expressed (i.e., the entity is
/// suppressing), compute a deterministic leak based on the suppression gap.
/// Stronger suppression gaps produce larger, longer micro-expressions.
///
/// Returns an empty vec if the entity is not suppressing anything.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn detect_micro_expressions(regulated: &RegulatedMood) -> Vec<MicroExpression> {
    if !regulated.is_suppressing() {
        return Vec::new();
    }

    let mut expressions = Vec::new();

    for &emotion in Emotion::ALL {
        let felt = regulated.felt.get(emotion);
        let expressed = regulated.expressed.get(emotion);
        let gap = (felt - expressed).abs();

        if gap < 0.05 {
            continue;
        }

        // Leak intensity scales with gap and felt intensity
        let leak_factor = gap * felt.abs() * 0.3;
        if leak_factor < 0.01 {
            continue;
        }

        // Duration: stronger leaks last longer (50–500ms range)
        let duration_ms = (50.0 + gap * 450.0).clamp(50.0, 500.0) as u32;

        expressions.push(MicroExpression {
            emotion,
            true_intensity: felt,
            expressed_intensity: leak_factor,
            duration_ms,
        });
    }

    expressions
}

/// Compute micro-expression leak multiplier from stress.
///
/// Stress degrades emotional control, making leaks more likely and larger.
/// Returns a multiplier: 1.0 at no stress, up to 2.0 at burnout.
#[must_use]
pub fn stress_leak_multiplier(stress_load: f32) -> f32 {
    1.0 + stress_load.clamp(0.0, 1.0)
}

/// Apply stress-modulated micro-expression detection.
///
/// Like `detect_micro_expressions` but scales leak intensity by stress.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn detect_micro_expressions_stressed(
    regulated: &RegulatedMood,
    stress_load: f32,
) -> Vec<MicroExpression> {
    let multiplier = stress_leak_multiplier(stress_load);
    let mut expressions = detect_micro_expressions(regulated);
    for expr in &mut expressions {
        expr.expressed_intensity = (expr.expressed_intensity * multiplier).clamp(0.0, 1.0);
    }
    expressions
}

/// Compute a summary leak vector — the aggregate micro-expression signal.
///
/// Useful for NPC renderers that need a single mood vector representing
/// what an observer might glimpse during micro-expressions.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn leak_vector(expressions: &[MicroExpression]) -> MoodVector {
    let mut v = MoodVector::neutral();
    for expr in expressions {
        v.nudge(expr.emotion, expr.expressed_intensity);
    }
    v
}

/// Detect micro-expressions with personality-modulated susceptibility.
///
/// Combines stress and personality factors: susceptible personalities leak
/// more, controlled personalities leak less.
#[cfg(feature = "traits")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn detect_micro_expressions_personality(
    regulated: &RegulatedMood,
    stress_load: f32,
    profile: &crate::traits::PersonalityProfile,
) -> Vec<MicroExpression> {
    let susceptibility = micro_expression_susceptibility(profile);
    let stress_mult = stress_leak_multiplier(stress_load);
    let combined = susceptibility * stress_mult;
    let mut expressions = detect_micro_expressions(regulated);
    for expr in &mut expressions {
        expr.expressed_intensity = (expr.expressed_intensity * combined).clamp(0.0, 1.0);
    }
    expressions
}

/// Susceptibility to micro-expressions based on personality traits.
///
/// High formality and confidence → better emotional control → less leaking.
/// High empathy and warmth → more expressive → more leaking.
///
/// Returns 0.0 (stone-faced) to 1.0 (highly expressive/leaky).
#[cfg(feature = "traits")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn micro_expression_susceptibility(profile: &crate::traits::PersonalityProfile) -> f32 {
    use crate::traits::TraitKind;
    let formality = profile.get_trait(TraitKind::Formality).normalized();
    let confidence = profile.get_trait(TraitKind::Confidence).normalized();
    let empathy = profile.get_trait(TraitKind::Empathy).normalized();
    let warmth = profile.get_trait(TraitKind::Warmth).normalized();

    // Control decreases susceptibility, expressiveness increases it
    let control = (formality + confidence) / 2.0; // -1..1
    let expressiveness = (empathy + warmth) / 2.0; // -1..1

    // Map to 0..1: base 0.5, shifted by traits
    (0.5 - control * 0.3 + expressiveness * 0.2).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mood::{Emotion, EmotionalState};
    use crate::regulation::{RegulatedMood, RegulationStrategy};

    fn suppressed_state() -> RegulatedMood {
        let mut state = EmotionalState::new();
        state.stimulate(Emotion::Frustration, 0.8);
        state.stimulate(Emotion::Arousal, 0.5);
        let mut reg = RegulatedMood::from_state(&state);
        reg.regulate(
            RegulationStrategy::Suppress {
                target: Emotion::Frustration,
                strength: 0.9,
            },
            1.0,
        );
        reg
    }

    #[test]
    fn test_no_leak_when_not_suppressing() {
        let state = EmotionalState::new();
        let reg = RegulatedMood::from_state(&state);
        let exprs = detect_micro_expressions(&reg);
        assert!(exprs.is_empty());
    }

    #[test]
    fn test_leak_when_suppressing() {
        let reg = suppressed_state();
        let exprs = detect_micro_expressions(&reg);
        assert!(!exprs.is_empty());
        // Should detect frustration leak
        assert!(exprs.iter().any(|e| e.emotion == Emotion::Frustration));
    }

    #[test]
    fn test_leak_intensity_bounded() {
        let reg = suppressed_state();
        let exprs = detect_micro_expressions(&reg);
        for expr in &exprs {
            assert!(expr.expressed_intensity >= 0.0);
            assert!(expr.expressed_intensity <= 1.0);
        }
    }

    #[test]
    fn test_leak_duration_bounded() {
        let reg = suppressed_state();
        let exprs = detect_micro_expressions(&reg);
        for expr in &exprs {
            assert!(expr.duration_ms >= 50);
            assert!(expr.duration_ms <= 500);
        }
    }

    #[test]
    fn test_leak_ratio() {
        let expr = MicroExpression {
            emotion: Emotion::Joy,
            true_intensity: 0.8,
            expressed_intensity: 0.2,
            duration_ms: 100,
        };
        assert!((expr.leak_ratio() - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn test_leak_ratio_zero_intensity() {
        let expr = MicroExpression {
            emotion: Emotion::Joy,
            true_intensity: 0.0,
            expressed_intensity: 0.0,
            duration_ms: 50,
        };
        assert!(expr.leak_ratio().abs() < f32::EPSILON);
    }

    #[test]
    fn test_stress_amplifies_leak() {
        let reg = suppressed_state();
        let normal = detect_micro_expressions(&reg);
        let stressed = detect_micro_expressions_stressed(&reg, 0.8);
        if let (Some(n), Some(s)) = (normal.first(), stressed.first()) {
            assert!(
                s.expressed_intensity >= n.expressed_intensity,
                "stressed={} normal={}",
                s.expressed_intensity,
                n.expressed_intensity
            );
        }
    }

    #[test]
    fn test_stress_leak_multiplier() {
        assert!((stress_leak_multiplier(0.0) - 1.0).abs() < f32::EPSILON);
        assert!((stress_leak_multiplier(1.0) - 2.0).abs() < f32::EPSILON);
        assert!((stress_leak_multiplier(0.5) - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_leak_vector() {
        let exprs = vec![
            MicroExpression {
                emotion: Emotion::Joy,
                true_intensity: 0.8,
                expressed_intensity: 0.1,
                duration_ms: 100,
            },
            MicroExpression {
                emotion: Emotion::Frustration,
                true_intensity: 0.6,
                expressed_intensity: 0.15,
                duration_ms: 200,
            },
        ];
        let v = leak_vector(&exprs);
        assert!((v.joy - 0.1).abs() < f32::EPSILON);
        assert!((v.frustration - 0.15).abs() < f32::EPSILON);
    }

    #[test]
    fn test_leak_vector_empty() {
        let v = leak_vector(&[]);
        assert!(v.intensity() < f32::EPSILON);
    }

    #[test]
    fn test_serde() {
        let expr = MicroExpression {
            emotion: Emotion::Trust,
            true_intensity: 0.5,
            expressed_intensity: 0.1,
            duration_ms: 150,
        };
        let json = serde_json::to_string(&expr).unwrap();
        let expr2: MicroExpression = serde_json::from_str(&json).unwrap();
        assert_eq!(expr2.emotion, expr.emotion);
        assert!((expr2.true_intensity - expr.true_intensity).abs() < f32::EPSILON);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_personality_detection_formal_leaks_less() {
        let reg = suppressed_state();
        let mut formal = crate::traits::PersonalityProfile::new("formal");
        formal.set_trait(
            crate::traits::TraitKind::Formality,
            crate::traits::TraitLevel::Highest,
        );
        formal.set_trait(
            crate::traits::TraitKind::Confidence,
            crate::traits::TraitLevel::Highest,
        );
        let mut warm = crate::traits::PersonalityProfile::new("warm");
        warm.set_trait(
            crate::traits::TraitKind::Empathy,
            crate::traits::TraitLevel::Highest,
        );
        warm.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Highest,
        );
        warm.set_trait(
            crate::traits::TraitKind::Formality,
            crate::traits::TraitLevel::Lowest,
        );
        let formal_exprs = detect_micro_expressions_personality(&reg, 0.0, &formal);
        let warm_exprs = detect_micro_expressions_personality(&reg, 0.0, &warm);
        // Formal agent should leak less than warm agent
        let formal_total: f32 = formal_exprs.iter().map(|e| e.expressed_intensity).sum();
        let warm_total: f32 = warm_exprs.iter().map(|e| e.expressed_intensity).sum();
        assert!(
            formal_total < warm_total,
            "formal={formal_total} warm={warm_total}"
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_susceptibility_formal() {
        let mut p = crate::traits::PersonalityProfile::new("formal");
        p.set_trait(
            crate::traits::TraitKind::Formality,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Confidence,
            crate::traits::TraitLevel::Highest,
        );
        let s = micro_expression_susceptibility(&p);
        assert!(s < 0.5, "formal agent should have low susceptibility: {s}");
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_susceptibility_empathetic() {
        let mut p = crate::traits::PersonalityProfile::new("warm");
        p.set_trait(
            crate::traits::TraitKind::Empathy,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Formality,
            crate::traits::TraitLevel::Lowest,
        );
        let s = micro_expression_susceptibility(&p);
        assert!(
            s > 0.5,
            "empathetic agent should have high susceptibility: {s}"
        );
    }
}
