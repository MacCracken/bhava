//! Sharira physiology integration — body state pressing on emotion and personality.
//!
//! Provides bridge functions between sharira's biomechanical body state and
//! bhava's emotion/personality systems. The body presses on mood constantly:
//! fatigue breeds irritability, pain drives stress, imbalance triggers anxiety,
//! exertion drains energy, and physical capability shapes confidence.
//!
//! Requires the `physiology` feature.
//!
//! # Layer Model
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Bhava (Personality Engine)      │
//! │  Mood, stress, energy, flow      │
//! ├──────────────────────────────────┤
//! │  This module (bridge)            │
//! │  Body state → Emotion effects    │
//! ├──────────────────────────────────┤
//! │  Sharira (Physiology Engine)     │
//! │  Skeleton, muscle, fatigue, gait │
//! └──────────────────────────────────┘
//! ```
//!
//! # Bridge Functions
//!
//! ## Fatigue → Mood
//! - [`mood_from_fatigue`] — muscular fatigue → irritability, reduced joy
//! - [`energy_drain_from_fatigue`] — fatigue capacity → energy depletion rate
//!
//! ## Pain / Injury → Stress
//! - [`stress_from_violation`] — joint constraint violation → stress input
//! - [`pain_intensity`] — total body violation → pain level (0–1)
//!
//! ## Balance → Anxiety
//! - [`anxiety_from_balance`] — stability margin → arousal/trust shift
//!
//! ## Exertion → Energy
//! - [`exertion_from_activation`] — muscle activation level → energy drain
//! - [`metabolic_load`] — allometric BMR → baseline energy cost
//!
//! ## Morphology → Confidence
//! - [`confidence_from_morphology`] — body mass/height → dominance bias
//!
//! ## Gait → Emotional Signal
//! - [`arousal_from_gait`] — locomotion type/speed → arousal level
//! - [`gait_emotional_valence`] — gait type → emotional association
//!
//! ## Heart Rate → Arousal
//! - [`arousal_from_heart_rate`] — allometric HR → physiological arousal

use crate::mood::MoodVector;

// ── Fatigue → Mood ─────────────────────────────────────────────────────

/// Convert muscular fatigue into a mood shift.
///
/// As fatigue capacity drops (1.0 = fresh, 0.0 = exhausted), joy decreases
/// and frustration increases. Moderate fatigue (capacity ~0.5) produces
/// irritability; severe fatigue (capacity < 0.2) produces despondency.
///
/// ```
/// use bhava::physiology::mood_from_fatigue;
///
/// let fresh = mood_from_fatigue(1.0);
/// assert!(fresh.frustration < 0.1);
///
/// let exhausted = mood_from_fatigue(0.1);
/// assert!(exhausted.frustration > 0.5);
/// assert!(exhausted.joy < 0.1);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn mood_from_fatigue(capacity: f32) -> MoodVector {
    let c = capacity.clamp(0.0, 1.0);
    let fatigue_level = 1.0 - c;
    MoodVector {
        joy: (c * 0.3).clamp(0.0, 1.0),
        arousal: (fatigue_level * -0.3).clamp(-1.0, 1.0),
        dominance: (c * 0.2 - 0.1).clamp(-1.0, 1.0),
        trust: 0.0,
        interest: (c * 0.2 - 0.1).clamp(-1.0, 1.0),
        frustration: (fatigue_level * 0.7).clamp(0.0, 1.0),
    }
}

/// Convert fatigue capacity to an energy drain rate multiplier.
///
/// Lower capacity → higher drain rate (the body works harder to compensate
/// for fatigued motor units). Returns a multiplier ≥ 1.0.
///
/// ```
/// use bhava::physiology::energy_drain_from_fatigue;
///
/// let fresh_drain = energy_drain_from_fatigue(1.0);
/// let tired_drain = energy_drain_from_fatigue(0.3);
/// assert!(tired_drain > fresh_drain);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn energy_drain_from_fatigue(capacity: f32) -> f32 {
    let c = capacity.clamp(0.01, 1.0);
    // Inverse relationship: less capacity → more drain. Floor at 1.0x.
    (1.0 / c).clamp(1.0, 5.0)
}

// ── Pain / Injury → Stress ─────────────────────────────────────────────

/// Convert total joint constraint violation to stress input.
///
/// Joint violations (in radians) indicate the body is being pushed beyond
/// its limits. Higher violation → higher stress accumulation rate.
/// Returns a stress input value suitable for bhava's stress system (0.0–1.0).
///
/// ```
/// use bhava::physiology::stress_from_violation;
///
/// assert!((stress_from_violation(0.0) - 0.0).abs() < 0.01);
/// assert!(stress_from_violation(1.0) > 0.3);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn stress_from_violation(total_violation_rad: f32) -> f32 {
    // Sigmoid mapping: violation → stress. 1 radian ≈ 0.46, 2 radians ≈ 0.73
    let v = total_violation_rad.max(0.0);
    (1.0 - (-v * 0.7).exp()).clamp(0.0, 1.0)
}

/// Convert total body violation to a normalized pain intensity.
///
/// Returns 0.0 (no pain) to 1.0 (extreme pain). Uses a diminishing-returns
/// curve — initial violations hurt most, subsequent ones add less.
///
/// ```
/// use bhava::physiology::pain_intensity;
///
/// assert!((pain_intensity(0.0) - 0.0).abs() < 0.01);
/// assert!(pain_intensity(0.5) > 0.0);
/// assert!(pain_intensity(3.0) > pain_intensity(1.0));
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn pain_intensity(total_violation_rad: f32) -> f32 {
    let v = total_violation_rad.max(0.0);
    // Logarithmic saturation: ln(1 + v) / ln(1 + max_expected)
    (1.0 + v).ln() / (1.0 + 5.0_f32).ln()
}

// ── Balance → Anxiety ──────────────────────────────────────────────────

/// Convert stability margin to an anxiety-related mood shift.
///
/// Positive margin = stable (calm). Zero or negative = falling (panic).
/// Returns a MoodVector biased toward anxiety when unstable.
///
/// ```
/// use bhava::physiology::anxiety_from_balance;
///
/// let stable = anxiety_from_balance(0.3);
/// assert!(stable.trust > 0.0);
/// assert!(stable.arousal < 0.1);
///
/// let falling = anxiety_from_balance(-0.5);
/// assert!(falling.arousal > 0.3);
/// assert!(falling.trust < 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn anxiety_from_balance(stability_margin: f32) -> MoodVector {
    let m = stability_margin.clamp(-1.0, 1.0);
    if m >= 0.0 {
        // Stable: calm, slight confidence
        MoodVector {
            joy: 0.0,
            arousal: (-m * 0.2).clamp(-1.0, 0.0),
            dominance: (m * 0.3).clamp(0.0, 1.0),
            trust: (m * 0.5).clamp(0.0, 1.0),
            interest: 0.0,
            frustration: 0.0,
        }
    } else {
        // Unstable: panic, fear
        let panic = (-m).clamp(0.0, 1.0);
        MoodVector {
            joy: 0.0,
            arousal: (panic * 0.9).clamp(0.0, 1.0),
            dominance: (-panic * 0.5).clamp(-1.0, 0.0),
            trust: (-panic * 0.7).clamp(-1.0, 0.0),
            interest: 0.0,
            frustration: (panic * 0.4).clamp(0.0, 1.0),
        }
    }
}

// ── Exertion → Energy ──────────────────────────────────────────────────

/// Convert average muscle activation level to energy exertion rate.
///
/// Higher activation = more energy spent. Returns exertion rate suitable
/// for bhava's energy system (0.0 = rest, 1.0 = maximum effort).
///
/// ```
/// use bhava::physiology::exertion_from_activation;
///
/// assert!(exertion_from_activation(0.0) < 0.1);
/// assert!(exertion_from_activation(0.8) > 0.5);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn exertion_from_activation(avg_activation: f32) -> f32 {
    // Quadratic: low activation costs little, high activation costs a lot
    let a = avg_activation.clamp(0.0, 1.0);
    (a * a).clamp(0.0, 1.0)
}

/// Compute baseline metabolic energy cost from body mass using Kleiber's law.
///
/// Returns basal metabolic rate in watts. Wraps sharira's allometric BMR.
/// Useful for setting bhava's energy system baseline drain rate.
///
/// ```
/// use bhava::physiology::metabolic_load;
///
/// let human = metabolic_load(70.0);
/// assert!(human > 50.0 && human < 150.0); // ~80W for a 70kg human
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn metabolic_load(mass_kg: f32) -> f64 {
    sharira::bridge::body_mass_to_bmr(mass_kg)
}

// ── Morphology → Confidence ────────────────────────────────────────────

/// Derive a dominance bias from body morphology.
///
/// Larger, heavier bodies produce a positive dominance bias (more confidence);
/// smaller bodies produce a negative bias (more caution). Based on mass
/// factor relative to average (1.0). Returns dominance modifier [-0.3, 0.3].
///
/// ```
/// use bhava::physiology::confidence_from_morphology;
///
/// let heavy = confidence_from_morphology(1.4);
/// assert!(heavy > 0.0);
///
/// let lean = confidence_from_morphology(0.75);
/// assert!(lean < 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn confidence_from_morphology(mass_factor: f32) -> f32 {
    // Center on 1.0 (average), scale ±0.3
    ((mass_factor - 1.0) * 0.5).clamp(-0.3, 0.3)
}

// ── Gait → Emotional Signal ───────────────────────────────────────────

/// Convert gait speed to arousal level.
///
/// Faster movement = higher physiological arousal. Walking pace (~1.4 m/s) is
/// low arousal; running (~3+ m/s) is high arousal; stillness is baseline.
///
/// ```
/// use bhava::physiology::arousal_from_gait;
///
/// let still = arousal_from_gait(0.0);
/// let walking = arousal_from_gait(1.4);
/// let running = arousal_from_gait(4.0);
/// assert!(running > walking);
/// assert!(walking > still);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn arousal_from_gait(speed_ms: f32) -> f32 {
    // Sigmoid: 0 m/s → ~0, 1.4 m/s → ~0.2, 3 m/s → ~0.5, 6 m/s → ~0.8
    let s = speed_ms.max(0.0);
    (1.0 - (-s * 0.4).exp()).clamp(0.0, 1.0)
}

/// Map gait type to an emotional valence association.
///
/// Different movement patterns carry emotional meaning:
/// walking = neutral, running = urgent/excited, crawling = desperation/stealth,
/// swimming/flying = freedom.
///
/// ```
/// use bhava::physiology::gait_emotional_valence;
///
/// let walk = gait_emotional_valence(sharira::GaitType::Walk);
/// let run = gait_emotional_valence(sharira::GaitType::Run);
/// assert!(run.abs() > walk.abs()); // running is more emotionally charged
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn gait_emotional_valence(gait_type: sharira::GaitType) -> f32 {
    #[allow(unreachable_patterns)]
    match gait_type {
        sharira::GaitType::Walk => 0.0,
        sharira::GaitType::Run => 0.3,
        sharira::GaitType::Crawl => -0.3,
        sharira::GaitType::Hop => 0.2,
        sharira::GaitType::Fly => 0.5,
        sharira::GaitType::Swim => 0.3,
        sharira::GaitType::Trot => 0.1,
        sharira::GaitType::Canter => 0.2,
        sharira::GaitType::Gallop => 0.4,
        sharira::GaitType::Slither => -0.1,
        _ => 0.0,
    }
}

// ── Heart Rate → Arousal ───────────────────────────────────────────────

/// Convert allometric resting heart rate to a physiological arousal baseline.
///
/// Smaller animals have higher resting HR and higher baseline arousal.
/// Maps HR to arousal [0, 1]: 60 bpm (human) ≈ 0.15, 600 bpm (mouse) ≈ 0.8.
///
/// ```
/// use bhava::physiology::arousal_from_heart_rate;
///
/// let human = arousal_from_heart_rate(70.0);
/// let mouse = arousal_from_heart_rate(600.0);
/// assert!(mouse > human);
/// assert!(human > 0.0 && human < 0.4);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn arousal_from_heart_rate(resting_bpm: f64) -> f32 {
    // Log scale: HR 60 → ~0.15, HR 200 → ~0.4, HR 600 → ~0.8
    let hr = resting_bpm.clamp(20.0, 1000.0);
    ((hr.ln() - 3.0) / 4.0) as f32
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Fatigue ────────────────────────────────────────────────────────

    #[test]
    fn fatigue_fresh_low_frustration() {
        let mood = mood_from_fatigue(1.0);
        assert!(mood.frustration < 0.1);
        assert!(mood.joy > 0.0);
    }

    #[test]
    fn fatigue_exhausted_high_frustration() {
        let mood = mood_from_fatigue(0.1);
        assert!(mood.frustration > 0.5);
    }

    #[test]
    fn fatigue_clamps_input() {
        let over = mood_from_fatigue(2.0);
        let under = mood_from_fatigue(-1.0);
        assert!(over.frustration < 0.1);
        assert!(under.frustration > 0.5);
    }

    #[test]
    fn energy_drain_increases_with_fatigue() {
        let fresh = energy_drain_from_fatigue(1.0);
        let tired = energy_drain_from_fatigue(0.3);
        assert!(tired > fresh);
        assert!(fresh >= 1.0);
    }

    // ── Pain ───────────────────────────────────────────────────────────

    #[test]
    fn no_violation_no_stress() {
        assert!((stress_from_violation(0.0) - 0.0).abs() < 0.01);
    }

    #[test]
    fn violation_produces_stress() {
        assert!(stress_from_violation(1.0) > 0.3);
        assert!(stress_from_violation(2.0) > stress_from_violation(1.0));
    }

    #[test]
    fn pain_zero_when_no_violation() {
        assert!((pain_intensity(0.0) - 0.0).abs() < 0.01);
    }

    #[test]
    fn pain_increases_with_violation() {
        assert!(pain_intensity(2.0) > pain_intensity(1.0));
    }

    #[test]
    fn pain_saturates() {
        let mid = pain_intensity(3.0);
        let high = pain_intensity(5.0);
        // Diminishing returns: gap between 3→5 smaller than 0→3
        assert!(high - mid < mid);
    }

    // ── Balance ────────────────────────────────────────────────────────

    #[test]
    fn stable_balance_calm() {
        let mood = anxiety_from_balance(0.3);
        assert!(mood.trust > 0.0);
        assert!(mood.arousal <= 0.0);
    }

    #[test]
    fn falling_balance_panic() {
        let mood = anxiety_from_balance(-0.5);
        assert!(mood.arousal > 0.3);
        assert!(mood.trust < 0.0);
    }

    #[test]
    fn zero_balance_neutral() {
        let mood = anxiety_from_balance(0.0);
        assert!(mood.arousal <= 0.0);
        assert!(mood.trust >= 0.0);
    }

    // ── Exertion ───────────────────────────────────────────────────────

    #[test]
    fn rest_low_exertion() {
        assert!(exertion_from_activation(0.0) < 0.01);
    }

    #[test]
    fn high_activation_high_exertion() {
        assert!(exertion_from_activation(0.9) > 0.5);
    }

    #[test]
    fn metabolic_load_human() {
        let bmr = metabolic_load(70.0);
        assert!(bmr > 50.0 && bmr < 150.0);
    }

    // ── Morphology ─────────────────────────────────────────────────────

    #[test]
    fn heavy_build_positive_confidence() {
        assert!(confidence_from_morphology(1.4) > 0.0);
    }

    #[test]
    fn lean_build_negative_confidence() {
        assert!(confidence_from_morphology(0.7) < 0.0);
    }

    #[test]
    fn average_build_neutral() {
        assert!(confidence_from_morphology(1.0).abs() < 0.01);
    }

    // ── Gait ───────────────────────────────────────────────────────────

    #[test]
    fn faster_gait_higher_arousal() {
        let walk = arousal_from_gait(1.4);
        let run = arousal_from_gait(4.0);
        assert!(run > walk);
    }

    #[test]
    fn still_gait_low_arousal() {
        assert!(arousal_from_gait(0.0) < 0.05);
    }

    #[test]
    fn gait_valence_run_positive() {
        let v = gait_emotional_valence(sharira::GaitType::Run);
        assert!(v > 0.0);
    }

    #[test]
    fn gait_valence_crawl_negative() {
        let v = gait_emotional_valence(sharira::GaitType::Crawl);
        assert!(v < 0.0);
    }

    #[test]
    fn gait_valence_walk_neutral() {
        let v = gait_emotional_valence(sharira::GaitType::Walk);
        assert!((v - 0.0).abs() < 0.01);
    }

    // ── Heart Rate ─────────────────────────────────────────────────────

    #[test]
    fn human_hr_low_arousal() {
        let a = arousal_from_heart_rate(70.0);
        assert!(a > 0.0 && a < 0.4);
    }

    #[test]
    fn mouse_hr_high_arousal() {
        let a = arousal_from_heart_rate(600.0);
        assert!(a > 0.5);
    }

    #[test]
    fn higher_hr_higher_arousal() {
        let low = arousal_from_heart_rate(60.0);
        let high = arousal_from_heart_rate(200.0);
        assert!(high > low);
    }
}
