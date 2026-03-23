use serde::{Deserialize, Serialize};

use super::kind::{TraitKind, TraitLevel};
use super::profile::PersonalityProfile;

// --- OCEAN / Big Five Conversion ---

/// Big Five (OCEAN) personality scores.
///
/// Each dimension ranges from -1.0 to 1.0:
/// - **Openness**: intellectual curiosity, creativity, novelty-seeking
/// - **Conscientiousness**: organization, precision, self-discipline
/// - **Extraversion**: sociability, assertiveness, warmth
/// - **Agreeableness**: cooperation, trust, empathy
/// - **Neuroticism**: emotional instability, anxiety, volatility
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OceanScores {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

impl PersonalityProfile {
    /// Convert this profile to Big Five (OCEAN) scores.
    #[must_use]
    pub fn to_ocean(&self) -> OceanScores {
        let n = |k: TraitKind| self.get_trait(k).normalized();

        OceanScores {
            openness: (n(TraitKind::Creativity) * 0.35
                + n(TraitKind::Curiosity) * 0.35
                + n(TraitKind::RiskTolerance) * 0.15
                - n(TraitKind::Precision) * 0.15)
                .clamp(-1.0, 1.0),
            conscientiousness: (n(TraitKind::Precision) * 0.4 + n(TraitKind::Formality) * 0.25
                - n(TraitKind::RiskTolerance) * 0.2
                + n(TraitKind::Autonomy) * 0.15)
                .clamp(-1.0, 1.0),
            extraversion: (n(TraitKind::Warmth) * 0.3
                + n(TraitKind::Humor) * 0.2
                + n(TraitKind::Verbosity) * 0.15
                + n(TraitKind::Confidence) * 0.2
                + n(TraitKind::Directness) * 0.15)
                .clamp(-1.0, 1.0),
            agreeableness: (n(TraitKind::Empathy) * 0.3
                + n(TraitKind::Warmth) * 0.25
                + n(TraitKind::Patience) * 0.2
                - n(TraitKind::Skepticism) * 0.15
                - n(TraitKind::Directness) * 0.1)
                .clamp(-1.0, 1.0),
            neuroticism: (-n(TraitKind::Patience) * 0.3 - n(TraitKind::Confidence) * 0.3
                + n(TraitKind::Skepticism) * 0.2
                - n(TraitKind::Empathy) * 0.2)
                .clamp(-1.0, 1.0),
        }
    }
}

/// Create a personality profile from Big Five (OCEAN) scores.
///
/// Each score should be in the range -1.0 to 1.0.
#[must_use]
pub fn profile_from_ocean(name: impl Into<String>, ocean: &OceanScores) -> PersonalityProfile {
    let mut p = PersonalityProfile::new(name);
    let (o, c, e, a, n) = (
        ocean.openness,
        ocean.conscientiousness,
        ocean.extraversion,
        ocean.agreeableness,
        ocean.neuroticism,
    );
    p.set_trait(TraitKind::Creativity, TraitLevel::from_normalized(o * 0.8));
    p.set_trait(TraitKind::Curiosity, TraitLevel::from_normalized(o * 0.8));
    p.set_trait(
        TraitKind::RiskTolerance,
        TraitLevel::from_normalized(o * 0.4 - c * 0.4),
    );
    p.set_trait(TraitKind::Precision, TraitLevel::from_normalized(c * 0.8));
    p.set_trait(
        TraitKind::Formality,
        TraitLevel::from_normalized(c * 0.5 - e * 0.3),
    );
    p.set_trait(TraitKind::Autonomy, TraitLevel::from_normalized(c * 0.4));
    p.set_trait(
        TraitKind::Warmth,
        TraitLevel::from_normalized(e * 0.5 + a * 0.4),
    );
    p.set_trait(TraitKind::Humor, TraitLevel::from_normalized(e * 0.6));
    p.set_trait(TraitKind::Verbosity, TraitLevel::from_normalized(e * 0.5));
    p.set_trait(
        TraitKind::Confidence,
        TraitLevel::from_normalized(e * 0.5 - n * 0.4),
    );
    p.set_trait(
        TraitKind::Directness,
        TraitLevel::from_normalized(e * 0.3 - a * 0.3),
    );
    p.set_trait(TraitKind::Empathy, TraitLevel::from_normalized(a * 0.7));
    p.set_trait(
        TraitKind::Patience,
        TraitLevel::from_normalized(a * 0.4 - n * 0.4),
    );
    p.set_trait(
        TraitKind::Skepticism,
        TraitLevel::from_normalized(-a * 0.4 + n * 0.3),
    );
    p.set_trait(
        TraitKind::Pedagogy,
        TraitLevel::from_normalized(a * 0.3 + o * 0.3),
    );
    p
}

/// Personality entropy — how scattered the trait distribution is (0.0–1.0).
#[must_use]
pub fn personality_entropy(profile: &PersonalityProfile) -> f32 {
    let mut counts = [0u32; 5];
    for &kind in TraitKind::ALL {
        let idx = (profile.get_trait(kind).numeric() + 2) as usize;
        counts[idx] += 1;
    }
    let total = TraitKind::COUNT as f32;
    let mut entropy = 0.0f32;
    for &c in &counts {
        if c > 0 {
            let p = c as f32 / total;
            entropy -= p * p.ln();
        }
    }
    entropy / 5.0f32.ln()
}

/// Personality extremity — average absolute deviation from Balanced (0.0–1.0).
#[must_use]
pub fn personality_extremity(profile: &PersonalityProfile) -> f32 {
    let sum: f32 = TraitKind::ALL
        .iter()
        .map(|&k| profile.get_trait(k).normalized().abs())
        .sum();
    sum / TraitKind::COUNT as f32
}
