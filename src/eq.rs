//! Emotional intelligence (EQ) — the Mayer-Salovey four-branch model.
//!
//! Quantifies an entity's emotional competence across four branches
//! (Mayer & Salovey 1997):
//!
//! 1. **Perception** — accuracy in identifying emotions in self, others,
//!    and stimuli. High perception → better at reading micro-expressions,
//!    detecting contagion, and recognizing compound emotions.
//! 2. **Facilitation** — using emotions to enhance cognitive processes.
//!    High facilitation → mood-congruent creativity boosts, better
//!    emotional memory recall, and flow-state sensitivity.
//! 3. **Understanding** — comprehending emotional vocabulary, blends,
//!    transitions, and causes. High understanding → richer appraisal,
//!    better prediction of emotional consequences.
//! 4. **Management** — regulating emotions in self and others.
//!    High management → more effective regulation strategies,
//!    better contagion control, faster stress recovery.
//!
//! EQ scores range from 0.0 (minimal competence) to 1.0 (exceptional).
//! A baseline can be derived from personality traits, then refined by
//! observed behavior over time.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Emotional intelligence profile — four-branch scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqProfile {
    /// Accuracy in identifying emotions (0.0–1.0).
    pub perception: f32,
    /// Using emotions to enhance thinking (0.0–1.0).
    pub facilitation: f32,
    /// Comprehending emotional language and transitions (0.0–1.0).
    pub understanding: f32,
    /// Regulating emotions in self and others (0.0–1.0).
    pub management: f32,
}

impl Default for EqProfile {
    fn default() -> Self {
        Self {
            perception: 0.5,
            facilitation: 0.5,
            understanding: 0.5,
            management: 0.5,
        }
    }
}

impl EqProfile {
    /// Create with default mid-range scores.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with explicit scores (clamped to 0.0–1.0).
    #[must_use]
    pub fn with_scores(
        perception: f32,
        facilitation: f32,
        understanding: f32,
        management: f32,
    ) -> Self {
        Self {
            perception: perception.clamp(0.0, 1.0),
            facilitation: facilitation.clamp(0.0, 1.0),
            understanding: understanding.clamp(0.0, 1.0),
            management: management.clamp(0.0, 1.0),
        }
    }

    /// Overall EQ score — weighted average of all branches.
    ///
    /// Weights follow the hierarchical model where higher branches
    /// (understanding, management) are weighted more heavily as they
    /// depend on the lower ones.
    #[must_use]
    #[inline]
    pub fn overall(&self) -> f32 {
        // Weights: perception 0.15, facilitation 0.20, understanding 0.30, management 0.35
        self.perception * 0.15
            + self.facilitation * 0.20
            + self.understanding * 0.30
            + self.management * 0.35
    }

    /// Get score for a specific branch.
    #[must_use]
    #[inline]
    pub fn get(&self, branch: EqBranch) -> f32 {
        match branch {
            EqBranch::Perception => self.perception,
            EqBranch::Facilitation => self.facilitation,
            EqBranch::Understanding => self.understanding,
            EqBranch::Management => self.management,
        }
    }

    /// Set score for a specific branch (clamped to 0.0–1.0).
    #[inline]
    pub fn set(&mut self, branch: EqBranch, value: f32) {
        let v = value.clamp(0.0, 1.0);
        match branch {
            EqBranch::Perception => self.perception = v,
            EqBranch::Facilitation => self.facilitation = v,
            EqBranch::Understanding => self.understanding = v,
            EqBranch::Management => self.management = v,
        }
    }

    /// Classify the overall EQ level.
    #[must_use]
    pub fn level(&self) -> EqLevel {
        let o = self.overall();
        if o >= 0.8 {
            EqLevel::Exceptional
        } else if o >= 0.6 {
            EqLevel::High
        } else if o >= 0.4 {
            EqLevel::Average
        } else if o >= 0.2 {
            EqLevel::Low
        } else {
            EqLevel::Minimal
        }
    }

    /// Micro-expression detection bonus from perception.
    ///
    /// Higher perception → better at noticing micro-expressions.
    /// Returns a multiplier: 0.5 (low perception) to 1.5 (high perception).
    #[must_use]
    pub fn perception_bonus(&self) -> f32 {
        0.5 + self.perception
    }

    /// Creativity/flow facilitation bonus.
    ///
    /// Higher facilitation → mood states boost cognitive tasks more.
    /// Returns a multiplier: 0.5 to 1.5.
    #[must_use]
    pub fn facilitation_bonus(&self) -> f32 {
        0.5 + self.facilitation
    }

    /// Emotion regulation effectiveness bonus from management.
    ///
    /// Higher management → regulation strategies work better.
    /// Returns a multiplier: 0.5 to 1.5.
    #[must_use]
    pub fn management_bonus(&self) -> f32 {
        0.5 + self.management
    }

    /// Stress recovery rate bonus from management.
    ///
    /// Higher management → faster stress recovery.
    /// Returns a multiplier: 0.8 to 1.5.
    #[must_use]
    pub fn stress_recovery_bonus(&self) -> f32 {
        0.8 + self.management * 0.7
    }

    /// Contagion resistance from management.
    ///
    /// Higher management → less susceptible to emotional contagion.
    /// Returns a resistance factor: 0.0 (fully susceptible) to 0.5 (resistant).
    #[must_use]
    pub fn contagion_resistance(&self) -> f32 {
        self.management * 0.5
    }

    /// Appraisal accuracy bonus from understanding.
    ///
    /// Higher understanding → more nuanced emotion generation from events.
    /// Returns a multiplier: 0.5 to 1.5.
    #[must_use]
    pub fn appraisal_bonus(&self) -> f32 {
        0.5 + self.understanding
    }
}

/// The four branches of emotional intelligence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EqBranch {
    /// Identifying emotions in self, others, and stimuli.
    Perception,
    /// Using emotions to enhance cognitive processes.
    Facilitation,
    /// Comprehending emotional language and transitions.
    Understanding,
    /// Regulating emotions in self and others.
    Management,
}

impl EqBranch {
    /// All branches in hierarchical order (lower → higher).
    pub const ALL: &'static [EqBranch] = &[
        Self::Perception,
        Self::Facilitation,
        Self::Understanding,
        Self::Management,
    ];
}

impl fmt::Display for EqBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Perception => "perception",
            Self::Facilitation => "facilitation",
            Self::Understanding => "understanding",
            Self::Management => "management",
        };
        f.write_str(s)
    }
}

/// Named EQ classification level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EqLevel {
    /// Overall < 0.2.
    Minimal,
    /// Overall 0.2–0.4.
    Low,
    /// Overall 0.4–0.6.
    Average,
    /// Overall 0.6–0.8.
    High,
    /// Overall >= 0.8.
    Exceptional,
}

impl fmt::Display for EqLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Minimal => "minimal",
            Self::Low => "low",
            Self::Average => "average",
            Self::High => "high",
            Self::Exceptional => "exceptional",
        };
        f.write_str(s)
    }
}

/// Derive a baseline EQ profile from personality traits.
///
/// Trait mappings (Mayer-Salovey → Big Five/bhava traits):
/// - **Perception** ← empathy + curiosity (attentiveness to emotional signals)
/// - **Facilitation** ← creativity + confidence (leveraging emotions for thinking)
/// - **Understanding** ← empathy + patience + pedagogy (emotional vocabulary depth)
/// - **Management** ← patience + confidence + formality (self-regulation capacity)
#[cfg(feature = "traits")]
#[must_use]
pub fn eq_from_personality(profile: &crate::traits::PersonalityProfile) -> EqProfile {
    use crate::traits::TraitKind;

    let empathy = profile.get_trait(TraitKind::Empathy).normalized();
    let curiosity = profile.get_trait(TraitKind::Curiosity).normalized();
    let creativity = profile.get_trait(TraitKind::Creativity).normalized();
    let confidence = profile.get_trait(TraitKind::Confidence).normalized();
    let patience = profile.get_trait(TraitKind::Patience).normalized();
    let pedagogy = profile.get_trait(TraitKind::Pedagogy).normalized();
    let formality = profile.get_trait(TraitKind::Formality).normalized();

    // Map trait averages from -1..1 to 0..1
    let to_score = |v: f32| ((v + 1.0) / 2.0).clamp(0.0, 1.0);

    EqProfile {
        perception: to_score((empathy + curiosity) / 2.0),
        facilitation: to_score((creativity + confidence) / 2.0),
        understanding: to_score((empathy + patience + pedagogy) / 3.0),
        management: to_score((patience + confidence + formality) / 3.0),
    }
}

/// Compose an EQ summary for system prompt injection.
#[must_use]
pub fn compose_eq_prompt(eq: &EqProfile) -> String {
    use std::fmt::Write;
    let mut prompt = String::with_capacity(200);
    prompt.push_str("## Emotional Intelligence\n\n");
    let _ = writeln!(
        prompt,
        "- Overall EQ: {} ({:.0}%)",
        eq.level(),
        eq.overall() * 100.0
    );
    for &branch in EqBranch::ALL {
        let score = eq.get(branch);
        let label = match score {
            s if s >= 0.8 => "exceptional",
            s if s >= 0.6 => "strong",
            s if s >= 0.4 => "moderate",
            s if s >= 0.2 => "developing",
            _ => "limited",
        };
        let _ = writeln!(prompt, "- {branch}: {label}");
    }
    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let eq = EqProfile::new();
        assert!((eq.perception - 0.5).abs() < f32::EPSILON);
        assert!((eq.facilitation - 0.5).abs() < f32::EPSILON);
        assert!((eq.understanding - 0.5).abs() < f32::EPSILON);
        assert!((eq.management - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_with_scores_clamps() {
        let eq = EqProfile::with_scores(1.5, -0.5, 0.7, 0.3);
        assert!((eq.perception - 1.0).abs() < f32::EPSILON);
        assert!(eq.facilitation.abs() < f32::EPSILON);
        assert!((eq.understanding - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn test_overall_weighted() {
        let eq = EqProfile::with_scores(1.0, 1.0, 1.0, 1.0);
        // 0.15 + 0.20 + 0.30 + 0.35 = 1.0
        assert!((eq.overall() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_overall_zero() {
        let eq = EqProfile::with_scores(0.0, 0.0, 0.0, 0.0);
        assert!(eq.overall().abs() < f32::EPSILON);
    }

    #[test]
    fn test_overall_management_heavy() {
        // Management has highest weight (0.35)
        let high_mgmt = EqProfile::with_scores(0.0, 0.0, 0.0, 1.0);
        let high_perc = EqProfile::with_scores(1.0, 0.0, 0.0, 0.0);
        assert!(high_mgmt.overall() > high_perc.overall());
    }

    #[test]
    fn test_get_set() {
        let mut eq = EqProfile::new();
        eq.set(EqBranch::Perception, 0.9);
        assert!((eq.get(EqBranch::Perception) - 0.9).abs() < f32::EPSILON);
        eq.set(EqBranch::Management, 1.5); // clamped
        assert!((eq.get(EqBranch::Management) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_level_exceptional() {
        let eq = EqProfile::with_scores(1.0, 1.0, 1.0, 1.0);
        assert_eq!(eq.level(), EqLevel::Exceptional);
    }

    #[test]
    fn test_level_minimal() {
        let eq = EqProfile::with_scores(0.0, 0.0, 0.0, 0.0);
        assert_eq!(eq.level(), EqLevel::Minimal);
    }

    #[test]
    fn test_level_average() {
        let eq = EqProfile::new(); // all 0.5
        assert_eq!(eq.level(), EqLevel::Average);
    }

    #[test]
    fn test_perception_bonus_range() {
        let low = EqProfile::with_scores(0.0, 0.5, 0.5, 0.5);
        let high = EqProfile::with_scores(1.0, 0.5, 0.5, 0.5);
        assert!((low.perception_bonus() - 0.5).abs() < f32::EPSILON);
        assert!((high.perception_bonus() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_management_bonus_range() {
        let low = EqProfile::with_scores(0.5, 0.5, 0.5, 0.0);
        let high = EqProfile::with_scores(0.5, 0.5, 0.5, 1.0);
        assert!((low.management_bonus() - 0.5).abs() < f32::EPSILON);
        assert!((high.management_bonus() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stress_recovery_bonus() {
        let eq = EqProfile::with_scores(0.5, 0.5, 0.5, 1.0);
        assert!((eq.stress_recovery_bonus() - 1.5).abs() < f32::EPSILON);
        let low = EqProfile::with_scores(0.5, 0.5, 0.5, 0.0);
        assert!((low.stress_recovery_bonus() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_contagion_resistance() {
        let eq = EqProfile::with_scores(0.5, 0.5, 0.5, 1.0);
        assert!((eq.contagion_resistance() - 0.5).abs() < f32::EPSILON);
        let low = EqProfile::with_scores(0.5, 0.5, 0.5, 0.0);
        assert!(low.contagion_resistance().abs() < f32::EPSILON);
    }

    #[test]
    fn test_branch_display() {
        assert_eq!(EqBranch::Perception.to_string(), "perception");
        assert_eq!(EqBranch::Management.to_string(), "management");
    }

    #[test]
    fn test_level_display() {
        assert_eq!(EqLevel::Exceptional.to_string(), "exceptional");
        assert_eq!(EqLevel::Minimal.to_string(), "minimal");
    }

    #[test]
    fn test_branch_all() {
        assert_eq!(EqBranch::ALL.len(), 4);
    }

    #[test]
    fn test_compose_prompt() {
        let eq = EqProfile::with_scores(0.9, 0.7, 0.8, 0.6);
        let prompt = compose_eq_prompt(&eq);
        assert!(prompt.contains("## Emotional Intelligence"));
        assert!(prompt.contains("perception"));
        assert!(prompt.contains("management"));
    }

    #[test]
    fn test_serde_profile() {
        let eq = EqProfile::with_scores(0.8, 0.6, 0.7, 0.9);
        let json = serde_json::to_string(&eq).unwrap();
        let eq2: EqProfile = serde_json::from_str(&json).unwrap();
        assert!((eq2.perception - eq.perception).abs() < f32::EPSILON);
        assert!((eq2.management - eq.management).abs() < f32::EPSILON);
    }

    #[test]
    fn test_serde_branch() {
        let b = EqBranch::Understanding;
        let json = serde_json::to_string(&b).unwrap();
        let b2: EqBranch = serde_json::from_str(&json).unwrap();
        assert_eq!(b2, b);
    }

    #[test]
    fn test_serde_level() {
        let l = EqLevel::High;
        let json = serde_json::to_string(&l).unwrap();
        let l2: EqLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(l2, l);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_eq_from_personality_empathetic() {
        let mut p = crate::traits::PersonalityProfile::new("empath");
        p.set_trait(
            crate::traits::TraitKind::Empathy,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Patience,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Curiosity,
            crate::traits::TraitLevel::High,
        );
        let eq = eq_from_personality(&p);
        assert!(eq.perception > 0.6, "perception: {}", eq.perception);
        assert!(
            eq.understanding > 0.6,
            "understanding: {}",
            eq.understanding
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_eq_from_personality_stoic() {
        let mut p = crate::traits::PersonalityProfile::new("stoic");
        p.set_trait(
            crate::traits::TraitKind::Formality,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Confidence,
            crate::traits::TraitLevel::Highest,
        );
        p.set_trait(
            crate::traits::TraitKind::Empathy,
            crate::traits::TraitLevel::Lowest,
        );
        let eq = eq_from_personality(&p);
        // High management (formality + confidence), low perception (low empathy)
        assert!(
            eq.management > eq.perception,
            "mgmt={} perc={}",
            eq.management,
            eq.perception
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_eq_from_personality_balanced() {
        let p = crate::traits::PersonalityProfile::new("balanced");
        let eq = eq_from_personality(&p);
        // All balanced traits → all scores near 0.5
        for &branch in EqBranch::ALL {
            let s = eq.get(branch);
            assert!((s - 0.5).abs() < 0.1, "{branch}: {s} (expected ~0.5)");
        }
    }
}
