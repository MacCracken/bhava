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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

/// An active emotional cause that suppresses decay for specific emotions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveCause {
    /// Identifier for this cause (e.g., "threat_active", "deadline_pressure").
    pub id: String,
    /// Which emotions this cause sustains.
    pub emotions: Vec<Emotion>,
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
    /// Active causes that suppress decay for specific emotions.
    #[serde(default)]
    pub active_causes: Vec<ActiveCause>,
}

impl EmotionalState {
    /// Create with neutral baseline and default decay (5 minutes).
    #[must_use]
    pub fn new() -> Self {
        Self {
            mood: MoodVector::neutral(),
            baseline: MoodVector::neutral(),
            decay_half_life_secs: 300.0,
            last_updated: Utc::now(),
            active_causes: Vec::new(),
        }
    }

    /// Create with a custom baseline.
    #[must_use]
    pub fn with_baseline(baseline: MoodVector) -> Self {
        Self {
            mood: baseline.clone(),
            baseline,
            decay_half_life_secs: 300.0,
            last_updated: Utc::now(),
            active_causes: Vec::new(),
        }
    }

    /// Set decay half-life.
    ///
    /// # Errors
    /// Returns `BhavaError::InvalidDecayRate` if `secs` is zero or negative.
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
        // Decay toward baseline, but skip emotions with active causes
        let blended = self.mood.blend(&self.baseline, factor);
        for &e in Emotion::ALL {
            if !self.is_cause_active(e) {
                self.mood.set(e, blended.get(e));
            }
        }
        self.last_updated = now;
    }

    /// Apply an emotional stimulus.
    pub fn stimulate(&mut self, emotion: Emotion, intensity: f32) {
        self.mood.nudge(emotion, intensity);
        self.last_updated = Utc::now();
    }

    /// Current mood intensity (distance from baseline).
    #[must_use]
    pub fn deviation(&self) -> f32 {
        let dj = self.mood.joy - self.baseline.joy;
        let da = self.mood.arousal - self.baseline.arousal;
        let dd = self.mood.dominance - self.baseline.dominance;
        let dt = self.mood.trust - self.baseline.trust;
        let di = self.mood.interest - self.baseline.interest;
        let df = self.mood.frustration - self.baseline.frustration;
        (dj * dj + da * da + dd * dd + dt * dt + di * di + df * df).sqrt()
    }

    /// Classify the current mood into a named emotional state.
    #[must_use]
    pub fn classify(&self) -> MoodState {
        let intensity = self.mood.intensity();
        let dominant = self.mood.dominant_emotion();
        let dom_val = self.mood.get(dominant);

        if intensity < 0.15 {
            return MoodState::Calm;
        }

        match (dominant, dom_val > 0.0) {
            (Emotion::Joy, true) if dom_val > 0.6 => MoodState::Euphoric,
            (Emotion::Joy, true) => MoodState::Content,
            (Emotion::Joy, false) => MoodState::Melancholy,
            (Emotion::Arousal, true) => MoodState::Agitated,
            (Emotion::Arousal, false) => MoodState::Calm,
            (Emotion::Dominance, true) => MoodState::Assertive,
            (Emotion::Dominance, false) => MoodState::Overwhelmed,
            (Emotion::Trust, true) => MoodState::Trusting,
            (Emotion::Trust, false) => MoodState::Guarded,
            (Emotion::Interest, true) => MoodState::Curious,
            (Emotion::Interest, false) => MoodState::Disengaged,
            (Emotion::Frustration, true) => MoodState::Frustrated,
            (Emotion::Frustration, false) => MoodState::Content,
        }
    }

    // --- Mood Triggers (v0.3) ---

    /// Apply a trigger, stimulating all its emotion responses.
    ///
    /// Batches all nudges and updates the timestamp once (avoids repeated `Utc::now()` calls).
    pub fn apply_trigger(&mut self, trigger: &super::MoodTrigger) {
        for &(emotion, intensity) in &trigger.responses {
            self.mood.nudge(emotion, intensity);
        }
        self.last_updated = Utc::now();
    }

    // --- Mood History (v0.3) ---

    /// Take a snapshot of the current mood with a timestamp.
    #[must_use]
    pub fn snapshot(&self) -> super::MoodSnapshot {
        super::MoodSnapshot {
            mood: self.mood.clone(),
            state: self.classify(),
            deviation: self.deviation(),
            timestamp: self.last_updated,
        }
    }

    // --- Cause-Tagged Decay ---

    /// Register an active cause that suppresses decay for specific emotions.
    ///
    /// While a cause is active, the specified emotions resist natural decay.
    /// Call `resolve_cause()` when the situation is resolved to allow normal decay.
    pub fn add_active_cause(&mut self, cause_id: impl Into<String>, emotions: Vec<Emotion>) {
        self.active_causes.push(ActiveCause {
            id: cause_id.into(),
            emotions,
        });
    }

    /// Resolve a cause, allowing suppressed emotions to decay normally.
    ///
    /// Returns true if the cause was found and removed.
    pub fn resolve_cause(&mut self, cause_id: &str) -> bool {
        let before = self.active_causes.len();
        self.active_causes.retain(|c| c.id != cause_id);
        self.active_causes.len() < before
    }

    /// Check if a specific emotion has an active cause suppressing its decay.
    #[must_use]
    pub fn is_cause_active(&self, emotion: Emotion) -> bool {
        self.active_causes
            .iter()
            .any(|c| c.emotions.contains(&emotion))
    }
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self::new()
    }
}

// --- Named Emotional States (v0.3) ---

/// Named mood states derived from the mood vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MoodState {
    /// Low intensity — near baseline.
    Calm,
    /// Positive joy, moderate.
    Content,
    /// Strong positive joy.
    Euphoric,
    /// Negative joy — sadness.
    Melancholy,
    /// High arousal — restless, energized.
    Agitated,
    /// High dominance — in control.
    Assertive,
    /// Low dominance — overwhelmed.
    Overwhelmed,
    /// High trust — open, connected.
    Trusting,
    /// Low trust — defensive, wary.
    Guarded,
    /// High interest — engaged, exploring.
    Curious,
    /// Low interest — bored, withdrawn.
    Disengaged,
    /// High frustration — irritated, blocked.
    Frustrated,
}

impl fmt::Display for MoodState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Calm => "calm",
            Self::Content => "content",
            Self::Euphoric => "euphoric",
            Self::Melancholy => "melancholy",
            Self::Agitated => "agitated",
            Self::Assertive => "assertive",
            Self::Overwhelmed => "overwhelmed",
            Self::Trusting => "trusting",
            Self::Guarded => "guarded",
            Self::Curious => "curious",
            Self::Disengaged => "disengaged",
            Self::Frustrated => "frustrated",
        };
        f.write_str(s)
    }
}
