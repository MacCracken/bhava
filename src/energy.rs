//! Energy / fatigue system — depletable resource with Banister fitness-fatigue model.
//!
//! Energy is fuel that depletes with any exertion (positive or negative) and
//! recovers during rest. Distinct from stress, which is chronic damage from
//! negative emotions specifically.
//!
//! The Banister impulse-response model (1975) tracks two hidden variables:
//! - **Fitness** — builds slowly with exertion, decays slowly (long-term adaptation)
//! - **Fatigue** — builds quickly with exertion, decays quickly (short-term cost)
//!
//! Performance = sigmoid(fitness - fatigue). This produces the characteristic
//! supercompensation curve: performance dips during exertion, then rebounds
//! above baseline during recovery as fatigue clears faster than fitness.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::mood::MoodVector;

/// Depletable energy resource with Banister fitness-fatigue performance model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyState {
    /// Current energy level: 0.0 (depleted) to 1.0 (full).
    pub energy: f32,
    /// Fitness impulse-response (builds slowly, decays slowly).
    pub fitness: f32,
    /// Fatigue impulse-response (builds quickly, decays quickly).
    pub fatigue: f32,
    /// Base energy recovery rate per tick during rest.
    pub recovery_rate: f32,
    /// Base energy drain rate per tick during exertion.
    pub drain_rate: f32,
    /// Fitness decay time constant (higher = slower decay).
    pub fitness_tau: f32,
    /// Fatigue decay time constant (lower = faster decay).
    pub fatigue_tau: f32,
    /// Fitness gain per unit of exertion (k₁).
    pub fitness_gain: f32,
    /// Fatigue gain per unit of exertion (k₂, typically > k₁).
    pub fatigue_gain: f32,
}

impl Default for EnergyState {
    fn default() -> Self {
        Self {
            energy: 1.0,
            fitness: 0.0,
            fatigue: 0.0,
            recovery_rate: 0.03,
            drain_rate: 0.02,
            fitness_tau: 60.0,
            fatigue_tau: 15.0,
            fitness_gain: 0.01,
            fatigue_gain: 0.03,
        }
    }
}

impl EnergyState {
    /// Create a new energy state at full energy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Tick the energy system.
    ///
    /// `exertion` is 0.0 (resting) to 1.0 (maximum effort). Drains energy,
    /// updates Banister fitness/fatigue impulse-response variables.
    pub fn tick(&mut self, exertion: f32) {
        let exertion = exertion.clamp(0.0, 1.0);

        // Banister impulse-response:
        //   fitness(n+1) = fitness(n) × e^(-1/τ₁) + k₁ × exertion
        //   fatigue(n+1) = fatigue(n) × e^(-1/τ₂) + k₂ × exertion
        if self.fitness_tau > 0.0 {
            self.fitness =
                self.fitness * (-1.0 / self.fitness_tau).exp() + self.fitness_gain * exertion;
        }
        if self.fatigue_tau > 0.0 {
            self.fatigue =
                self.fatigue * (-1.0 / self.fatigue_tau).exp() + self.fatigue_gain * exertion;
        }

        // Energy resource: drain under exertion, recover at rest
        if exertion > 0.1 {
            self.energy -= exertion * self.drain_rate;
        } else {
            self.energy += self.recovery_rate * (1.0 - exertion);
        }

        self.energy = self.energy.clamp(0.0, 1.0);
        self.fitness = self.fitness.clamp(0.0, 5.0);
        self.fatigue = self.fatigue.clamp(0.0, 5.0);
    }

    /// Banister cognitive performance: sigmoid of (fitness - fatigue).
    ///
    /// Returns 0.0–1.0. Above 0.5 means net-positive adaptation (trained).
    /// Below 0.5 means overreached (fatigue dominates). The sigmoid
    /// `1 / (1 + e^(-4x))` gives the characteristic S-curve.
    #[must_use]
    #[inline]
    pub fn performance(&self) -> f32 {
        let raw = self.fitness - self.fatigue;
        1.0 / (1.0 + (-4.0 * raw).exp())
    }

    /// Energy level classification.
    #[must_use]
    pub fn level(&self) -> EnergyLevel {
        if self.energy < 0.1 {
            EnergyLevel::Depleted
        } else if self.energy < 0.3 {
            EnergyLevel::Low
        } else if self.energy < 0.6 {
            EnergyLevel::Moderate
        } else if self.energy < 0.9 {
            EnergyLevel::High
        } else {
            EnergyLevel::Full
        }
    }

    /// Whether the entity has enough energy for flow state entry.
    #[must_use]
    #[inline]
    pub fn can_enter_flow(&self) -> bool {
        self.energy >= 0.3
    }

    /// Regulation effectiveness modifier from energy.
    ///
    /// Low energy reduces ability to regulate emotions.
    /// Returns 1.0 at full energy, down to 0.5 when depleted.
    #[must_use]
    #[inline]
    pub fn regulation_effectiveness(&self) -> f32 {
        0.5 + self.energy * 0.5
    }

    /// Whether the entity is depleted (energy < 0.1).
    #[must_use]
    #[inline]
    pub fn is_depleted(&self) -> bool {
        self.energy < 0.1
    }

    /// Apply a recovery modifier (e.g., from circadian alertness).
    ///
    /// Modifier > 1.0 accelerates recovery; < 1.0 slows it.
    /// Only applies bonus recovery — does not drain.
    pub fn apply_recovery_modifier(&mut self, modifier: f32) {
        let bonus = self.recovery_rate * (modifier - 1.0).max(0.0);
        self.energy = (self.energy + bonus).min(1.0);
    }
}

/// Named energy level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EnergyLevel {
    /// Energy < 0.1 — cannot function effectively.
    Depleted,
    /// Energy 0.1–0.3 — impaired, cannot enter flow.
    Low,
    /// Energy 0.3–0.6 — functional, can enter flow.
    Moderate,
    /// Energy 0.6–0.9 — good capacity.
    High,
    /// Energy >= 0.9 — fully charged.
    Full,
}

impl fmt::Display for EnergyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Depleted => "depleted",
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Full => "full",
        };
        f.write_str(s)
    }
}

/// Compute exertion level from current mood.
///
/// Any intense emotional state costs energy, not just negative ones.
/// Uses mood intensity (L2 norm) as a proxy for cognitive load.
/// Returns 0.0 (neutral) to 1.0 (maximum intensity).
#[must_use]
#[inline]
pub fn exertion_from_mood(mood: &MoodVector) -> f32 {
    // Max intensity is sqrt(6) ≈ 2.449 (all dimensions at ±1.0)
    (mood.intensity() / 2.449).clamp(0.0, 1.0)
}

/// Derive energy parameters from personality.
///
/// - High patience + confidence → better recovery (resilient)
/// - High curiosity → slightly higher drain (active mind)
#[cfg(feature = "traits")]
#[must_use]
pub fn energy_from_personality(profile: &crate::traits::PersonalityProfile) -> EnergyState {
    use crate::traits::TraitKind;
    let patience = profile.get_trait(TraitKind::Patience).normalized();
    let confidence = profile.get_trait(TraitKind::Confidence).normalized();
    let curiosity = profile.get_trait(TraitKind::Curiosity).normalized();

    let resilience = (patience + confidence) / 2.0;

    EnergyState {
        recovery_rate: (0.03 + resilience * 0.01).clamp(0.01, 0.06),
        drain_rate: (0.02 + curiosity * 0.005).clamp(0.01, 0.04),
        ..EnergyState::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mood::Emotion;

    #[test]
    fn test_energy_new() {
        let e = EnergyState::new();
        assert!((e.energy - 1.0).abs() < f32::EPSILON);
        assert_eq!(e.level(), EnergyLevel::Full);
    }

    #[test]
    fn test_tick_rest_recovers() {
        let mut e = EnergyState::new();
        e.energy = 0.5;
        e.tick(0.0); // rest
        assert!(e.energy > 0.5, "rest should recover: {}", e.energy);
    }

    #[test]
    fn test_tick_exertion_drains() {
        let mut e = EnergyState::new();
        e.tick(0.8);
        assert!(e.energy < 1.0, "exertion should drain: {}", e.energy);
    }

    #[test]
    fn test_energy_clamped() {
        let mut e = EnergyState::new();
        // Drain to zero
        for _ in 0..200 {
            e.tick(1.0);
        }
        assert!(e.energy >= 0.0);
        assert!(e.energy <= 1.0);

        // Recover to full
        for _ in 0..200 {
            e.tick(0.0);
        }
        assert!(e.energy <= 1.0);
    }

    #[test]
    fn test_banister_fitness_builds() {
        let mut e = EnergyState::new();
        for _ in 0..20 {
            e.tick(0.8);
        }
        assert!(e.fitness > 0.0, "fitness should build: {}", e.fitness);
    }

    #[test]
    fn test_banister_fatigue_builds_faster() {
        let mut e = EnergyState::new();
        for _ in 0..10 {
            e.tick(0.8);
        }
        // fatigue_gain (0.03) > fitness_gain (0.01), so fatigue > fitness initially
        assert!(
            e.fatigue > e.fitness,
            "fatigue={} should exceed fitness={}",
            e.fatigue,
            e.fitness
        );
    }

    #[test]
    fn test_fatigue_decays_faster_than_fitness() {
        let mut e = EnergyState::new();
        // Build up both
        for _ in 0..20 {
            e.tick(0.8);
        }
        let fitness_after_exertion = e.fitness;
        let fatigue_after_exertion = e.fatigue;
        // Rest for a while
        for _ in 0..30 {
            e.tick(0.0);
        }
        // Fatigue should have decayed proportionally more
        let fitness_decay_ratio = e.fitness / fitness_after_exertion;
        let fatigue_decay_ratio = e.fatigue / fatigue_after_exertion;
        assert!(
            fatigue_decay_ratio < fitness_decay_ratio,
            "fatigue_ratio={} fitness_ratio={}",
            fatigue_decay_ratio,
            fitness_decay_ratio
        );
    }

    #[test]
    fn test_performance_sigmoid_range() {
        let mut e = EnergyState::new();
        assert!(e.performance() >= 0.0 && e.performance() <= 1.0);
        // At default (fitness=0, fatigue=0), performance = sigmoid(0) = 0.5
        assert!(
            (e.performance() - 0.5).abs() < f32::EPSILON,
            "zero state: {}",
            e.performance()
        );

        // After lots of exertion, fatigue dominates → performance < 0.5
        for _ in 0..50 {
            e.tick(1.0);
        }
        assert!(e.performance() < 0.5, "overreached: {}", e.performance());
    }

    #[test]
    fn test_supercompensation() {
        let mut e = EnergyState::new();
        // Train
        for _ in 0..20 {
            e.tick(0.5);
        }
        // Rest — fatigue clears faster, leaving net fitness
        for _ in 0..100 {
            e.tick(0.0);
        }
        assert!(
            e.performance() > 0.5,
            "supercompensation should raise performance: {}",
            e.performance()
        );
    }

    #[test]
    fn test_can_enter_flow() {
        let mut e = EnergyState::new();
        assert!(e.can_enter_flow());
        e.energy = 0.2;
        assert!(!e.can_enter_flow());
        e.energy = 0.3;
        assert!(e.can_enter_flow());
    }

    #[test]
    fn test_regulation_effectiveness() {
        let mut e = EnergyState::new();
        assert!((e.regulation_effectiveness() - 1.0).abs() < f32::EPSILON);
        e.energy = 0.0;
        assert!((e.regulation_effectiveness() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_is_depleted() {
        let mut e = EnergyState::new();
        assert!(!e.is_depleted());
        e.energy = 0.05;
        assert!(e.is_depleted());
    }

    #[test]
    fn test_level_classification() {
        let mut e = EnergyState::new();
        e.energy = 0.05;
        assert_eq!(e.level(), EnergyLevel::Depleted);
        e.energy = 0.2;
        assert_eq!(e.level(), EnergyLevel::Low);
        e.energy = 0.5;
        assert_eq!(e.level(), EnergyLevel::Moderate);
        e.energy = 0.8;
        assert_eq!(e.level(), EnergyLevel::High);
        e.energy = 0.95;
        assert_eq!(e.level(), EnergyLevel::Full);
    }

    #[test]
    fn test_exertion_from_mood_neutral() {
        let mood = MoodVector::neutral();
        assert!(exertion_from_mood(&mood) < f32::EPSILON);
    }

    #[test]
    fn test_exertion_from_mood_intense() {
        let mut mood = MoodVector::neutral();
        mood.set(Emotion::Joy, 0.9);
        mood.set(Emotion::Arousal, 0.8);
        let ex = exertion_from_mood(&mood);
        assert!(ex > 0.3, "intense mood exertion: {ex}");
        assert!(ex <= 1.0);
    }

    #[test]
    fn test_apply_recovery_modifier() {
        let mut e = EnergyState::new();
        e.energy = 0.5;
        e.apply_recovery_modifier(1.5);
        assert!(e.energy > 0.5, "modifier > 1 should boost: {}", e.energy);
    }

    #[test]
    fn test_apply_recovery_modifier_no_drain() {
        let mut e = EnergyState::new();
        e.energy = 0.5;
        let before = e.energy;
        e.apply_recovery_modifier(0.5); // below 1.0
        assert!(
            (e.energy - before).abs() < f32::EPSILON,
            "modifier < 1 should not drain"
        );
    }

    #[test]
    fn test_zero_tau_safe() {
        let mut e = EnergyState::new();
        e.fitness_tau = 0.0;
        e.fatigue_tau = 0.0;
        e.tick(0.5); // should not panic or NaN
        assert!(e.fitness.is_finite());
        assert!(e.fatigue.is_finite());
    }

    #[test]
    fn test_energy_level_display() {
        assert_eq!(EnergyLevel::Depleted.to_string(), "depleted");
        assert_eq!(EnergyLevel::Full.to_string(), "full");
    }

    #[test]
    fn test_serde() {
        let e = EnergyState::new();
        let json = serde_json::to_string(&e).unwrap();
        let e2: EnergyState = serde_json::from_str(&json).unwrap();
        assert!((e2.energy - e.energy).abs() < f32::EPSILON);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_energy_from_personality() {
        let mut patient = crate::traits::PersonalityProfile::new("patient");
        patient.set_trait(
            crate::traits::TraitKind::Patience,
            crate::traits::TraitLevel::Highest,
        );
        patient.set_trait(
            crate::traits::TraitKind::Confidence,
            crate::traits::TraitLevel::Highest,
        );
        let e = energy_from_personality(&patient);
        assert!(e.recovery_rate > 0.03, "patient should recover faster");

        let mut curious = crate::traits::PersonalityProfile::new("curious");
        curious.set_trait(
            crate::traits::TraitKind::Curiosity,
            crate::traits::TraitLevel::Highest,
        );
        let e2 = energy_from_personality(&curious);
        assert!(e2.drain_rate > 0.02, "curious should drain faster");
    }
}
