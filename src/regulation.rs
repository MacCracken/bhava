//! Emotion regulation — strategies for managing emotional responses.
//!
//! Models the Gross (1998) process model: agents actively manage emotions via
//! suppression, reappraisal, or distraction. Introduces the felt/expressed mood
//! split — what the agent feels internally vs what it shows externally.

use serde::{Deserialize, Serialize};

use crate::mood::{Emotion, EmotionalState, MoodVector};

/// An emotion regulation strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RegulationStrategy {
    /// No regulation — express what you feel.
    Accept,
    /// Reduce emotional expression without changing the feeling.
    /// Arousal stays high internally; expressed mood is dampened.
    Suppress { target: Emotion, strength: f32 },
    /// Reinterpret the event to change the emotional response.
    /// Actually modifies felt mood (unlike suppress which only hides it).
    Reappraise { target: Emotion, reduction: f32 },
    /// Accelerate decay by redirecting attention away from the emotion.
    Distract { decay_boost: f32 },
}

/// Regulated emotional output — separates felt from expressed mood.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatedMood {
    /// What the agent actually feels (internal state).
    pub felt: MoodVector,
    /// What the agent shows externally (may differ from felt).
    pub expressed: MoodVector,
    /// Active regulation strategies.
    pub strategies: Vec<RegulationStrategy>,
}

impl RegulatedMood {
    /// Create from an emotional state with no regulation.
    #[must_use]
    pub fn from_state(state: &EmotionalState) -> Self {
        Self {
            felt: state.mood.clone(),
            expressed: state.mood.clone(),
            strategies: vec![RegulationStrategy::Accept],
        }
    }

    /// Apply a regulation strategy.
    ///
    /// Modifies the expressed mood (and felt mood for reappraisal).
    /// The `effectiveness` parameter (0.0–1.0) can be reduced by stress.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn regulate(&mut self, strategy: RegulationStrategy, effectiveness: f32) {
        let eff = effectiveness.clamp(0.0, 1.0);
        match &strategy {
            RegulationStrategy::Accept => {}
            RegulationStrategy::Suppress { target, strength } => {
                let current = self.expressed.get(*target);
                let suppressed = current * (1.0 - strength * eff);
                self.expressed.set(*target, suppressed);
                // Suppression increases arousal (the effort of hiding)
                let arousal_cost = strength * eff * 0.1;
                self.felt
                    .set(Emotion::Arousal, self.felt.arousal + arousal_cost);
            }
            RegulationStrategy::Reappraise { target, reduction } => {
                let delta = reduction * eff;
                // Reappraisal changes the actual feeling
                let current = self.felt.get(*target);
                let reappraised = current * (1.0 - delta);
                self.felt.set(*target, reappraised);
                self.expressed.set(*target, reappraised);
            }
            RegulationStrategy::Distract { decay_boost } => {
                let factor = decay_boost * eff;
                // Distraction decays all non-baseline emotions
                self.felt.decay(factor);
                self.expressed.decay(factor);
            }
        }
        self.strategies.push(strategy);
    }

    /// Gap between felt and expressed mood (suppression indicator).
    ///
    /// Higher values mean the agent is hiding more.
    #[must_use]
    pub fn suppression_gap(&self) -> f32 {
        let mut sum_sq = 0.0f32;
        for &e in Emotion::ALL {
            let diff = self.felt.get(e) - self.expressed.get(e);
            sum_sq += diff * diff;
        }
        sum_sq.sqrt()
    }

    /// Whether the agent is actively suppressing (gap > threshold).
    #[must_use]
    pub fn is_suppressing(&self) -> bool {
        self.suppression_gap() > 0.1
    }
}

/// Derive a default regulation strategy from personality traits.
///
/// - High openness/creativity → reappraisal (reframe the situation)
/// - High formality/skepticism → suppression (hide emotions)
/// - High patience → accept (tolerate the feeling)
/// - Low patience + high arousal → distraction (redirect attention)
#[cfg(feature = "traits")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn default_strategy(
    profile: &crate::traits::PersonalityProfile,
    dominant_emotion: Emotion,
) -> RegulationStrategy {
    use crate::traits::TraitKind;
    let openness = profile.get_trait(TraitKind::Creativity).normalized();
    let formality = profile.get_trait(TraitKind::Formality).normalized();
    let patience = profile.get_trait(TraitKind::Patience).normalized();

    if patience > 0.3 {
        RegulationStrategy::Accept
    } else if openness > 0.3 {
        RegulationStrategy::Reappraise {
            target: dominant_emotion,
            reduction: 0.4,
        }
    } else if formality > 0.3 {
        RegulationStrategy::Suppress {
            target: dominant_emotion,
            strength: 0.5,
        }
    } else {
        RegulationStrategy::Distract { decay_boost: 0.3 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> EmotionalState {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Frustration, 0.8);
        s.stimulate(Emotion::Arousal, 0.5);
        s
    }

    #[test]
    fn test_from_state() {
        let state = test_state();
        let reg = RegulatedMood::from_state(&state);
        assert!((reg.felt.frustration - reg.expressed.frustration).abs() < f32::EPSILON);
        assert!(!reg.is_suppressing());
    }

    #[test]
    fn test_suppress() {
        let state = test_state();
        let mut reg = RegulatedMood::from_state(&state);
        reg.regulate(
            RegulationStrategy::Suppress {
                target: Emotion::Frustration,
                strength: 0.8,
            },
            1.0,
        );
        assert!(reg.expressed.frustration < reg.felt.frustration);
        assert!(reg.is_suppressing());
        assert!(reg.suppression_gap() > 0.1);
        // Suppression should increase internal arousal
        assert!(reg.felt.arousal > state.mood.arousal);
    }

    #[test]
    fn test_reappraise() {
        let state = test_state();
        let before = state.mood.frustration;
        let mut reg = RegulatedMood::from_state(&state);
        reg.regulate(
            RegulationStrategy::Reappraise {
                target: Emotion::Frustration,
                reduction: 0.6,
            },
            1.0,
        );
        assert!(reg.felt.frustration < before);
        // Reappraisal changes both felt and expressed
        assert!((reg.felt.frustration - reg.expressed.frustration).abs() < f32::EPSILON);
    }

    #[test]
    fn test_distract() {
        let state = test_state();
        let before = state.mood.frustration;
        let mut reg = RegulatedMood::from_state(&state);
        reg.regulate(RegulationStrategy::Distract { decay_boost: 0.5 }, 1.0);
        assert!(reg.felt.frustration < before);
    }

    #[test]
    fn test_accept() {
        let state = test_state();
        let mut reg = RegulatedMood::from_state(&state);
        reg.regulate(RegulationStrategy::Accept, 1.0);
        assert!(!reg.is_suppressing());
    }

    #[test]
    fn test_low_effectiveness() {
        let state = test_state();
        let mut full = RegulatedMood::from_state(&state);
        let mut half = RegulatedMood::from_state(&state);
        full.regulate(
            RegulationStrategy::Suppress {
                target: Emotion::Frustration,
                strength: 0.8,
            },
            1.0,
        );
        half.regulate(
            RegulationStrategy::Suppress {
                target: Emotion::Frustration,
                strength: 0.8,
            },
            0.3,
        );
        // Full effectiveness should suppress more
        assert!(full.expressed.frustration < half.expressed.frustration);
    }

    #[test]
    fn test_serde() {
        let state = test_state();
        let reg = RegulatedMood::from_state(&state);
        let json = serde_json::to_string(&reg).unwrap();
        let reg2: RegulatedMood = serde_json::from_str(&json).unwrap();
        assert!((reg2.felt.frustration - reg.felt.frustration).abs() < 0.01);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_default_strategy_patient() {
        let mut p = crate::traits::PersonalityProfile::new("patient");
        p.set_trait(
            crate::traits::TraitKind::Patience,
            crate::traits::TraitLevel::Highest,
        );
        let s = default_strategy(&p, Emotion::Frustration);
        assert!(matches!(s, RegulationStrategy::Accept));
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_default_strategy_creative() {
        let mut p = crate::traits::PersonalityProfile::new("creative");
        p.set_trait(
            crate::traits::TraitKind::Creativity,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Patience,
            crate::traits::TraitLevel::Lowest,
        );
        let s = default_strategy(&p, Emotion::Frustration);
        assert!(matches!(s, RegulationStrategy::Reappraise { .. }));
    }
}
