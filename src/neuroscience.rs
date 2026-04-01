//! Mastishk neuroscience integration — brain chemistry pressing on emotion and personality.
//!
//! Provides bridge functions between mastishk's neural state and bhava's
//! emotion/personality systems. The brain presses on mood constantly:
//! serotonin sets the baseline, dopamine drives reward, cortisol amplifies
//! stress, norepinephrine modulates arousal, and sleep debt saps energy.
//!
//! Requires the `neuroscience` feature.
//!
//! # Layer Model
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Bhava (Personality Engine)      │
//! │  Mood, stress, energy, flow      │
//! ├──────────────────────────────────┤
//! │  This module (bridge)            │
//! │  BrainMoodEffect → bhava states  │
//! ├──────────────────────────────────┤
//! │  Mastishk (Neuroscience Engine)  │
//! │  NT, HPA, sleep, DMN, regions    │
//! └──────────────────────────────────┘
//! ```
//!
//! # Bridge Functions
//!
//! ## Mood
//! - [`mood_from_brain`] — serotonin baseline + amygdala fear + seasonal modifier → MoodVector
//!
//! ## Stress
//! - [`stress_from_brain`] — cortisol amplifier + rumination + burnout → stress multiplier
//!
//! ## Energy
//! - [`energy_from_brain`] — sleep debt + drowsiness + sickness → energy modifiers
//!
//! ## Flow / Focus
//! - [`flow_from_brain`] — ACh focus + executive control + working memory → flow threshold
//!
//! ## Growth / Plasticity
//! - [`growth_from_brain`] — BDNF + learning rate → growth rate modifier
//!
//! ## Regulation
//! - [`regulation_from_brain`] — meditation boost + executive control → regulation effectiveness
//!
//! # Example
//!
//! ```
//! use bhava::neuroscience::mood_from_brain;
//! use mastishk::bridge::BrainMoodEffect;
//!
//! // Default brain state → neutral mood shift
//! let effect = mastishk::bridge::brain_mood_modifiers(&mastishk::brain::BrainState::default());
//! let mood = mood_from_brain(&effect);
//! assert!(mood.joy.abs() < 0.5);
//! ```

use crate::mood::MoodVector;

// ── Mood ──────────────────────────────────────────────────────────────

/// Map brain chemistry to mood vector offsets.
///
/// Serotonin drives the joy baseline. Norepinephrine drives arousal.
/// Amygdala fear reduces trust and dominance. Seasonal serotonin modifier
/// scales the mood effect across seasons (SAD pattern).
///
/// Returns a `MoodVector` of deltas — apply via `mood.nudge()`.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn mood_from_brain(effect: &mastishk::bridge::BrainMoodEffect) -> MoodVector {
    let seasonal_scale = effect.seasonal_modifier; // 0.7–1.3

    MoodVector {
        joy: (effect.mood_offset * 0.3 * seasonal_scale) as f32,
        arousal: (effect.arousal * 0.4 - 0.2) as f32, // 0.0→-0.2, 0.5→0.0, 1.0→+0.2
        dominance: (-effect.fear_level * 0.2) as f32, // fear reduces dominance
        trust: (-effect.fear_level * 0.15 - effect.interoceptive_anxiety * 0.1) as f32,
        interest: (effect.reward_sensitivity * 0.2 - 0.1) as f32,
        frustration: (effect.anxiety * 0.2) as f32,
    }
}

// ── Stress ────────────────────────────────────────────────────────────

/// Map brain chemistry to stress accumulation modifier.
///
/// Cortisol amplifies stress accumulation (1.0–3.0×). Rumination adds
/// chronic stress input. Allostatic load / burnout reduces stress ceiling.
/// Endorphins dampen stress recovery (1.0–2.0× recovery boost).
///
/// Returns `(accumulation_multiplier, recovery_multiplier)`.
///
/// # Example
///
/// ```
/// use bhava::neuroscience::stress_from_brain;
/// use mastishk::bridge::BrainMoodEffect;
///
/// let effect = mastishk::bridge::brain_mood_modifiers(&mastishk::brain::BrainState::default());
/// let (accum, recovery) = stress_from_brain(&effect);
/// assert!(accum >= 1.0);
/// assert!(recovery >= 1.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn stress_from_brain(effect: &mastishk::bridge::BrainMoodEffect) -> (f32, f32) {
    // Cortisol drives accumulation (1.0–3.0×), rumination adds on top
    let accumulation = (effect.stress_multiplier + effect.rumination_stress * 0.5) as f32;
    // Endorphins boost recovery, parasympathetic activation helps
    let recovery = (effect.pain_dampening * (1.0 + effect.parasympathetic * 0.3)) as f32;
    (accumulation.clamp(1.0, 4.0), recovery.clamp(1.0, 3.0))
}

// ── Energy ────────────────────────────────────────────────────────────

/// Map brain chemistry to energy drain/recovery modifiers.
///
/// Sleep debt increases drain and reduces recovery. Drowsiness (melatonin)
/// reduces alertness. Sickness behavior drains energy. Sympathetic activation
/// provides short-term energy boost at the cost of faster depletion.
///
/// Returns `(drain_multiplier, recovery_multiplier)`.
///
/// # Example
///
/// ```
/// use bhava::neuroscience::energy_from_brain;
/// use mastishk::bridge::BrainMoodEffect;
///
/// let effect = mastishk::bridge::brain_mood_modifiers(&mastishk::brain::BrainState::default());
/// let (drain, recovery) = energy_from_brain(&effect);
/// assert!(drain >= 0.5);
/// assert!(recovery > 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn energy_from_brain(effect: &mastishk::bridge::BrainMoodEffect) -> (f32, f32) {
    // Sleep debt + sickness + sympathetic overdrive increase drain
    let drain = 1.0
        + effect.energy_penalty * 0.5         // sleep debt
        + effect.sickness_behavior * 0.4      // inflammation fatigue
        + effect.sympathetic * 0.2; // fight-or-flight cost

    // Recovery boosted by sleep stage quality, dampened by drowsiness and inflammation
    let recovery = (1.0 + effect.recovery_rate * 0.5)
        * (1.0 - effect.drowsiness * 0.3)
        * (1.0 - effect.sickness_behavior * 0.3);

    (
        (drain as f32).clamp(0.5, 3.0),
        (recovery as f32).clamp(0.2, 2.0),
    )
}

// ── Flow / Focus ──────────────────────────────────────────────────────

/// Map brain chemistry to flow state threshold modifier.
///
/// High acetylcholine + executive control + working memory capacity lower
/// the threshold for entering flow. Anxiety, drowsiness, and sickness
/// raise it. Returns a multiplier on the flow entry threshold (lower = easier
/// to enter flow).
///
/// # Example
///
/// ```
/// use bhava::neuroscience::flow_from_brain;
/// use mastishk::bridge::BrainMoodEffect;
///
/// let effect = mastishk::bridge::brain_mood_modifiers(&mastishk::brain::BrainState::default());
/// let threshold_mult = flow_from_brain(&effect);
/// assert!(threshold_mult > 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn flow_from_brain(effect: &mastishk::bridge::BrainMoodEffect) -> f32 {
    // Focus components lower threshold (good)
    let focus_bonus =
        effect.focus * 0.3 + effect.executive_control * 0.2 + effect.working_memory * 0.2;

    // Disruption components raise threshold (bad)
    let disruption =
        effect.anxiety * 0.3 + effect.drowsiness * 0.2 + effect.sickness_behavior * 0.2;

    let threshold = 1.0 - focus_bonus + disruption;
    (threshold as f32).clamp(0.3, 2.0)
}

// ── Growth / Plasticity ───────────────────────────────────────────────

/// Map brain chemistry to trait growth rate modifier.
///
/// BDNF drives neuroplasticity — high BDNF = faster personality adaptation.
/// Hippocampus learning rate contributes. Sleep quality supports consolidation.
/// Returns a multiplier on trait pressure accumulation rate.
///
/// # Example
///
/// ```
/// use bhava::neuroscience::growth_from_brain;
/// use mastishk::bridge::BrainMoodEffect;
///
/// let effect = mastishk::bridge::brain_mood_modifiers(&mastishk::brain::BrainState::default());
/// let rate = growth_from_brain(&effect);
/// assert!(rate > 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn growth_from_brain(effect: &mastishk::bridge::BrainMoodEffect) -> f32 {
    let plasticity =
        effect.growth_plasticity * 0.5 + effect.learning_rate * 0.3 + effect.recovery_rate * 0.2; // sleep consolidation
    (plasticity as f32).clamp(0.1, 2.0)
}

// ── Regulation ────────────────────────────────────────────────────────

/// Map brain chemistry to emotion regulation effectiveness modifier.
///
/// Meditation boosts regulation (1.0–2.0×). Executive control from PFC
/// supports cognitive reappraisal. High HRV indicates strong vagal tone
/// and better regulation capacity. Returns a multiplier on regulation
/// effectiveness.
///
/// # Example
///
/// ```
/// use bhava::neuroscience::regulation_from_brain;
/// use mastishk::bridge::BrainMoodEffect;
///
/// let effect = mastishk::bridge::brain_mood_modifiers(&mastishk::brain::BrainState::default());
/// let reg = regulation_from_brain(&effect);
/// assert!(reg >= 0.5);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn regulation_from_brain(effect: &mastishk::bridge::BrainMoodEffect) -> f32 {
    let base = effect.regulation_boost * 0.5 + effect.executive_control * 0.3 + effect.hrv * 0.2;
    (base as f32).clamp(0.3, 2.5)
}

// ── Salience ──────────────────────────────────────────────────────────

/// Map brain chemistry to salience sensitivity modifier.
///
/// Amygdala emotional salience + norepinephrine arousal amplify salience.
/// Returns a multiplier on salience scoring.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn salience_from_brain(effect: &mastishk::bridge::BrainMoodEffect) -> f32 {
    let amp = 1.0 + effect.emotional_salience * 0.3 + effect.arousal * 0.2;
    (amp as f32).clamp(0.5, 2.0)
}

// ── Convenience: apply all ────────────────────────────────────────────

/// Apply all brain chemistry effects to bhava module states in one call.
///
/// Adjusts energy drain/recovery rates, stress accumulation/recovery, and
/// nudges mood baselines from brain state. Call once per tick with current
/// brain chemistry.
///
/// # Example
///
/// ```
/// use bhava::neuroscience::apply_brain_state;
/// use bhava::energy::EnergyState;
/// use bhava::stress::StressState;
/// use bhava::mood::MoodVector;
/// use mastishk::bridge::BrainMoodEffect;
///
/// let mut energy = EnergyState::new();
/// let mut stress = StressState::new();
/// let mut mood = MoodVector::neutral();
/// let effect = mastishk::bridge::brain_mood_modifiers(&mastishk::brain::BrainState::default());
///
/// apply_brain_state(&effect, &mut energy, &mut stress, &mut mood);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
pub fn apply_brain_state(
    effect: &mastishk::bridge::BrainMoodEffect,
    energy: &mut crate::energy::EnergyState,
    stress: &mut crate::stress::StressState,
    mood: &mut crate::mood::MoodVector,
) {
    // Energy
    let (drain, recovery) = energy_from_brain(effect);
    energy.drain_rate *= drain;
    energy.recovery_rate *= recovery;

    // Stress
    let (accum, recov) = stress_from_brain(effect);
    stress.accumulation_rate *= accum;
    stress.recovery_rate *= recov;

    // Mood
    let mood_delta = mood_from_brain(effect);
    use crate::mood::Emotion;
    mood.nudge(Emotion::Joy, mood_delta.joy);
    mood.nudge(Emotion::Arousal, mood_delta.arousal);
    mood.nudge(Emotion::Dominance, mood_delta.dominance);
    mood.nudge(Emotion::Trust, mood_delta.trust);
    mood.nudge(Emotion::Interest, mood_delta.interest);
    mood.nudge(Emotion::Frustration, mood_delta.frustration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mastishk::bridge::BrainMoodEffect;

    fn default_effect() -> BrainMoodEffect {
        BrainMoodEffect {
            mood_offset: 0.0,
            reward_sensitivity: 0.5,
            arousal: 0.5,
            anxiety: 0.3,
            focus: 0.5,
            pain_dampening: 1.0,
            stress_multiplier: 1.0,
            burnout: 0.0,
            energy_penalty: 0.0,
            recovery_rate: 0.0,
            drowsiness: 0.0,
            rumination_stress: 0.0,
            regulation_boost: 1.0,
            growth_plasticity: 0.5,
            executive_control: 0.5,
            working_memory: 0.5,
            fear_level: 0.0,
            emotional_salience: 0.3,
            learning_rate: 0.5,
            action_drive: 0.5,
            habit_level: 0.3,
            motor_quality: 0.5,
            sickness_behavior: 0.0,
            sympathetic: 0.3,
            parasympathetic: 0.5,
            hrv: 0.5,
            interoceptive_anxiety: 0.0,
            seasonal_modifier: 1.0,
        }
    }

    #[test]
    fn mood_from_brain_default_near_neutral() {
        let effect = default_effect();
        let mood = mood_from_brain(&effect);
        assert!(mood.joy.abs() < 0.5);
        assert!(mood.arousal.abs() < 0.5);
        assert!(mood.trust.abs() < 0.5);
    }

    #[test]
    fn mood_high_serotonin_positive() {
        let mut effect = default_effect();
        effect.mood_offset = 1.0; // high serotonin
        effect.seasonal_modifier = 1.0;
        let mood = mood_from_brain(&effect);
        assert!(mood.joy > 0.0);
    }

    #[test]
    fn mood_low_serotonin_negative() {
        let mut effect = default_effect();
        effect.mood_offset = -1.0; // depleted serotonin
        effect.seasonal_modifier = 1.0;
        let mood = mood_from_brain(&effect);
        assert!(mood.joy < 0.0);
    }

    #[test]
    fn mood_fear_reduces_trust_and_dominance() {
        let mut effect = default_effect();
        effect.fear_level = 0.8;
        let mood = mood_from_brain(&effect);
        assert!(mood.trust < 0.0);
        assert!(mood.dominance < 0.0);
    }

    #[test]
    fn stress_cortisol_amplifies() {
        let mut effect = default_effect();
        effect.stress_multiplier = 3.0; // high cortisol
        let (accum, _) = stress_from_brain(&effect);
        assert!(accum > 2.0);
    }

    #[test]
    fn stress_endorphins_boost_recovery() {
        let mut effect = default_effect();
        effect.pain_dampening = 2.0; // high endorphins
        let (_, recovery) = stress_from_brain(&effect);
        assert!(recovery > 1.5);
    }

    #[test]
    fn stress_rumination_adds() {
        let mut effect = default_effect();
        effect.stress_multiplier = 1.0;
        let (base_accum, _) = stress_from_brain(&effect);
        effect.rumination_stress = 0.8;
        let (rum_accum, _) = stress_from_brain(&effect);
        assert!(rum_accum > base_accum);
    }

    #[test]
    fn energy_sleep_debt_drains() {
        let mut effect = default_effect();
        effect.energy_penalty = 0.8;
        let (drain, _) = energy_from_brain(&effect);
        assert!(drain > 1.2);
    }

    #[test]
    fn energy_sickness_drains() {
        let mut effect = default_effect();
        effect.sickness_behavior = 0.8;
        let (drain, recovery) = energy_from_brain(&effect);
        assert!(drain > 1.2);
        assert!(recovery < 1.0); // sickness impairs recovery
    }

    #[test]
    fn energy_deep_sleep_recovers() {
        let mut effect = default_effect();
        effect.recovery_rate = 1.0; // deep NREM3
        effect.drowsiness = 0.0;
        effect.sickness_behavior = 0.0;
        let (_, recovery) = energy_from_brain(&effect);
        assert!(recovery > 1.2);
    }

    #[test]
    fn flow_focus_lowers_threshold() {
        let mut effect = default_effect();
        effect.focus = 0.9;
        effect.executive_control = 0.8;
        effect.working_memory = 0.8;
        effect.anxiety = 0.0;
        let threshold = flow_from_brain(&effect);
        assert!(threshold < 1.0); // easier to enter flow
    }

    #[test]
    fn flow_anxiety_raises_threshold() {
        let mut effect = default_effect();
        effect.anxiety = 0.9;
        effect.focus = 0.1;
        effect.executive_control = 0.1;
        effect.working_memory = 0.1;
        let threshold = flow_from_brain(&effect);
        assert!(threshold > 1.0); // harder to enter flow
    }

    #[test]
    fn growth_bdnf_amplifies() {
        let mut effect = default_effect();
        effect.growth_plasticity = 0.9;
        effect.learning_rate = 0.8;
        let rate = growth_from_brain(&effect);
        assert!(rate > 0.5);
    }

    #[test]
    fn regulation_meditation_boosts() {
        let mut effect = default_effect();
        effect.regulation_boost = 2.0; // deep meditation
        effect.executive_control = 0.9;
        effect.hrv = 0.9;
        let reg = regulation_from_brain(&effect);
        assert!(reg > 1.4);
    }

    #[test]
    fn salience_amygdala_amplifies() {
        let mut effect = default_effect();
        effect.emotional_salience = 0.9;
        effect.arousal = 0.8;
        let sal = salience_from_brain(&effect);
        assert!(sal > 1.3);
    }

    #[test]
    fn apply_brain_state_mutates() {
        let mut energy = crate::energy::EnergyState::new();
        let mut stress = crate::stress::StressState::new();
        let mut mood = crate::mood::MoodVector::neutral();

        let mut effect = default_effect();
        effect.stress_multiplier = 2.0;
        effect.energy_penalty = 0.5;

        let base_drain = energy.drain_rate;
        let base_stress = stress.accumulation_rate;

        apply_brain_state(&effect, &mut energy, &mut stress, &mut mood);

        assert!(energy.drain_rate > base_drain);
        assert!(stress.accumulation_rate > base_stress);
    }

    #[test]
    fn all_modifiers_clamped() {
        // Extreme values should not produce unbounded results
        let effect = BrainMoodEffect {
            mood_offset: 10.0,
            reward_sensitivity: 10.0,
            arousal: 10.0,
            anxiety: 10.0,
            focus: 10.0,
            pain_dampening: 10.0,
            stress_multiplier: 10.0,
            burnout: 10.0,
            energy_penalty: 10.0,
            recovery_rate: 10.0,
            drowsiness: 10.0,
            rumination_stress: 10.0,
            regulation_boost: 10.0,
            growth_plasticity: 10.0,
            executive_control: 10.0,
            working_memory: 10.0,
            fear_level: 10.0,
            emotional_salience: 10.0,
            learning_rate: 10.0,
            action_drive: 10.0,
            habit_level: 10.0,
            motor_quality: 10.0,
            sickness_behavior: 10.0,
            sympathetic: 10.0,
            parasympathetic: 10.0,
            hrv: 10.0,
            interoceptive_anxiety: 10.0,
            seasonal_modifier: 10.0,
        };
        let (accum, recov) = stress_from_brain(&effect);
        assert!(accum <= 4.0);
        assert!(recov <= 3.0);
        let (drain, recovery) = energy_from_brain(&effect);
        assert!(drain <= 3.0);
        assert!(recovery <= 2.0);
        assert!(flow_from_brain(&effect) <= 2.0);
        assert!(growth_from_brain(&effect) <= 2.0);
        assert!(regulation_from_brain(&effect) <= 2.5);
        assert!(salience_from_brain(&effect) <= 2.0);
    }

    #[test]
    fn serde_roundtrip_brain_mood_effect() {
        let effect = default_effect();
        let json = serde_json::to_string(&effect).unwrap();
        let deser: BrainMoodEffect = serde_json::from_str(&json).unwrap();
        assert!((effect.mood_offset - deser.mood_offset).abs() < f64::EPSILON);
    }
}
