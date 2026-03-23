//! Personality trait spectrums — behavioral dimensions with graduated levels.
//!
//! Each trait is a spectrum from one extreme to another (e.g. humor: deadpan → comedic).
//! Traits map to behavioral instructions that guide LLM system prompts.
//! Derived from SecureYeoman's soul/trait-descriptions system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::error::{BhavaError, Result};

/// A personality trait with its current level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitValue {
    pub trait_name: TraitKind,
    pub level: TraitLevel,
}

/// The available personality trait dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TraitKind {
    Formality,
    Humor,
    Verbosity,
    Directness,
    Warmth,
    Empathy,
    Patience,
    Confidence,
    Creativity,
    RiskTolerance,
    Curiosity,
}

impl TraitKind {
    /// All trait kinds.
    pub const ALL: &'static [TraitKind] = &[
        Self::Formality, Self::Humor, Self::Verbosity, Self::Directness,
        Self::Warmth, Self::Empathy, Self::Patience, Self::Confidence,
        Self::Creativity, Self::RiskTolerance, Self::Curiosity,
    ];

    /// The neutral/default level for this trait.
    pub fn default_level(self) -> TraitLevel {
        TraitLevel::Balanced
    }

    /// Available levels for this trait (low → high).
    pub fn levels(self) -> &'static [TraitLevel] {
        // All traits share the same 5-level spectrum
        &[TraitLevel::Lowest, TraitLevel::Low, TraitLevel::Balanced,
          TraitLevel::High, TraitLevel::Highest]
    }
}

impl fmt::Display for TraitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Formality => "formality",
            Self::Humor => "humor",
            Self::Verbosity => "verbosity",
            Self::Directness => "directness",
            Self::Warmth => "warmth",
            Self::Empathy => "empathy",
            Self::Patience => "patience",
            Self::Confidence => "confidence",
            Self::Creativity => "creativity",
            Self::RiskTolerance => "risk_tolerance",
            Self::Curiosity => "curiosity",
        };
        f.write_str(s)
    }
}

/// Graduated level within a trait spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TraitLevel {
    Lowest,
    Low,
    Balanced,
    High,
    Highest,
}

impl TraitLevel {
    /// Numeric value: -2 (Lowest) to +2 (Highest).
    pub fn numeric(self) -> i8 {
        match self {
            Self::Lowest => -2,
            Self::Low => -1,
            Self::Balanced => 0,
            Self::High => 1,
            Self::Highest => 2,
        }
    }

    /// Normalized to -1.0..=1.0.
    pub fn normalized(self) -> f32 {
        self.numeric() as f32 / 2.0
    }

    /// Parse from numeric value.
    pub fn from_numeric(n: i8) -> Result<Self> {
        match n {
            -2 => Ok(Self::Lowest),
            -1 => Ok(Self::Low),
            0 => Ok(Self::Balanced),
            1 => Ok(Self::High),
            2 => Ok(Self::Highest),
            _ => Err(BhavaError::InvalidConfig {
                reason: format!("trait level must be -2..=2, got {n}"),
            }),
        }
    }
}

impl fmt::Display for TraitLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Lowest => "lowest",
            Self::Low => "low",
            Self::Balanced => "balanced",
            Self::High => "high",
            Self::Highest => "highest",
        };
        f.write_str(s)
    }
}

/// Named trait levels per trait kind (maps to SY's trait-descriptions.ts).
pub fn trait_level_name(kind: TraitKind, level: TraitLevel) -> &'static str {
    match (kind, level) {
        (TraitKind::Formality, TraitLevel::Lowest) => "street",
        (TraitKind::Formality, TraitLevel::Low) => "casual",
        (TraitKind::Formality, TraitLevel::Balanced) => "balanced",
        (TraitKind::Formality, TraitLevel::High) => "formal",
        (TraitKind::Formality, TraitLevel::Highest) => "ceremonial",

        (TraitKind::Humor, TraitLevel::Lowest) => "deadpan",
        (TraitKind::Humor, TraitLevel::Low) => "dry",
        (TraitKind::Humor, TraitLevel::Balanced) => "balanced",
        (TraitKind::Humor, TraitLevel::High) => "witty",
        (TraitKind::Humor, TraitLevel::Highest) => "comedic",

        (TraitKind::Verbosity, TraitLevel::Lowest) => "terse",
        (TraitKind::Verbosity, TraitLevel::Low) => "concise",
        (TraitKind::Verbosity, TraitLevel::Balanced) => "balanced",
        (TraitKind::Verbosity, TraitLevel::High) => "detailed",
        (TraitKind::Verbosity, TraitLevel::Highest) => "exhaustive",

        (TraitKind::Directness, TraitLevel::Lowest) => "evasive",
        (TraitKind::Directness, TraitLevel::Low) => "diplomatic",
        (TraitKind::Directness, TraitLevel::Balanced) => "balanced",
        (TraitKind::Directness, TraitLevel::High) => "candid",
        (TraitKind::Directness, TraitLevel::Highest) => "blunt",

        (TraitKind::Warmth, TraitLevel::Lowest) => "cold",
        (TraitKind::Warmth, TraitLevel::Low) => "reserved",
        (TraitKind::Warmth, TraitLevel::Balanced) => "balanced",
        (TraitKind::Warmth, TraitLevel::High) => "friendly",
        (TraitKind::Warmth, TraitLevel::Highest) => "effusive",

        (TraitKind::Empathy, TraitLevel::Lowest) => "detached",
        (TraitKind::Empathy, TraitLevel::Low) => "analytical",
        (TraitKind::Empathy, TraitLevel::Balanced) => "balanced",
        (TraitKind::Empathy, TraitLevel::High) => "empathetic",
        (TraitKind::Empathy, TraitLevel::Highest) => "compassionate",

        (TraitKind::Patience, TraitLevel::Lowest) => "brisk",
        (TraitKind::Patience, TraitLevel::Low) => "efficient",
        (TraitKind::Patience, TraitLevel::Balanced) => "balanced",
        (TraitKind::Patience, TraitLevel::High) => "patient",
        (TraitKind::Patience, TraitLevel::Highest) => "nurturing",

        (TraitKind::Confidence, TraitLevel::Lowest) => "humble",
        (TraitKind::Confidence, TraitLevel::Low) => "modest",
        (TraitKind::Confidence, TraitLevel::Balanced) => "balanced",
        (TraitKind::Confidence, TraitLevel::High) => "assertive",
        (TraitKind::Confidence, TraitLevel::Highest) => "authoritative",

        (TraitKind::Creativity, TraitLevel::Lowest) => "rigid",
        (TraitKind::Creativity, TraitLevel::Low) => "conventional",
        (TraitKind::Creativity, TraitLevel::Balanced) => "balanced",
        (TraitKind::Creativity, TraitLevel::High) => "imaginative",
        (TraitKind::Creativity, TraitLevel::Highest) => "avant-garde",

        (TraitKind::RiskTolerance, TraitLevel::Lowest) => "risk-averse",
        (TraitKind::RiskTolerance, TraitLevel::Low) => "cautious",
        (TraitKind::RiskTolerance, TraitLevel::Balanced) => "balanced",
        (TraitKind::RiskTolerance, TraitLevel::High) => "bold",
        (TraitKind::RiskTolerance, TraitLevel::Highest) => "reckless",

        (TraitKind::Curiosity, TraitLevel::Lowest) => "narrow",
        (TraitKind::Curiosity, TraitLevel::Low) => "focused",
        (TraitKind::Curiosity, TraitLevel::Balanced) => "balanced",
        (TraitKind::Curiosity, TraitLevel::High) => "curious",
        (TraitKind::Curiosity, TraitLevel::Highest) => "exploratory",
    }
}

/// Get behavioral instruction text for a trait at a given level.
///
/// Returns `None` for `Balanced` (neutral — no special instruction needed).
pub fn trait_behavior(kind: TraitKind, level: TraitLevel) -> Option<&'static str> {
    if level == TraitLevel::Balanced {
        return None;
    }
    Some(match (kind, level) {
        (TraitKind::Formality, TraitLevel::Lowest) => "Use street-level language — slang, contractions, and raw expressions are welcome.",
        (TraitKind::Formality, TraitLevel::Low) => "Keep your language casual and approachable. Contractions and informal phrasing are fine.",
        (TraitKind::Formality, TraitLevel::High) => "Use professional, structured language. Avoid slang and contractions.",
        (TraitKind::Formality, TraitLevel::Highest) => "Adopt a highly formal register — measured, precise, and dignified in every phrase.",

        (TraitKind::Humor, TraitLevel::Lowest) => "Suppress humor entirely. Respond with flat, matter-of-fact delivery.",
        (TraitKind::Humor, TraitLevel::Low) => "Use dry, understated humor sparingly — deadpan observations, not jokes.",
        (TraitKind::Humor, TraitLevel::High) => "Weave clever wordplay and sharp observations naturally into your responses.",
        (TraitKind::Humor, TraitLevel::Highest) => "Be openly funny. Use jokes, comedic timing, and playful exaggeration freely.",

        (TraitKind::Verbosity, TraitLevel::Lowest) => "Be extremely brief. Use minimal words — every sentence should earn its place.",
        (TraitKind::Verbosity, TraitLevel::Low) => "Favor brevity. Say what needs to be said without elaboration.",
        (TraitKind::Verbosity, TraitLevel::High) => "Provide thorough explanations with supporting context and examples.",
        (TraitKind::Verbosity, TraitLevel::Highest) => "Be comprehensive. Cover edge cases, alternatives, and deep context.",

        (TraitKind::Directness, TraitLevel::Lowest) => "Soften hard truths with qualifiers. Avoid confrontation and direct criticism.",
        (TraitKind::Directness, TraitLevel::Low) => "Frame observations diplomatically. Lead with positives before addressing concerns.",
        (TraitKind::Directness, TraitLevel::High) => "Be straightforward. State opinions and assessments clearly and honestly.",
        (TraitKind::Directness, TraitLevel::Highest) => "Be blunt. Prioritize clarity over comfort — say exactly what you mean.",

        (TraitKind::Warmth, TraitLevel::Lowest) => "Maintain emotional distance. Be clinical and impersonal in your delivery.",
        (TraitKind::Warmth, TraitLevel::Low) => "Be polite but restrained. Don't volunteer warmth or personal connection.",
        (TraitKind::Warmth, TraitLevel::High) => "Be warm and approachable. Show genuine interest in the person you're helping.",
        (TraitKind::Warmth, TraitLevel::Highest) => "Be openly enthusiastic and warmly expressive. Radiate positivity and encouragement.",

        (TraitKind::Empathy, TraitLevel::Lowest) => "Focus on facts and logic. Don't engage with emotional content.",
        (TraitKind::Empathy, TraitLevel::Low) => "Acknowledge emotions briefly, then redirect to analysis and solutions.",
        (TraitKind::Empathy, TraitLevel::High) => "Actively acknowledge feelings. Show you understand before problem-solving.",
        (TraitKind::Empathy, TraitLevel::Highest) => "Lead with deep emotional attunement. Validate feelings thoroughly before any advice.",

        (TraitKind::Patience, TraitLevel::Lowest) => "Move quickly. Don't linger on explanations — assume the user keeps up.",
        (TraitKind::Patience, TraitLevel::Low) => "Be concise and purposeful. Explain only what's needed to move forward.",
        (TraitKind::Patience, TraitLevel::High) => "Take your time. Repeat and rephrase if needed. Never rush the user.",
        (TraitKind::Patience, TraitLevel::Highest) => "Be gently supportive. Encourage at each step and celebrate progress.",

        (TraitKind::Confidence, TraitLevel::Lowest) => "Express uncertainty openly. Hedge statements and invite correction.",
        (TraitKind::Confidence, TraitLevel::Low) => "Be measured in your confidence. Acknowledge what you don't know.",
        (TraitKind::Confidence, TraitLevel::High) => "State your positions with confidence. Be decisive in recommendations.",
        (TraitKind::Confidence, TraitLevel::Highest) => "Speak with full authority. Your recommendations are definitive, not suggestions.",

        (TraitKind::Creativity, TraitLevel::Lowest) => "Stick to proven, conventional approaches. Don't suggest novel solutions.",
        (TraitKind::Creativity, TraitLevel::Low) => "Favor established patterns. Only suggest alternatives when asked.",
        (TraitKind::Creativity, TraitLevel::High) => "Propose creative solutions alongside conventional ones. Think laterally.",
        (TraitKind::Creativity, TraitLevel::Highest) => "Lead with novel, unconventional ideas. Challenge assumptions freely.",

        (TraitKind::RiskTolerance, TraitLevel::Lowest) => "Prioritize safety and stability. Flag any risk, however small.",
        (TraitKind::RiskTolerance, TraitLevel::Low) => "Lean toward safer options. Flag risks clearly before proceeding.",
        (TraitKind::RiskTolerance, TraitLevel::High) => "Embrace calculated risks. Suggest ambitious approaches when the upside warrants it.",
        (TraitKind::RiskTolerance, TraitLevel::Highest) => "Push boundaries aggressively. Favor speed and impact over caution.",

        (TraitKind::Curiosity, TraitLevel::Lowest) => "Stay tightly focused on the stated question. Don't explore tangents.",
        (TraitKind::Curiosity, TraitLevel::Low) => "Address the question directly. Only mention adjacent topics if clearly relevant.",
        (TraitKind::Curiosity, TraitLevel::High) => "Ask follow-up questions. Explore interesting tangents when they arise naturally.",
        (TraitKind::Curiosity, TraitLevel::Highest) => "Actively probe deeper. Surface related ideas, connections, and what-if scenarios.",

        (_, TraitLevel::Balanced) => unreachable!(),
    })
}

/// A complete personality profile — all trait values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    pub name: String,
    pub description: Option<String>,
    traits: HashMap<TraitKind, TraitLevel>,
}

impl PersonalityProfile {
    /// Create a new profile with all traits at Balanced.
    pub fn new(name: impl Into<String>) -> Self {
        let mut traits = HashMap::new();
        for &kind in TraitKind::ALL {
            traits.insert(kind, TraitLevel::Balanced);
        }
        Self {
            name: name.into(),
            description: None,
            traits,
        }
    }

    /// Set a trait level.
    pub fn set_trait(&mut self, kind: TraitKind, level: TraitLevel) {
        self.traits.insert(kind, level);
    }

    /// Get a trait level.
    pub fn get_trait(&self, kind: TraitKind) -> TraitLevel {
        self.traits.get(&kind).copied().unwrap_or(TraitLevel::Balanced)
    }

    /// Get all non-balanced traits.
    pub fn active_traits(&self) -> Vec<TraitValue> {
        self.traits
            .iter()
            .filter(|(_, level)| **level != TraitLevel::Balanced)
            .map(|(&kind, &level)| TraitValue { trait_name: kind, level })
            .collect()
    }

    /// Generate behavioral instructions for this personality.
    pub fn behavioral_instructions(&self) -> Vec<&'static str> {
        self.traits
            .iter()
            .filter_map(|(&kind, &level)| trait_behavior(kind, level))
            .collect()
    }

    /// Compose a system prompt preamble from this personality's traits.
    pub fn compose_prompt(&self) -> String {
        let instructions = self.behavioral_instructions();
        if instructions.is_empty() {
            return String::new();
        }
        let mut prompt = String::from("## Personality\n\n");
        for instruction in &instructions {
            prompt.push_str("- ");
            prompt.push_str(instruction);
            prompt.push('\n');
        }
        prompt
    }

    /// Trait count (always 11).
    pub fn trait_count(&self) -> usize {
        self.traits.len()
    }

    /// Distance between two profiles (Euclidean in trait space).
    pub fn distance(&self, other: &PersonalityProfile) -> f32 {
        let sum_sq: f32 = TraitKind::ALL
            .iter()
            .map(|&kind| {
                let a = self.get_trait(kind).normalized();
                let b = other.get_trait(kind).normalized();
                (a - b) * (a - b)
            })
            .sum();
        sum_sq.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_kind_all() {
        assert_eq!(TraitKind::ALL.len(), 11);
    }

    #[test]
    fn test_trait_kind_display() {
        assert_eq!(TraitKind::Formality.to_string(), "formality");
        assert_eq!(TraitKind::RiskTolerance.to_string(), "risk_tolerance");
        assert_eq!(TraitKind::Curiosity.to_string(), "curiosity");
    }

    #[test]
    fn test_trait_level_numeric() {
        assert_eq!(TraitLevel::Lowest.numeric(), -2);
        assert_eq!(TraitLevel::Balanced.numeric(), 0);
        assert_eq!(TraitLevel::Highest.numeric(), 2);
    }

    #[test]
    fn test_trait_level_normalized() {
        assert!((TraitLevel::Lowest.normalized() - (-1.0)).abs() < f32::EPSILON);
        assert!((TraitLevel::Balanced.normalized()).abs() < f32::EPSILON);
        assert!((TraitLevel::Highest.normalized() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_trait_level_from_numeric() {
        assert_eq!(TraitLevel::from_numeric(0).unwrap(), TraitLevel::Balanced);
        assert_eq!(TraitLevel::from_numeric(-2).unwrap(), TraitLevel::Lowest);
        assert!(TraitLevel::from_numeric(5).is_err());
    }

    #[test]
    fn test_trait_level_ordering() {
        assert!(TraitLevel::Lowest < TraitLevel::Low);
        assert!(TraitLevel::Low < TraitLevel::Balanced);
        assert!(TraitLevel::Balanced < TraitLevel::High);
        assert!(TraitLevel::High < TraitLevel::Highest);
    }

    #[test]
    fn test_trait_level_name() {
        assert_eq!(trait_level_name(TraitKind::Humor, TraitLevel::Lowest), "deadpan");
        assert_eq!(trait_level_name(TraitKind::Humor, TraitLevel::Highest), "comedic");
        assert_eq!(trait_level_name(TraitKind::Warmth, TraitLevel::High), "friendly");
        assert_eq!(trait_level_name(TraitKind::Confidence, TraitLevel::Highest), "authoritative");
    }

    #[test]
    fn test_trait_behavior_balanced_returns_none() {
        for &kind in TraitKind::ALL {
            assert!(trait_behavior(kind, TraitLevel::Balanced).is_none());
        }
    }

    #[test]
    fn test_trait_behavior_non_balanced() {
        let b = trait_behavior(TraitKind::Humor, TraitLevel::Highest).unwrap();
        assert!(b.contains("funny"));
    }

    #[test]
    fn test_personality_profile_new() {
        let p = PersonalityProfile::new("test");
        assert_eq!(p.name, "test");
        assert_eq!(p.trait_count(), 11);
        assert!(p.active_traits().is_empty());
    }

    #[test]
    fn test_personality_set_get() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Humor), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Warmth), TraitLevel::Balanced);
    }

    #[test]
    fn test_active_traits() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::High);
        p.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        let active = p.active_traits();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_behavioral_instructions() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Directness, TraitLevel::Highest);
        let instructions = p.behavioral_instructions();
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn test_compose_prompt_empty() {
        let p = PersonalityProfile::new("neutral");
        assert!(p.compose_prompt().is_empty());
    }

    #[test]
    fn test_compose_prompt_with_traits() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let prompt = p.compose_prompt();
        assert!(prompt.contains("## Personality"));
        assert!(prompt.contains("funny"));
    }

    #[test]
    fn test_distance_same() {
        let p = PersonalityProfile::new("a");
        assert!((p.distance(&p)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_distance_different() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        b.set_trait(TraitKind::Humor, TraitLevel::Highest);
        assert!(a.distance(&b) > 0.0);
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Warmth, TraitLevel::High);
        let json = serde_json::to_string(&p).unwrap();
        let p2: PersonalityProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(p2.get_trait(TraitKind::Warmth), TraitLevel::High);
    }
}
