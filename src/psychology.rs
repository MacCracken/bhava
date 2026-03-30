//! Bodh psychology math integration — validated computational psychology backing bhava's emotion systems.
//!
//! Provides bridge functions between bodh's empirically-validated psychology formulas
//! and bhava's personality/emotion types. Bodh provides the mathematical foundations
//! (circumplex affect, Scherer appraisal, ACT-R memory, Gross regulation, Yerkes-Dodson);
//! bhava provides the personality engine that composes them.
//!
//! Requires the `psychology` feature.
//!
//! # Layer Model
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Bhava (Personality Engine)      │
//! │  Traits, mood, beliefs, growth   │
//! ├──────────────────────────────────┤
//! │  This module (bridge)            │
//! │  Psychology math → Bhava types   │
//! ├──────────────────────────────────┤
//! │  Bodh (Psychology Math)          │
//! │  Affect, memory, cognition       │
//! └──────────────────────────────────┘
//! ```
//!
//! # Bridge Functions
//!
//! ## Affect / Mood Conversion
//! - [`affect_from_mood`] — bhava MoodVector → bodh Affect (circumplex)
//! - [`mood_from_affect`] — bodh Affect → bhava MoodVector
//! - [`classify_mood`] — MoodVector → Ekman basic emotion via circumplex
//!
//! ## Appraisal
//! - [`appraisal_to_scherer`] — bhava OCC appraisal → Scherer SEC dimensions
//! - [`affect_from_appraisal`] — OCC appraisal → Affect via Scherer pipeline
//!
//! ## Regulation
//! - [`regulation_effectiveness`] — Gross meta-analytic effectiveness for bhava strategies
//!
//! ## Psychometrics
//! - [`ocean_from_big_five`] — Big Five scores → bhava OceanScores
//! - [`trait_reliability`] — Cronbach's alpha for personality measurements
//!
//! ## ACT-R Memory
//! - [`base_level_activation`] — validated ACT-R base-level formula
//! - [`retrieval_probability`] — softmax retrieval gate
//!
//! ## Cognition
//! - [`cognitive_load`] — task load / working memory capacity
//! - [`yerkes_dodson_performance`] — arousal-performance inverted-U curve
//!
//! ## Social Psychology
//! - [`mood_congruent_bias`] — mood-congruent memory retrieval bias
//! - [`attribution_type`] — Kelley attribution model

use crate::mood::MoodVector;

// ── Affect / Mood Conversion ───────────────────────────────────────────

/// Convert a bhava MoodVector to a bodh circumplex Affect.
///
/// Maps bhava's 6-dimensional PAD-extended mood to bodh's 2D valence × arousal
/// circumplex. Valence is derived from (joy − frustration + trust) / 3;
/// arousal maps directly.
///
/// ```
/// use bhava::mood::MoodVector;
/// use bhava::psychology::affect_from_mood;
///
/// let mood = MoodVector { joy: 0.8, arousal: 0.6, dominance: 0.3,
///     trust: 0.5, interest: 0.4, frustration: 0.1 };
/// let affect = affect_from_mood(&mood);
/// assert!(affect.valence > 0.0);
/// assert!((affect.arousal - 0.6).abs() < 0.01);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn affect_from_mood(mood: &MoodVector) -> bodh::emotion::Affect {
    let valence = f64::from((mood.joy - mood.frustration + mood.trust) / 3.0).clamp(-1.0, 1.0);
    let arousal = f64::from(mood.arousal).clamp(-1.0, 1.0);
    bodh::emotion::Affect::new(valence, arousal).unwrap_or(bodh::emotion::Affect {
        valence: 0.0,
        arousal: 0.0,
    })
}

/// Convert a bodh circumplex Affect to a bhava MoodVector.
///
/// Positive valence maps to joy; negative valence maps to frustration.
/// Arousal maps directly. Dominance, trust, and interest default to 0.
///
/// ```
/// use bhava::psychology::mood_from_affect;
///
/// let affect = bodh::emotion::Affect { valence: 0.7, arousal: 0.3 };
/// let mood = mood_from_affect(affect);
/// assert!(mood.joy > 0.0);
/// assert!(mood.frustration == 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn mood_from_affect(affect: bodh::emotion::Affect) -> MoodVector {
    let v = affect.valence as f32;
    MoodVector {
        joy: v.max(0.0),
        arousal: (affect.arousal as f32).clamp(-1.0, 1.0),
        dominance: 0.0,
        trust: 0.0,
        interest: 0.0,
        frustration: (-v).max(0.0),
    }
}

/// Classify a bhava MoodVector into an Ekman basic emotion via the circumplex.
///
/// Converts the mood to a bodh Affect, then finds the nearest canonical
/// emotion position (Happiness, Sadness, Anger, Fear, Disgust, Surprise).
///
/// ```
/// use bhava::mood::MoodVector;
/// use bhava::psychology::classify_mood;
///
/// let happy = MoodVector { joy: 0.9, arousal: 0.3, dominance: 0.2,
///     trust: 0.5, interest: 0.3, frustration: 0.0 };
/// let emotion = classify_mood(&happy);
/// assert!(matches!(emotion, bodh::emotion::BasicEmotion::Happiness));
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn classify_mood(mood: &MoodVector) -> bodh::emotion::BasicEmotion {
    let affect = affect_from_mood(mood);
    bodh::emotion::classify_emotion(affect)
}

// ── Appraisal ──────────────────────────────────────────────────────────

/// Convert a bhava OCC appraisal to Scherer's stimulus evaluation check dimensions.
///
/// Maps: desirability → goal conduciveness, praiseworthiness → norm compatibility,
/// (1 − likelihood) → novelty, is_self → coping potential.
///
/// ```
/// use bhava::appraisal::Appraisal;
/// use bhava::psychology::appraisal_to_scherer;
///
/// let appraisal = Appraisal::event("test event", 0.8)
///     .with_praise(0.5)
///     .with_likelihood(0.7);
/// let dims = appraisal_to_scherer(&appraisal);
/// assert!(dims.goal_conduciveness > 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn appraisal_to_scherer(
    appraisal: &crate::appraisal::Appraisal,
) -> bodh::emotion::AppraisalDimensions {
    bodh::emotion::AppraisalDimensions {
        novelty: f64::from(1.0 - appraisal.likelihood).clamp(-1.0, 1.0),
        pleasantness: f64::from(appraisal.desirability).clamp(-1.0, 1.0),
        goal_conduciveness: f64::from(appraisal.desirability).clamp(-1.0, 1.0),
        coping_potential: if appraisal.is_self { 0.5 } else { -0.2 },
        norm_compatibility: f64::from(appraisal.praiseworthiness).clamp(-1.0, 1.0),
    }
}

/// Convert a bhava OCC appraisal to a bodh Affect via the Scherer pipeline.
///
/// Full pipeline: bhava Appraisal → Scherer SEC dimensions → bodh Affect.
/// Falls back to neutral affect (0, 0) if Scherer appraisal fails.
///
/// ```
/// use bhava::appraisal::Appraisal;
/// use bhava::psychology::affect_from_appraisal;
///
/// let appraisal = Appraisal::event("good news", 0.9);
/// let affect = affect_from_appraisal(&appraisal);
/// assert!(affect.valence > 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn affect_from_appraisal(appraisal: &crate::appraisal::Appraisal) -> bodh::emotion::Affect {
    let dims = appraisal_to_scherer(appraisal);
    bodh::emotion::appraise(&dims).unwrap_or(bodh::emotion::Affect {
        valence: 0.0,
        arousal: 0.0,
    })
}

// ── Regulation ─────────────────────────────────────────────────────────

/// Return the Gross meta-analytic effectiveness coefficient for a bhava regulation strategy.
///
/// Maps bhava's strategy variants to bodh's empirically-derived effectiveness:
/// - Accept → 1.0 (no regulation, full felt emotion)
/// - Suppress → ResponseModulation effectiveness (0.30)
/// - Reappraise → CognitiveChange effectiveness (0.85)
/// - Distract → AttentionalDeployment effectiveness (0.45)
///
/// ```
/// use bhava::regulation::RegulationStrategy;
/// use bhava::psychology::regulation_effectiveness;
///
/// let suppress = RegulationStrategy::Suppress {
///     target: bhava::mood::Emotion::Frustration,
///     strength: 0.8,
/// };
/// let eff = regulation_effectiveness(&suppress);
/// assert!((eff - 0.30).abs() < 0.01);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn regulation_effectiveness(strategy: &crate::regulation::RegulationStrategy) -> f64 {
    use crate::regulation::RegulationStrategy;
    #[allow(unreachable_patterns)]
    match strategy {
        RegulationStrategy::Accept => 1.0,
        RegulationStrategy::Suppress { .. } => {
            bodh::emotion::RegulationStrategy::ResponseModulation.effectiveness()
        }
        RegulationStrategy::Reappraise { .. } => {
            bodh::emotion::RegulationStrategy::CognitiveChange.effectiveness()
        }
        RegulationStrategy::Distract { .. } => {
            bodh::emotion::RegulationStrategy::AttentionalDeployment.effectiveness()
        }
        _ => 0.5,
    }
}

// ── Psychometrics ──────────────────────────────────────────────────────

/// Convert raw Big Five dimension scores to bhava OceanScores.
///
/// Input array order: [Openness, Conscientiousness, Extraversion, Agreeableness, Neuroticism].
/// Values are clamped to [-1.0, 1.0].
///
/// ```
/// use bhava::psychology::ocean_from_big_five;
///
/// let scores = [0.7_f32, 0.5, 0.8, 0.6, 0.3];
/// let ocean = ocean_from_big_five(&scores);
/// assert!((ocean.openness - 0.7).abs() < 0.01);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn ocean_from_big_five(scores: &[f32; 5]) -> crate::traits::OceanScores {
    crate::traits::OceanScores {
        openness: scores[0].clamp(-1.0, 1.0),
        conscientiousness: scores[1].clamp(-1.0, 1.0),
        extraversion: scores[2].clamp(-1.0, 1.0),
        agreeableness: scores[3].clamp(-1.0, 1.0),
        neuroticism: scores[4].clamp(-1.0, 1.0),
    }
}

/// Compute Cronbach's alpha reliability for a set of trait measurements.
///
/// Each inner `Vec<f32>` represents one item's scores across respondents.
/// Returns alpha in [0.0, 1.0]; values ≥ 0.7 indicate acceptable reliability.
/// Returns 0.0 if bodh computation fails (e.g., insufficient data).
///
/// ```
/// use bhava::psychology::trait_reliability;
///
/// let items = vec![
///     vec![3.0, 4.0, 5.0, 3.0, 4.0],
///     vec![2.0, 3.0, 4.0, 3.0, 3.0],
///     vec![4.0, 5.0, 5.0, 4.0, 5.0],
/// ];
/// let alpha = trait_reliability(&items);
/// assert!((0.0..=1.0).contains(&alpha));
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn trait_reliability(items: &[Vec<f32>]) -> f64 {
    bodh::psychometrics::cronbachs_alpha(items)
        .unwrap_or(0.0)
        .clamp(0.0, 1.0)
}

// ── ACT-R Memory ───────────────────────────────────────────────────────

/// Compute ACT-R base-level activation using bodh's validated formula.
///
/// `ages` contains the time (in seconds) since each prior presentation.
/// `decay` is the power-law decay parameter (typically 0.5).
/// Returns activation value; higher = more accessible. Falls back to 0.0 on error.
///
/// Formula: B_i = ln(Σ t_j^(-d))
///
/// ```
/// use bhava::psychology::base_level_activation;
///
/// let ages = vec![10.0, 100.0, 1000.0];
/// let activation = base_level_activation(&ages, 0.5);
/// assert!(activation.is_finite());
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn base_level_activation(ages: &[f64], decay: f64) -> f64 {
    let history = bodh::memory::ChunkHistory {
        presentation_ages: ages.to_vec(),
    };
    bodh::memory::base_level_activation(&history, decay).unwrap_or(0.0)
}

/// Compute ACT-R retrieval probability using bodh's softmax gate.
///
/// Returns probability [0.0, 1.0] that a chunk with the given activation
/// exceeds the retrieval threshold. Uses default noise of 0.4.
///
/// Formula: P = 1 / (1 + e^((τ − A) / s))
///
/// ```
/// use bhava::psychology::retrieval_probability;
///
/// let prob = retrieval_probability(1.0, 0.5);
/// assert!(prob > 0.5);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn retrieval_probability(activation: f64, threshold: f64) -> f64 {
    bodh::memory::retrieval_probability(activation, threshold, 0.4)
        .unwrap_or(0.5)
        .clamp(0.0, 1.0)
}

// ── Cognition ──────────────────────────────────────────────────────────

/// Compute cognitive load as ratio of task demands to working memory capacity.
///
/// `task_loads` contains (intrinsic, extraneous) load pairs per task element.
/// `capacity` is the total working memory capacity. Returns load ratio where
/// 1.0 = saturated. Falls back to 1.0 (saturated) on error.
///
/// ```
/// use bhava::psychology::cognitive_load;
///
/// let loads = vec![(0.3, 0.1), (0.2, 0.1)];
/// let load = cognitive_load(&loads, 7.0);
/// assert!(load > 0.0 && load < 1.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn cognitive_load(task_loads: &[(f64, f64)], capacity: f64) -> f64 {
    bodh::cognition::cognitive_load(task_loads, capacity)
        .unwrap_or(1.0)
        .clamp(0.0, f64::MAX)
}

/// Compute Yerkes-Dodson arousal → performance via bodh's inverted-U curve.
///
/// Returns performance [0.0, 1.0] given current arousal, optimal arousal point,
/// and spread (controls curve width). Higher spread = broader optimum.
///
/// ```
/// use bhava::psychology::yerkes_dodson_performance;
///
/// // At optimal arousal, performance should be near maximum
/// let perf = yerkes_dodson_performance(0.5, 0.5, 0.3);
/// assert!(perf > 0.9);
///
/// // Far from optimal, performance drops
/// let low = yerkes_dodson_performance(0.0, 0.5, 0.3);
/// assert!(low < perf);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn yerkes_dodson_performance(arousal: f32, optimal: f32, spread: f32) -> f64 {
    bodh::emotion::yerkes_dodson(f64::from(arousal), f64::from(optimal), f64::from(spread))
        .unwrap_or(0.5)
        .clamp(0.0, 1.0)
}

// ── Social Psychology ──────────────────────────────────────────────────

/// Compute mood-congruent memory retrieval bias.
///
/// Given a base retrieval probability and the entity's current mood, returns
/// a biased probability for memories with the given valence. Positive mood
/// boosts positive memory retrieval; negative mood boosts negative memories.
///
/// ```
/// use bhava::mood::MoodVector;
/// use bhava::psychology::mood_congruent_bias;
///
/// let happy = MoodVector { joy: 0.8, arousal: 0.3, dominance: 0.2,
///     trust: 0.5, interest: 0.3, frustration: 0.0 };
/// let bias = mood_congruent_bias(0.5, &happy, 0.7);
/// assert!(bias >= 0.5); // positive mood boosts positive memory retrieval
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn mood_congruent_bias(base_prob: f64, mood: &MoodVector, memory_valence: f32) -> f64 {
    let current = affect_from_mood(mood);
    let memory_affect = bodh::emotion::Affect::new(f64::from(memory_valence).clamp(-1.0, 1.0), 0.0)
        .unwrap_or(bodh::emotion::Affect {
            valence: 0.0,
            arousal: 0.0,
        });
    bodh::emotion::mood_congruent_bias(base_prob, current, memory_affect, 0.5)
        .unwrap_or(base_prob)
        .clamp(0.0, 1.0)
}

/// Determine attribution type using Kelley's covariation model.
///
/// Given consensus, distinctiveness, and consistency scores (each 0.0 to 1.0),
/// returns whether the cause is attributed externally, internally, or to
/// circumstances.
///
/// ```
/// use bhava::psychology::attribution_type;
///
/// // High consensus + high distinctiveness + high consistency → External
/// let attr = attribution_type(0.9, 0.9, 0.9);
/// assert!(matches!(attr, bodh::social::AttributionType::External));
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn attribution_type(
    consensus: f64,
    distinctiveness: f64,
    consistency: f64,
) -> bodh::social::AttributionType {
    let info = bodh::social::CovariationInfo {
        consensus: consensus.clamp(0.0, 1.0),
        distinctiveness: distinctiveness.clamp(0.0, 1.0),
        consistency: consistency.clamp(0.0, 1.0),
    };
    bodh::social::kelley_attribution(&info)
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Affect / Mood Conversion ───────────────────────────────────────

    #[test]
    fn affect_from_mood_positive() {
        let mood = MoodVector {
            joy: 0.9,
            arousal: 0.5,
            dominance: 0.3,
            trust: 0.6,
            interest: 0.4,
            frustration: 0.0,
        };
        let affect = affect_from_mood(&mood);
        assert!(
            affect.valence > 0.0,
            "positive mood should give positive valence"
        );
        assert!((affect.arousal - 0.5).abs() < 0.01);
    }

    #[test]
    fn affect_from_mood_negative() {
        let mood = MoodVector {
            joy: 0.0,
            arousal: 0.8,
            dominance: -0.3,
            trust: 0.0,
            interest: 0.1,
            frustration: 0.9,
        };
        let affect = affect_from_mood(&mood);
        assert!(
            affect.valence < 0.0,
            "frustrated mood should give negative valence"
        );
    }

    #[test]
    fn mood_from_affect_positive_valence() {
        let affect = bodh::emotion::Affect {
            valence: 0.7,
            arousal: 0.3,
        };
        let mood = mood_from_affect(affect);
        assert!(mood.joy > 0.0);
        assert_eq!(mood.frustration, 0.0);
        assert!((mood.arousal - 0.3).abs() < 0.01);
    }

    #[test]
    fn mood_from_affect_negative_valence() {
        let affect = bodh::emotion::Affect {
            valence: -0.6,
            arousal: 0.5,
        };
        let mood = mood_from_affect(affect);
        assert_eq!(mood.joy, 0.0);
        assert!(mood.frustration > 0.0);
    }

    #[test]
    fn affect_mood_roundtrip_preserves_sign() {
        let original = MoodVector {
            joy: 0.7,
            arousal: 0.4,
            dominance: 0.0,
            trust: 0.3,
            interest: 0.0,
            frustration: 0.1,
        };
        let affect = affect_from_mood(&original);
        let restored = mood_from_affect(affect);
        // Valence direction should be preserved
        assert!(restored.joy > restored.frustration);
    }

    #[test]
    fn classify_mood_happy() {
        let mood = MoodVector {
            joy: 0.9,
            arousal: 0.3,
            dominance: 0.2,
            trust: 0.5,
            interest: 0.3,
            frustration: 0.0,
        };
        let emotion = classify_mood(&mood);
        assert!(matches!(emotion, bodh::emotion::BasicEmotion::Happiness));
    }

    #[test]
    fn classify_mood_sad() {
        let mood = MoodVector {
            joy: 0.0,
            arousal: -0.5,
            dominance: -0.3,
            trust: 0.0,
            interest: 0.0,
            frustration: 0.7,
        };
        let emotion = classify_mood(&mood);
        assert!(matches!(emotion, bodh::emotion::BasicEmotion::Sadness));
    }

    // ── Appraisal ──────────────────────────────────────────────────────

    #[test]
    fn scherer_from_desirable_event() {
        let appraisal = crate::appraisal::Appraisal::event("good news", 0.8)
            .with_praise(0.5)
            .with_likelihood(0.9);
        let dims = appraisal_to_scherer(&appraisal);
        assert!(dims.goal_conduciveness > 0.0);
        assert!(dims.norm_compatibility > 0.0);
        // High likelihood → low novelty
        assert!(dims.novelty < 0.2);
    }

    #[test]
    fn affect_from_desirable_appraisal_positive() {
        let appraisal = crate::appraisal::Appraisal::event("promotion", 0.9);
        let affect = affect_from_appraisal(&appraisal);
        assert!(affect.valence > 0.0);
    }

    #[test]
    fn affect_from_undesirable_appraisal_negative() {
        let appraisal = crate::appraisal::Appraisal::event("bad news", -0.8);
        let affect = affect_from_appraisal(&appraisal);
        assert!(affect.valence < 0.0);
    }

    // ── Regulation ─────────────────────────────────────────────────────

    #[test]
    fn regulation_accept_full_effectiveness() {
        let strategy = crate::regulation::RegulationStrategy::Accept;
        assert!((regulation_effectiveness(&strategy) - 1.0).abs() < 0.01);
    }

    #[test]
    fn regulation_suppress_low_effectiveness() {
        let strategy = crate::regulation::RegulationStrategy::Suppress {
            target: crate::mood::Emotion::Frustration,
            strength: 0.8,
        };
        let eff = regulation_effectiveness(&strategy);
        assert!(
            eff < 0.5,
            "suppression should have low effectiveness: {eff}"
        );
    }

    #[test]
    fn regulation_reappraise_high_effectiveness() {
        let strategy = crate::regulation::RegulationStrategy::Reappraise {
            target: crate::mood::Emotion::Frustration,
            reduction: 0.5,
        };
        let eff = regulation_effectiveness(&strategy);
        assert!(
            eff > 0.7,
            "reappraisal should have high effectiveness: {eff}"
        );
    }

    // ── Psychometrics ──────────────────────────────────────────────────

    #[test]
    fn ocean_from_big_five_mapping() {
        let scores = [0.7_f32, 0.5, 0.8, 0.6, 0.3];
        let ocean = ocean_from_big_five(&scores);
        assert!((ocean.openness - 0.7).abs() < 0.01);
        assert!((ocean.conscientiousness - 0.5).abs() < 0.01);
        assert!((ocean.extraversion - 0.8).abs() < 0.01);
        assert!((ocean.agreeableness - 0.6).abs() < 0.01);
        assert!((ocean.neuroticism - 0.3).abs() < 0.01);
    }

    #[test]
    fn ocean_from_big_five_clamps() {
        let scores = [2.0_f32, -2.0, 0.5, 0.5, 0.5];
        let ocean = ocean_from_big_five(&scores);
        assert!((ocean.openness - 1.0).abs() < 0.01);
        assert!((ocean.conscientiousness - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn trait_reliability_consistent_data() {
        let items = vec![
            vec![3.0, 4.0, 5.0, 3.0, 4.0],
            vec![3.0, 4.0, 5.0, 3.0, 4.0],
            vec![3.0, 4.0, 5.0, 3.0, 4.0],
        ];
        let alpha = trait_reliability(&items);
        assert!((0.0..=1.0).contains(&alpha));
    }

    #[test]
    fn trait_reliability_empty_returns_zero() {
        let alpha = trait_reliability(&[]);
        assert!((alpha - 0.0).abs() < 0.01);
    }

    // ── ACT-R Memory ───────────────────────────────────────────────────

    #[test]
    fn base_level_recent_higher() {
        let recent = base_level_activation(&[1.0, 10.0], 0.5);
        let old = base_level_activation(&[100.0, 1000.0], 0.5);
        assert!(
            recent > old,
            "recent presentations should give higher activation"
        );
    }

    #[test]
    fn base_level_empty_ages() {
        let activation = base_level_activation(&[], 0.5);
        assert!((activation - 0.0).abs() < 0.01);
    }

    #[test]
    fn retrieval_high_activation_high_prob() {
        let prob = retrieval_probability(2.0, 0.5);
        assert!(prob > 0.5);
    }

    #[test]
    fn retrieval_low_activation_low_prob() {
        let prob = retrieval_probability(-2.0, 0.5);
        assert!(prob < 0.5);
    }

    // ── Cognition ──────────────────────────────────────────────────────

    #[test]
    fn cognitive_load_within_capacity() {
        let loads = vec![(0.3, 0.1), (0.2, 0.1)];
        let load = cognitive_load(&loads, 7.0);
        assert!(load > 0.0 && load < 1.0);
    }

    #[test]
    fn yerkes_dodson_at_optimal() {
        let perf = yerkes_dodson_performance(0.5, 0.5, 0.3);
        assert!(
            perf > 0.8,
            "at optimal arousal, performance should be high: {perf}"
        );
    }

    #[test]
    fn yerkes_dodson_away_from_optimal() {
        let optimal = yerkes_dodson_performance(0.5, 0.5, 0.3);
        let low = yerkes_dodson_performance(0.0, 0.5, 0.3);
        let high = yerkes_dodson_performance(1.0, 0.5, 0.3);
        assert!(low < optimal);
        assert!(high < optimal);
    }

    // ── Social Psychology ──────────────────────────────────────────────

    #[test]
    fn mood_congruent_bias_positive_mood_positive_memory() {
        let happy = MoodVector {
            joy: 0.8,
            arousal: 0.3,
            dominance: 0.2,
            trust: 0.5,
            interest: 0.3,
            frustration: 0.0,
        };
        let bias = mood_congruent_bias(0.5, &happy, 0.7);
        assert!(
            bias >= 0.5,
            "positive mood should boost positive memory retrieval: {bias}"
        );
    }

    #[test]
    fn mood_congruent_bias_bounded() {
        let mood = MoodVector {
            joy: 1.0,
            arousal: 1.0,
            dominance: 1.0,
            trust: 1.0,
            interest: 1.0,
            frustration: 0.0,
        };
        let bias = mood_congruent_bias(0.5, &mood, 1.0);
        assert!((0.0..=1.0).contains(&bias));
    }

    #[test]
    fn attribution_external() {
        let attr = attribution_type(0.9, 0.9, 0.9);
        assert!(matches!(attr, bodh::social::AttributionType::External));
    }

    #[test]
    fn attribution_internal() {
        let attr = attribution_type(0.1, 0.1, 0.9);
        assert!(matches!(attr, bodh::social::AttributionType::Internal));
    }
}
