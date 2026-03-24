//! Flow state detection — threshold detector over mood dimensions.
//!
//! Models Csikszentmihalyi's flow (1990) as a state machine with hysteresis.
//! Flow requires sustained mood conditions:
//!
//! - **Moderate arousal** — not too relaxed, not overwhelmed
//! - **High interest** — engaged with the task
//! - **Low frustration** — not blocked or irritated
//! - **Positive dominance** — sense of control/mastery
//! - **Sufficient energy** — fuel for sustained focus
//! - **Adequate alertness** — circadian support
//!
//! Flow builds slowly (accumulator pattern, ~20 ticks to enter) and breaks
//! instantly when any condition is violated. While in flow: performance bonus,
//! reduced energy drain, reduced stress accumulation.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::mood::MoodVector;

/// Flow state phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FlowPhase {
    /// Not in flow, conditions not met.
    Inactive,
    /// Conditions met, building toward flow.
    Building,
    /// In flow state — performance bonus active.
    Active,
    /// Flow just broke — brief refractory period.
    Disrupted,
}

impl fmt::Display for FlowPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Inactive => "inactive",
            Self::Building => "building",
            Self::Active => "in flow",
            Self::Disrupted => "disrupted",
        };
        f.write_str(s)
    }
}

/// Flow state detector with hysteresis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowState {
    /// Current phase.
    pub phase: FlowPhase,
    /// Accumulator: builds toward `entry_threshold` when conditions are sustained.
    pub accumulator: f32,
    /// Ticks spent in active flow (for bonus scaling).
    pub flow_duration: u32,

    // ── Thresholds ──
    /// Minimum interest required. Default: 0.4.
    pub interest_threshold: f32,
    /// Maximum frustration allowed. Default: 0.3.
    pub frustration_ceiling: f32,
    /// Minimum arousal (not too calm). Default: 0.1.
    pub arousal_floor: f32,
    /// Maximum arousal (not overwhelmed). Default: 0.7.
    pub arousal_ceiling: f32,
    /// Minimum dominance (sense of control). Default: 0.1.
    pub dominance_floor: f32,
    /// Minimum energy to enter/maintain flow. Default: 0.3.
    pub energy_threshold: f32,
    /// Minimum circadian alertness for flow. Default: 0.3.
    pub alertness_threshold: f32,

    // ── Rates ──
    /// How fast the accumulator fills when conditions are met. Default: 0.05.
    pub build_rate: f32,
    /// Accumulator value needed to enter Active phase. Default: 1.0.
    pub entry_threshold: f32,
}

impl Default for FlowState {
    fn default() -> Self {
        Self {
            phase: FlowPhase::Inactive,
            accumulator: 0.0,
            flow_duration: 0,
            interest_threshold: 0.4,
            frustration_ceiling: 0.3,
            arousal_floor: 0.1,
            arousal_ceiling: 0.7,
            dominance_floor: 0.1,
            energy_threshold: 0.3,
            alertness_threshold: 0.3,
            build_rate: 0.05,
            entry_threshold: 1.0,
        }
    }
}

impl FlowState {
    /// Create with default thresholds.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Check whether mood/energy/alertness conditions for flow are met.
    #[must_use]
    #[inline]
    pub fn check_conditions(
        &self,
        mood: &MoodVector,
        energy: f32,
        alertness: f32,
    ) -> FlowConditions {
        FlowConditions {
            interest_met: mood.interest >= self.interest_threshold,
            frustration_met: mood.frustration <= self.frustration_ceiling,
            arousal_met: mood.arousal >= self.arousal_floor && mood.arousal <= self.arousal_ceiling,
            dominance_met: mood.dominance >= self.dominance_floor,
            energy_met: energy >= self.energy_threshold,
            alertness_met: alertness >= self.alertness_threshold,
        }
    }

    /// Tick the flow state machine.
    ///
    /// Pass current mood, energy level (0.0–1.0), and circadian alertness (0.0–1.0).
    /// Updates phase, accumulator, and flow duration.
    pub fn tick(&mut self, mood: &MoodVector, energy: f32, alertness: f32) {
        let all_met = self.check_conditions(mood, energy, alertness).all_met();

        match self.phase {
            FlowPhase::Inactive => {
                if all_met {
                    self.phase = FlowPhase::Building;
                    self.accumulator = self.build_rate;
                }
            }
            FlowPhase::Building => {
                if all_met {
                    self.accumulator += self.build_rate;
                    if self.accumulator >= self.entry_threshold {
                        self.phase = FlowPhase::Active;
                        self.flow_duration = 0;
                    }
                } else {
                    // Conditions broken — reset, back to inactive
                    self.accumulator = 0.0;
                    self.phase = FlowPhase::Inactive;
                }
            }
            FlowPhase::Active => {
                if all_met {
                    self.flow_duration = self.flow_duration.saturating_add(1);
                } else {
                    // Flow breaks instantly
                    self.phase = FlowPhase::Disrupted;
                    self.accumulator = 0.0;
                }
            }
            FlowPhase::Disrupted => {
                // One-tick refractory, then back to checking
                self.phase = FlowPhase::Inactive;
                self.flow_duration = 0;
            }
        }
    }

    /// Whether currently in active flow.
    #[must_use]
    #[inline]
    pub fn is_in_flow(&self) -> bool {
        self.phase == FlowPhase::Active
    }

    /// Whether currently building toward flow.
    #[must_use]
    #[inline]
    pub fn is_building(&self) -> bool {
        self.phase == FlowPhase::Building
    }

    /// Build progress as a fraction (0.0–1.0).
    #[must_use]
    #[inline]
    pub fn build_progress(&self) -> f32 {
        if self.entry_threshold <= 0.0 {
            return 0.0;
        }
        (self.accumulator / self.entry_threshold).clamp(0.0, 1.0)
    }

    /// Performance bonus from flow state.
    ///
    /// Returns 1.0 when not in flow. Ramps from 1.1 to 1.3 during flow,
    /// scaling with duration (deeper flow = stronger bonus).
    #[must_use]
    #[inline]
    pub fn performance_bonus(&self) -> f32 {
        if self.phase != FlowPhase::Active {
            return 1.0;
        }
        let ramp = (self.flow_duration as f32 / 60.0).min(1.0);
        1.1 + ramp * 0.2
    }

    /// Energy drain modifier during flow.
    ///
    /// Flow is an efficient state — reduces energy drain.
    /// Returns 1.0 when not in flow, 0.5 during flow.
    #[must_use]
    #[inline]
    pub fn energy_drain_modifier(&self) -> f32 {
        if self.phase == FlowPhase::Active {
            0.5
        } else {
            1.0
        }
    }

    /// Stress accumulation modifier during flow.
    ///
    /// Flow shields from stress (positive engaged state).
    /// Returns 1.0 when not in flow, 0.3 during flow.
    #[must_use]
    #[inline]
    pub fn stress_accumulation_modifier(&self) -> f32 {
        if self.phase == FlowPhase::Active {
            0.3
        } else {
            1.0
        }
    }
}

/// Snapshot of flow conditions — which requirements are currently met.
#[derive(Debug, Clone, Copy)]
pub struct FlowConditions {
    /// Interest >= threshold.
    pub interest_met: bool,
    /// Frustration <= ceiling.
    pub frustration_met: bool,
    /// Arousal within floor..=ceiling.
    pub arousal_met: bool,
    /// Dominance >= floor.
    pub dominance_met: bool,
    /// Energy >= threshold.
    pub energy_met: bool,
    /// Alertness >= threshold.
    pub alertness_met: bool,
}

impl FlowConditions {
    /// Whether all conditions for flow are met.
    #[must_use]
    #[inline]
    pub fn all_met(&self) -> bool {
        self.interest_met
            && self.frustration_met
            && self.arousal_met
            && self.dominance_met
            && self.energy_met
            && self.alertness_met
    }

    /// Count of conditions met (out of 6).
    #[must_use]
    pub fn count_met(&self) -> u8 {
        let bools = [
            self.interest_met,
            self.frustration_met,
            self.arousal_met,
            self.dominance_met,
            self.energy_met,
            self.alertness_met,
        ];
        bools.iter().filter(|&&b| b).count() as u8
    }
}

/// Derive flow thresholds from personality.
///
/// - High curiosity + creativity → easier flow entry (lower thresholds)
/// - High patience → faster build rate (sustains conditions longer)
/// - High confidence → higher arousal ceiling (handles more excitement)
#[cfg(feature = "traits")]
#[must_use]
pub fn flow_from_personality(profile: &crate::traits::PersonalityProfile) -> FlowState {
    use crate::traits::TraitKind;
    let curiosity = profile.get_trait(TraitKind::Curiosity).normalized();
    let creativity = profile.get_trait(TraitKind::Creativity).normalized();
    let patience = profile.get_trait(TraitKind::Patience).normalized();
    let confidence = profile.get_trait(TraitKind::Confidence).normalized();

    let flow_affinity = (curiosity + creativity) / 2.0;

    FlowState {
        interest_threshold: (0.4 - flow_affinity * 0.1).clamp(0.2, 0.6),
        frustration_ceiling: (0.3 + patience * 0.1).clamp(0.1, 0.5),
        arousal_ceiling: (0.7 + confidence * 0.1).clamp(0.5, 0.9),
        build_rate: (0.05 + patience * 0.02).clamp(0.02, 0.1),
        ..FlowState::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mood::Emotion;

    /// Create a mood vector with flow-friendly conditions.
    fn flow_mood() -> MoodVector {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Interest, 0.6);
        m.set(Emotion::Arousal, 0.3);
        m.set(Emotion::Dominance, 0.3);
        m.set(Emotion::Frustration, 0.1);
        m
    }

    /// Create a mood that breaks flow (high frustration).
    fn breaking_mood() -> MoodVector {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, 0.8);
        m
    }

    #[test]
    fn test_flow_new() {
        let f = FlowState::new();
        assert_eq!(f.phase, FlowPhase::Inactive);
        assert!(f.accumulator.abs() < f32::EPSILON);
        assert_eq!(f.flow_duration, 0);
    }

    #[test]
    fn test_conditions_all_met() {
        let f = FlowState::new();
        let cond = f.check_conditions(&flow_mood(), 0.5, 0.5);
        assert!(cond.all_met());
        assert_eq!(cond.count_met(), 6);
    }

    #[test]
    fn test_conditions_interest_not_met() {
        let f = FlowState::new();
        let mut m = flow_mood();
        m.set(Emotion::Interest, 0.1); // below 0.4 threshold
        let cond = f.check_conditions(&m, 0.5, 0.5);
        assert!(!cond.interest_met);
        assert!(!cond.all_met());
        assert_eq!(cond.count_met(), 5);
    }

    #[test]
    fn test_conditions_frustration_too_high() {
        let f = FlowState::new();
        let mut m = flow_mood();
        m.set(Emotion::Frustration, 0.5);
        let cond = f.check_conditions(&m, 0.5, 0.5);
        assert!(!cond.frustration_met);
    }

    #[test]
    fn test_conditions_arousal_too_low() {
        let f = FlowState::new();
        let mut m = flow_mood();
        m.set(Emotion::Arousal, 0.0);
        let cond = f.check_conditions(&m, 0.5, 0.5);
        assert!(!cond.arousal_met);
    }

    #[test]
    fn test_conditions_arousal_too_high() {
        let f = FlowState::new();
        let mut m = flow_mood();
        m.set(Emotion::Arousal, 0.9);
        let cond = f.check_conditions(&m, 0.5, 0.5);
        assert!(!cond.arousal_met);
    }

    #[test]
    fn test_conditions_low_energy() {
        let f = FlowState::new();
        let cond = f.check_conditions(&flow_mood(), 0.1, 0.5);
        assert!(!cond.energy_met);
    }

    #[test]
    fn test_conditions_low_alertness() {
        let f = FlowState::new();
        let cond = f.check_conditions(&flow_mood(), 0.5, 0.1);
        assert!(!cond.alertness_met);
    }

    #[test]
    fn test_flow_builds_gradually() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        // Should need ~20 ticks (build_rate 0.05, threshold 1.0)
        for _ in 0..10 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert_eq!(f.phase, FlowPhase::Building, "should still be building");
        assert!(!f.is_in_flow());
        assert!(f.is_building());
        assert!(f.build_progress() > 0.4);
    }

    #[test]
    fn test_flow_enters_after_sustained_conditions() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        for _ in 0..25 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert_eq!(f.phase, FlowPhase::Active, "should be in flow");
        assert!(f.is_in_flow());
    }

    #[test]
    fn test_flow_breaks_instantly() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        // Enter flow
        for _ in 0..25 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert!(f.is_in_flow());
        // One bad tick
        f.tick(&breaking_mood(), 0.5, 0.5);
        assert_eq!(f.phase, FlowPhase::Disrupted);
        assert!(!f.is_in_flow());
    }

    #[test]
    fn test_flow_refractory() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        // Enter flow
        for _ in 0..25 {
            f.tick(&mood, 0.5, 0.5);
        }
        // Break
        f.tick(&breaking_mood(), 0.5, 0.5);
        assert_eq!(f.phase, FlowPhase::Disrupted);
        // One tick clears refractory
        f.tick(&mood, 0.5, 0.5);
        assert_eq!(f.phase, FlowPhase::Inactive);
    }

    #[test]
    fn test_accumulator_resets_on_break() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        // Build partway
        for _ in 0..10 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert!(f.accumulator > 0.0);
        // Break
        f.tick(&breaking_mood(), 0.5, 0.5);
        assert!(f.accumulator.abs() < f32::EPSILON, "should reset");
    }

    #[test]
    fn test_full_lifecycle() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        // Inactive → Building
        f.tick(&mood, 0.5, 0.5);
        assert_eq!(f.phase, FlowPhase::Building);
        // Building → Active
        for _ in 0..24 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert_eq!(f.phase, FlowPhase::Active);
        // Active → Disrupted
        f.tick(&breaking_mood(), 0.5, 0.5);
        assert_eq!(f.phase, FlowPhase::Disrupted);
        // Disrupted → Inactive
        f.tick(&mood, 0.5, 0.5);
        assert_eq!(f.phase, FlowPhase::Inactive);
    }

    #[test]
    fn test_performance_bonus_inactive() {
        let f = FlowState::new();
        assert!((f.performance_bonus() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_performance_bonus_ramps() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        // Enter flow
        for _ in 0..25 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert!(f.is_in_flow());
        let early = f.performance_bonus();
        assert!(early >= 1.1, "early bonus: {early}");
        // Run flow for a while
        for _ in 0..60 {
            f.tick(&mood, 0.5, 0.5);
        }
        let deep = f.performance_bonus();
        assert!(deep > early, "deep={deep} should > early={early}");
        assert!(
            deep <= 1.3 + f32::EPSILON,
            "deep bonus should cap at ~1.3: {deep}"
        );
    }

    #[test]
    fn test_energy_drain_modifier() {
        let mut f = FlowState::new();
        assert!((f.energy_drain_modifier() - 1.0).abs() < f32::EPSILON);
        let mood = flow_mood();
        for _ in 0..25 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert!((f.energy_drain_modifier() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stress_modifier() {
        let mut f = FlowState::new();
        assert!((f.stress_accumulation_modifier() - 1.0).abs() < f32::EPSILON);
        let mood = flow_mood();
        for _ in 0..25 {
            f.tick(&mood, 0.5, 0.5);
        }
        assert!((f.stress_accumulation_modifier() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_flow_requires_energy() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        for _ in 0..30 {
            f.tick(&mood, 0.1, 0.5); // low energy
        }
        assert!(!f.is_in_flow(), "should not enter flow with low energy");
    }

    #[test]
    fn test_flow_requires_alertness() {
        let mut f = FlowState::new();
        let mood = flow_mood();
        for _ in 0..30 {
            f.tick(&mood, 0.5, 0.1); // low alertness
        }
        assert!(!f.is_in_flow(), "should not enter flow when sleepy");
    }

    #[test]
    fn test_build_progress() {
        let mut f = FlowState::new();
        assert!(f.build_progress().abs() < f32::EPSILON);
        let mood = flow_mood();
        for _ in 0..10 {
            f.tick(&mood, 0.5, 0.5);
        }
        let progress = f.build_progress();
        assert!(progress > 0.4 && progress < 0.7, "progress: {progress}");
    }

    #[test]
    fn test_build_progress_zero_threshold() {
        let mut f = FlowState::new();
        f.entry_threshold = 0.0;
        assert!(f.build_progress().abs() < f32::EPSILON);
    }

    #[test]
    fn test_flow_duration_saturates() {
        let mut f = FlowState::new();
        f.phase = FlowPhase::Active;
        f.flow_duration = u32::MAX;
        f.tick(&flow_mood(), 0.5, 0.5);
        assert_eq!(f.flow_duration, u32::MAX); // saturating_add
    }

    #[test]
    fn test_phase_display() {
        assert_eq!(FlowPhase::Inactive.to_string(), "inactive");
        assert_eq!(FlowPhase::Building.to_string(), "building");
        assert_eq!(FlowPhase::Active.to_string(), "in flow");
        assert_eq!(FlowPhase::Disrupted.to_string(), "disrupted");
    }

    #[test]
    fn test_serde() {
        let mut f = FlowState::new();
        f.phase = FlowPhase::Active;
        f.flow_duration = 42;
        let json = serde_json::to_string(&f).unwrap();
        let f2: FlowState = serde_json::from_str(&json).unwrap();
        assert_eq!(f2.phase, FlowPhase::Active);
        assert_eq!(f2.flow_duration, 42);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_flow_from_personality_curious() {
        let mut p = crate::traits::PersonalityProfile::new("curious");
        p.set_trait(
            crate::traits::TraitKind::Curiosity,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Creativity,
            crate::traits::TraitLevel::Highest,
        );
        let f = flow_from_personality(&p);
        assert!(
            f.interest_threshold < 0.4,
            "curious should enter flow easier: {}",
            f.interest_threshold
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_flow_from_personality_patient() {
        let mut p = crate::traits::PersonalityProfile::new("patient");
        p.set_trait(
            crate::traits::TraitKind::Patience,
            crate::traits::TraitLevel::Highest,
        );
        let f = flow_from_personality(&p);
        assert!(
            f.build_rate > 0.05,
            "patient should build flow faster: {}",
            f.build_rate
        );
        assert!(
            f.frustration_ceiling > 0.3,
            "patient should tolerate more frustration: {}",
            f.frustration_ceiling
        );
    }
}
