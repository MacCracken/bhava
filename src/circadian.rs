//! Circadian rhythm — 24-hour alertness and mood cycle.
//!
//! Dual-cosine model based on Borbély's two-process model of sleep regulation
//! (1982). A primary 24-hour cosine for the circadian alertness cycle plus a
//! secondary 12-hour cosine for the post-lunch dip.
//!
//! Chronotype (morning/evening preference) shifts the phase, producing
//! individual variation in peak alertness timing.
//!
//! Modulates:
//! - **Baseline mood** — higher alertness boosts joy/interest/arousal
//! - **Decay rate** — sharper alertness = faster emotional processing
//! - **Energy recovery** — recovery is faster during high-alertness windows

use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::mood::{Emotion, MoodVector};

/// Chronotype — morning vs evening preference.
///
/// Shifts the circadian phase, moving peak alertness earlier or later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Chronotype {
    /// Peak alertness ~08:00 local. Phase shift: -2h.
    EarlyBird,
    /// Peak alertness ~09:00 local. Phase shift: -1h.
    MorningLeaning,
    /// Peak alertness ~10:00 local. No phase shift.
    Neutral,
    /// Peak alertness ~11:00 local. Phase shift: +1h.
    EveningLeaning,
    /// Peak alertness ~12:00 local. Phase shift: +2h.
    NightOwl,
}

impl Chronotype {
    /// All chronotypes.
    pub const ALL: &'static [Chronotype] = &[
        Self::EarlyBird,
        Self::MorningLeaning,
        Self::Neutral,
        Self::EveningLeaning,
        Self::NightOwl,
    ];

    /// Phase shift in hours from neutral.
    #[must_use]
    #[inline]
    pub fn phase_shift_hours(self) -> f32 {
        match self {
            Self::EarlyBird => -2.0,
            Self::MorningLeaning => -1.0,
            Self::Neutral => 0.0,
            Self::EveningLeaning => 1.0,
            Self::NightOwl => 2.0,
        }
    }
}

impl fmt::Display for Chronotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EarlyBird => "early bird",
            Self::MorningLeaning => "morning-leaning",
            Self::Neutral => "neutral",
            Self::EveningLeaning => "evening-leaning",
            Self::NightOwl => "night owl",
        };
        f.write_str(s)
    }
}

/// 24-hour circadian alertness and mood cycle.
///
/// Dual-cosine: primary 24h cycle (peak late morning, trough ~04:00)
/// plus secondary 12h cycle (post-lunch dip at ~14:00).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianRhythm {
    /// Chronotype: phase shift for individual variation.
    pub chronotype: Chronotype,
    /// Amplitude of primary 24h cycle. Default: 0.3.
    pub primary_amplitude: f32,
    /// Amplitude of secondary 12h cycle (post-lunch dip). Default: 0.1.
    pub secondary_amplitude: f32,
    /// UTC offset in hours for the entity's local time.
    pub utc_offset_hours: f32,
}

impl Default for CircadianRhythm {
    fn default() -> Self {
        Self {
            chronotype: Chronotype::Neutral,
            primary_amplitude: 0.3,
            secondary_amplitude: 0.1,
            utc_offset_hours: 0.0,
        }
    }
}

impl CircadianRhythm {
    /// Create with default parameters (neutral chronotype, UTC).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with a specific chronotype.
    #[must_use]
    pub fn with_chronotype(chronotype: Chronotype) -> Self {
        Self {
            chronotype,
            ..Self::default()
        }
    }

    /// Compute the local fractional hour (0.0–24.0) from a UTC timestamp.
    #[must_use]
    #[inline]
    fn local_hour(&self, now: DateTime<Utc>) -> f64 {
        let utc_hour =
            now.hour() as f64 + now.minute() as f64 / 60.0 + now.second() as f64 / 3600.0;
        let shifted =
            utc_hour + self.utc_offset_hours as f64 + self.chronotype.phase_shift_hours() as f64;
        shifted.rem_euclid(24.0)
    }

    /// Compute alertness factor at the given time.
    ///
    /// Returns 0.0 (minimum alertness, ~04:00 local) to 1.0 (peak, ~10:00 local).
    /// The exact timing depends on chronotype and UTC offset.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    #[inline]
    pub fn alertness(&self, now: DateTime<Utc>) -> f32 {
        let h = self.local_hour(now);
        let tau = std::f64::consts::TAU;

        // Primary 24h cycle: cosine peaking at hour 10
        let primary = (tau * (h - 10.0) / 24.0).cos() as f32;

        // Secondary 12h cycle: negative cosine dipping at hour 14
        let secondary = -(tau * (h - 14.0) / 12.0).cos() as f32;

        let raw = 0.5 + self.primary_amplitude * primary + self.secondary_amplitude * secondary;
        raw.clamp(0.0, 1.0)
    }

    /// Mood baseline modulation from circadian alertness.
    ///
    /// Higher alertness slightly boosts joy and interest;
    /// low alertness slightly reduces arousal.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn mood_modulation(&self, now: DateTime<Utc>) -> MoodVector {
        let a = self.alertness(now);
        let deviation = a - 0.5; // -0.5 to +0.5

        let mut delta = MoodVector::neutral();
        delta.set(Emotion::Joy, deviation * 0.1);
        delta.set(Emotion::Interest, deviation * 0.15);
        delta.set(Emotion::Arousal, deviation * 0.1);
        delta
    }

    /// Decay rate modifier from circadian alertness.
    ///
    /// Higher alertness = faster emotional processing = faster decay.
    /// Returns 0.7 (sluggish, low alertness) to 1.3 (sharp, high alertness).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    #[inline]
    pub fn decay_rate_modifier(&self, now: DateTime<Utc>) -> f32 {
        let a = self.alertness(now);
        0.7 + a * 0.6
    }

    /// Energy recovery modifier from circadian alertness.
    ///
    /// Recovery is faster during high-alertness periods.
    /// Returns 0.6 (low alertness) to 1.4 (peak alertness).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    #[inline]
    pub fn energy_recovery_modifier(&self, now: DateTime<Utc>) -> f32 {
        let a = self.alertness(now);
        0.6 + a * 0.8
    }
}

/// Derive circadian parameters from personality.
///
/// - High precision + formality → early bird (structured, early riser)
/// - High creativity + risk tolerance → night owl (unconventional schedule)
#[cfg(feature = "traits")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn circadian_from_personality(profile: &crate::traits::PersonalityProfile) -> CircadianRhythm {
    use crate::traits::TraitKind;
    let precision = profile.get_trait(TraitKind::Precision).normalized();
    let formality = profile.get_trait(TraitKind::Formality).normalized();
    let creativity = profile.get_trait(TraitKind::Creativity).normalized();
    let risk_tolerance = profile.get_trait(TraitKind::RiskTolerance).normalized();

    let morning_pull = (precision + formality) / 2.0;
    let evening_pull = (creativity + risk_tolerance) / 2.0;
    let net = morning_pull - evening_pull;

    let chronotype = if net > 0.5 {
        Chronotype::EarlyBird
    } else if net > 0.15 {
        Chronotype::MorningLeaning
    } else if net < -0.5 {
        Chronotype::NightOwl
    } else if net < -0.15 {
        Chronotype::EveningLeaning
    } else {
        Chronotype::Neutral
    };

    CircadianRhythm {
        chronotype,
        ..CircadianRhythm::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn time_at_hour(hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 15, hour, 0, 0).unwrap()
    }

    fn time_at_hour_min(hour: u32, min: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 15, hour, min, 0).unwrap()
    }

    #[test]
    fn test_circadian_default() {
        let c = CircadianRhythm::new();
        assert_eq!(c.chronotype, Chronotype::Neutral);
        assert!((c.primary_amplitude - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_alertness_bounded() {
        let c = CircadianRhythm::new();
        for hour in 0..24 {
            let a = c.alertness(time_at_hour(hour));
            assert!(
                (0.0..=1.0).contains(&a),
                "hour {hour}: alertness {a} out of bounds"
            );
        }
    }

    #[test]
    fn test_alertness_peak_morning() {
        let c = CircadianRhythm::new();
        let morning = c.alertness(time_at_hour(10)); // peak for neutral
        let afternoon = c.alertness(time_at_hour(16));
        let night = c.alertness(time_at_hour(4));
        assert!(
            morning > afternoon,
            "morning={morning} should > afternoon={afternoon}"
        );
        assert!(morning > night, "morning={morning} should > night={night}");
    }

    #[test]
    fn test_alertness_trough_night() {
        let c = CircadianRhythm::new();
        let night = c.alertness(time_at_hour(4));
        let day = c.alertness(time_at_hour(12));
        assert!(night < day, "night={night} should < day={day}");
    }

    #[test]
    fn test_post_lunch_dip() {
        let c = CircadianRhythm::new();
        let pre_lunch = c.alertness(time_at_hour(11));
        let post_lunch = c.alertness(time_at_hour_min(14, 0));
        // Post-lunch should be lower than pre-lunch (the secondary dip)
        assert!(
            post_lunch < pre_lunch,
            "post_lunch={post_lunch} should < pre_lunch={pre_lunch}"
        );
    }

    #[test]
    fn test_chronotype_shifts_peak() {
        let early = CircadianRhythm::with_chronotype(Chronotype::EarlyBird);
        let late = CircadianRhythm::with_chronotype(Chronotype::NightOwl);
        // At UTC 12:00:
        //   Early bird local = 12 + (-2) = 10 → at peak
        //   Night owl local = 12 + 2 = 14 → post-lunch dip
        let t = time_at_hour(12);
        assert!(
            early.alertness(t) > late.alertness(t),
            "early={} late={}",
            early.alertness(t),
            late.alertness(t)
        );
    }

    #[test]
    fn test_utc_offset() {
        let mut c = CircadianRhythm::new();
        c.utc_offset_hours = 5.0; // UTC+5
        // At UTC 05:00, local time is 10:00 → should be near peak
        let a = c.alertness(time_at_hour(5));
        assert!(a > 0.6, "UTC+5 at 05:00 UTC (10:00 local): {a}");
    }

    #[test]
    fn test_mood_modulation_bounded() {
        let c = CircadianRhythm::new();
        for hour in 0..24 {
            let delta = c.mood_modulation(time_at_hour(hour));
            for &e in Emotion::ALL {
                let v = delta.get(e);
                assert!((-1.0..=1.0).contains(&v), "hour {hour} {e}: {v}");
            }
        }
    }

    #[test]
    fn test_mood_modulation_peak_positive() {
        let c = CircadianRhythm::new();
        let delta = c.mood_modulation(time_at_hour(10)); // peak alertness
        // At peak, alertness > 0.5, so deviation positive → positive joy/interest
        assert!(delta.joy > 0.0, "peak joy: {}", delta.joy);
        assert!(delta.interest > 0.0, "peak interest: {}", delta.interest);
    }

    #[test]
    fn test_decay_rate_modifier_range() {
        let c = CircadianRhythm::new();
        for hour in 0..24 {
            let m = c.decay_rate_modifier(time_at_hour(hour));
            assert!((0.7..=1.3).contains(&m), "hour {hour}: decay modifier {m}");
        }
    }

    #[test]
    fn test_energy_recovery_modifier_range() {
        let c = CircadianRhythm::new();
        for hour in 0..24 {
            let m = c.energy_recovery_modifier(time_at_hour(hour));
            assert!(
                (0.6..=1.4).contains(&m),
                "hour {hour}: recovery modifier {m}"
            );
        }
    }

    #[test]
    fn test_chronotype_display() {
        assert_eq!(Chronotype::EarlyBird.to_string(), "early bird");
        assert_eq!(Chronotype::NightOwl.to_string(), "night owl");
        assert_eq!(Chronotype::Neutral.to_string(), "neutral");
    }

    #[test]
    fn test_chronotype_all() {
        assert_eq!(Chronotype::ALL.len(), 5);
    }

    #[test]
    fn test_chronotype_phase_shifts() {
        assert!((Chronotype::EarlyBird.phase_shift_hours() - (-2.0)).abs() < f32::EPSILON);
        assert!(Chronotype::Neutral.phase_shift_hours().abs() < f32::EPSILON);
        assert!((Chronotype::NightOwl.phase_shift_hours() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_serde_rhythm() {
        let c = CircadianRhythm::with_chronotype(Chronotype::NightOwl);
        let json = serde_json::to_string(&c).unwrap();
        let c2: CircadianRhythm = serde_json::from_str(&json).unwrap();
        assert_eq!(c2.chronotype, Chronotype::NightOwl);
    }

    #[test]
    fn test_serde_chronotype() {
        let ct = Chronotype::EarlyBird;
        let json = serde_json::to_string(&ct).unwrap();
        let ct2: Chronotype = serde_json::from_str(&json).unwrap();
        assert_eq!(ct2, ct);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_circadian_from_personality_precise() {
        let mut p = crate::traits::PersonalityProfile::new("precise");
        p.set_trait(
            crate::traits::TraitKind::Precision,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Formality,
            crate::traits::TraitLevel::Highest,
        );
        let c = circadian_from_personality(&p);
        assert_eq!(c.chronotype, Chronotype::EarlyBird);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_circadian_from_personality_creative() {
        let mut p = crate::traits::PersonalityProfile::new("creative");
        p.set_trait(
            crate::traits::TraitKind::Creativity,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::RiskTolerance,
            crate::traits::TraitLevel::Highest,
        );
        let c = circadian_from_personality(&p);
        assert_eq!(c.chronotype, Chronotype::NightOwl);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_circadian_from_personality_balanced() {
        let p = crate::traits::PersonalityProfile::new("balanced");
        let c = circadian_from_personality(&p);
        assert_eq!(c.chronotype, Chronotype::Neutral);
    }
}
