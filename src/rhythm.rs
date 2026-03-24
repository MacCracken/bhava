//! Biological rhythms — periodic mood modulation cycles.
//!
//! Models three classes of biological rhythm that modulate an entity's mood:
//!
//! - **Ultradian** — 90–120 minute focus/rest cycles (BRAC model, Kleitman 1963).
//!   Modulates interest and arousal with a sine wave.
//! - **Seasonal** — Long-period mood variation mapped to simulation seasons.
//!   Sensitivity parameter captures SAD-like effects (Rosenthal et al. 1984).
//! - **Biorhythm** — Multiple overlapping sine waves at incommensurate periods
//!   for NPC individuation. Deterministic but complex variation.

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

use crate::mood::{Emotion, MoodVector};

// ─── Ultradian Rhythm ───────────────────────────────────────────────────────

/// 90–120 minute focus/rest cycle (Basic Rest-Activity Cycle).
///
/// Produces a sine wave that modulates interest and arousal. The cycle
/// alternates between high-focus peaks and low-energy troughs, matching
/// Kleitman's BRAC model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UltradianRhythm {
    /// Cycle period in seconds. Default: 5400 (90 minutes).
    pub period_secs: f64,
    /// Peak amplitude for modulated dimensions. Default: 0.15.
    pub amplitude: f32,
    /// Phase offset in radians for individual variation.
    pub phase_offset: f64,
}

impl Default for UltradianRhythm {
    fn default() -> Self {
        Self {
            period_secs: 5400.0,
            amplitude: 0.15,
            phase_offset: 0.0,
        }
    }
}

impl UltradianRhythm {
    /// Create with default 90-minute period.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with a custom period (clamped to 60..=14400 seconds).
    #[must_use]
    pub fn with_period(period_secs: f64) -> Self {
        Self {
            period_secs: period_secs.clamp(60.0, 14400.0),
            ..Self::default()
        }
    }

    /// Compute mood modulation at the given time.
    ///
    /// Returns a `MoodVector` of additive deltas. Interest and arousal
    /// oscillate in phase (high focus = high interest + moderate arousal).
    #[must_use]
    pub fn modulate(&self, now: DateTime<Utc>) -> MoodVector {
        let t = now.timestamp() as f64;
        let phase = std::f64::consts::TAU * t / self.period_secs + self.phase_offset;
        let wave = phase.sin() as f32;

        let mut delta = MoodVector::neutral();
        delta.set(Emotion::Interest, wave * self.amplitude);
        delta.set(Emotion::Arousal, wave * self.amplitude * 0.6);
        delta
    }
}

// ─── Seasonal Rhythm ────────────────────────────────────────────────────────

/// Long-period mood variation mapped to seasons.
///
/// Models Seasonal Affective Disorder (SAD) effects (Rosenthal et al. 1984).
/// Joy and interest peak at `peak_day` and trough 180 days later. The
/// `sensitivity` parameter controls how strongly seasons affect mood —
/// 0.0 means no effect, 1.0 means strong SAD-like swings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalRhythm {
    /// Length of a year in days. Default: 365.25. Use shorter values for
    /// accelerated simulation time.
    pub year_length_days: f64,
    /// Day-of-year when mood peaks (0-based). Default: 172 (summer solstice).
    pub peak_day: f64,
    /// SAD sensitivity: 0.0 (immune) to 1.0 (strongly affected).
    pub sensitivity: f32,
}

impl Default for SeasonalRhythm {
    fn default() -> Self {
        Self {
            year_length_days: 365.25,
            peak_day: 172.0,
            sensitivity: 0.3,
        }
    }
}

impl SeasonalRhythm {
    /// Create with default parameters (moderate sensitivity, summer peak).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with a specific SAD sensitivity (clamped to 0.0..=1.0).
    #[must_use]
    pub fn with_sensitivity(sensitivity: f32) -> Self {
        Self {
            sensitivity: sensitivity.clamp(0.0, 1.0),
            ..Self::default()
        }
    }

    /// Compute mood modulation at the given time.
    ///
    /// Returns additive deltas for joy and interest, peaking at `peak_day`.
    #[must_use]
    pub fn modulate(&self, now: DateTime<Utc>) -> MoodVector {
        let day_of_year = now.ordinal0() as f64;
        let phase = std::f64::consts::TAU * (day_of_year - self.peak_day) / self.year_length_days;
        let wave = phase.cos() as f32; // cos so peak_day = maximum

        let strength = wave * self.sensitivity * 0.2;

        let mut delta = MoodVector::neutral();
        delta.set(Emotion::Joy, strength);
        delta.set(Emotion::Interest, strength * 0.5);
        delta
    }
}

// ─── Biorhythm Cycles ───────────────────────────────────────────────────────

/// A single biorhythm sine wave targeting one emotion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiorhythmCycle {
    /// Period in seconds.
    pub period_secs: f64,
    /// Peak amplitude.
    pub amplitude: f32,
    /// Which emotion this cycle modulates.
    pub target: Emotion,
}

/// A set of overlapping biorhythm cycles for NPC individuation.
///
/// Uses multiple sine waves at incommensurate periods to produce
/// deterministic but apparently complex mood variation. Each NPC gets
/// a unique `epoch` (birth/spawn time) so their cycles never align.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiorhythmSet {
    /// Individual cycles.
    pub cycles: Vec<BiorhythmCycle>,
    /// The entity's birth/spawn time — all cycles are relative to this.
    pub epoch: DateTime<Utc>,
}

impl BiorhythmSet {
    /// Create an empty biorhythm set.
    #[must_use]
    pub fn new(epoch: DateTime<Utc>) -> Self {
        Self {
            cycles: Vec::new(),
            epoch,
        }
    }

    /// Add a cycle.
    pub fn add_cycle(&mut self, cycle: BiorhythmCycle) {
        self.cycles.push(cycle);
    }

    /// Compute the combined mood modulation at the given time.
    ///
    /// Each cycle contributes an additive delta to its target emotion.
    /// Multiple cycles targeting the same emotion sum together.
    #[must_use]
    pub fn modulate(&self, now: DateTime<Utc>) -> MoodVector {
        let elapsed = (now - self.epoch).num_milliseconds() as f64 / 1000.0;
        let mut delta = MoodVector::neutral();

        for cycle in &self.cycles {
            let phase = std::f64::consts::TAU * elapsed / cycle.period_secs;
            let wave = phase.sin() as f32;
            delta.nudge(cycle.target, wave * cycle.amplitude);
        }

        delta
    }

    /// Number of active cycles.
    #[must_use]
    pub fn cycle_count(&self) -> usize {
        self.cycles.len()
    }
}

/// Create a default biorhythm set with classic incommensurate periods.
///
/// Uses three cycles with periods chosen to avoid simple ratios:
/// - Physical (joy): 23 hours
/// - Emotional (trust): 28 hours
/// - Intellectual (interest): 33 hours
///
/// These mirror the classic biorhythm periods (23/28/33 days) scaled to
/// hours for game/simulation use. All amplitudes are moderate (0.1).
#[must_use]
pub fn default_biorhythm(epoch: DateTime<Utc>) -> BiorhythmSet {
    BiorhythmSet {
        cycles: vec![
            BiorhythmCycle {
                period_secs: 23.0 * 3600.0, // 23 hours
                amplitude: 0.1,
                target: Emotion::Joy,
            },
            BiorhythmCycle {
                period_secs: 28.0 * 3600.0, // 28 hours
                amplitude: 0.1,
                target: Emotion::Trust,
            },
            BiorhythmCycle {
                period_secs: 33.0 * 3600.0, // 33 hours
                amplitude: 0.1,
                target: Emotion::Interest,
            },
        ],
        epoch,
    }
}

/// Apply all rhythm modulations to a mood vector.
///
/// Convenience function that sums deltas from ultradian, seasonal, and
/// biorhythm cycles into the given mood. Values are clamped to -1.0..=1.0
/// by `MoodVector::nudge`.
pub fn apply_rhythms(
    mood: &mut MoodVector,
    now: DateTime<Utc>,
    ultradian: Option<&UltradianRhythm>,
    seasonal: Option<&SeasonalRhythm>,
    biorhythm: Option<&BiorhythmSet>,
) {
    let deltas: [Option<MoodVector>; 3] = [
        ultradian.map(|r| r.modulate(now)),
        seasonal.map(|r| r.modulate(now)),
        biorhythm.map(|r| r.modulate(now)),
    ];

    for delta in deltas.into_iter().flatten() {
        for &e in Emotion::ALL {
            mood.nudge(e, delta.get(e));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn fixed_time() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 21, 12, 0, 0).unwrap()
    }

    // ── Ultradian ──

    #[test]
    fn test_ultradian_default() {
        let r = UltradianRhythm::new();
        assert!((r.period_secs - 5400.0).abs() < f64::EPSILON);
        assert!((r.amplitude - 0.15).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ultradian_modulate_bounded() {
        let r = UltradianRhythm::new();
        let delta = r.modulate(fixed_time());
        for &e in Emotion::ALL {
            let v = delta.get(e);
            assert!((-1.0..=1.0).contains(&v), "{e}: {v}");
        }
    }

    #[test]
    fn test_ultradian_varies_over_time() {
        let r = UltradianRhythm::new();
        // Use times at known cycle positions: 1/4 and 3/4 period from epoch
        let epoch = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let t1 = epoch + chrono::Duration::seconds((r.period_secs * 0.25) as i64);
        let t2 = epoch + chrono::Duration::seconds((r.period_secs * 0.75) as i64);
        let d1 = r.modulate(t1);
        let d2 = r.modulate(t2);
        // Quarter and three-quarter should be at opposite peaks
        assert!(
            (d1.interest - d2.interest).abs() > 0.1,
            "d1={} d2={}",
            d1.interest,
            d2.interest
        );
    }

    #[test]
    fn test_ultradian_with_period_clamps() {
        let r = UltradianRhythm::with_period(10.0);
        assert!((r.period_secs - 60.0).abs() < f64::EPSILON);
        let r2 = UltradianRhythm::with_period(100_000.0);
        assert!((r2.period_secs - 14400.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ultradian_phase_offset() {
        let mut r1 = UltradianRhythm::new();
        r1.phase_offset = 0.0;
        let mut r2 = UltradianRhythm::new();
        r2.phase_offset = std::f64::consts::PI;
        let t = fixed_time();
        let d1 = r1.modulate(t);
        let d2 = r2.modulate(t);
        // Opposite phases should produce opposite signs (approximately)
        assert!(
            (d1.interest + d2.interest).abs() < 0.01,
            "d1={} d2={}",
            d1.interest,
            d2.interest
        );
    }

    // ── Seasonal ──

    #[test]
    fn test_seasonal_default() {
        let r = SeasonalRhythm::new();
        assert!((r.sensitivity - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_seasonal_modulate_bounded() {
        let r = SeasonalRhythm::with_sensitivity(1.0);
        let delta = r.modulate(fixed_time());
        for &e in Emotion::ALL {
            let v = delta.get(e);
            assert!((-1.0..=1.0).contains(&v), "{e}: {v}");
        }
    }

    #[test]
    fn test_seasonal_peak_day() {
        // June 21 = day 172, which is the default peak_day
        let r = SeasonalRhythm::with_sensitivity(1.0);
        let summer = Utc.with_ymd_and_hms(2026, 6, 22, 12, 0, 0).unwrap(); // day ~172
        let winter = Utc.with_ymd_and_hms(2026, 12, 21, 12, 0, 0).unwrap(); // day ~355
        let d_summer = r.modulate(summer);
        let d_winter = r.modulate(winter);
        assert!(
            d_summer.joy > d_winter.joy,
            "summer={} winter={}",
            d_summer.joy,
            d_winter.joy
        );
    }

    #[test]
    fn test_seasonal_zero_sensitivity() {
        let r = SeasonalRhythm::with_sensitivity(0.0);
        let delta = r.modulate(fixed_time());
        assert!(delta.joy.abs() < f32::EPSILON);
        assert!(delta.interest.abs() < f32::EPSILON);
    }

    #[test]
    fn test_seasonal_sensitivity_clamps() {
        let r = SeasonalRhythm::with_sensitivity(5.0);
        assert!((r.sensitivity - 1.0).abs() < f32::EPSILON);
        let r2 = SeasonalRhythm::with_sensitivity(-1.0);
        assert!(r2.sensitivity.abs() < f32::EPSILON);
    }

    // ── Biorhythm ──

    #[test]
    fn test_biorhythm_default() {
        let b = default_biorhythm(fixed_time());
        assert_eq!(b.cycle_count(), 3);
    }

    #[test]
    fn test_biorhythm_modulate_bounded() {
        let b = default_biorhythm(fixed_time());
        let later = fixed_time() + chrono::Duration::hours(12);
        let delta = b.modulate(later);
        for &e in Emotion::ALL {
            let v = delta.get(e);
            assert!((-1.0..=1.0).contains(&v), "{e}: {v}");
        }
    }

    #[test]
    fn test_biorhythm_epoch_matters() {
        let t = fixed_time() + chrono::Duration::hours(10);
        let b1 = default_biorhythm(fixed_time());
        let b2 = default_biorhythm(fixed_time() + chrono::Duration::hours(5));
        let d1 = b1.modulate(t);
        let d2 = b2.modulate(t);
        // Different epochs should produce different values
        assert!(
            (d1.joy - d2.joy).abs() > 0.001,
            "d1={} d2={}",
            d1.joy,
            d2.joy
        );
    }

    #[test]
    fn test_biorhythm_at_epoch_zero() {
        let b = default_biorhythm(fixed_time());
        let delta = b.modulate(b.epoch);
        // sin(0) = 0, so all deltas should be near zero at epoch
        for &e in Emotion::ALL {
            assert!(
                delta.get(e).abs() < 0.01,
                "{e}: {} (expected ~0 at epoch)",
                delta.get(e)
            );
        }
    }

    #[test]
    fn test_biorhythm_empty() {
        let b = BiorhythmSet::new(fixed_time());
        assert_eq!(b.cycle_count(), 0);
        let delta = b.modulate(fixed_time() + chrono::Duration::hours(1));
        assert!(delta.intensity() < f32::EPSILON);
    }

    #[test]
    fn test_biorhythm_add_cycle() {
        let mut b = BiorhythmSet::new(fixed_time());
        b.add_cycle(BiorhythmCycle {
            period_secs: 3600.0,
            amplitude: 0.2,
            target: Emotion::Frustration,
        });
        assert_eq!(b.cycle_count(), 1);
        let delta = b.modulate(fixed_time() + chrono::Duration::minutes(15)); // quarter period
        assert!(delta.frustration.abs() > 0.1);
    }

    // ── apply_rhythms ──

    #[test]
    fn test_apply_rhythms_all() {
        let mut mood = MoodVector::neutral();
        let now = fixed_time() + chrono::Duration::hours(6);
        let u = UltradianRhythm::new();
        let s = SeasonalRhythm::new();
        let b = default_biorhythm(fixed_time());
        apply_rhythms(&mut mood, now, Some(&u), Some(&s), Some(&b));
        // Mood should have shifted from neutral
        assert!(mood.intensity() > 0.0);
    }

    #[test]
    fn test_apply_rhythms_none() {
        let mut mood = MoodVector::neutral();
        apply_rhythms(&mut mood, fixed_time(), None, None, None);
        assert!(mood.intensity() < f32::EPSILON);
    }

    // ── Serde ──

    #[test]
    fn test_serde_ultradian() {
        let r = UltradianRhythm::new();
        let json = serde_json::to_string(&r).unwrap();
        let r2: UltradianRhythm = serde_json::from_str(&json).unwrap();
        assert!((r2.period_secs - r.period_secs).abs() < f64::EPSILON);
    }

    #[test]
    fn test_serde_seasonal() {
        let r = SeasonalRhythm::new();
        let json = serde_json::to_string(&r).unwrap();
        let r2: SeasonalRhythm = serde_json::from_str(&json).unwrap();
        assert!((r2.sensitivity - r.sensitivity).abs() < f32::EPSILON);
    }

    #[test]
    fn test_serde_biorhythm() {
        let b = default_biorhythm(fixed_time());
        let json = serde_json::to_string(&b).unwrap();
        let b2: BiorhythmSet = serde_json::from_str(&json).unwrap();
        assert_eq!(b2.cycle_count(), b.cycle_count());
    }
}
