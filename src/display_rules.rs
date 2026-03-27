//! Cultural display rules — context-dependent expression modification.
//!
//! Models Matsumoto's (1990) cultural display rules framework. Different
//! cultural or social contexts prescribe how emotions should be expressed:
//!
//! - **Amplify** — exaggerate the felt emotion (e.g., showing extra gratitude)
//! - **De-amplify** — understate the felt emotion (e.g., British restraint)
//! - **Mask** — replace the felt emotion with a different expression
//!   (e.g., smiling when angry in a professional setting)
//! - **Neutralize** — suppress all expression to show nothing
//! - **Qualify** — add a secondary expression that modifies the primary
//!   (e.g., smiling while expressing sadness to signal coping)
//!
//! This module extends the regulation module's felt/expressed split by
//! applying culturally-appropriate transformations to the expressed mood.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::mood::{Emotion, MoodVector};
use crate::regulation::RegulatedMood;

/// A display rule — how to transform expressed emotion in a given context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DisplayRule {
    /// Exaggerate expression of the target emotion.
    /// `factor` > 1.0 amplifies (e.g., 1.5 = 50% more expressive).
    Amplify { target: Emotion, factor: f32 },
    /// Understate expression of the target emotion.
    /// `factor` < 1.0 reduces (e.g., 0.5 = half as expressive).
    DeAmplify { target: Emotion, factor: f32 },
    /// Replace expression of `source` emotion with `replacement` expression.
    Mask {
        source: Emotion,
        replacement: Emotion,
        replacement_intensity: f32,
    },
    /// Suppress all emotional expression to neutral.
    Neutralize,
    /// Add a qualifying secondary expression alongside the primary.
    Qualify { qualifier: Emotion, intensity: f32 },
}

impl fmt::Display for DisplayRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Amplify { target, factor } => write!(f, "amplify {target} ×{factor:.1}"),
            Self::DeAmplify { target, factor } => write!(f, "de-amplify {target} ×{factor:.1}"),
            Self::Mask {
                source,
                replacement,
                ..
            } => write!(f, "mask {source} → {replacement}"),
            Self::Neutralize => f.write_str("neutralize"),
            Self::Qualify {
                qualifier,
                intensity,
            } => write!(f, "qualify with {qualifier} ({intensity:.2})"),
        }
    }
}

/// A named cultural or social context with associated display rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContext {
    /// Name of the context (e.g., "professional", "funeral", "celebration").
    pub name: String,
    /// Display rules active in this context.
    pub rules: Vec<DisplayRule>,
}

impl CulturalContext {
    /// Create a new context with no rules.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rules: Vec::new(),
        }
    }

    /// Add a display rule to this context.
    pub fn add_rule(&mut self, rule: DisplayRule) {
        self.rules.push(rule);
    }

    /// Builder: add a rule and return self.
    #[must_use]
    pub fn with_rule(mut self, rule: DisplayRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Number of rules in this context.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl fmt::Display for CulturalContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} rules)", self.name, self.rules.len())
    }
}

/// Apply cultural display rules to a regulated mood.
///
/// Transforms the `expressed` mood vector according to the rules in
/// the given cultural context. The `felt` mood is unchanged.
/// Rules are applied in order — later rules can override earlier ones.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
pub fn apply_display_rules(regulated: &mut RegulatedMood, context: &CulturalContext) {
    for rule in &context.rules {
        apply_single_rule(&mut regulated.expressed, rule);
    }
}

/// Apply a single display rule to an expressed mood vector.
fn apply_single_rule(expressed: &mut MoodVector, rule: &DisplayRule) {
    match rule {
        DisplayRule::Amplify { target, factor } => {
            let current = expressed.get(*target);
            expressed.set(*target, current * factor.max(0.0));
        }
        DisplayRule::DeAmplify { target, factor } => {
            let current = expressed.get(*target);
            expressed.set(*target, current * factor.clamp(0.0, 1.0));
        }
        DisplayRule::Mask {
            source,
            replacement,
            replacement_intensity,
        } => {
            // Zero out the source emotion in expression
            expressed.set(*source, 0.0);
            // Show the replacement instead
            expressed.set(*replacement, replacement_intensity.clamp(-1.0, 1.0));
        }
        DisplayRule::Neutralize => {
            for &e in Emotion::ALL {
                expressed.set(e, 0.0);
            }
        }
        DisplayRule::Qualify {
            qualifier,
            intensity,
        } => {
            expressed.nudge(*qualifier, *intensity);
        }
    }
}

/// Compute the cultural distortion — how much display rules changed expression.
///
/// Returns the Euclidean distance between pre-rule and post-rule expressed mood.
/// Higher values mean the context demands more emotional modification.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn cultural_distortion(
    original_expressed: &MoodVector,
    modified_expressed: &MoodVector,
) -> f32 {
    let mut sum = 0.0f32;
    for &e in Emotion::ALL {
        let diff = original_expressed.get(e) - modified_expressed.get(e);
        sum += diff * diff;
    }
    sum.sqrt()
}

// ─── Preset Contexts ────────────────────────────────────────────────────────

/// Professional/workplace context — de-amplify negative emotions, neutralize frustration.
#[must_use]
pub fn professional_context() -> CulturalContext {
    CulturalContext::new("professional")
        .with_rule(DisplayRule::DeAmplify {
            target: Emotion::Frustration,
            factor: 0.3,
        })
        .with_rule(DisplayRule::DeAmplify {
            target: Emotion::Arousal,
            factor: 0.5,
        })
        .with_rule(DisplayRule::Qualify {
            qualifier: Emotion::Trust,
            intensity: 0.1,
        })
}

/// Formal ceremony context — neutralize most emotions, show restrained joy.
#[must_use]
pub fn formal_context() -> CulturalContext {
    CulturalContext::new("formal")
        .with_rule(DisplayRule::DeAmplify {
            target: Emotion::Joy,
            factor: 0.6,
        })
        .with_rule(DisplayRule::DeAmplify {
            target: Emotion::Arousal,
            factor: 0.3,
        })
        .with_rule(DisplayRule::DeAmplify {
            target: Emotion::Frustration,
            factor: 0.2,
        })
}

/// Celebration context — amplify positive emotions.
#[must_use]
pub fn celebration_context() -> CulturalContext {
    CulturalContext::new("celebration")
        .with_rule(DisplayRule::Amplify {
            target: Emotion::Joy,
            factor: 1.5,
        })
        .with_rule(DisplayRule::Amplify {
            target: Emotion::Arousal,
            factor: 1.3,
        })
        .with_rule(DisplayRule::Amplify {
            target: Emotion::Trust,
            factor: 1.2,
        })
}

/// Grief/mourning context — mask joy, amplify sadness signals.
#[must_use]
pub fn mourning_context() -> CulturalContext {
    CulturalContext::new("mourning")
        .with_rule(DisplayRule::Mask {
            source: Emotion::Joy,
            replacement: Emotion::Trust,
            replacement_intensity: 0.1,
        })
        .with_rule(DisplayRule::DeAmplify {
            target: Emotion::Arousal,
            factor: 0.3,
        })
}

/// High-stakes / adversarial context — mask vulnerability, project dominance.
#[must_use]
pub fn adversarial_context() -> CulturalContext {
    CulturalContext::new("adversarial")
        .with_rule(DisplayRule::Mask {
            source: Emotion::Trust,
            replacement: Emotion::Dominance,
            replacement_intensity: 0.4,
        })
        .with_rule(DisplayRule::DeAmplify {
            target: Emotion::Joy,
            factor: 0.4,
        })
        .with_rule(DisplayRule::Amplify {
            target: Emotion::Dominance,
            factor: 1.5,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mood::{Emotion, EmotionalState};
    use crate::regulation::RegulatedMood;

    fn emotional_state() -> RegulatedMood {
        let mut state = EmotionalState::new();
        state.stimulate(Emotion::Joy, 0.6);
        state.stimulate(Emotion::Frustration, 0.5);
        state.stimulate(Emotion::Arousal, 0.4);
        RegulatedMood::from_state(&state)
    }

    // ── DisplayRule ──

    #[test]
    fn test_amplify() {
        let mut reg = emotional_state();
        let before = reg.expressed.joy;
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::Amplify {
                target: Emotion::Joy,
                factor: 1.5,
            }),
        );
        assert!(
            reg.expressed.joy > before,
            "after={} before={}",
            reg.expressed.joy,
            before
        );
    }

    #[test]
    fn test_amplify_clamped() {
        let mut reg = emotional_state();
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::Amplify {
                target: Emotion::Joy,
                factor: 10.0,
            }),
        );
        // MoodVector::set clamps to -1..1
        assert!(reg.expressed.joy <= 1.0);
    }

    #[test]
    fn test_deamplify() {
        let mut reg = emotional_state();
        let before = reg.expressed.frustration;
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::DeAmplify {
                target: Emotion::Frustration,
                factor: 0.3,
            }),
        );
        assert!(
            reg.expressed.frustration < before,
            "after={} before={}",
            reg.expressed.frustration,
            before
        );
    }

    #[test]
    fn test_mask() {
        let mut reg = emotional_state();
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::Mask {
                source: Emotion::Frustration,
                replacement: Emotion::Joy,
                replacement_intensity: 0.3,
            }),
        );
        assert!(
            reg.expressed.frustration.abs() < f32::EPSILON,
            "frustration should be zeroed"
        );
    }

    #[test]
    fn test_neutralize() {
        let mut reg = emotional_state();
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::Neutralize),
        );
        assert!(
            reg.expressed.intensity() < f32::EPSILON,
            "neutralize should zero all expressions"
        );
    }

    #[test]
    fn test_qualify() {
        let mut reg = emotional_state();
        let before_trust = reg.expressed.trust;
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::Qualify {
                qualifier: Emotion::Trust,
                intensity: 0.2,
            }),
        );
        assert!(
            reg.expressed.trust > before_trust,
            "qualify should add trust: after={} before={}",
            reg.expressed.trust,
            before_trust
        );
    }

    #[test]
    fn test_felt_unchanged() {
        let mut reg = emotional_state();
        let felt_before = reg.felt.clone();
        apply_display_rules(&mut reg, &professional_context());
        // Felt mood should be unchanged by display rules
        for &e in Emotion::ALL {
            assert!(
                (reg.felt.get(e) - felt_before.get(e)).abs() < f32::EPSILON,
                "{e}: felt changed from {} to {}",
                felt_before.get(e),
                reg.felt.get(e)
            );
        }
    }

    #[test]
    fn test_multiple_rules_ordered() {
        let mut reg = emotional_state();
        let original_joy = reg.expressed.joy;
        let ctx = CulturalContext::new("test")
            .with_rule(DisplayRule::Amplify {
                target: Emotion::Joy,
                factor: 1.5,
            })
            .with_rule(DisplayRule::DeAmplify {
                target: Emotion::Joy,
                factor: 0.5,
            });
        apply_display_rules(&mut reg, &ctx);
        // Amplify ×1.5 then de-amplify ×0.5 → net ×0.75
        let expected = original_joy * 1.5 * 0.5;
        assert!(
            (reg.expressed.joy - expected).abs() < 0.01,
            "expressed={} expected={}",
            reg.expressed.joy,
            expected
        );
    }

    #[test]
    fn test_mask_self_referential() {
        // Mask source == replacement: zeros source, sets replacement to intensity
        let mut reg = emotional_state();
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::Mask {
                source: Emotion::Joy,
                replacement: Emotion::Joy,
                replacement_intensity: 0.2,
            }),
        );
        // Net result: joy = 0.2 (set to 0, then set to 0.2)
        assert!(
            (reg.expressed.joy - 0.2).abs() < f32::EPSILON,
            "self-mask should set to replacement_intensity: {}",
            reg.expressed.joy
        );
    }

    #[test]
    fn test_amplify_negative_factor() {
        // Negative factor clamped to 0.0 by .max(0.0)
        let mut reg = emotional_state();
        apply_display_rules(
            &mut reg,
            &CulturalContext::new("test").with_rule(DisplayRule::Amplify {
                target: Emotion::Joy,
                factor: -5.0,
            }),
        );
        assert!(
            reg.expressed.joy.abs() < f32::EPSILON,
            "negative factor should zero the emotion: {}",
            reg.expressed.joy
        );
    }

    // ── Cultural distortion ──

    #[test]
    fn test_distortion_zero() {
        let mood = MoodVector::neutral();
        assert!(cultural_distortion(&mood, &mood).abs() < f32::EPSILON);
    }

    #[test]
    fn test_distortion_nonzero() {
        let mut original = MoodVector::neutral();
        original.set(Emotion::Joy, 0.5);
        let mut modified = MoodVector::neutral();
        modified.set(Emotion::Joy, 0.0);
        let d = cultural_distortion(&original, &modified);
        assert!((d - 0.5).abs() < f32::EPSILON);
    }

    // ── Preset contexts ──

    #[test]
    fn test_professional_reduces_frustration() {
        let mut reg = emotional_state();
        let before = reg.expressed.frustration;
        apply_display_rules(&mut reg, &professional_context());
        assert!(
            reg.expressed.frustration < before,
            "professional should reduce frustration"
        );
    }

    #[test]
    fn test_celebration_amplifies_joy() {
        let mut reg = emotional_state();
        let before = reg.expressed.joy;
        apply_display_rules(&mut reg, &celebration_context());
        assert!(reg.expressed.joy > before, "celebration should amplify joy");
    }

    #[test]
    fn test_mourning_masks_joy() {
        let mut reg = emotional_state();
        apply_display_rules(&mut reg, &mourning_context());
        assert!(
            reg.expressed.joy.abs() < f32::EPSILON,
            "mourning should mask joy: {}",
            reg.expressed.joy
        );
    }

    #[test]
    fn test_formal_restrains() {
        let mut reg = emotional_state();
        let before_arousal = reg.expressed.arousal;
        apply_display_rules(&mut reg, &formal_context());
        assert!(reg.expressed.arousal < before_arousal);
    }

    #[test]
    fn test_adversarial_projects_dominance() {
        let mut reg = emotional_state();
        apply_display_rules(&mut reg, &adversarial_context());
        assert!(
            reg.expressed.dominance > 0.0,
            "adversarial should project dominance"
        );
    }

    // ── Context builder ──

    #[test]
    fn test_context_builder() {
        let ctx = CulturalContext::new("custom")
            .with_rule(DisplayRule::Neutralize)
            .with_rule(DisplayRule::Amplify {
                target: Emotion::Joy,
                factor: 1.2,
            });
        assert_eq!(ctx.rule_count(), 2);
        assert_eq!(ctx.name, "custom");
    }

    #[test]
    fn test_context_display() {
        let ctx = professional_context();
        let text = ctx.to_string();
        assert!(text.contains("professional"));
        assert!(text.contains("3 rules"));
    }

    #[test]
    fn test_rule_display() {
        assert_eq!(
            DisplayRule::Amplify {
                target: Emotion::Joy,
                factor: 1.5
            }
            .to_string(),
            "amplify joy ×1.5"
        );
        assert_eq!(DisplayRule::Neutralize.to_string(), "neutralize");
    }

    // ── Serde ──

    #[test]
    fn test_serde_context() {
        let ctx = professional_context();
        let json = serde_json::to_string(&ctx).unwrap();
        let ctx2: CulturalContext = serde_json::from_str(&json).unwrap();
        assert_eq!(ctx2.name, ctx.name);
        assert_eq!(ctx2.rule_count(), ctx.rule_count());
    }

    #[test]
    fn test_serde_rule() {
        let rule = DisplayRule::Mask {
            source: Emotion::Frustration,
            replacement: Emotion::Joy,
            replacement_intensity: 0.4,
        };
        let json = serde_json::to_string(&rule).unwrap();
        let rule2: DisplayRule = serde_json::from_str(&json).unwrap();
        assert!(matches!(rule2, DisplayRule::Mask { .. }));
    }
}
