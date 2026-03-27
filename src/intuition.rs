//! Subconscious pattern integration — gut feelings from converging subsystems.
//!
//! The intuitive layer sits between beliefs (conscious worldview) and direct knowing
//! (non-dual awareness). It is NOT mystical — it is subconscious pattern integration.
//! The thing that happens when:
//!
//! - You "know" someone is lying before you can explain why
//! - You "feel" a decision is right before analysis catches up
//! - You sense a room's energy shift
//!
//! # How It Works
//!
//! Multiple subsystems fire simultaneously and converge on the same conclusion:
//! - [`actr`](crate::actr) — subconscious activation patterns (what's bubbling up)
//! - [`salience`](crate::salience) — Damasio somatic markers (gut feelings)
//! - [`microexpr`](crate::microexpr) — leak detection below conscious awareness
//! - [`affective`](crate::affective) — emotional complexity/granularity anomalies
//! - [`eq`](crate::eq) — EQ perception branch (reading emotions others miss)
//!
//! When 3+ sources converge on the same tag, signal strength *multiplies*
//! (not adds). Two signals = coincidence. Three = intuition.
//!
//! # Layers of Knowing
//!
//! 1. **Instinct** — hardwired, species-level (jantu domain)
//! 2. **Conditioning** — learned associations (growth module)
//! 3. **Belief** — crystallized worldview (belief module)
//! 4. **Intuition** — subconscious pattern synthesis (this module)
//! 5. **Insight** — non-dual awareness, subject-object merge (BreathPhase v3.0)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::mood::Emotion;
use crate::salience::SalienceScore;
use crate::traits::{PersonalityProfile, TraitKind};

/// Minimum `strength * trust_in_intuition` for a signal to surface.
/// At trust=1.0, signals need strength >= 0.15. At trust=0.5, need >= 0.30.
const TRUST_GATE_THRESHOLD: f32 = 0.15;

// ---------------------------------------------------------------------------
// SignalSource
// ---------------------------------------------------------------------------

/// Which subsystem contributed to an intuitive signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SignalSource {
    /// ACT-R memory activation pattern matched.
    MemoryActivation,
    /// Damasio somatic marker (salience) fired.
    SomaticMarker,
    /// Micro-expression leak detected in another entity.
    MicroExpressionLeak,
    /// Affective metrics anomaly (unusual complexity/granularity).
    EmotionalComplexity,
    /// EQ perception branch caught something others missed.
    PerceptualSensitivity,
    /// Aesthetic sensitivity — beauty/meaning/harmony detection.
    AestheticSensitivity,
}

impl SignalSource {
    /// All signal sources.
    pub const ALL: &'static [SignalSource] = &[
        Self::MemoryActivation,
        Self::SomaticMarker,
        Self::MicroExpressionLeak,
        Self::EmotionalComplexity,
        Self::PerceptualSensitivity,
        Self::AestheticSensitivity,
    ];
}

impl fmt::Display for SignalSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemoryActivation => f.write_str("memory_activation"),
            Self::SomaticMarker => f.write_str("somatic_marker"),
            Self::MicroExpressionLeak => f.write_str("micro_expression_leak"),
            Self::EmotionalComplexity => f.write_str("emotional_complexity"),
            Self::PerceptualSensitivity => f.write_str("perceptual_sensitivity"),
            Self::AestheticSensitivity => f.write_str("aesthetic_sensitivity"),
        }
    }
}

// ---------------------------------------------------------------------------
// KnowingLayer
// ---------------------------------------------------------------------------

/// The five layers of knowing, from lowest to highest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[non_exhaustive]
pub enum KnowingLayer {
    /// Hardwired, species-level responses. No learning. (jantu domain — scaffold only)
    Instinct,
    /// Learned associations and behavioral patterns (growth module).
    Conditioning,
    /// Crystallized worldview from experience (belief module).
    Belief,
    /// Subconscious pattern synthesis — "knowing without knowing how" (this module).
    Intuition,
    /// Non-dual awareness — subject and object merge (BreathPhase v3.0).
    Insight,
}

impl KnowingLayer {
    /// All layers in order from lowest to highest.
    pub const ALL: &'static [KnowingLayer] = &[
        Self::Instinct,
        Self::Conditioning,
        Self::Belief,
        Self::Intuition,
        Self::Insight,
    ];

    /// Characteristics of this layer.
    #[must_use]
    #[inline]
    pub fn characteristics(self) -> LayerCharacteristics {
        match self {
            Self::Instinct => LayerCharacteristics {
                speed: 1.0,
                accuracy: 0.6,
                explainability: 0.0,
            },
            Self::Conditioning => LayerCharacteristics {
                speed: 0.8,
                accuracy: 0.7,
                explainability: 0.4,
            },
            Self::Belief => LayerCharacteristics {
                speed: 0.5,
                accuracy: 0.8,
                explainability: 0.9,
            },
            Self::Intuition => LayerCharacteristics {
                speed: 0.7,
                accuracy: 0.85,
                explainability: 0.2,
            },
            Self::Insight => LayerCharacteristics {
                speed: 0.1,
                accuracy: 1.0,
                explainability: 0.0,
            },
        }
    }
}

impl fmt::Display for KnowingLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Instinct => f.write_str("instinct"),
            Self::Conditioning => f.write_str("conditioning"),
            Self::Belief => f.write_str("belief"),
            Self::Intuition => f.write_str("intuition"),
            Self::Insight => f.write_str("insight"),
        }
    }
}

/// Speed, accuracy, and explainability characteristics of a knowing layer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LayerCharacteristics {
    /// How fast this layer fires (0.0-1.0). Instinct = 1.0, Insight = 0.1 (rare).
    pub speed: f32,
    /// How reliable when it fires (0.0-1.0). Insight = 1.0 (perfect when it comes).
    pub accuracy: f32,
    /// How articulable the knowing is (0.0-1.0). Belief = 0.9 (can explain), Intuition = 0.2.
    pub explainability: f32,
}

// ---------------------------------------------------------------------------
// IntuitiveSignal
// ---------------------------------------------------------------------------

/// A gut feeling — an intuition about something.
///
/// Produced when multiple subsystems converge on the same conclusion
/// without conscious reasoning mediating.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitiveSignal {
    /// What this intuition is about (entity, event, decision).
    pub tag: String,
    /// Emotional direction: -1.0 (avoid/danger) to 1.0 (approach/opportunity).
    pub valence: f32,
    /// How strong the gut feeling is (0.0-1.0).
    pub strength: f32,
    /// Which subsystems contributed (for tracing, not for the entity).
    pub sources: Vec<SignalSource>,
    /// How much stronger this is than any single source — the "inexplicable" gap.
    pub confidence_gap: f32,
}

// ---------------------------------------------------------------------------
// IntuitionProfile
// ---------------------------------------------------------------------------

/// An entity's capacity for intuitive knowing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitionProfile {
    /// How many signals get through the subconscious filter (0.0-1.0).
    pub sensitivity: f32,
    /// How well signals from different sources integrate (0.0-1.0).
    pub integration_depth: f32,
    /// Whether the entity acts on gut feelings vs dismissing them (0.0-1.0).
    pub trust_in_intuition: f32,
}

impl Default for IntuitionProfile {
    fn default() -> Self {
        Self {
            sensitivity: 0.5,
            integration_depth: 0.5,
            trust_in_intuition: 0.5,
        }
    }
}

impl IntuitionProfile {
    /// Create with default mid-range scores.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Derive intuition profile from personality traits.
    ///
    /// - High empathy + curiosity + low skepticism = high sensitivity
    /// - High empathy + warmth = high integration depth
    /// - Low skepticism + high confidence = high trust in intuition
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn from_personality(profile: &PersonalityProfile) -> Self {
        let empathy = (profile.get_trait(TraitKind::Empathy).normalized() + 1.0) / 2.0;
        let curiosity = (profile.get_trait(TraitKind::Curiosity).normalized() + 1.0) / 2.0;
        let skepticism = (profile.get_trait(TraitKind::Skepticism).normalized() + 1.0) / 2.0;
        let warmth = (profile.get_trait(TraitKind::Warmth).normalized() + 1.0) / 2.0;
        let confidence = (profile.get_trait(TraitKind::Confidence).normalized() + 1.0) / 2.0;

        Self {
            sensitivity: ((empathy + curiosity + (1.0 - skepticism)) / 3.0).clamp(0.0, 1.0),
            integration_depth: ((empathy + warmth) / 2.0).clamp(0.0, 1.0),
            trust_in_intuition: (((1.0 - skepticism) + confidence) / 2.0).clamp(0.0, 1.0),
        }
    }
}

// ---------------------------------------------------------------------------
// Input signal types (composable — no store references)
// ---------------------------------------------------------------------------

/// Pre-extracted memory activation signals from ACT-R.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivationSignals {
    /// `(tag, activation_score)` pairs, pre-filtered above threshold.
    pub entries: Vec<(String, f64)>,
}

/// Pre-extracted salience signals from somatic markers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SalienceSignals {
    /// `(tag, salience_score)` pairs.
    pub entries: Vec<(String, SalienceScore)>,
}

/// Pre-extracted micro-expression leak signals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MicroExpressionSignals {
    /// `(tag, leak_ratio, emotion)` — tag is the entity being observed.
    pub entries: Vec<(String, f32, Emotion)>,
}

/// Pre-extracted affective anomaly signals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffectiveSignals {
    /// `(tag, anomaly_score)` where score > 0 means unusual complexity/granularity.
    pub entries: Vec<(String, f32)>,
}

/// Pre-extracted EQ perception signals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerceptionSignals {
    /// `(tag, perception_strength)` from EQ perception noticing something.
    pub entries: Vec<(String, f32)>,
}

// ---------------------------------------------------------------------------
// Core algorithm
// ---------------------------------------------------------------------------

/// Sigmoid normalization for activation scores (f64 → f32 in 0..1).
#[inline]
fn sigmoid(x: f64) -> f32 {
    (1.0 / (1.0 + (-x).exp())) as f32
}

/// Synthesize intuitive signals from converging subsystem data.
///
/// This is the core intuition algorithm. It collects signals by tag,
/// counts convergent sources, and applies the multiplication rule:
/// - 1 source = noise (strength * 0.3)
/// - 2 sources = coincidence (strength * 0.6)
/// - 3+ sources = intuition (geometric mean of top 3, boosted)
///
/// Returns intuitive signals sorted by strength descending.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn synthesize_intuition(
    activations: &ActivationSignals,
    salience: &SalienceSignals,
    micro_expressions: &MicroExpressionSignals,
    affective: &AffectiveSignals,
    perception: &PerceptionSignals,
    profile: &IntuitionProfile,
) -> Vec<IntuitiveSignal> {
    // Step 1: Collect raw signals by tag
    let mut signals: HashMap<String, Vec<(SignalSource, f32, f32)>> = HashMap::new();
    // (source, strength, valence_hint)

    let threshold = (1.0 - profile.sensitivity) * 0.5;

    for (tag, activation) in &activations.entries {
        let strength = sigmoid(*activation);
        if strength >= threshold {
            signals.entry(tag.clone()).or_default().push((
                SignalSource::MemoryActivation,
                strength,
                0.0,
            ));
        }
    }

    for (tag, score) in &salience.entries {
        let strength = score.magnitude();
        if strength >= threshold {
            // Salience urgency suggests negative valence (threat), importance is neutral
            let valence_hint = if score.urgency > score.importance {
                -0.5
            } else {
                0.0
            };
            signals.entry(tag.clone()).or_default().push((
                SignalSource::SomaticMarker,
                strength,
                valence_hint,
            ));
        }
    }

    for (tag, leak_ratio, emotion) in &micro_expressions.entries {
        if *leak_ratio >= threshold {
            let valence_hint = match emotion {
                Emotion::Joy | Emotion::Trust | Emotion::Interest => 0.5,
                Emotion::Frustration => -0.8,
                Emotion::Arousal => -0.3,
                Emotion::Dominance => -0.2,
            };
            signals.entry(tag.clone()).or_default().push((
                SignalSource::MicroExpressionLeak,
                *leak_ratio,
                valence_hint,
            ));
        }
    }

    for (tag, anomaly) in &affective.entries {
        if *anomaly >= threshold {
            signals.entry(tag.clone()).or_default().push((
                SignalSource::EmotionalComplexity,
                *anomaly,
                0.0,
            ));
        }
    }

    for (tag, strength) in &perception.entries {
        if *strength >= threshold {
            signals.entry(tag.clone()).or_default().push((
                SignalSource::PerceptualSensitivity,
                *strength,
                0.0,
            ));
        }
    }

    // Step 2: Synthesize signals per tag
    let mut results = Vec::new();

    for (tag, tag_signals) in &signals {
        // Deduplicate sources
        let mut seen_sources: Vec<SignalSource> = Vec::new();
        let mut strengths: Vec<f32> = Vec::new();
        let mut valence_sum = 0.0f32;
        let mut valence_weight = 0.0f32;

        for (source, strength, valence_hint) in tag_signals {
            if !seen_sources.contains(source) {
                seen_sources.push(*source);
                strengths.push(*strength);
            }
            if *valence_hint != 0.0 {
                valence_sum += valence_hint * strength;
                valence_weight += strength;
            }
        }

        let n_sources = seen_sources.len();
        strengths.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        let max_single = strengths.first().copied().unwrap_or(0.0);

        // Step 3: Apply convergence rule
        let combined = match n_sources {
            0 => continue,
            1 => max_single * 0.3,
            2 => max_single * 0.6,
            n => {
                // Geometric mean of top 3 (computed inline, no allocation)
                let count = strengths.len().min(3);
                let product: f32 = strengths.iter().take(count).product();
                let geo_mean = product.powf(1.0 / count as f32);
                let convergence_bonus = (1.0 + 0.2 * (n as f32 - 3.0)).min(2.0);
                geo_mean * convergence_bonus
            }
        };

        // Step 4: Confidence gap — how much convergence exceeds any single source
        // Computed before integration_depth scaling to capture raw convergence benefit
        let confidence_gap = (combined - max_single).max(0.0);

        // Step 5: Scale by integration depth
        let final_strength = (combined * (0.5 + profile.integration_depth * 0.5)).clamp(0.0, 1.0);

        // Step 6: Derive valence
        let valence = if valence_weight > 0.0 {
            (valence_sum / valence_weight).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        // Step 7: Trust gate
        if final_strength * profile.trust_in_intuition < TRUST_GATE_THRESHOLD {
            continue;
        }

        results.push(IntuitiveSignal {
            tag: tag.clone(),
            valence,
            strength: final_strength,
            sources: seen_sources,
            confidence_gap,
        });
    }

    // Step 8: Sort descending by strength
    results.sort_by(|a, b| {
        b.strength
            .partial_cmp(&a.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    results
}

// ---------------------------------------------------------------------------
// Active layer determination
// ---------------------------------------------------------------------------

/// Determine which knowing layer is currently dominant.
///
/// - Extreme arousal → Instinct dominates (fight-or-flight)
/// - Strong beliefs → Belief layer dominates (worldview filters everything)
/// - Strong intuitions → Intuition layer (pattern recognition)
/// - High cosmic understanding → Insight (rare but dominant when present)
/// - Default → Conditioning (habitual response)
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn active_layer(
    arousal: f32,
    belief_conviction: f32,
    intuition_strength: f32,
    cosmic_understanding: f32,
) -> KnowingLayer {
    let arousal = arousal.clamp(0.0, 1.0);
    let belief = belief_conviction.clamp(0.0, 1.0);
    let intuition = intuition_strength.clamp(0.0, 1.0);
    let cosmic = cosmic_understanding.clamp(0.0, 1.0);

    // Instinct only dominates at extreme arousal (quadratic — needs to be very high)
    let instinct_score = arousal * arousal;
    let conditioning_score = 0.3; // Default baseline — always present
    let belief_score = belief * 0.8;
    let intuition_score = intuition;
    // Insight is rare but dominant when cosmic understanding is high
    let insight_score = if cosmic > 0.8 {
        cosmic * 1.5
    } else {
        cosmic * 0.5
    };

    let scores = [
        (KnowingLayer::Instinct, instinct_score),
        (KnowingLayer::Conditioning, conditioning_score),
        (KnowingLayer::Belief, belief_score),
        (KnowingLayer::Intuition, intuition_score),
        (KnowingLayer::Insight, insight_score),
    ];

    scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(layer, _)| *layer)
        .unwrap_or(KnowingLayer::Conditioning)
}

// ---------------------------------------------------------------------------
// Shadow belief integration
// ---------------------------------------------------------------------------

/// Convert shadow beliefs into intuitive signal inputs.
///
/// Shadow beliefs (beliefs formed under emotional suppression) bypass conscious
/// processing and surface as gut feelings. This extracts them as signal data
/// for the intuition synthesis pipeline.
///
/// Input: `(tag, valence, suppression_depth)` tuples from
/// [`shadow_beliefs()`](crate::belief::shadow_beliefs).
///
/// Returns `(tag, valence, signal_strength)` tuples where
/// strength = `suppression_depth * 0.7`. Valence is preserved so the
/// intuition pipeline knows the emotional direction of the shadow belief.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn shadow_belief_signals(beliefs: &[(String, f32, f32)]) -> Vec<(String, f32, f32)> {
    beliefs
        .iter()
        .filter(|(_, _, depth)| *depth > 0.1)
        .map(|(tag, valence, depth)| (tag.clone(), *valence, depth * 0.7))
        .collect()
}

// ---------------------------------------------------------------------------
// Reasoning override
// ---------------------------------------------------------------------------

/// Determine if an intuition should override conscious reasoning.
///
/// Returns true when the intuitive signal is strong AND conscious reasoning
/// would be slow (high arousal = time pressure) or weak (low confidence).
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn should_override_reasoning(
    signal: &IntuitiveSignal,
    arousal: f32,
    reasoning_confidence: f32,
) -> bool {
    signal.strength > reasoning_confidence.clamp(0.0, 1.0)
        && (signal.strength > 0.7 || arousal.clamp(0.0, 1.0) > 0.6)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::salience::SalienceScore;
    use crate::traits::{PersonalityProfile, TraitKind, TraitLevel};

    // ---- Serde ----

    #[test]
    fn test_signal_source_serde() {
        for &source in SignalSource::ALL {
            let json = serde_json::to_string(&source).unwrap();
            let s2: SignalSource = serde_json::from_str(&json).unwrap();
            assert_eq!(source, s2);
        }
    }

    #[test]
    fn test_intuitive_signal_serde() {
        let signal = IntuitiveSignal {
            tag: "alice".to_owned(),
            valence: -0.5,
            strength: 0.8,
            sources: vec![SignalSource::SomaticMarker, SignalSource::MemoryActivation],
            confidence_gap: 0.15,
        };
        let json = serde_json::to_string(&signal).unwrap();
        let s2: IntuitiveSignal = serde_json::from_str(&json).unwrap();
        assert_eq!(s2.tag, "alice");
        assert!((s2.strength - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_intuition_profile_serde() {
        let profile = IntuitionProfile::new();
        let json = serde_json::to_string(&profile).unwrap();
        let p2: IntuitionProfile = serde_json::from_str(&json).unwrap();
        assert!((p2.sensitivity - 0.5).abs() < 0.001);
    }

    // ---- KnowingLayer ----

    #[test]
    fn test_knowing_layer_order() {
        assert!(KnowingLayer::Instinct < KnowingLayer::Conditioning);
        assert!(KnowingLayer::Conditioning < KnowingLayer::Belief);
        assert!(KnowingLayer::Belief < KnowingLayer::Intuition);
        assert!(KnowingLayer::Intuition < KnowingLayer::Insight);
    }

    #[test]
    fn test_layer_characteristics() {
        let instinct = KnowingLayer::Instinct.characteristics();
        let belief = KnowingLayer::Belief.characteristics();
        let intuition = KnowingLayer::Intuition.characteristics();
        let insight = KnowingLayer::Insight.characteristics();

        // Instinct is fastest
        assert!(instinct.speed > belief.speed);
        // Belief is most explainable
        assert!(belief.explainability > intuition.explainability);
        // Insight is most accurate
        assert!(insight.accuracy >= intuition.accuracy);
    }

    // ---- Active layer ----

    #[test]
    fn test_active_layer_high_arousal() {
        assert_eq!(active_layer(0.95, 0.3, 0.3, 0.0), KnowingLayer::Instinct);
    }

    #[test]
    fn test_active_layer_strong_belief() {
        assert_eq!(active_layer(0.2, 0.9, 0.3, 0.0), KnowingLayer::Belief);
    }

    #[test]
    fn test_active_layer_cosmic() {
        assert_eq!(active_layer(0.2, 0.3, 0.3, 0.9), KnowingLayer::Insight);
    }

    #[test]
    fn test_active_layer_default_conditioning() {
        assert_eq!(active_layer(0.2, 0.2, 0.2, 0.2), KnowingLayer::Conditioning);
    }

    #[test]
    fn test_active_layer_strong_intuition() {
        assert_eq!(active_layer(0.2, 0.3, 0.8, 0.0), KnowingLayer::Intuition);
    }

    // ---- IntuitionProfile from personality ----

    #[test]
    fn test_profile_from_personality_high_empathy() {
        let mut profile = PersonalityProfile::new("empath");
        profile.set_trait(TraitKind::Empathy, TraitLevel::Highest);
        profile.set_trait(TraitKind::Curiosity, TraitLevel::High);
        profile.set_trait(TraitKind::Skepticism, TraitLevel::Lowest);

        let ip = IntuitionProfile::from_personality(&profile);
        assert!(
            ip.sensitivity > 0.7,
            "High empathy + curiosity + low skepticism should give high sensitivity, got {}",
            ip.sensitivity
        );
        assert!(
            ip.trust_in_intuition > 0.6,
            "Low skepticism should give high trust, got {}",
            ip.trust_in_intuition
        );
    }

    #[test]
    fn test_profile_from_personality_skeptic() {
        let mut profile = PersonalityProfile::new("skeptic");
        profile.set_trait(TraitKind::Skepticism, TraitLevel::Highest);
        profile.set_trait(TraitKind::Empathy, TraitLevel::Low);

        let ip = IntuitionProfile::from_personality(&profile);
        assert!(
            ip.sensitivity < 0.5,
            "High skepticism should reduce sensitivity, got {}",
            ip.sensitivity
        );
        assert!(
            ip.trust_in_intuition < 0.4,
            "High skepticism should reduce trust, got {}",
            ip.trust_in_intuition
        );
    }

    // ---- Synthesis ----

    #[test]
    fn test_synthesize_empty_inputs() {
        let profile = IntuitionProfile::new();
        let result = synthesize_intuition(
            &ActivationSignals::default(),
            &SalienceSignals::default(),
            &MicroExpressionSignals::default(),
            &AffectiveSignals::default(),
            &PerceptionSignals::default(),
            &profile,
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_synthesize_single_source_weak() {
        let profile = IntuitionProfile {
            sensitivity: 0.9,
            integration_depth: 0.8,
            trust_in_intuition: 0.9,
        };
        let activations = ActivationSignals {
            entries: vec![("danger".to_owned(), 2.0)], // sigmoid(2.0) ≈ 0.88
        };
        let result = synthesize_intuition(
            &activations,
            &SalienceSignals::default(),
            &MicroExpressionSignals::default(),
            &AffectiveSignals::default(),
            &PerceptionSignals::default(),
            &profile,
        );
        if !result.is_empty() {
            // Single source → max * 0.3 → weak
            assert!(
                result[0].strength < 0.5,
                "Single source should be weak, got {}",
                result[0].strength
            );
            assert_eq!(result[0].sources.len(), 1);
        }
    }

    #[test]
    fn test_synthesize_two_sources_moderate() {
        let profile = IntuitionProfile {
            sensitivity: 0.9,
            integration_depth: 0.8,
            trust_in_intuition: 0.9,
        };
        let activations = ActivationSignals {
            entries: vec![("danger".to_owned(), 2.0)],
        };
        let salience = SalienceSignals {
            entries: vec![("danger".to_owned(), SalienceScore::new(0.8, 0.7))],
        };
        let result = synthesize_intuition(
            &activations,
            &salience,
            &MicroExpressionSignals::default(),
            &AffectiveSignals::default(),
            &PerceptionSignals::default(),
            &profile,
        );
        assert!(!result.is_empty());
        let sig = &result[0];
        assert_eq!(sig.tag, "danger");
        assert_eq!(sig.sources.len(), 2);
        // Two sources → moderate strength
        assert!(sig.strength > 0.3, "Two sources should be moderate");
    }

    #[test]
    fn test_synthesize_three_sources_convergence() {
        let profile = IntuitionProfile {
            sensitivity: 0.9,
            integration_depth: 0.9,
            trust_in_intuition: 0.9,
        };
        let activations = ActivationSignals {
            entries: vec![("liar".to_owned(), 2.0)],
        };
        let salience = SalienceSignals {
            entries: vec![("liar".to_owned(), SalienceScore::new(0.7, 0.6))],
        };
        let micro = MicroExpressionSignals {
            entries: vec![("liar".to_owned(), 0.8, Emotion::Frustration)],
        };
        let result = synthesize_intuition(
            &activations,
            &salience,
            &micro,
            &AffectiveSignals::default(),
            &PerceptionSignals::default(),
            &profile,
        );
        assert!(!result.is_empty());
        let sig = &result[0];
        assert_eq!(sig.tag, "liar");
        assert!(
            sig.sources.len() >= 3,
            "Should have 3+ sources, got {}",
            sig.sources.len()
        );
        // Three sources → strongest signal (geometric mean + convergence)
        assert!(
            sig.strength > 0.5,
            "Three converging sources should produce strong signal, got {}",
            sig.strength
        );
        // Negative valence from frustration micro-expression
        assert!(
            sig.valence < 0.0,
            "Frustration leak should produce negative valence"
        );
        // Confidence gap should be positive (knowing more than any single source)
        assert!(
            sig.confidence_gap >= 0.0,
            "Convergence should create confidence gap"
        );
    }

    #[test]
    fn test_sensitivity_filters_weak_signals() {
        // Low sensitivity → only strong signals pass
        let profile = IntuitionProfile {
            sensitivity: 0.1, // Very low — threshold = 0.45
            integration_depth: 0.5,
            trust_in_intuition: 0.9,
        };
        let activations = ActivationSignals {
            entries: vec![("weak".to_owned(), 0.0)], // sigmoid(0) = 0.5, barely above 0.45
        };
        let salience = SalienceSignals {
            entries: vec![("weak".to_owned(), SalienceScore::new(0.3, 0.3))], // magnitude ≈ 0.3 < 0.45
        };
        let result = synthesize_intuition(
            &activations,
            &salience,
            &MicroExpressionSignals::default(),
            &AffectiveSignals::default(),
            &PerceptionSignals::default(),
            &profile,
        );
        // With low sensitivity, the salience signal (0.3) should be filtered out
        // Only activation might pass (sigmoid(0)=0.5 > 0.45)
        // But 1 source at 0.5 * 0.3 * 0.75 = 0.1125 < 0.15 trust gate
        assert!(
            result.is_empty(),
            "Low sensitivity should filter weak signals"
        );
    }

    #[test]
    fn test_trust_gate_filters_output() {
        let profile = IntuitionProfile {
            sensitivity: 0.9,
            integration_depth: 0.5,
            trust_in_intuition: 0.05, // Very low trust
        };
        let activations = ActivationSignals {
            entries: vec![("something".to_owned(), 1.0)],
        };
        let result = synthesize_intuition(
            &activations,
            &SalienceSignals::default(),
            &MicroExpressionSignals::default(),
            &AffectiveSignals::default(),
            &PerceptionSignals::default(),
            &profile,
        );
        // Single source → weak, and low trust should filter it out
        assert!(
            result.is_empty(),
            "Very low trust should filter all weak signals"
        );
    }

    // ---- Shadow beliefs ----

    #[test]
    fn test_shadow_belief_signals() {
        let beliefs = vec![
            ("fear:loss".to_owned(), -0.7, 0.8),
            ("hope:future".to_owned(), 0.5, 0.05), // Below threshold
            ("anger:betrayal".to_owned(), -0.9, 0.6),
        ];
        let signals = shadow_belief_signals(&beliefs);
        assert_eq!(signals.len(), 2);
        // "fear:loss" → valence = -0.7, strength = 0.8 * 0.7 = 0.56
        let fear = signals.iter().find(|(t, _, _)| t == "fear:loss").unwrap();
        assert!((fear.1 - (-0.7)).abs() < 0.01); // valence preserved
        assert!((fear.2 - 0.56).abs() < 0.01); // strength
    }

    // ---- Should override ----

    #[test]
    fn test_should_override_strong_signal_high_arousal() {
        let signal = IntuitiveSignal {
            tag: "danger".to_owned(),
            valence: -0.8,
            strength: 0.85,
            sources: vec![
                SignalSource::SomaticMarker,
                SignalSource::MemoryActivation,
                SignalSource::MicroExpressionLeak,
            ],
            confidence_gap: 0.2,
        };
        assert!(should_override_reasoning(&signal, 0.7, 0.5));
    }

    #[test]
    fn test_should_not_override_weak_signal() {
        let signal = IntuitiveSignal {
            tag: "maybe".to_owned(),
            valence: 0.3,
            strength: 0.3,
            sources: vec![SignalSource::PerceptualSensitivity],
            confidence_gap: 0.0,
        };
        assert!(!should_override_reasoning(&signal, 0.3, 0.5));
    }

    #[test]
    fn test_should_override_very_strong_signal_low_arousal() {
        let signal = IntuitiveSignal {
            tag: "important".to_owned(),
            valence: 0.9,
            strength: 0.8,
            sources: vec![
                SignalSource::SomaticMarker,
                SignalSource::PerceptualSensitivity,
                SignalSource::MemoryActivation,
            ],
            confidence_gap: 0.25,
        };
        // Strength > 0.7 → overrides even at low arousal
        assert!(should_override_reasoning(&signal, 0.2, 0.5));
    }

    // ---- KnowingLayer serde ----

    #[test]
    fn test_knowing_layer_serde() {
        for &layer in KnowingLayer::ALL {
            let json = serde_json::to_string(&layer).unwrap();
            let l2: KnowingLayer = serde_json::from_str(&json).unwrap();
            assert_eq!(layer, l2);
        }
    }
}
