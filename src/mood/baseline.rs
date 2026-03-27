use serde::{Deserialize, Serialize};

use super::types::{Emotion, MoodVector};

// --- Emotion Amplifier ---

/// Compute a stimulus amplification factor from personality.
///
/// Returns a multiplier (0.5–2.0) that should be applied to incoming
/// emotional stimuli before they affect the mood vector.
/// High neuroticism amplifies negative stimuli; high agreeableness amplifies social stimuli.
#[cfg(feature = "traits")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn emotion_amplifier(
    profile: &crate::traits::PersonalityProfile,
    emotion: Emotion,
    stimulus_valence: f32,
) -> f32 {
    use crate::traits::TraitKind;
    let patience = profile.get_trait(TraitKind::Patience).normalized();
    let confidence = profile.get_trait(TraitKind::Confidence).normalized();
    let empathy = profile.get_trait(TraitKind::Empathy).normalized();
    let skepticism = profile.get_trait(TraitKind::Skepticism).normalized();

    let base = 1.0;
    let modifier = match emotion {
        // Negative stimuli amplified by low patience/confidence (neuroticism proxy)
        Emotion::Frustration | Emotion::Arousal if stimulus_valence < 0.0 => {
            -patience * 0.3 - confidence * 0.2 + skepticism * 0.1
        }
        // Trust stimuli amplified by empathy, dampened by skepticism
        Emotion::Trust => empathy * 0.3 - skepticism * 0.2,
        // Joy amplified by empathy for positive, patience for negative
        Emotion::Joy if stimulus_valence > 0.0 => empathy * 0.2,
        Emotion::Joy => -patience * 0.2 - confidence * 0.1,
        // Interest amplified by curiosity
        Emotion::Interest => {
            let curiosity = profile.get_trait(TraitKind::Curiosity).normalized();
            curiosity * 0.3
        }
        _ => 0.0,
    };

    (base + modifier).clamp(0.5, 2.0)
}

// --- Mood Influence on Traits (v0.3) ---

/// Compute a trait-level modifier based on current mood.
///
/// Returns a value from -1.0 to 1.0 that can be used to adjust trait expression.
/// For example, high frustration amplifies directness; high joy softens formality.
#[cfg(feature = "traits")]
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
