//! Emotional state vectors with time-based decay.
//!
//! Models emotion as a multidimensional vector that shifts in response to
//! events and decays toward a baseline over time.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{BhavaError, Result};

/// Emotional dimensions — based on the PAD (Pleasure-Arousal-Dominance) model
/// extended with domain-specific dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Emotion {
    /// Positive ↔ negative valence (happy ↔ sad).
    Joy,
    /// High ↔ low activation (excited ↔ calm).
    Arousal,
    /// In-control ↔ overwhelmed.
    Dominance,
    /// Connected ↔ isolated.
    Trust,
    /// Curious ↔ indifferent.
    Interest,
    /// Frustrated ↔ satisfied.
    Frustration,
}

impl Emotion {
    pub const ALL: &'static [Emotion] = &[
        Self::Joy,
        Self::Arousal,
        Self::Dominance,
        Self::Trust,
        Self::Interest,
        Self::Frustration,
    ];
}

impl fmt::Display for Emotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Joy => "joy",
            Self::Arousal => "arousal",
            Self::Dominance => "dominance",
            Self::Trust => "trust",
            Self::Interest => "interest",
            Self::Frustration => "frustration",
        };
        f.write_str(s)
    }
}

/// A mood vector — emotional state across all dimensions.
/// Values range from -1.0 (negative extreme) to 1.0 (positive extreme).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodVector {
    pub joy: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub trust: f32,
    pub interest: f32,
    pub frustration: f32,
}

impl MoodVector {
    /// Neutral mood — all dimensions at 0.
    pub fn neutral() -> Self {
        Self {
            joy: 0.0,
            arousal: 0.0,
            dominance: 0.0,
            trust: 0.0,
            interest: 0.0,
            frustration: 0.0,
        }
    }

    /// Get value for a dimension.
    #[inline]
    pub fn get(&self, emotion: Emotion) -> f32 {
        match emotion {
            Emotion::Joy => self.joy,
            Emotion::Arousal => self.arousal,
            Emotion::Dominance => self.dominance,
            Emotion::Trust => self.trust,
            Emotion::Interest => self.interest,
            Emotion::Frustration => self.frustration,
        }
    }

    /// Set value for a dimension (clamped to -1.0..=1.0).
    #[inline]
    pub fn set(&mut self, emotion: Emotion, value: f32) {
        let clamped = value.clamp(-1.0, 1.0);
        match emotion {
            Emotion::Joy => self.joy = clamped,
            Emotion::Arousal => self.arousal = clamped,
            Emotion::Dominance => self.dominance = clamped,
            Emotion::Trust => self.trust = clamped,
            Emotion::Interest => self.interest = clamped,
            Emotion::Frustration => self.frustration = clamped,
        }
    }

    /// Apply a delta to a dimension (result clamped).
    #[inline]
    pub fn nudge(&mut self, emotion: Emotion, delta: f32) {
        self.set(emotion, self.get(emotion) + delta);
    }

    /// Magnitude of the mood vector (distance from neutral).
    pub fn intensity(&self) -> f32 {
        let sum = self.joy * self.joy
            + self.arousal * self.arousal
            + self.dominance * self.dominance
            + self.trust * self.trust
            + self.interest * self.interest
            + self.frustration * self.frustration;
        sum.sqrt()
    }

    /// Dominant emotion (highest absolute value).
    pub fn dominant_emotion(&self) -> Emotion {
        let mut best = Emotion::Joy;
        let mut best_val = 0.0f32;
        for &e in Emotion::ALL {
            let v = self.get(e).abs();
            if v > best_val {
                best_val = v;
                best = e;
            }
        }
        best
    }

    /// Decay toward neutral by a factor (0.0 = no decay, 1.0 = instant reset).
    pub fn decay(&mut self, factor: f32) {
        let f = factor.clamp(0.0, 1.0);
        self.joy *= 1.0 - f;
        self.arousal *= 1.0 - f;
        self.dominance *= 1.0 - f;
        self.trust *= 1.0 - f;
        self.interest *= 1.0 - f;
        self.frustration *= 1.0 - f;
    }

    /// Blend with another mood vector (linear interpolation).
    pub fn blend(&self, other: &MoodVector, t: f32) -> MoodVector {
        let t = t.clamp(0.0, 1.0);
        MoodVector {
            joy: self.joy + (other.joy - self.joy) * t,
            arousal: self.arousal + (other.arousal - self.arousal) * t,
            dominance: self.dominance + (other.dominance - self.dominance) * t,
            trust: self.trust + (other.trust - self.trust) * t,
            interest: self.interest + (other.interest - self.interest) * t,
            frustration: self.frustration + (other.frustration - self.frustration) * t,
        }
    }
}

/// Emotional state with timestamp tracking and automatic decay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Current mood.
    pub mood: MoodVector,
    /// Baseline mood (personality's default emotional state).
    pub baseline: MoodVector,
    /// Decay half-life in seconds (how long it takes to decay halfway to baseline).
    pub decay_half_life_secs: f64,
    /// When the mood was last updated.
    pub last_updated: DateTime<Utc>,
}

impl EmotionalState {
    /// Create with neutral baseline and default decay (5 minutes).
    pub fn new() -> Self {
        Self {
            mood: MoodVector::neutral(),
            baseline: MoodVector::neutral(),
            decay_half_life_secs: 300.0,
            last_updated: Utc::now(),
        }
    }

    /// Create with a custom baseline.
    pub fn with_baseline(baseline: MoodVector) -> Self {
        Self {
            mood: baseline.clone(),
            baseline,
            decay_half_life_secs: 300.0,
            last_updated: Utc::now(),
        }
    }

    /// Set decay half-life.
    pub fn set_decay_half_life(&mut self, secs: f64) -> Result<()> {
        if secs <= 0.0 {
            return Err(BhavaError::InvalidDecayRate { rate: secs as f32 });
        }
        self.decay_half_life_secs = secs;
        Ok(())
    }

    /// Apply time-based decay toward baseline.
    /// Call this before reading the mood to get the current state.
    pub fn apply_decay(&mut self, now: DateTime<Utc>) {
        let elapsed = (now - self.last_updated).num_milliseconds() as f64 / 1000.0;
        if elapsed <= 0.0 {
            return;
        }
        // Exponential decay: factor = 1 - 2^(-elapsed / half_life)
        let factor = 1.0 - (2.0f64.powf(-elapsed / self.decay_half_life_secs)) as f32;
        // Decay toward baseline, not toward zero
        self.mood = self.mood.blend(&self.baseline, factor);
        self.last_updated = now;
    }

    /// Apply an emotional stimulus.
    pub fn stimulate(&mut self, emotion: Emotion, intensity: f32) {
        self.mood.nudge(emotion, intensity);
        self.last_updated = Utc::now();
    }

    /// Current mood intensity (distance from baseline).
    pub fn deviation(&self) -> f32 {
        let diff = MoodVector {
            joy: self.mood.joy - self.baseline.joy,
            arousal: self.mood.arousal - self.baseline.arousal,
            dominance: self.mood.dominance - self.baseline.dominance,
            trust: self.mood.trust - self.baseline.trust,
            interest: self.mood.interest - self.baseline.interest,
            frustration: self.mood.frustration - self.baseline.frustration,
        };
        diff.intensity()
    }
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neutral_mood() {
        let m = MoodVector::neutral();
        assert!((m.intensity()).abs() < f32::EPSILON);
    }

    #[test]
    fn test_get_set() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        assert!((m.get(Emotion::Joy) - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_clamp() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 5.0);
        assert!((m.get(Emotion::Joy) - 1.0).abs() < f32::EPSILON);
        m.set(Emotion::Joy, -5.0);
        assert!((m.get(Emotion::Joy) - (-1.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_nudge() {
        let mut m = MoodVector::neutral();
        m.nudge(Emotion::Trust, 0.3);
        m.nudge(Emotion::Trust, 0.3);
        assert!((m.get(Emotion::Trust) - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_intensity() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 1.0);
        assert!((m.intensity() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_dominant_emotion() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, -0.9);
        m.set(Emotion::Joy, 0.3);
        assert_eq!(m.dominant_emotion(), Emotion::Frustration);
    }

    #[test]
    fn test_decay() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 1.0);
        m.decay(0.5);
        assert!((m.get(Emotion::Joy) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_blend() {
        let a = MoodVector::neutral();
        let mut b = MoodVector::neutral();
        b.set(Emotion::Joy, 1.0);
        let c = a.blend(&b, 0.5);
        assert!((c.joy - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_emotional_state_new() {
        let s = EmotionalState::new();
        assert!(s.deviation().abs() < f32::EPSILON);
    }

    #[test]
    fn test_stimulate() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.5);
        assert!(s.mood.joy > 0.0);
        assert!(s.deviation() > 0.0);
    }

    #[test]
    fn test_invalid_decay_rate() {
        let mut s = EmotionalState::new();
        assert!(s.set_decay_half_life(-1.0).is_err());
        assert!(s.set_decay_half_life(0.0).is_err());
        assert!(s.set_decay_half_life(60.0).is_ok());
    }

    #[test]
    fn test_emotion_display() {
        assert_eq!(Emotion::Joy.to_string(), "joy");
        assert_eq!(Emotion::Frustration.to_string(), "frustration");
    }

    #[test]
    fn test_mood_serde() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.7);
        m.set(Emotion::Trust, -0.3);
        let json = serde_json::to_string(&m).unwrap();
        let m2: MoodVector = serde_json::from_str(&json).unwrap();
        assert!((m2.joy - 0.7).abs() < 0.01);
        assert!((m2.trust - (-0.3)).abs() < 0.01);
    }
}
