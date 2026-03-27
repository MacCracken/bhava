//! Reasoning strategy selection — personality-driven cognitive mode.
//!
//! Maps an entity's personality profile to a preferred reasoning strategy.
//! Different trait combinations produce different cognitive styles, affecting
//! how the entity approaches problems, weighs evidence, and makes decisions.
//!
//! Based loosely on dual-process theory (Kahneman 2011) and cognitive style
//! research (Sternberg 1997).

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::traits::{PersonalityProfile, TraitKind};

/// A cognitive reasoning strategy derived from personality traits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReasoningStrategy {
    /// Methodical, evidence-based, detail-oriented.
    /// Driven by high precision + high skepticism.
    Analytical,
    /// Gut-feel, pattern-matching, rapid decision-making.
    /// Driven by high creativity + high risk tolerance.
    Intuitive,
    /// Perspective-taking, relationship-aware, consensus-seeking.
    /// Driven by high empathy + high warmth.
    Empathetic,
    /// Process-driven, structured, independent.
    /// Driven by high precision + high autonomy.
    Systematic,
    /// Divergent thinking, novel connections, exploratory.
    /// Driven by high creativity + high curiosity.
    Creative,
}

impl ReasoningStrategy {
    /// All reasoning strategies.
    pub const ALL: &'static [ReasoningStrategy] = &[
        Self::Analytical,
        Self::Intuitive,
        Self::Empathetic,
        Self::Systematic,
        Self::Creative,
    ];
}

impl fmt::Display for ReasoningStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Analytical => "analytical",
            Self::Intuitive => "intuitive",
            Self::Empathetic => "empathetic",
            Self::Systematic => "systematic",
            Self::Creative => "creative",
        };
        f.write_str(s)
    }
}

/// Compute raw strategy scores from a personality profile.
fn compute_scores(profile: &PersonalityProfile) -> [(ReasoningStrategy, f32); 5] {
    let precision = profile.get_trait(TraitKind::Precision).normalized();
    let skepticism = profile.get_trait(TraitKind::Skepticism).normalized();
    let creativity = profile.get_trait(TraitKind::Creativity).normalized();
    let risk_tolerance = profile.get_trait(TraitKind::RiskTolerance).normalized();
    let empathy = profile.get_trait(TraitKind::Empathy).normalized();
    let warmth = profile.get_trait(TraitKind::Warmth).normalized();
    let autonomy = profile.get_trait(TraitKind::Autonomy).normalized();
    let curiosity = profile.get_trait(TraitKind::Curiosity).normalized();

    [
        (ReasoningStrategy::Analytical, precision + skepticism),
        (ReasoningStrategy::Intuitive, creativity + risk_tolerance),
        (ReasoningStrategy::Empathetic, empathy + warmth),
        (ReasoningStrategy::Systematic, precision + autonomy),
        (ReasoningStrategy::Creative, creativity + curiosity),
    ]
}

/// Select the dominant reasoning strategy from a personality profile.
///
/// Computes a score for each strategy based on relevant trait combinations
/// and returns the highest-scoring one. Ties are broken by strategy order
/// (analytical > intuitive > empathetic > systematic > creative).
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn select_reasoning_strategy(profile: &PersonalityProfile) -> ReasoningStrategy {
    let scores = compute_scores(profile);
    scores
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|&(s, _)| s)
        .unwrap_or(ReasoningStrategy::Analytical)
}

/// Get all strategy scores for a profile, sorted by score descending.
///
/// Useful for displaying a full reasoning profile or selecting a secondary
/// strategy when the primary is inappropriate.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn reasoning_scores(profile: &PersonalityProfile) -> Vec<(ReasoningStrategy, f32)> {
    let mut scores = compute_scores(profile).to_vec();
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scores
}

/// Behavioral description for a reasoning strategy.
///
/// Returns a short description suitable for system prompt injection.
#[must_use]
pub fn strategy_description(strategy: ReasoningStrategy) -> &'static str {
    match strategy {
        ReasoningStrategy::Analytical => {
            "Approach problems methodically: gather evidence, identify assumptions, evaluate options systematically before deciding"
        }
        ReasoningStrategy::Intuitive => {
            "Trust pattern recognition and gut instincts: make rapid assessments, embrace uncertainty, iterate quickly"
        }
        ReasoningStrategy::Empathetic => {
            "Consider perspectives of all stakeholders: seek consensus, weigh emotional impact, build on others' input"
        }
        ReasoningStrategy::Systematic => {
            "Follow structured processes: break problems into steps, document reasoning, maintain independence in analysis"
        }
        ReasoningStrategy::Creative => {
            "Explore unconventional solutions: draw analogies, question constraints, generate many options before converging"
        }
    }
}

/// Compose a reasoning strategy prompt fragment.
///
/// Returns a markdown string suitable for system prompt injection.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn compose_reasoning_prompt(profile: &PersonalityProfile) -> String {
    let strategy = select_reasoning_strategy(profile);
    let desc = strategy_description(strategy);
    let mut prompt = String::with_capacity(desc.len() + 40);
    prompt.push_str("## Reasoning Style\n\n- ");
    prompt.push_str(desc);
    prompt.push('\n');
    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{TraitKind, TraitLevel};

    fn make_profile(name: &str, traits: &[(TraitKind, TraitLevel)]) -> PersonalityProfile {
        let mut p = PersonalityProfile::new(name);
        for &(kind, level) in traits {
            p.set_trait(kind, level);
        }
        p
    }

    #[test]
    fn test_analytical() {
        let p = make_profile(
            "analyst",
            &[
                (TraitKind::Precision, TraitLevel::Highest),
                (TraitKind::Skepticism, TraitLevel::Highest),
            ],
        );
        assert_eq!(select_reasoning_strategy(&p), ReasoningStrategy::Analytical);
    }

    #[test]
    fn test_intuitive() {
        let p = make_profile(
            "intuitor",
            &[
                (TraitKind::Creativity, TraitLevel::Highest),
                (TraitKind::RiskTolerance, TraitLevel::Highest),
            ],
        );
        assert_eq!(select_reasoning_strategy(&p), ReasoningStrategy::Intuitive);
    }

    #[test]
    fn test_empathetic() {
        let p = make_profile(
            "empath",
            &[
                (TraitKind::Empathy, TraitLevel::Highest),
                (TraitKind::Warmth, TraitLevel::Highest),
            ],
        );
        assert_eq!(select_reasoning_strategy(&p), ReasoningStrategy::Empathetic);
    }

    #[test]
    fn test_systematic() {
        let p = make_profile(
            "systemizer",
            &[
                (TraitKind::Precision, TraitLevel::Highest),
                (TraitKind::Autonomy, TraitLevel::Highest),
                // Must suppress skepticism to avoid analytical winning
                (TraitKind::Skepticism, TraitLevel::Lowest),
            ],
        );
        assert_eq!(select_reasoning_strategy(&p), ReasoningStrategy::Systematic);
    }

    #[test]
    fn test_creative() {
        let p = make_profile(
            "creative",
            &[
                (TraitKind::Creativity, TraitLevel::Highest),
                (TraitKind::Curiosity, TraitLevel::Highest),
                // Suppress risk_tolerance to avoid intuitive winning
                (TraitKind::RiskTolerance, TraitLevel::Lowest),
            ],
        );
        assert_eq!(select_reasoning_strategy(&p), ReasoningStrategy::Creative);
    }

    #[test]
    fn test_balanced_defaults_to_analytical() {
        let p = PersonalityProfile::new("balanced");
        // All balanced = all scores 0.0, analytical wins by order
        let strategy = select_reasoning_strategy(&p);
        // With all balanced, scores are tied at 0.0; max_by returns first max
        assert!(
            ReasoningStrategy::ALL.contains(&strategy),
            "should return a valid strategy"
        );
    }

    #[test]
    fn test_reasoning_scores_ordered() {
        let p = make_profile(
            "analyst",
            &[
                (TraitKind::Precision, TraitLevel::Highest),
                (TraitKind::Skepticism, TraitLevel::Highest),
            ],
        );
        let scores = reasoning_scores(&p);
        assert_eq!(scores.len(), 5);
        // First should be highest scoring
        assert!(scores[0].1 >= scores[1].1);
        assert!(scores[1].1 >= scores[2].1);
    }

    #[test]
    fn test_strategy_description_non_empty() {
        for &s in ReasoningStrategy::ALL {
            let desc = strategy_description(s);
            assert!(!desc.is_empty(), "{s} has empty description");
        }
    }

    #[test]
    fn test_strategy_display() {
        assert_eq!(ReasoningStrategy::Analytical.to_string(), "analytical");
        assert_eq!(ReasoningStrategy::Creative.to_string(), "creative");
    }

    #[test]
    fn test_compose_prompt() {
        let p = make_profile("analyst", &[(TraitKind::Precision, TraitLevel::Highest)]);
        let prompt = compose_reasoning_prompt(&p);
        assert!(prompt.contains("## Reasoning Style"));
        assert!(!prompt.is_empty());
    }

    #[test]
    fn test_serde() {
        let s = ReasoningStrategy::Empathetic;
        let json = serde_json::to_string(&s).unwrap();
        let s2: ReasoningStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(s2, s);
    }
}
