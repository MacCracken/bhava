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
    #[must_use]
    pub fn new() -> Self {
        Self {
            mood: MoodVector::neutral(),
            baseline: MoodVector::neutral(),
            decay_half_life_secs: 300.0,
            last_updated: Utc::now(),
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
    pub fn apply_trigger(&mut self, trigger: &MoodTrigger) {
        for &(emotion, intensity) in &trigger.responses {
            self.mood.nudge(emotion, intensity);
        }
        self.last_updated = Utc::now();
    }

    // --- Mood History (v0.3) ---

    /// Take a snapshot of the current mood with a timestamp.
    #[must_use]
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

/// Built-in trigger: criticized (joy-, dominance-, frustration+).
pub fn trigger_criticized() -> MoodTrigger {
    MoodTrigger::new("criticized")
        .respond(Emotion::Joy, -0.3)
        .respond(Emotion::Dominance, -0.2)
        .respond(Emotion::Frustration, 0.3)
}

/// Built-in trigger: surprised (arousal+, interest+).
pub fn trigger_surprised() -> MoodTrigger {
    MoodTrigger::new("surprised")
        .respond(Emotion::Arousal, 0.5)
        .respond(Emotion::Interest, 0.3)
}

/// Built-in trigger: threatened (arousal+, trust-, dominance-, frustration+).
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
        TraitKind::Skepticism => -mood.trust * 0.4 + mood.frustration * 0.2,
        TraitKind::Autonomy => mood.dominance * 0.4 + mood.arousal * 0.2,
        TraitKind::Pedagogy => mood.interest * 0.3 + mood.joy * 0.2 - mood.frustration * 0.2,
        TraitKind::Precision => -mood.arousal * 0.3 + mood.dominance * 0.2,
    }
    .clamp(-1.0, 1.0)
}

// --- Trait-to-Mood Baseline (SY parity) ---

/// Valence/arousal modifier for a single trait level.
#[cfg(feature = "traits")]
struct TraitMoodModifier {
    valence: f32,
    arousal: f32,
}

/// Get the valence/arousal contribution of a trait at a given level.
///
/// Ported from SecureYeoman's TRAIT_VALUE_MODIFIERS.
#[cfg(feature = "traits")]
fn trait_mood_modifier(
    kind: crate::traits::TraitKind,
    level: crate::traits::TraitLevel,
) -> TraitMoodModifier {
    use crate::traits::{TraitKind as TK, TraitLevel as TL};
    let (v, a) = match (kind, level) {
        (TK::Formality, TL::Lowest) => (0.05, 0.15),
        (TK::Formality, TL::Low) => (0.05, 0.05),
        (TK::Formality, TL::High) => (-0.05, -0.1),
        (TK::Formality, TL::Highest) => (-0.05, -0.15),

        (TK::Humor, TL::Lowest) => (-0.1, -0.1),
        (TK::Humor, TL::Low) => (-0.05, -0.05),
        (TK::Humor, TL::High) => (0.15, 0.1),
        (TK::Humor, TL::Highest) => (0.2, 0.15),

        (TK::Warmth, TL::Lowest) => (-0.25, -0.15),
        (TK::Warmth, TL::Low) => (-0.1, -0.1),
        (TK::Warmth, TL::High) => (0.2, 0.1),
        (TK::Warmth, TL::Highest) => (0.3, 0.2),

        (TK::Empathy, TL::Lowest) => (-0.15, -0.1),
        (TK::Empathy, TL::Low) => (-0.05, -0.05),
        (TK::Empathy, TL::High) => (0.1, 0.0),
        (TK::Empathy, TL::Highest) => (0.15, -0.05),

        (TK::Patience, TL::Lowest) => (-0.1, 0.15),
        (TK::Patience, TL::Low) => (-0.05, 0.05),
        (TK::Patience, TL::High) => (0.1, -0.1),
        (TK::Patience, TL::Highest) => (0.15, -0.15),

        (TK::Confidence, TL::Lowest) => (-0.15, -0.1),
        (TK::Confidence, TL::Low) => (-0.05, -0.05),
        (TK::Confidence, TL::High) => (0.1, 0.1),
        (TK::Confidence, TL::Highest) => (0.15, 0.15),

        (TK::Creativity, TL::Lowest) => (-0.05, -0.1),
        (TK::Creativity, TL::Low) => (0.0, -0.05),
        (TK::Creativity, TL::High) => (0.1, 0.1),
        (TK::Creativity, TL::Highest) => (0.15, 0.15),

        (TK::RiskTolerance, TL::Lowest) => (-0.1, -0.15),
        (TK::RiskTolerance, TL::Low) => (-0.05, -0.05),
        (TK::RiskTolerance, TL::High) => (0.05, 0.1),
        (TK::RiskTolerance, TL::Highest) => (0.1, 0.2),

        (TK::Curiosity, TL::Lowest) => (-0.05, -0.1),
        (TK::Curiosity, TL::Low) => (0.0, -0.05),
        (TK::Curiosity, TL::High) => (0.1, 0.1),
        (TK::Curiosity, TL::Highest) => (0.15, 0.15),

        (TK::Verbosity, TL::Lowest) => (0.0, -0.05),
        (TK::Verbosity, TL::Low) => (0.0, -0.02),
        (TK::Verbosity, TL::High) => (0.0, 0.05),
        (TK::Verbosity, TL::Highest) => (0.0, 0.1),

        (TK::Directness, TL::Lowest) => (0.0, -0.05),
        (TK::Directness, TL::Low) => (0.0, -0.02),
        (TK::Directness, TL::High) => (0.0, 0.05),
        (TK::Directness, TL::Highest) => (-0.05, 0.1),

        (TK::Skepticism, TL::Lowest) => (0.05, -0.05),
        (TK::Skepticism, TL::Low) => (0.02, -0.02),
        (TK::Skepticism, TL::High) => (-0.05, 0.05),
        (TK::Skepticism, TL::Highest) => (-0.1, 0.1),

        (TK::Autonomy, TL::Lowest) => (-0.05, -0.1),
        (TK::Autonomy, TL::Low) => (-0.02, -0.05),
        (TK::Autonomy, TL::High) => (0.05, 0.1),
        (TK::Autonomy, TL::Highest) => (0.1, 0.15),

        (TK::Pedagogy, TL::Lowest) => (0.0, -0.05),
        (TK::Pedagogy, TL::Low) => (0.0, -0.02),
        (TK::Pedagogy, TL::High) => (0.05, 0.05),
        (TK::Pedagogy, TL::Highest) => (0.1, 0.1),

        (TK::Precision, TL::Lowest) => (0.0, -0.05),
        (TK::Precision, TL::Low) => (0.0, -0.02),
        (TK::Precision, TL::High) => (-0.02, 0.05),
        (TK::Precision, TL::Highest) => (-0.05, 0.1),

        (_, TL::Balanced) => (0.0, 0.0),
    };
    TraitMoodModifier {
        valence: v,
        arousal: a,
    }
}

/// Derive a mood baseline from a personality profile.
///
/// Each trait's level contributes valence and arousal modifiers. The baseline
/// is the average of all contributions, producing the personality's natural
/// emotional resting state.
///
/// Returns a `MoodVector` with `joy` set to the derived valence and `arousal`
/// set to the derived arousal. Other dimensions are zero.
#[cfg(feature = "traits")]
#[must_use]
pub fn derive_mood_baseline(profile: &crate::traits::PersonalityProfile) -> MoodVector {
    use crate::traits::TraitKind;

    let mut total_v = 0.0f32;
    let mut total_a = 0.0f32;

    for &kind in TraitKind::ALL {
        let level = profile.get_trait(kind);
        let m = trait_mood_modifier(kind, level);
        total_v += m.valence;
        total_a += m.arousal;
    }

    // Average across all traits
    let count = TraitKind::COUNT as f32;
    let base_v = (total_v / count).clamp(-1.0, 1.0);
    let base_a = (total_a / count).clamp(-1.0, 1.0);

    // Apply compound effects
    let compound = compute_compound_effects(profile);
    let final_v = (base_v + compound.0).clamp(-1.0, 1.0);
    let final_a = (base_a + compound.1).clamp(-1.0, 1.0);

    let mut baseline = MoodVector::neutral();
    baseline.joy = final_v;
    baseline.arousal = final_a;
    baseline
}

// --- Compound Trait Effects (SY parity) ---

/// Compute compound mood effects from trait combinations.
///
/// Returns (valence_delta, arousal_delta) from emergent trait interactions.
/// For example, high warmth + high humor → "playful" (+0.1 valence, +0.1 arousal).
#[cfg(feature = "traits")]
fn compute_compound_effects(profile: &crate::traits::PersonalityProfile) -> (f32, f32) {
    use crate::traits::{TraitKind as TK, TraitLevel as TL};

    let mut v = 0.0f32;
    let mut a = 0.0f32;

    let get = |k: TK| profile.get_trait(k);
    let high_or_above = |l: TL| l >= TL::High;
    let low_or_below = |l: TL| l <= TL::Low;

    // Playful: warm + funny
    if high_or_above(get(TK::Warmth)) && high_or_above(get(TK::Humor)) {
        v += 0.1;
        a += 0.1;
    }
    // Nurturing: warm + empathetic
    if high_or_above(get(TK::Warmth)) && high_or_above(get(TK::Empathy)) {
        v += 0.1;
        a -= 0.05;
    }
    // Mentoring: patient + pedagogical
    if high_or_above(get(TK::Patience)) && high_or_above(get(TK::Pedagogy)) {
        v += 0.1;
        a -= 0.1;
    }
    // Driven: confident + autonomous
    if high_or_above(get(TK::Confidence)) && high_or_above(get(TK::Autonomy)) {
        v += 0.05;
        a += 0.15;
    }
    // Guarded: skeptical + cold
    if high_or_above(get(TK::Skepticism)) && low_or_below(get(TK::Warmth)) {
        v -= 0.1;
        a += 0.05;
    }
    // Anxious: low confidence + low risk tolerance
    if low_or_below(get(TK::Confidence)) && low_or_below(get(TK::RiskTolerance)) {
        v -= 0.15;
        a += 0.1;
    }
    // Investigative: curious + precise
    if high_or_above(get(TK::Curiosity)) && high_or_above(get(TK::Precision)) {
        v += 0.05;
        a += 0.05;
    }

    (v, a)
}

// --- Mood Tone Guides (SY parity) ---

/// Get a prompt-injectable tone guide for a named mood state.
///
/// These are short behavioral instructions that can be injected into LLM
/// system prompts to color the agent's communication style based on current mood.
#[must_use]
pub fn mood_tone_guide(state: MoodState) -> &'static str {
    match state {
        MoodState::Euphoric => {
            "Speak with enthusiasm and unbridled joy. Be effusive and celebratory."
        }
        MoodState::Content => "Be relaxed and satisfied. Communicate with gentle warmth.",
        MoodState::Calm => "Speak with measured tranquility. Be steady and reassuring.",
        MoodState::Melancholy => {
            "Communicate with quiet thoughtfulness. Be reflective and subdued."
        }
        MoodState::Agitated => {
            "Communicate with energy and urgency. Be animated and forward-leaning."
        }
        MoodState::Assertive => "Speak with authority and conviction. Be decisive and commanding.",
        MoodState::Overwhelmed => {
            "Communicate with caution and hesitation. Seek clarity before acting."
        }
        MoodState::Trusting => "Be open and collaborative. Share freely and assume good intent.",
        MoodState::Guarded => "Be measured and careful. Verify before trusting. Keep things close.",
        MoodState::Curious => "Be inquisitive and engaged. Ask questions and explore tangents.",
        MoodState::Disengaged => "Be brief and perfunctory. Conserve energy for what matters.",
        MoodState::Frustrated => "Be terse and impatient. Cut to the point. Tolerate no fluff.",
    }
}

/// Compose a mood prompt fragment for injection into a system prompt.
///
/// Combines the current mood label with its tone guide.
#[must_use]
pub fn compose_mood_prompt(state: &EmotionalState) -> String {
    let mood_state = state.classify();
    let guide = mood_tone_guide(mood_state);
    format!("## Current Mood: {}\n\n{}\n", mood_state, guide)
}

// --- Action Tendencies ---

/// Behavioral impulse derived from current emotional state.
///
/// Tells consumers what the agent *wants to do* based on mood.
/// Ported from WASABI (Affect Simulation for Agents with Believable Interactivity).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ActionTendency {
    /// Positive engagement — seek interaction, share, help.
    Approach { intensity: f32 },
    /// Negative avoidance — retreat, disengage, flee.
    Avoid { intensity: f32 },
    /// Confrontational — challenge, argue, push back.
    Confront { intensity: f32 },
    /// Withdrawal — disengage, conserve energy, self-isolate.
    Withdraw { intensity: f32 },
    /// Protective — guard, defend, shield others.
    Protect { intensity: f32 },
    /// No strong impulse.
    Neutral,
}

/// Derive the dominant action tendency from a mood vector.
#[must_use]
pub fn action_tendency(mood: &MoodVector) -> ActionTendency {
    let joy = mood.joy;
    let arousal = mood.arousal;
    let dominance = mood.dominance;
    let trust = mood.trust;
    let frustration = mood.frustration;

    // Approach: positive joy + trust
    let approach = (joy * 0.5 + trust * 0.3 + arousal * 0.2).max(0.0);
    // Avoid: negative trust + negative dominance
    let avoid = (-trust * 0.4 - dominance * 0.3 + arousal * 0.2).max(0.0);
    // Confront: frustration + dominance + arousal
    let confront = (frustration * 0.4 + dominance * 0.3 + arousal * 0.3).max(0.0);
    // Withdraw: negative joy + negative arousal
    let withdraw = (-joy * 0.4 - arousal * 0.3 - dominance * 0.2).max(0.0);
    // Protect: trust + dominance + negative frustration
    let protect = (trust * 0.3 + dominance * 0.4 - frustration * 0.2).max(0.0);

    let candidates = [
        (approach, "approach"),
        (avoid, "avoid"),
        (confront, "confront"),
        (withdraw, "withdraw"),
        (protect, "protect"),
    ];

    let (max_val, max_label) = candidates
        .iter()
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    if *max_val < 0.1 {
        return ActionTendency::Neutral;
    }

    match *max_label {
        "approach" => ActionTendency::Approach {
            intensity: *max_val,
        },
        "avoid" => ActionTendency::Avoid {
            intensity: *max_val,
        },
        "confront" => ActionTendency::Confront {
            intensity: *max_val,
        },
        "withdraw" => ActionTendency::Withdraw {
            intensity: *max_val,
        },
        "protect" => ActionTendency::Protect {
            intensity: *max_val,
        },
        _ => ActionTendency::Neutral,
    }
}

// --- Emotional Contagion ---

/// Parameters controlling how strongly an entity broadcasts and receives emotions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContagionParams {
    /// How strongly this entity broadcasts emotions (0.0–1.0).
    pub expressiveness: f32,
    /// How easily this entity catches emotions (0.0–1.0).
    pub susceptibility: f32,
}

impl Default for ContagionParams {
    fn default() -> Self {
        Self {
            expressiveness: 0.5,
            susceptibility: 0.5,
        }
    }
}

/// Derive contagion parameters from a personality profile.
#[cfg(feature = "traits")]
#[must_use]
pub fn contagion_from_personality(profile: &crate::traits::PersonalityProfile) -> ContagionParams {
    use crate::traits::TraitKind;
    let warmth = profile.get_trait(TraitKind::Warmth).normalized();
    let empathy = profile.get_trait(TraitKind::Empathy).normalized();
    let confidence = profile.get_trait(TraitKind::Confidence).normalized();
    let skepticism = profile.get_trait(TraitKind::Skepticism).normalized();

    ContagionParams {
        expressiveness: (warmth * 0.4 + confidence * 0.3 + 0.5).clamp(0.0, 1.0),
        susceptibility: (empathy * 0.5 - skepticism * 0.3 + 0.5).clamp(0.0, 1.0),
    }
}

/// Compute emotional contagion from a sender to a receiver.
///
/// Returns a mood delta to apply to the receiver's emotional state.
/// The delta is modulated by sender expressiveness, receiver susceptibility,
/// and the relationship affinity between them.
#[must_use]
pub fn compute_contagion(
    sender_mood: &MoodVector,
    sender_params: &ContagionParams,
    receiver_params: &ContagionParams,
    affinity: f32,
) -> MoodVector {
    let strength = sender_params.expressiveness * receiver_params.susceptibility * affinity.abs();
    let sign = if affinity >= 0.0 { 1.0 } else { -1.0 }; // rivals invert

    let mut delta = MoodVector::neutral();
    for &e in Emotion::ALL {
        let val = sender_mood.get(e) * strength * sign;
        delta.set(e, val);
    }
    delta
}

/// Compute the group emotional centroid (average mood of a group).
#[must_use]
pub fn group_mood(members: &[&MoodVector]) -> MoodVector {
    if members.is_empty() {
        return MoodVector::neutral();
    }
    let mut sum = MoodVector::neutral();
    for m in members {
        for &e in Emotion::ALL {
            sum.set(e, sum.get(e) + m.get(e));
        }
    }
    let n = members.len() as f32;
    for &e in Emotion::ALL {
        sum.set(e, sum.get(e) / n);
    }
    sum
}

// --- Adaptive Baselines ---

/// Adaptive baseline that drifts based on accumulated emotional experience.
///
/// Models the "hedonic treadmill": sustained positive experiences raise the
/// baseline; sustained negative experiences lower it. The baseline recovers
/// toward the core (trait-derived) baseline over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveBaseline {
    /// Core baseline derived from personality traits (never changes).
    pub core: MoodVector,
    /// Current effective baseline (drifts with experience).
    pub adapted: MoodVector,
    /// How fast the baseline shifts toward recent mood (0.001–0.05).
    pub adaptation_rate: f32,
    /// How fast the baseline recovers toward core (hedonic treadmill) (0.01–0.1).
    pub recovery_rate: f32,
}

impl AdaptiveBaseline {
    /// Create from a core baseline.
    #[must_use]
    pub fn new(core: MoodVector) -> Self {
        Self {
            adapted: core.clone(),
            core,
            adaptation_rate: 0.01,
            recovery_rate: 0.05,
        }
    }

    /// Update the adaptive baseline based on recent mood.
    ///
    /// Call periodically (e.g., once per session or every N interactions).
    pub fn adapt(&mut self, recent_mood: &MoodVector) {
        // Shift toward recent mood
        self.adapted = self.adapted.blend(recent_mood, self.adaptation_rate);
        // Pull back toward core (hedonic treadmill)
        self.adapted = self.adapted.blend(&self.core, self.recovery_rate);
    }

    /// Get the current effective baseline.
    #[must_use]
    pub fn current(&self) -> &MoodVector {
        &self.adapted
    }

    /// How far the adapted baseline has drifted from core.
    #[must_use]
    pub fn drift(&self) -> f32 {
        let dj = self.adapted.joy - self.core.joy;
        let da = self.adapted.arousal - self.core.arousal;
        let dd = self.adapted.dominance - self.core.dominance;
        let dt = self.adapted.trust - self.core.trust;
        let di = self.adapted.interest - self.core.interest;
        let df = self.adapted.frustration - self.core.frustration;
        (dj * dj + da * da + dd * dd + dt * dt + di * di + df * df).sqrt()
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

    // --- Trait-to-mood baseline ---

    #[cfg(feature = "traits")]
    #[test]
    fn test_derive_baseline_balanced_near_zero() {
        let profile = crate::traits::PersonalityProfile::new("neutral");
        let baseline = derive_mood_baseline(&profile);
        // All balanced traits → near-zero baseline
        assert!(baseline.joy.abs() < 0.01);
        assert!(baseline.arousal.abs() < 0.01);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_derive_baseline_warm_positive() {
        let mut profile = crate::traits::PersonalityProfile::new("warm");
        profile.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Highest,
        );
        profile.set_trait(
            crate::traits::TraitKind::Humor,
            crate::traits::TraitLevel::Highest,
        );
        let baseline = derive_mood_baseline(&profile);
        assert!(
            baseline.joy > 0.0,
            "warm+funny should have positive valence"
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_derive_baseline_cold_negative() {
        let mut profile = crate::traits::PersonalityProfile::new("cold");
        profile.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Lowest,
        );
        profile.set_trait(
            crate::traits::TraitKind::Empathy,
            crate::traits::TraitLevel::Lowest,
        );
        let baseline = derive_mood_baseline(&profile);
        assert!(
            baseline.joy < 0.0,
            "cold+detached should have negative valence"
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_derive_baseline_with_compound_effects() {
        let mut profile = crate::traits::PersonalityProfile::new("playful");
        profile.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Highest,
        );
        profile.set_trait(
            crate::traits::TraitKind::Humor,
            crate::traits::TraitLevel::Highest,
        );

        let mut baseline_profile = crate::traits::PersonalityProfile::new("just_warm");
        baseline_profile.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Highest,
        );

        let playful = derive_mood_baseline(&profile);
        let just_warm = derive_mood_baseline(&baseline_profile);
        // Compound "playful" effect should boost valence beyond just warmth
        assert!(playful.joy > just_warm.joy);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_derive_baseline_clamped() {
        use crate::traits::{TraitKind, TraitLevel};
        let mut profile = crate::traits::PersonalityProfile::new("extreme");
        for &kind in TraitKind::ALL {
            profile.set_trait(kind, TraitLevel::Highest);
        }
        let baseline = derive_mood_baseline(&profile);
        assert!(((-1.0)..=1.0).contains(&baseline.joy));
        assert!(((-1.0)..=1.0).contains(&baseline.arousal));
    }

    // --- Mood tone guides ---

    #[test]
    fn test_mood_tone_guide_all_states() {
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
            let guide = mood_tone_guide(state);
            assert!(!guide.is_empty(), "{state} has empty tone guide");
        }
    }

    #[test]
    fn test_compose_mood_prompt() {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        let prompt = compose_mood_prompt(&s);
        assert!(prompt.contains("## Current Mood:"));
        assert!(prompt.contains("euphoric") || prompt.contains("content"));
    }

    #[test]
    fn test_compose_mood_prompt_calm() {
        let s = EmotionalState::new();
        let prompt = compose_mood_prompt(&s);
        assert!(prompt.contains("calm"));
    }

    // --- Action Tendencies ---

    #[test]
    fn test_action_tendency_positive() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        m.set(Emotion::Trust, 0.5);
        match action_tendency(&m) {
            ActionTendency::Approach { intensity } => assert!(intensity > 0.1),
            other => panic!("expected Approach, got {other:?}"),
        }
    }

    #[test]
    fn test_action_tendency_frustrated() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, 0.8);
        m.set(Emotion::Dominance, 0.5);
        m.set(Emotion::Arousal, 0.6);
        match action_tendency(&m) {
            ActionTendency::Confront { intensity } => assert!(intensity > 0.1),
            other => panic!("expected Confront, got {other:?}"),
        }
    }

    #[test]
    fn test_action_tendency_withdraw() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, -0.8);
        m.set(Emotion::Arousal, -0.5);
        match action_tendency(&m) {
            ActionTendency::Withdraw { intensity } => assert!(intensity > 0.1),
            other => panic!("expected Withdraw, got {other:?}"),
        }
    }

    #[test]
    fn test_action_tendency_neutral() {
        let m = MoodVector::neutral();
        assert!(matches!(action_tendency(&m), ActionTendency::Neutral));
    }

    // --- Emotional Contagion ---

    #[test]
    fn test_contagion_basic() {
        let mut sender = MoodVector::neutral();
        sender.set(Emotion::Joy, 0.8);
        let sp = ContagionParams {
            expressiveness: 0.8,
            susceptibility: 0.0,
        };
        let rp = ContagionParams {
            expressiveness: 0.0,
            susceptibility: 0.8,
        };
        let delta = compute_contagion(&sender, &sp, &rp, 0.7);
        assert!(delta.joy > 0.0);
    }

    #[test]
    fn test_contagion_rival_inverts() {
        let mut sender = MoodVector::neutral();
        sender.set(Emotion::Joy, 0.8);
        let sp = ContagionParams {
            expressiveness: 0.8,
            susceptibility: 0.0,
        };
        let rp = ContagionParams {
            expressiveness: 0.0,
            susceptibility: 0.8,
        };
        let delta = compute_contagion(&sender, &sp, &rp, -0.5);
        assert!(delta.joy < 0.0); // rival's joy → receiver's sadness
    }

    #[test]
    fn test_contagion_zero_affinity() {
        let mut sender = MoodVector::neutral();
        sender.set(Emotion::Joy, 0.8);
        let sp = ContagionParams::default();
        let rp = ContagionParams::default();
        let delta = compute_contagion(&sender, &sp, &rp, 0.0);
        assert!(delta.joy.abs() < f32::EPSILON);
    }

    #[test]
    fn test_group_mood_single() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.6);
        let result = group_mood(&[&m]);
        assert!((result.joy - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn test_group_mood_average() {
        let mut a = MoodVector::neutral();
        a.set(Emotion::Joy, 0.8);
        let mut b = MoodVector::neutral();
        b.set(Emotion::Joy, 0.2);
        let result = group_mood(&[&a, &b]);
        assert!((result.joy - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_group_mood_empty() {
        let result = group_mood(&[]);
        assert!(result.intensity() < f32::EPSILON);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_contagion_from_personality() {
        let mut profile = crate::traits::PersonalityProfile::new("warm");
        profile.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Highest,
        );
        profile.set_trait(
            crate::traits::TraitKind::Empathy,
            crate::traits::TraitLevel::Highest,
        );
        let params = contagion_from_personality(&profile);
        assert!(params.expressiveness > 0.5);
        assert!(params.susceptibility > 0.5);
    }

    // --- Adaptive Baselines ---

    #[test]
    fn test_adaptive_baseline_new() {
        let core = MoodVector::neutral();
        let ab = AdaptiveBaseline::new(core);
        assert!(ab.drift() < f32::EPSILON);
    }

    #[test]
    fn test_adaptive_baseline_adapts() {
        let core = MoodVector::neutral();
        let mut ab = AdaptiveBaseline::new(core);
        let mut positive = MoodVector::neutral();
        positive.set(Emotion::Joy, 0.8);
        // Adapt many times toward positive mood
        for _ in 0..100 {
            ab.adapt(&positive);
        }
        assert!(
            ab.adapted.joy > 0.0,
            "baseline should shift toward positive"
        );
        assert!(ab.drift() > 0.0);
    }

    #[test]
    fn test_adaptive_baseline_recovery() {
        let core = MoodVector::neutral();
        let mut ab = AdaptiveBaseline::new(core);
        ab.adapted.joy = 0.5; // artificially shift
        // Recovery pulls back toward core (0.0)
        for _ in 0..200 {
            ab.adapt(&MoodVector::neutral());
        }
        assert!(
            ab.adapted.joy.abs() < 0.1,
            "baseline should recover toward core"
        );
    }

    #[test]
    fn test_adaptive_baseline_serde() {
        let ab = AdaptiveBaseline::new(MoodVector::neutral());
        let json = serde_json::to_string(&ab).unwrap();
        let ab2: AdaptiveBaseline = serde_json::from_str(&json).unwrap();
        assert!((ab2.adaptation_rate - ab.adaptation_rate).abs() < f32::EPSILON);
    }
}
