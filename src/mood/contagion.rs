use serde::{Deserialize, Serialize};

use super::types::{Emotion, MoodVector};

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
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
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
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn group_mood(members: &[&MoodVector]) -> MoodVector {
    if members.is_empty() {
        return MoodVector::neutral();
    }
    let mut sum = MoodVector::neutral();
    for &m in members {
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
