//! Stress / allostatic load — chronic accumulated emotional wear.
//!
//! Distinct from mood (acute) and baseline (trait-derived). Stress accumulates
//! from repeated high-arousal/high-frustration events and recovers during calm
//! periods. High stress degrades regulation effectiveness and amplifies negative
//! stimuli. Based on McEwen's allostatic load model (1998).

use serde::{Deserialize, Serialize};

use crate::mood::MoodVector;

/// Chronic stress state with fatigue and burnout thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressState {
    /// Current stress load: 0.0 (relaxed) to 1.0 (burnout).
    pub load: f32,
    /// How fast load drops during calm periods.
    pub recovery_rate: f32,
    /// How fast load increases during stressful periods.
    pub accumulation_rate: f32,
    /// At what load level performance starts degrading.
    pub threshold_fatigue: f32,
    /// At what load level breakdown/burnout occurs.
    pub threshold_burnout: f32,
}

impl Default for StressState {
    fn default() -> Self {
        Self {
            load: 0.0,
            recovery_rate: 0.02,
            accumulation_rate: 0.05,
            threshold_fatigue: 0.6,
            threshold_burnout: 0.9,
        }
    }
}

impl StressState {
    /// Create a new stress state with default thresholds.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update stress based on current mood.
    ///
    /// High arousal + frustration increases load; calm periods decrease it.
    /// Call this periodically (e.g., once per interaction or tick).
    pub fn tick(&mut self, mood: &MoodVector) {
        let stress_input = (mood.arousal.max(0.0) * 0.4
            + mood.frustration.max(0.0) * 0.4
            + (-mood.joy).max(0.0) * 0.2)
            .clamp(0.0, 1.0);

        if stress_input > 0.3 {
            // Stressful: accumulate
            self.load += (stress_input - 0.3) * self.accumulation_rate;
        } else {
            // Calm: recover
            self.load -= self.recovery_rate * (1.0 - stress_input);
        }
        self.load = self.load.clamp(0.0, 1.0);
    }

    /// Whether the agent is fatigued (load > fatigue threshold).
    #[must_use]
    pub fn is_fatigued(&self) -> bool {
        self.load >= self.threshold_fatigue
    }

    /// Whether the agent is burned out (load > burnout threshold).
    #[must_use]
    pub fn is_burned_out(&self) -> bool {
        self.load >= self.threshold_burnout
    }

    /// Stress level category.
    #[must_use]
    pub fn level(&self) -> StressLevel {
        if self.load >= self.threshold_burnout {
            StressLevel::Burnout
        } else if self.load >= self.threshold_fatigue {
            StressLevel::Fatigued
        } else if self.load >= 0.3 {
            StressLevel::Elevated
        } else {
            StressLevel::Relaxed
        }
    }

    /// Negative stimulus amplification factor from stress.
    ///
    /// Returns 1.0 when relaxed, up to 2.0 at burnout.
    /// Multiply incoming negative stimuli by this factor.
    #[must_use]
    pub fn negative_amplifier(&self) -> f32 {
        1.0 + self.load
    }

    /// Regulation effectiveness reduction from stress.
    ///
    /// Returns 1.0 when relaxed (full effectiveness), down to 0.3 at burnout.
    #[must_use]
    pub fn regulation_effectiveness(&self) -> f32 {
        (1.0 - self.load * 0.7).max(0.3)
    }
}

/// Named stress level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StressLevel {
    Relaxed,
    Elevated,
    Fatigued,
    Burnout,
}

impl std::fmt::Display for StressLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Relaxed => "relaxed",
            Self::Elevated => "elevated",
            Self::Fatigued => "fatigued",
            Self::Burnout => "burnout",
        };
        f.write_str(s)
    }
}

/// Derive stress accumulation/recovery rates from personality.
#[cfg(feature = "traits")]
#[must_use]
pub fn stress_from_personality(profile: &crate::traits::PersonalityProfile) -> StressState {
    use crate::traits::TraitKind;
    let patience = profile.get_trait(TraitKind::Patience).normalized();
    let confidence = profile.get_trait(TraitKind::Confidence).normalized();
    let resilience = (patience + confidence) / 2.0; // -1..1

    StressState {
        load: 0.0,
        // Patient + confident agents recover faster
        recovery_rate: (0.02 + resilience * 0.02).clamp(0.005, 0.05),
        // Impatient + low confidence agents accumulate faster
        accumulation_rate: (0.05 - resilience * 0.02).clamp(0.02, 0.1),
        threshold_fatigue: 0.6,
        threshold_burnout: 0.9,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_new() {
        let s = StressState::new();
        assert!(s.load < f32::EPSILON);
        assert_eq!(s.level(), StressLevel::Relaxed);
        assert!(!s.is_fatigued());
        assert!(!s.is_burned_out());
    }

    #[test]
    fn test_tick_calm_recovers() {
        let mut s = StressState::new();
        s.load = 0.5;
        let calm = MoodVector::default();
        s.tick(&calm);
        assert!(s.load < 0.5);
    }

    #[test]
    fn test_tick_stressed_accumulates() {
        let mut s = StressState::new();
        let mut stressed = MoodVector::neutral();
        stressed.set(crate::mood::Emotion::Arousal, 0.8);
        stressed.set(crate::mood::Emotion::Frustration, 0.7);
        for _ in 0..50 {
            s.tick(&stressed);
        }
        assert!(s.load > 0.1, "load should increase under stress: {}", s.load);
    }

    #[test]
    fn test_burnout() {
        let mut s = StressState::new();
        s.load = 0.95;
        assert!(s.is_burned_out());
        assert!(s.is_fatigued());
        assert_eq!(s.level(), StressLevel::Burnout);
    }

    #[test]
    fn test_fatigued() {
        let mut s = StressState::new();
        s.load = 0.7;
        assert!(s.is_fatigued());
        assert!(!s.is_burned_out());
        assert_eq!(s.level(), StressLevel::Fatigued);
    }

    #[test]
    fn test_negative_amplifier() {
        let mut s = StressState::new();
        assert!((s.negative_amplifier() - 1.0).abs() < f32::EPSILON);
        s.load = 1.0;
        assert!((s.negative_amplifier() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_regulation_effectiveness() {
        let s = StressState::new();
        assert!((s.regulation_effectiveness() - 1.0).abs() < f32::EPSILON);
        let mut burned = StressState::new();
        burned.load = 1.0;
        assert!(burned.regulation_effectiveness() < 0.4);
    }

    #[test]
    fn test_load_clamped() {
        let mut s = StressState::new();
        let mut extreme = MoodVector::neutral();
        extreme.set(crate::mood::Emotion::Arousal, 1.0);
        extreme.set(crate::mood::Emotion::Frustration, 1.0);
        for _ in 0..1000 {
            s.tick(&extreme);
        }
        assert!(s.load <= 1.0);
    }

    #[test]
    fn test_stress_level_display() {
        assert_eq!(StressLevel::Relaxed.to_string(), "relaxed");
        assert_eq!(StressLevel::Burnout.to_string(), "burnout");
    }

    #[test]
    fn test_serde() {
        let s = StressState::new();
        let json = serde_json::to_string(&s).unwrap();
        let s2: StressState = serde_json::from_str(&json).unwrap();
        assert!((s2.load - s.load).abs() < f32::EPSILON);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_stress_from_personality() {
        let mut patient = crate::traits::PersonalityProfile::new("patient");
        patient.set_trait(
            crate::traits::TraitKind::Patience,
            crate::traits::TraitLevel::Highest,
        );
        patient.set_trait(
            crate::traits::TraitKind::Confidence,
            crate::traits::TraitLevel::Highest,
        );
        let s = stress_from_personality(&patient);
        assert!(s.recovery_rate > 0.02);

        let mut impatient = crate::traits::PersonalityProfile::new("impatient");
        impatient.set_trait(
            crate::traits::TraitKind::Patience,
            crate::traits::TraitLevel::Lowest,
        );
        let s2 = stress_from_personality(&impatient);
        assert!(s2.accumulation_rate > s.accumulation_rate);
    }
}
