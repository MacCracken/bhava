//! Emotional state vectors with time-based decay.
//!
//! Models emotion as a multidimensional vector that shifts in response to
//! events and decays toward a baseline over time.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
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

    /// Classify the current mood into a named emotional state.
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
    pub fn apply_trigger(&mut self, trigger: &MoodTrigger) {
        for &(emotion, intensity) in &trigger.responses {
            self.mood.nudge(emotion, intensity);
        }
        self.last_updated = Utc::now();
    }

    // --- Mood History (v0.3) ---

    /// Take a snapshot of the current mood with a timestamp.
    pub fn snapshot(&self) -> MoodSnapshot {
        MoodSnapshot {
            mood: self.mood.clone(),
            state: self.classify(),
            deviation: self.deviation(),
            timestamp: self.last_updated,
        }
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

// --- Mood Triggers (v0.3) ---

/// A stimulus-response mapping: a named event that affects multiple emotions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodTrigger {
    /// Trigger name (e.g., "praised", "criticized", "surprised").
    pub name: String,
    /// Emotion responses: each pair is (emotion, intensity delta).
    pub responses: Vec<(Emotion, f32)>,
}

impl MoodTrigger {
    /// Create a new trigger with the given name and no responses.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            responses: Vec::new(),
        }
    }

    /// Add an emotion response to this trigger.
    pub fn respond(mut self, emotion: Emotion, intensity: f32) -> Self {
        self.responses.push((emotion, intensity));
        self
    }
}

/// Built-in trigger presets for common emotional stimuli.
pub fn trigger_praised() -> MoodTrigger {
    MoodTrigger::new("praised")
        .respond(Emotion::Joy, 0.4)
        .respond(Emotion::Dominance, 0.2)
        .respond(Emotion::Trust, 0.1)
}

pub fn trigger_criticized() -> MoodTrigger {
    MoodTrigger::new("criticized")
        .respond(Emotion::Joy, -0.3)
        .respond(Emotion::Dominance, -0.2)
        .respond(Emotion::Frustration, 0.3)
}

pub fn trigger_surprised() -> MoodTrigger {
    MoodTrigger::new("surprised")
        .respond(Emotion::Arousal, 0.5)
        .respond(Emotion::Interest, 0.3)
}

pub fn trigger_threatened() -> MoodTrigger {
    MoodTrigger::new("threatened")
        .respond(Emotion::Arousal, 0.4)
        .respond(Emotion::Trust, -0.4)
        .respond(Emotion::Dominance, -0.3)
        .respond(Emotion::Frustration, 0.2)
}

// --- Mood History (v0.3) ---

/// A point-in-time snapshot of mood state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodSnapshot {
    pub mood: MoodVector,
    pub state: MoodState,
    pub deviation: f32,
    pub timestamp: DateTime<Utc>,
}

/// Ring buffer of mood snapshots for trend analysis.
///
/// Uses `VecDeque` for O(1) insertion and eviction at capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodHistory {
    snapshots: VecDeque<MoodSnapshot>,
    capacity: usize,
}

impl MoodHistory {
    /// Create a history buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            snapshots: VecDeque::with_capacity(cap.min(1024)),
            capacity: cap,
        }
    }

    /// Record a snapshot. Drops the oldest if at capacity.
    pub fn record(&mut self, snapshot: MoodSnapshot) {
        if self.snapshots.len() >= self.capacity {
            self.snapshots.pop_front();
        }
        self.snapshots.push_back(snapshot);
    }

    /// All snapshots as a slice pair (VecDeque may be non-contiguous).
    /// Use `iter()` for iteration.
    pub fn snapshots(&self) -> (&[MoodSnapshot], &[MoodSnapshot]) {
        self.snapshots.as_slices()
    }

    /// Iterator over snapshots, oldest first.
    pub fn iter(&self) -> impl Iterator<Item = &MoodSnapshot> {
        self.snapshots.iter()
    }

    /// Number of recorded snapshots.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    /// Whether the history is empty.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }

    /// Average deviation across all snapshots.
    pub fn average_deviation(&self) -> f32 {
        if self.snapshots.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.snapshots.iter().map(|s| s.deviation).sum();
        sum / self.snapshots.len() as f32
    }

    /// Most recent mood state, if any.
    pub fn latest_state(&self) -> Option<MoodState> {
        self.snapshots.back().map(|s| s.state)
    }

    /// Count occurrences of each mood state in history.
    pub fn state_distribution(&self) -> Vec<(MoodState, usize)> {
        use std::collections::HashMap;
        let mut counts: HashMap<MoodState, usize> = HashMap::new();
        for snap in &self.snapshots {
            *counts.entry(snap.state).or_insert(0) += 1;
        }
        let mut dist: Vec<_> = counts.into_iter().collect();
        dist.sort_by(|a, b| b.1.cmp(&a.1));
        dist
    }

    /// Trend: is deviation increasing or decreasing?
    /// Returns positive for escalating, negative for calming, near-zero for stable.
    pub fn deviation_trend(&self) -> f32 {
        if self.snapshots.len() < 2 {
            return 0.0;
        }
        let half = self.snapshots.len() / 2;
        let first_half: f32 = self.snapshots.iter().take(half).map(|s| s.deviation).sum();
        let second_half: f32 = self.snapshots.iter().skip(half).map(|s| s.deviation).sum();
        let first_avg = first_half / half as f32;
        let second_avg = second_half / (self.snapshots.len() - half) as f32;
        second_avg - first_avg
    }
}

// --- Mood Influence on Traits (v0.3) ---

/// Compute a trait-level modifier based on current mood.
///
/// Returns a value from -1.0 to 1.0 that can be used to adjust trait expression.
/// For example, high frustration amplifies directness; high joy softens formality.
#[cfg(feature = "traits")]
pub fn mood_trait_influence(mood: &MoodVector, trait_kind: crate::traits::TraitKind) -> f32 {
    use crate::traits::TraitKind;
    match trait_kind {
        TraitKind::Directness => mood.frustration * 0.5 + mood.dominance * 0.3,
        TraitKind::Warmth => mood.joy * 0.4 + mood.trust * 0.3,
        TraitKind::Patience => -mood.frustration * 0.5 - mood.arousal * 0.3,
        TraitKind::Confidence => mood.dominance * 0.4 + mood.joy * 0.2,
        TraitKind::Humor => mood.joy * 0.4 - mood.frustration * 0.3,
        TraitKind::Empathy => mood.trust * 0.3 + mood.joy * 0.2,
        TraitKind::Curiosity => mood.interest * 0.5 + mood.arousal * 0.2,
        TraitKind::Creativity => mood.interest * 0.3 + mood.arousal * 0.2 + mood.joy * 0.2,
        TraitKind::Formality => -mood.arousal * 0.3 - mood.frustration * 0.2,
        TraitKind::Verbosity => mood.arousal * 0.3 + mood.interest * 0.2,
        TraitKind::RiskTolerance => {
            mood.dominance * 0.3 + mood.arousal * 0.2 - mood.frustration * 0.2
        }
    }
    .clamp(-1.0, 1.0)
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

    #[test]
    fn test_emotion_all() {
        assert_eq!(Emotion::ALL.len(), 6);
    }

    #[test]
    fn test_emotion_display_all() {
        let names: Vec<String> = Emotion::ALL.iter().map(|e| e.to_string()).collect();
        assert!(names.contains(&"joy".to_string()));
        assert!(names.contains(&"arousal".to_string()));
        assert!(names.contains(&"dominance".to_string()));
        assert!(names.contains(&"trust".to_string()));
        assert!(names.contains(&"interest".to_string()));
        assert!(names.contains(&"frustration".to_string()));
    }

    #[test]
    fn test_set_all_dimensions() {
        let mut m = MoodVector::neutral();
        for (i, &e) in Emotion::ALL.iter().enumerate() {
            let val = (i as f32 + 1.0) * 0.15;
            m.set(e, val);
            assert!((m.get(e) - val).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_nudge_clamps() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.9);
        m.nudge(Emotion::Joy, 0.5);
        assert!((m.get(Emotion::Joy) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_dominant_emotion_neutral() {
        let m = MoodVector::neutral();
        // When all zero, returns Joy (first checked)
        let _ = m.dominant_emotion(); // just ensure no panic
    }

    #[test]
    fn test_dominant_emotion_negative() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, -0.2);
        m.set(Emotion::Frustration, -0.9);
        assert_eq!(m.dominant_emotion(), Emotion::Frustration);
    }

    #[test]
    fn test_decay_zero() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        m.decay(0.0);
        assert!((m.get(Emotion::Joy) - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_decay_full() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        m.set(Emotion::Trust, -0.5);
        m.decay(1.0);
        assert!(m.get(Emotion::Joy).abs() < f32::EPSILON);
        assert!(m.get(Emotion::Trust).abs() < f32::EPSILON);
    }

    #[test]
    fn test_decay_clamps_factor() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        m.decay(5.0); // should clamp to 1.0
        assert!(m.get(Emotion::Joy).abs() < f32::EPSILON);
    }

    #[test]
    fn test_blend_zero() {
        let mut a = MoodVector::neutral();
        a.set(Emotion::Joy, 0.5);
        let b = MoodVector::neutral();
        let c = a.blend(&b, 0.0);
        assert!((c.joy - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_blend_one() {
        let a = MoodVector::neutral();
        let mut b = MoodVector::neutral();
        b.set(Emotion::Joy, 1.0);
        let c = a.blend(&b, 1.0);
        assert!((c.joy - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_blend_clamps_t() {
        let a = MoodVector::neutral();
        let mut b = MoodVector::neutral();
        b.set(Emotion::Joy, 1.0);
        let c = a.blend(&b, 5.0); // should clamp to 1.0
        assert!((c.joy - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_intensity_multiple_dimensions() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.6);
        m.set(Emotion::Trust, 0.8);
        let expected = (0.6f32 * 0.6 + 0.8 * 0.8).sqrt();
        assert!((m.intensity() - expected).abs() < 0.01);
    }

    #[test]
    fn test_emotional_state_default() {
        let s = EmotionalState::default();
        assert!(s.deviation().abs() < f32::EPSILON);
        assert!((s.decay_half_life_secs - 300.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_emotional_state_with_baseline() {
        let mut baseline = MoodVector::neutral();
        baseline.set(Emotion::Joy, 0.5);
        let s = EmotionalState::with_baseline(baseline);
        assert!((s.mood.joy - 0.5).abs() < f32::EPSILON);
        assert!((s.baseline.joy - 0.5).abs() < f32::EPSILON);
        assert!(s.deviation().abs() < f32::EPSILON);
    }

    #[test]
    fn test_set_decay_half_life_valid() {
        let mut s = EmotionalState::new();
        assert!(s.set_decay_half_life(60.0).is_ok());
        assert!((s.decay_half_life_secs - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_apply_decay_no_time() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        let before = s.mood.joy;
        s.apply_decay(s.last_updated); // zero elapsed
        assert!((s.mood.joy - before).abs() < f32::EPSILON);
    }

    #[test]
    fn test_apply_decay_toward_baseline() {
        let mut baseline = MoodVector::neutral();
        baseline.set(Emotion::Joy, 0.3);
        let mut s = EmotionalState::with_baseline(baseline);
        s.stimulate(Emotion::Joy, 0.5); // mood.joy now ~0.8

        let future = s.last_updated + chrono::Duration::hours(1);
        s.apply_decay(future);
        // After long decay, should approach baseline (0.3)
        assert!((s.mood.joy - 0.3).abs() < 0.05);
    }

    #[test]
    fn test_apply_decay_negative_elapsed() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        let before = s.mood.joy;
        let past = s.last_updated - chrono::Duration::minutes(5);
        s.apply_decay(past); // negative elapsed, should be no-op
        assert!((s.mood.joy - before).abs() < f32::EPSILON);
    }

    #[test]
    fn test_deviation_with_baseline() {
        let mut baseline = MoodVector::neutral();
        baseline.set(Emotion::Joy, 0.5);
        let mut s = EmotionalState::with_baseline(baseline);
        // mood starts at baseline, deviation is 0
        assert!(s.deviation().abs() < f32::EPSILON);
        s.stimulate(Emotion::Joy, 0.3);
        assert!(s.deviation() > 0.0);
    }

    #[test]
    fn test_emotional_state_serde() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.5);
        s.stimulate(Emotion::Frustration, -0.3);
        let json = serde_json::to_string(&s).unwrap();
        let s2: EmotionalState = serde_json::from_str(&json).unwrap();
        assert!((s2.mood.joy - s.mood.joy).abs() < 0.01);
        assert!((s2.mood.frustration - s.mood.frustration).abs() < 0.01);
        assert!((s2.decay_half_life_secs - s.decay_half_life_secs).abs() < 0.01);
    }

    #[test]
    fn test_emotion_serde() {
        for &e in Emotion::ALL {
            let json = serde_json::to_string(&e).unwrap();
            let e2: Emotion = serde_json::from_str(&json).unwrap();
            assert_eq!(e2, e);
        }
    }

    // --- v0.3: MoodState ---

    #[test]
    fn test_classify_calm() {
        let s = EmotionalState::new();
        assert_eq!(s.classify(), MoodState::Calm);
    }

    #[test]
    fn test_classify_euphoric() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        assert_eq!(s.classify(), MoodState::Euphoric);
    }

    #[test]
    fn test_classify_frustrated() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Frustration, 0.7);
        assert_eq!(s.classify(), MoodState::Frustrated);
    }

    #[test]
    fn test_classify_guarded() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Trust, -0.6);
        assert_eq!(s.classify(), MoodState::Guarded);
    }

    #[test]
    fn test_classify_curious() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Interest, 0.5);
        assert_eq!(s.classify(), MoodState::Curious);
    }

    #[test]
    fn test_classify_agitated() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Arousal, 0.6);
        assert_eq!(s.classify(), MoodState::Agitated);
    }

    #[test]
    fn test_mood_state_display() {
        assert_eq!(MoodState::Calm.to_string(), "calm");
        assert_eq!(MoodState::Euphoric.to_string(), "euphoric");
        assert_eq!(MoodState::Frustrated.to_string(), "frustrated");
    }

    #[test]
    fn test_mood_state_serde() {
        let json = serde_json::to_string(&MoodState::Euphoric).unwrap();
        let restored: MoodState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, MoodState::Euphoric);
    }

    // --- v0.3: MoodTrigger ---

    #[test]
    fn test_trigger_builder() {
        let t = MoodTrigger::new("test")
            .respond(Emotion::Joy, 0.5)
            .respond(Emotion::Trust, 0.3);
        assert_eq!(t.name, "test");
        assert_eq!(t.responses.len(), 2);
    }

    #[test]
    fn test_apply_trigger() {
        let mut s = EmotionalState::new();
        let t = trigger_praised();
        s.apply_trigger(&t);
        assert!(s.mood.joy > 0.0);
        assert!(s.mood.dominance > 0.0);
        assert!(s.mood.trust > 0.0);
    }

    #[test]
    fn test_trigger_criticized() {
        let mut s = EmotionalState::new();
        s.apply_trigger(&trigger_criticized());
        assert!(s.mood.joy < 0.0);
        assert!(s.mood.frustration > 0.0);
    }

    #[test]
    fn test_trigger_surprised() {
        let mut s = EmotionalState::new();
        s.apply_trigger(&trigger_surprised());
        assert!(s.mood.arousal > 0.0);
        assert!(s.mood.interest > 0.0);
    }

    #[test]
    fn test_trigger_threatened() {
        let mut s = EmotionalState::new();
        s.apply_trigger(&trigger_threatened());
        assert!(s.mood.trust < 0.0);
        assert!(s.mood.dominance < 0.0);
    }

    #[test]
    fn test_trigger_serde() {
        let t = trigger_praised();
        let json = serde_json::to_string(&t).unwrap();
        let t2: MoodTrigger = serde_json::from_str(&json).unwrap();
        assert_eq!(t2.name, "praised");
        assert_eq!(t2.responses.len(), t.responses.len());
    }

    // --- v0.3: MoodHistory ---

    #[test]
    fn test_history_new() {
        let h = MoodHistory::new(10);
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn test_history_record() {
        let mut h = MoodHistory::new(10);
        let s = EmotionalState::new();
        h.record(s.snapshot());
        assert_eq!(h.len(), 1);
        assert!(!h.is_empty());
    }

    #[test]
    fn test_history_capacity() {
        let mut h = MoodHistory::new(3);
        let s = EmotionalState::new();
        for _ in 0..5 {
            h.record(s.snapshot());
        }
        assert_eq!(h.len(), 3);
    }

    #[test]
    fn test_history_average_deviation() {
        let mut h = MoodHistory::new(10);
        let s = EmotionalState::new();
        h.record(s.snapshot());
        assert!(h.average_deviation().abs() < f32::EPSILON);
    }

    #[test]
    fn test_history_average_deviation_empty() {
        let h = MoodHistory::new(10);
        assert!(h.average_deviation().abs() < f32::EPSILON);
    }

    #[test]
    fn test_history_latest_state() {
        let mut h = MoodHistory::new(10);
        assert!(h.latest_state().is_none());

        let s = EmotionalState::new();
        h.record(s.snapshot());
        assert_eq!(h.latest_state(), Some(MoodState::Calm));
    }

    #[test]
    fn test_history_state_distribution() {
        let mut h = MoodHistory::new(10);
        let mut s = EmotionalState::new();
        h.record(s.snapshot()); // calm

        s.stimulate(Emotion::Joy, 0.8);
        h.record(s.snapshot()); // euphoric

        let dist = h.state_distribution();
        assert_eq!(dist.len(), 2);
    }

    #[test]
    fn test_history_deviation_trend_stable() {
        let mut h = MoodHistory::new(10);
        let s = EmotionalState::new();
        for _ in 0..4 {
            h.record(s.snapshot());
        }
        assert!(h.deviation_trend().abs() < f32::EPSILON);
    }

    #[test]
    fn test_history_deviation_trend_escalating() {
        let mut h = MoodHistory::new(10);
        let mut s = EmotionalState::new();
        // First half: calm
        h.record(s.snapshot());
        h.record(s.snapshot());
        // Second half: stimulated
        s.stimulate(Emotion::Frustration, 0.8);
        h.record(s.snapshot());
        h.record(s.snapshot());
        assert!(h.deviation_trend() > 0.0);
    }

    #[test]
    fn test_history_serde() {
        let mut h = MoodHistory::new(5);
        let s = EmotionalState::new();
        h.record(s.snapshot());
        let json = serde_json::to_string(&h).unwrap();
        let h2: MoodHistory = serde_json::from_str(&json).unwrap();
        assert_eq!(h2.len(), 1);
    }

    #[test]
    fn test_snapshot_fields() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        let snap = s.snapshot();
        assert_eq!(snap.state, MoodState::Euphoric);
        assert!(snap.deviation > 0.0);
        assert!((snap.mood.joy - s.mood.joy).abs() < f32::EPSILON);
    }

    // --- v0.3: Mood Influence ---

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_neutral() {
        let m = MoodVector::neutral();
        for &kind in crate::traits::TraitKind::ALL {
            let inf = mood_trait_influence(&m, kind);
            assert!(
                inf.abs() < f32::EPSILON,
                "{kind} influence should be 0 for neutral mood"
            );
        }
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_frustration_boosts_directness() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, 0.8);
        let inf = mood_trait_influence(&m, crate::traits::TraitKind::Directness);
        assert!(inf > 0.0);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_joy_boosts_warmth() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        let inf = mood_trait_influence(&m, crate::traits::TraitKind::Warmth);
        assert!(inf > 0.0);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_clamped() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, 1.0);
        m.set(Emotion::Dominance, 1.0);
        m.set(Emotion::Arousal, 1.0);
        for &kind in crate::traits::TraitKind::ALL {
            let inf = mood_trait_influence(&m, kind);
            assert!(
                ((-1.0)..=1.0).contains(&inf),
                "{kind} influence {inf} out of range"
            );
        }
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_all_traits_covered() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.5);
        m.set(Emotion::Frustration, 0.3);
        m.set(Emotion::Interest, 0.4);
        // Just ensure no panic for every trait
        for &kind in crate::traits::TraitKind::ALL {
            let _ = mood_trait_influence(&m, kind);
        }
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_frustration_reduces_patience() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, 0.8);
        let inf = mood_trait_influence(&m, crate::traits::TraitKind::Patience);
        assert!(inf < 0.0);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_interest_boosts_curiosity() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Interest, 0.8);
        let inf = mood_trait_influence(&m, crate::traits::TraitKind::Curiosity);
        assert!(inf > 0.0);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_mood_trait_influence_dominance_boosts_confidence() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Dominance, 0.8);
        let inf = mood_trait_influence(&m, crate::traits::TraitKind::Confidence);
        assert!(inf > 0.0);
    }

    // --- Additional MoodState coverage ---

    #[test]
    fn test_classify_melancholy() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, -0.5);
        assert_eq!(s.classify(), MoodState::Melancholy);
    }

    #[test]
    fn test_classify_assertive() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Dominance, 0.6);
        assert_eq!(s.classify(), MoodState::Assertive);
    }

    #[test]
    fn test_classify_overwhelmed() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Dominance, -0.6);
        assert_eq!(s.classify(), MoodState::Overwhelmed);
    }

    #[test]
    fn test_classify_trusting() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Trust, 0.5);
        assert_eq!(s.classify(), MoodState::Trusting);
    }

    #[test]
    fn test_classify_disengaged() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Interest, -0.5);
        assert_eq!(s.classify(), MoodState::Disengaged);
    }

    #[test]
    fn test_classify_content() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.3); // positive but not euphoric
        assert_eq!(s.classify(), MoodState::Content);
    }

    // --- Additional MoodHistory coverage ---

    #[test]
    fn test_history_state_distribution_empty() {
        let h = MoodHistory::new(10);
        assert!(h.state_distribution().is_empty());
    }

    #[test]
    fn test_history_deviation_trend_two_snapshots() {
        let mut h = MoodHistory::new(10);
        let s = EmotionalState::new();
        h.record(s.snapshot());
        let mut s2 = EmotionalState::new();
        s2.stimulate(Emotion::Joy, 0.8);
        h.record(s2.snapshot());
        // With 2 snapshots, half=1, first=[0..1], second=[1..]
        assert!(h.deviation_trend() > 0.0);
    }

    #[test]
    fn test_history_deviation_trend_calming() {
        let mut h = MoodHistory::new(10);
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Frustration, 0.9);
        h.record(s.snapshot());
        h.record(s.snapshot());
        // Second half: calm
        let calm = EmotionalState::new();
        h.record(calm.snapshot());
        h.record(calm.snapshot());
        assert!(h.deviation_trend() < 0.0);
    }

    #[test]
    fn test_history_iter() {
        let mut h = MoodHistory::new(10);
        let s = EmotionalState::new();
        h.record(s.snapshot());
        h.record(s.snapshot());
        assert_eq!(h.iter().count(), 2);
    }

    #[test]
    fn test_history_capacity_zero_becomes_one() {
        let h = MoodHistory::new(0);
        // capacity should be clamped to 1
        assert!(h.is_empty());
    }

    // --- MoodState Display completeness ---

    #[test]
    fn test_mood_state_display_all() {
        let states = [
            MoodState::Calm,
            MoodState::Content,
            MoodState::Euphoric,
            MoodState::Melancholy,
            MoodState::Agitated,
            MoodState::Assertive,
            MoodState::Overwhelmed,
            MoodState::Trusting,
            MoodState::Guarded,
            MoodState::Curious,
            MoodState::Disengaged,
            MoodState::Frustrated,
        ];
        for state in states {
            assert!(!state.to_string().is_empty());
        }
    }

    #[test]
    fn test_mood_state_serde_all() {
        let states = [
            MoodState::Calm,
            MoodState::Content,
            MoodState::Euphoric,
            MoodState::Melancholy,
            MoodState::Agitated,
            MoodState::Assertive,
            MoodState::Overwhelmed,
            MoodState::Trusting,
            MoodState::Guarded,
            MoodState::Curious,
            MoodState::Disengaged,
            MoodState::Frustrated,
        ];
        for state in states {
            let json = serde_json::to_string(&state).unwrap();
            let restored: MoodState = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, state);
        }
    }

    // --- MoodTrigger edge cases ---

    #[test]
    fn test_trigger_empty_responses() {
        let t = MoodTrigger::new("empty");
        let mut s = EmotionalState::new();
        s.apply_trigger(&t);
        assert!(s.deviation().abs() < f32::EPSILON);
    }

    #[test]
    fn test_trigger_multiple_same_emotion() {
        let t = MoodTrigger::new("double")
            .respond(Emotion::Joy, 0.3)
            .respond(Emotion::Joy, 0.3);
        let mut s = EmotionalState::new();
        s.apply_trigger(&t);
        assert!((s.mood.joy - 0.6).abs() < 0.01);
    }
}
