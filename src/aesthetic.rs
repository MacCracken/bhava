//! Aesthetic attribution — repeated aesthetic exposure crystallizes into beliefs.
//!
//! Models how entities develop aesthetic sensibility through exposure to art,
//! music, narrative, beauty, and other aesthetic stimuli. Preferences form via
//! the existing [`PreferenceStore`] EMA
//! learning mechanism, with two modifications:
//!
//! 1. **Mere-exposure effect** (Zajonc 1968): aesthetic preferences decay at
//!    half the normal rate — familiarity breeds liking.
//! 2. **Positive bias**: aesthetic experiences carry a 1.3x positive gain,
//!    modelling the hedonic default of aesthetic engagement.
//!
//! When aesthetic preference conviction exceeds a threshold, the preference
//! *crystallizes* into a [`BeliefKind::WorldBelief`]
//! or [`BeliefKind::SelfBelief`] via the existing
//! belief system. Sustained exposure also generates trait pressure toward
//! Creativity, Curiosity, and Empathy.
//!
//! # Dimensions
//!
//! Five aesthetic dimensions capture the space of aesthetic experience:
//!
//! | Dimension | Positive pole | Negative pole |
//! |-----------|--------------|---------------|
//! | Beauty    | Pleasing form | Ugliness      |
//! | Harmony   | Coherence     | Chaos         |
//! | Sublimity | Awe           | Banality      |
//! | Meaning   | Depth         | Emptiness     |
//! | Novelty   | Surprise      | Tedium        |
//!
//! # Integration
//!
//! - **Preference** — EMA learning with mere-exposure bias
//! - **Belief** — crystallization into world/self beliefs at threshold
//! - **Growth** — trait pressure from sustained aesthetic exposure
//! - **Mood** — aesthetic stimuli nudge mood dimensions
//! - **Intuition** — aesthetic sensitivity feeds signal synthesis

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::belief::{BeliefKind, BeliefSystem};
use crate::eq::EqProfile;
use crate::mood::MoodVector;
use crate::preference::{PreferenceBias, PreferenceStore};
use crate::traits::TraitKind;

/// Mere-exposure effect: aesthetic preferences decay at half the normal rate.
const AESTHETIC_DECAY_FACTOR: f32 = 0.5;

/// Positive bias multiplier for aesthetic exposure (familiarity → liking).
const AESTHETIC_POSITIVE_BIAS: f32 = 1.3;

/// Preference |valence| above which aesthetic preferences crystallize into beliefs.
const CRYSTALLIZATION_THRESHOLD: f32 = 0.6;

/// Minimum total exposure count before self-beliefs can form.
const SELF_BELIEF_EXPOSURE_THRESHOLD: u32 = 10;

/// Trait pressure coefficient per unit of preference valence.
const TRAIT_PRESSURE_COEFF: f32 = 0.15;

/// Number of aesthetic dimensions.
const N_DIMENSIONS: usize = 5;

// ---------------------------------------------------------------------------
// AestheticDimension
// ---------------------------------------------------------------------------

/// The five dimensions of aesthetic experience.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AestheticDimension {
    /// Form, visual/auditory pleasure — beauty vs ugliness.
    Beauty,
    /// Coherence, balance, proportion — harmony vs chaos.
    Harmony,
    /// Awe, vastness, transcendence — sublimity vs banality.
    Sublimity,
    /// Narrative significance, symbolic depth — meaning vs emptiness.
    Meaning,
    /// Surprise, originality, defamiliarization — novelty vs tedium.
    Novelty,
}

impl AestheticDimension {
    /// All dimensions.
    pub const ALL: &'static [AestheticDimension] = &[
        Self::Beauty,
        Self::Harmony,
        Self::Sublimity,
        Self::Meaning,
        Self::Novelty,
    ];

    /// Index into fixed-size arrays.
    #[must_use]
    #[inline]
    pub fn index(self) -> usize {
        match self {
            Self::Beauty => 0,
            Self::Harmony => 1,
            Self::Sublimity => 2,
            Self::Meaning => 3,
            Self::Novelty => 4,
        }
    }

    /// Preference tag prefix for this dimension.
    #[must_use]
    #[inline]
    fn tag_prefix(self) -> &'static str {
        match self {
            Self::Beauty => "aesthetic:beauty",
            Self::Harmony => "aesthetic:harmony",
            Self::Sublimity => "aesthetic:sublimity",
            Self::Meaning => "aesthetic:meaning",
            Self::Novelty => "aesthetic:novelty",
        }
    }

    /// World-belief tag for positive valence in this dimension.
    #[must_use]
    #[inline]
    fn positive_world_tag(self) -> &'static str {
        match self {
            Self::Beauty => "world:beautiful",
            Self::Harmony => "world:ordered",
            Self::Sublimity => "world:transcendent",
            Self::Meaning => "world:meaningful",
            Self::Novelty => "world:surprising",
        }
    }

    /// World-belief tag for negative valence in this dimension.
    #[must_use]
    #[inline]
    fn negative_world_tag(self) -> &'static str {
        match self {
            Self::Beauty => "world:ugly",
            Self::Harmony => "world:chaotic",
            Self::Sublimity => "world:banal",
            Self::Meaning => "world:empty",
            Self::Novelty => "world:tedious",
        }
    }
}

impl fmt::Display for AestheticDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Beauty => f.write_str("beauty"),
            Self::Harmony => f.write_str("harmony"),
            Self::Sublimity => f.write_str("sublimity"),
            Self::Meaning => f.write_str("meaning"),
            Self::Novelty => f.write_str("novelty"),
        }
    }
}

// ---------------------------------------------------------------------------
// AestheticExposure
// ---------------------------------------------------------------------------

/// A single aesthetic experience to record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticExposure {
    /// Which aesthetic dimension this experience touches.
    pub dimension: AestheticDimension,
    /// Specific stimulus tag (e.g., `"music:classical"`, `"art:impressionist"`).
    pub tag: String,
    /// Intensity of the experience: -1.0 (aversive) to 1.0 (deeply moving).
    pub intensity: f32,
}

// ---------------------------------------------------------------------------
// AestheticProfile
// ---------------------------------------------------------------------------

/// An entity's aesthetic sensibility — built from repeated exposure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticProfile {
    /// Per-dimension preference learning via EMA.
    preferences: PreferenceStore,
    /// Exposure count per dimension.
    exposure_counts: [u32; N_DIMENSIONS],
    /// Aesthetic sensitivity (0.0-1.0), derived from EQ perception + facilitation.
    sensitivity: f32,
}

impl Default for AestheticProfile {
    fn default() -> Self {
        Self {
            preferences: PreferenceStore::with_bias(
                64,
                PreferenceBias {
                    positive_gain: AESTHETIC_POSITIVE_BIAS,
                    negative_gain: 1.0,
                },
            ),
            exposure_counts: [0; N_DIMENSIONS],
            sensitivity: 0.5,
        }
    }
}

impl AestheticProfile {
    /// Create a new profile with default sensitivity.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with sensitivity derived from emotional intelligence.
    ///
    /// Perception branch detects beauty; facilitation branch enhances
    /// the emotional response to aesthetic stimuli.
    #[must_use]
    pub fn with_eq(eq: &EqProfile) -> Self {
        let sensitivity = ((eq.perception + eq.facilitation) / 2.0).clamp(0.0, 1.0);
        Self {
            sensitivity,
            ..Self::default()
        }
    }

    /// Record an aesthetic exposure.
    ///
    /// The experience intensity is scaled by sensitivity — entities with
    /// higher aesthetic sensitivity register stronger responses.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn record_exposure(&mut self, exposure: &AestheticExposure, now: DateTime<Utc>) {
        let scaled_intensity =
            (exposure.intensity * (0.5 + self.sensitivity * 0.5)).clamp(-1.0, 1.0);
        let tag = exposure.dimension.tag_prefix();
        self.preferences.record_outcome(tag, scaled_intensity, now);
        self.exposure_counts[exposure.dimension.index()] =
            self.exposure_counts[exposure.dimension.index()].saturating_add(1);
    }

    /// Current preference valence for a dimension (-1.0 to 1.0).
    #[must_use]
    #[inline]
    pub fn preference(&self, dimension: AestheticDimension) -> f32 {
        self.preferences
            .preference_for(dimension.tag_prefix())
            .unwrap_or(0.0)
    }

    /// Exposure count for a specific dimension.
    #[must_use]
    #[inline]
    pub fn exposure_count(&self, dimension: AestheticDimension) -> u32 {
        self.exposure_counts[dimension.index()]
    }

    /// Total aesthetic exposures across all dimensions.
    #[must_use]
    #[inline]
    pub fn total_exposure(&self) -> u32 {
        self.exposure_counts.iter().sum()
    }

    /// Current aesthetic sensitivity.
    #[must_use]
    #[inline]
    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }

    /// Decay aesthetic preferences toward neutral.
    ///
    /// Uses half the given rate (mere-exposure effect: aesthetic preferences
    /// are more durable than transient preferences).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decay(&mut self, rate: f32) {
        self.preferences.decay(rate * AESTHETIC_DECAY_FACTOR);
    }
}

// ---------------------------------------------------------------------------
// Belief crystallization
// ---------------------------------------------------------------------------

/// Crystallize aesthetic preferences into beliefs.
///
/// When a dimension's preference |valence| exceeds the crystallization
/// threshold (0.6), it is reinforced as a world belief. If total exposure
/// is high enough, self-beliefs also form (`"self:appreciative"`,
/// `"self:creative"`).
///
/// Returns the tags of beliefs that were created or reinforced.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn crystallize_beliefs(
    profile: &AestheticProfile,
    beliefs: &mut BeliefSystem,
    now: DateTime<Utc>,
) -> Vec<String> {
    let mut created = Vec::new();

    for &dim in AestheticDimension::ALL {
        let valence = profile.preference(dim);
        if valence.abs() < CRYSTALLIZATION_THRESHOLD {
            continue;
        }

        let tag = if valence > 0.0 {
            dim.positive_world_tag()
        } else {
            dim.negative_world_tag()
        };

        beliefs.reinforce_or_create(BeliefKind::WorldBelief, tag, valence, dim.tag_prefix(), now);
        created.push(tag.to_owned());
    }

    // Self-beliefs form when total exposure is high enough.
    if profile.total_exposure() >= SELF_BELIEF_EXPOSURE_THRESHOLD {
        let avg_positive: f32 = AestheticDimension::ALL
            .iter()
            .map(|d| profile.preference(*d).max(0.0))
            .sum::<f32>()
            / N_DIMENSIONS as f32;

        if avg_positive > 0.3 {
            beliefs.reinforce_or_create(
                BeliefKind::SelfBelief,
                "self:appreciative",
                avg_positive,
                "aesthetic_exposure",
                now,
            );
            created.push("self:appreciative".to_owned());
        }

        // Novelty + meaning exposure drives creative self-concept.
        let creative_signal = (profile.preference(AestheticDimension::Novelty).max(0.0)
            + profile.preference(AestheticDimension::Meaning).max(0.0))
            / 2.0;

        if creative_signal > 0.4 {
            beliefs.reinforce_or_create(
                BeliefKind::SelfBelief,
                "self:creative",
                creative_signal,
                "aesthetic_exposure",
                now,
            );
            created.push("self:creative".to_owned());
        }
    }

    created
}

// ---------------------------------------------------------------------------
// Trait pressure
// ---------------------------------------------------------------------------

/// Compute trait pressure from aesthetic preferences.
///
/// Sustained aesthetic exposure creates pressure on personality traits:
/// - Beauty + Harmony exposure -> Creativity pressure
/// - Novelty exposure -> Curiosity pressure
/// - Meaning + Sublimity exposure -> Empathy pressure
///
/// The output feeds into
/// [`GrowthLedger::apply_pressure()`](crate::growth::GrowthLedger::apply_pressure).
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn aesthetic_trait_pressure(profile: &AestheticProfile) -> [f32; TraitKind::COUNT] {
    let mut pressure = [0.0f32; TraitKind::COUNT];

    let beauty = profile.preference(AestheticDimension::Beauty).max(0.0);
    let harmony = profile.preference(AestheticDimension::Harmony).max(0.0);
    let sublimity = profile.preference(AestheticDimension::Sublimity).max(0.0);
    let meaning = profile.preference(AestheticDimension::Meaning).max(0.0);
    let novelty = profile.preference(AestheticDimension::Novelty).max(0.0);

    // Beauty + Harmony → Creativity
    pressure[TraitKind::Creativity.index()] += (beauty + harmony) * TRAIT_PRESSURE_COEFF;

    // Novelty → Curiosity
    pressure[TraitKind::Curiosity.index()] += novelty * TRAIT_PRESSURE_COEFF;

    // Meaning + Sublimity → Empathy
    pressure[TraitKind::Empathy.index()] += (meaning + sublimity) * TRAIT_PRESSURE_COEFF;

    pressure
}

// ---------------------------------------------------------------------------
// Mood effects
// ---------------------------------------------------------------------------

/// Compute the mood shift from an aesthetic exposure.
///
/// Maps aesthetic dimensions to existing emotion dimensions (no new categories):
/// - Beauty -> Joy (pleasure) — negative intensity decreases joy
/// - Novelty -> Interest (curiosity) — negative intensity decreases interest
/// - Harmony -> Trust (coherence/safety) — negative intensity decreases trust
/// - Sublimity -> Arousal (awe as high activation) — always activating regardless of sign
/// - Meaning -> Joy + Interest blend — negative intensity decreases both
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn aesthetic_mood_shift(exposure: &AestheticExposure, sensitivity: f32) -> MoodVector {
    let scale = exposure.intensity.clamp(-1.0, 1.0) * (0.5 + sensitivity * 0.5) * 0.3;
    let mut mood = MoodVector::default();

    match exposure.dimension {
        AestheticDimension::Beauty => {
            mood.joy = scale;
        }
        AestheticDimension::Novelty => {
            mood.interest = scale;
        }
        AestheticDimension::Harmony => {
            mood.trust = scale;
        }
        AestheticDimension::Sublimity => {
            mood.arousal = scale.abs(); // Awe is always activating
        }
        AestheticDimension::Meaning => {
            mood.joy = scale * 0.5;
            mood.interest = scale * 0.5;
        }
    }

    mood
}

// ---------------------------------------------------------------------------
// Intuition integration
// ---------------------------------------------------------------------------

/// Generate an intuition signal tag and strength from aesthetic sensitivity.
///
/// When an entity has strong aesthetic preferences, their aesthetic
/// sensitivity can surface as intuitive signals (e.g., "this music feels
/// meaningful"). Returns `None` if no aesthetic signal is strong enough.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn aesthetic_intuition_signal(profile: &AestheticProfile) -> Option<(String, f32)> {
    let mut strongest_dim = None;
    let mut strongest_val = 0.0f32;

    for &dim in AestheticDimension::ALL {
        let v = profile.preference(dim).abs();
        if v > strongest_val {
            strongest_val = v;
            strongest_dim = Some(dim);
        }
    }

    let dim = strongest_dim?;
    // Only surface as intuition if strong enough and entity is sensitive.
    let signal_strength = strongest_val * profile.sensitivity;
    if signal_strength < 0.2 {
        return None;
    }

    Some((dim.tag_prefix().to_owned(), signal_strength))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::belief::BeliefSystem;
    use chrono::Utc;

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn exposure(dim: AestheticDimension, intensity: f32) -> AestheticExposure {
        AestheticExposure {
            dimension: dim,
            tag: format!("test:{dim}"),
            intensity,
        }
    }

    // ---- AestheticDimension ----

    #[test]
    fn test_dimension_all_count() {
        assert_eq!(AestheticDimension::ALL.len(), N_DIMENSIONS);
    }

    #[test]
    fn test_dimension_indices_unique() {
        let mut seen = [false; N_DIMENSIONS];
        for &d in AestheticDimension::ALL {
            assert!(!seen[d.index()]);
            seen[d.index()] = true;
        }
    }

    #[test]
    fn test_dimension_display() {
        assert_eq!(AestheticDimension::Beauty.to_string(), "beauty");
        assert_eq!(AestheticDimension::Sublimity.to_string(), "sublimity");
    }

    #[test]
    fn test_dimension_serde() {
        for &d in AestheticDimension::ALL {
            let json = serde_json::to_string(&d).unwrap();
            let d2: AestheticDimension = serde_json::from_str(&json).unwrap();
            assert_eq!(d, d2);
        }
    }

    // ---- AestheticProfile ----

    #[test]
    fn test_new_profile_neutral() {
        let p = AestheticProfile::new();
        for &d in AestheticDimension::ALL {
            assert!((p.preference(d)).abs() < f32::EPSILON);
            assert_eq!(p.exposure_count(d), 0);
        }
        assert_eq!(p.total_exposure(), 0);
    }

    #[test]
    fn test_with_eq_sensitivity() {
        let eq = EqProfile {
            perception: 0.9,
            facilitation: 0.7,
            understanding: 0.5,
            management: 0.5,
        };
        let p = AestheticProfile::with_eq(&eq);
        assert!((p.sensitivity() - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_record_exposure_positive() {
        let mut p = AestheticProfile::new();
        p.record_exposure(&exposure(AestheticDimension::Beauty, 0.8), now());
        assert!(p.preference(AestheticDimension::Beauty) > 0.0);
        assert_eq!(p.exposure_count(AestheticDimension::Beauty), 1);
        assert_eq!(p.total_exposure(), 1);
    }

    #[test]
    fn test_record_exposure_negative() {
        let mut p = AestheticProfile::new();
        p.record_exposure(&exposure(AestheticDimension::Novelty, -0.6), now());
        assert!(p.preference(AestheticDimension::Novelty) < 0.0);
    }

    #[test]
    fn test_repeated_exposure_strengthens() {
        let mut p = AestheticProfile::new();
        for _ in 0..15 {
            p.record_exposure(&exposure(AestheticDimension::Harmony, 0.7), now());
        }
        let v = p.preference(AestheticDimension::Harmony);
        assert!(v > 0.4, "repeated positive exposure should strengthen: {v}");
    }

    #[test]
    fn test_sensitivity_scales_response() {
        let mut low = AestheticProfile::new();
        low.sensitivity = 0.1;
        let mut high = AestheticProfile::new();
        high.sensitivity = 0.9;

        low.record_exposure(&exposure(AestheticDimension::Beauty, 0.5), now());
        high.record_exposure(&exposure(AestheticDimension::Beauty, 0.5), now());

        assert!(
            high.preference(AestheticDimension::Beauty).abs()
                > low.preference(AestheticDimension::Beauty).abs(),
            "higher sensitivity should produce stronger response"
        );
    }

    #[test]
    fn test_decay_mere_exposure_effect() {
        let mut aesthetic = AestheticProfile::new();
        let mut plain = PreferenceStore::new(64);

        for _ in 0..10 {
            aesthetic.record_exposure(&exposure(AestheticDimension::Beauty, 0.8), now());
            plain.record_outcome("test", 0.8, now());
        }

        let before_aesthetic = aesthetic.preference(AestheticDimension::Beauty);
        let before_plain = plain.preference_for("test").unwrap();

        aesthetic.decay(0.3);
        plain.decay(0.3);

        let after_aesthetic = aesthetic.preference(AestheticDimension::Beauty);
        let after_plain = plain.preference_for("test").unwrap();

        let aesthetic_loss = (before_aesthetic - after_aesthetic).abs();
        let plain_loss = (before_plain - after_plain).abs();

        assert!(
            aesthetic_loss < plain_loss,
            "aesthetic should decay slower: aesthetic_loss={aesthetic_loss}, plain_loss={plain_loss}"
        );
    }

    #[test]
    fn test_profile_serde() {
        let mut p = AestheticProfile::new();
        p.record_exposure(&exposure(AestheticDimension::Meaning, 0.9), now());
        let json = serde_json::to_string(&p).unwrap();
        let p2: AestheticProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(p2.exposure_count(AestheticDimension::Meaning), 1);
    }

    // ---- Belief crystallization ----

    #[test]
    fn test_crystallize_below_threshold() {
        let mut p = AestheticProfile::new();
        p.record_exposure(&exposure(AestheticDimension::Beauty, 0.3), now());
        let mut bs = BeliefSystem::new(32);
        let tags = crystallize_beliefs(&p, &mut bs, now());
        assert!(tags.is_empty(), "weak preferences should not crystallize");
    }

    #[test]
    fn test_crystallize_positive_world_belief() {
        let mut p = AestheticProfile::new();
        for _ in 0..20 {
            p.record_exposure(&exposure(AestheticDimension::Beauty, 0.9), now());
        }
        let v = p.preference(AestheticDimension::Beauty);
        assert!(
            v > CRYSTALLIZATION_THRESHOLD,
            "should be above threshold: {v}"
        );

        let mut bs = BeliefSystem::new(32);
        let tags = crystallize_beliefs(&p, &mut bs, now());
        assert!(tags.contains(&"world:beautiful".to_owned()));
        assert!(bs.get("world:beautiful").is_some());
    }

    #[test]
    fn test_crystallize_negative_world_belief() {
        let mut p = AestheticProfile::new();
        for _ in 0..20 {
            p.record_exposure(&exposure(AestheticDimension::Harmony, -0.9), now());
        }

        let mut bs = BeliefSystem::new(32);
        let tags = crystallize_beliefs(&p, &mut bs, now());
        assert!(tags.contains(&"world:chaotic".to_owned()));
    }

    #[test]
    fn test_crystallize_self_beliefs_with_high_exposure() {
        let mut p = AestheticProfile::new();
        // Build up enough exposure for self-beliefs
        for _ in 0..15 {
            p.record_exposure(&exposure(AestheticDimension::Beauty, 0.8), now());
            p.record_exposure(&exposure(AestheticDimension::Meaning, 0.7), now());
            p.record_exposure(&exposure(AestheticDimension::Novelty, 0.6), now());
        }

        let mut bs = BeliefSystem::new(32);
        let tags = crystallize_beliefs(&p, &mut bs, now());
        assert!(
            tags.contains(&"self:appreciative".to_owned()),
            "high aesthetic exposure should form self:appreciative belief, tags: {tags:?}"
        );
    }

    // ---- Trait pressure ----

    #[test]
    fn test_trait_pressure_beauty_to_creativity() {
        let mut p = AestheticProfile::new();
        for _ in 0..15 {
            p.record_exposure(&exposure(AestheticDimension::Beauty, 0.9), now());
        }
        let pressure = aesthetic_trait_pressure(&p);
        assert!(
            pressure[TraitKind::Creativity.index()] > 0.0,
            "beauty should create creativity pressure"
        );
    }

    #[test]
    fn test_trait_pressure_novelty_to_curiosity() {
        let mut p = AestheticProfile::new();
        for _ in 0..15 {
            p.record_exposure(&exposure(AestheticDimension::Novelty, 0.8), now());
        }
        let pressure = aesthetic_trait_pressure(&p);
        assert!(
            pressure[TraitKind::Curiosity.index()] > 0.0,
            "novelty should create curiosity pressure"
        );
    }

    #[test]
    fn test_trait_pressure_meaning_to_empathy() {
        let mut p = AestheticProfile::new();
        for _ in 0..15 {
            p.record_exposure(&exposure(AestheticDimension::Meaning, 0.8), now());
        }
        let pressure = aesthetic_trait_pressure(&p);
        assert!(
            pressure[TraitKind::Empathy.index()] > 0.0,
            "meaning should create empathy pressure"
        );
    }

    #[test]
    fn test_trait_pressure_negative_no_effect() {
        let mut p = AestheticProfile::new();
        for _ in 0..15 {
            p.record_exposure(&exposure(AestheticDimension::Beauty, -0.9), now());
        }
        let pressure = aesthetic_trait_pressure(&p);
        assert!(
            pressure[TraitKind::Creativity.index()] <= 0.0,
            "negative beauty should not create creativity pressure"
        );
    }

    #[test]
    fn test_trait_pressure_neutral_zero() {
        let p = AestheticProfile::new();
        let pressure = aesthetic_trait_pressure(&p);
        for val in pressure {
            assert!(
                (val).abs() < f32::EPSILON,
                "neutral profile should have zero pressure"
            );
        }
    }

    // ---- Mood effects ----

    #[test]
    fn test_mood_shift_beauty_joy() {
        let shift = aesthetic_mood_shift(&exposure(AestheticDimension::Beauty, 0.8), 0.5);
        assert!(shift.joy > 0.0, "beauty should increase joy");
    }

    #[test]
    fn test_mood_shift_novelty_interest() {
        let shift = aesthetic_mood_shift(&exposure(AestheticDimension::Novelty, 0.7), 0.5);
        assert!(shift.interest > 0.0, "novelty should increase interest");
    }

    #[test]
    fn test_mood_shift_harmony_trust() {
        let shift = aesthetic_mood_shift(&exposure(AestheticDimension::Harmony, 0.6), 0.5);
        assert!(shift.trust > 0.0, "harmony should increase trust");
    }

    #[test]
    fn test_mood_shift_sublimity_arousal() {
        let shift = aesthetic_mood_shift(&exposure(AestheticDimension::Sublimity, 0.9), 0.5);
        assert!(
            shift.arousal > 0.0,
            "sublimity should increase arousal (awe)"
        );
    }

    #[test]
    fn test_mood_shift_meaning_blend() {
        let shift = aesthetic_mood_shift(&exposure(AestheticDimension::Meaning, 0.8), 0.5);
        assert!(shift.joy > 0.0, "meaning should nudge joy");
        assert!(shift.interest > 0.0, "meaning should nudge interest");
    }

    #[test]
    fn test_mood_shift_sensitivity_scales() {
        let low = aesthetic_mood_shift(&exposure(AestheticDimension::Beauty, 0.8), 0.1);
        let high = aesthetic_mood_shift(&exposure(AestheticDimension::Beauty, 0.8), 0.9);
        assert!(
            high.joy > low.joy,
            "higher sensitivity should produce stronger mood shift"
        );
    }

    #[test]
    fn test_mood_shift_negative_beauty() {
        let shift = aesthetic_mood_shift(&exposure(AestheticDimension::Beauty, -0.8), 0.5);
        assert!(shift.joy < 0.0, "negative beauty should decrease joy");
    }

    // ---- Intuition integration ----

    #[test]
    fn test_intuition_signal_none_when_neutral() {
        let p = AestheticProfile::new();
        assert!(aesthetic_intuition_signal(&p).is_none());
    }

    #[test]
    fn test_intuition_signal_strong_preference() {
        let mut p = AestheticProfile::new();
        p.sensitivity = 0.8;
        for _ in 0..20 {
            p.record_exposure(&exposure(AestheticDimension::Sublimity, 0.9), now());
        }
        let signal = aesthetic_intuition_signal(&p);
        assert!(
            signal.is_some(),
            "strong aesthetic preference should produce signal"
        );
        let (tag, strength) = signal.unwrap();
        assert!(tag.contains("sublimity"));
        assert!(strength > 0.2);
    }

    #[test]
    fn test_intuition_signal_low_sensitivity_filtered() {
        let mut p = AestheticProfile::new();
        p.sensitivity = 0.05;
        for _ in 0..5 {
            p.record_exposure(&exposure(AestheticDimension::Beauty, 0.5), now());
        }
        let signal = aesthetic_intuition_signal(&p);
        assert!(
            signal.is_none(),
            "low sensitivity should filter weak signals"
        );
    }

    #[test]
    fn test_intuition_signal_zero_sensitivity() {
        let mut p = AestheticProfile::new();
        p.sensitivity = 0.0;
        for _ in 0..20 {
            p.record_exposure(&exposure(AestheticDimension::Beauty, 0.9), now());
        }
        assert!(aesthetic_intuition_signal(&p).is_none());
    }

    #[test]
    fn test_crystallize_all_zero_valence() {
        let p = AestheticProfile::new(); // all preferences are 0.0
        let mut bs = BeliefSystem::new(32);
        let tags = crystallize_beliefs(&p, &mut bs, now());
        assert!(
            tags.is_empty(),
            "zero-valence preferences should not crystallize"
        );
    }

    #[test]
    fn test_mood_shift_sublimity_negative_still_activating() {
        let shift = aesthetic_mood_shift(&exposure(AestheticDimension::Sublimity, -0.8), 0.5);
        assert!(
            shift.arousal > 0.0,
            "negative sublimity should still activate arousal (awe is always activating)"
        );
    }

    // ---- Exposure serde ----

    #[test]
    fn test_exposure_serde() {
        let e = exposure(AestheticDimension::Meaning, 0.7);
        let json = serde_json::to_string(&e).unwrap();
        let e2: AestheticExposure = serde_json::from_str(&json).unwrap();
        assert_eq!(e2.dimension, AestheticDimension::Meaning);
        assert!((e2.intensity - 0.7).abs() < 0.001);
    }
}
