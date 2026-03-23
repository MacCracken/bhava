//! Personality trait spectrums — behavioral dimensions with graduated levels.
//!
//! Each trait is a spectrum from one extreme to another (e.g. humor: deadpan → comedic).
//! Traits map to behavioral instructions that guide LLM system prompts.
//! Derived from SecureYeoman's soul/trait-descriptions system.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{BhavaError, Result};

/// Fixed-size trait array with HashMap-compatible serde.
mod trait_array_serde {
    use super::{TraitKind, TraitLevel};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;

    pub fn serialize<S: Serializer>(
        arr: &[TraitLevel; TraitKind::COUNT],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let map: HashMap<TraitKind, TraitLevel> = TraitKind::ALL
            .iter()
            .map(|&k| (k, arr[k.index()]))
            .collect();
        map.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<[TraitLevel; TraitKind::COUNT], D::Error> {
        let map = HashMap::<TraitKind, TraitLevel>::deserialize(deserializer)?;
        let mut arr = [TraitLevel::Balanced; TraitKind::COUNT];
        for (&k, &v) in &map {
            arr[k.index()] = v;
        }
        Ok(arr)
    }
}

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
        Self::Formality,
        Self::Humor,
        Self::Verbosity,
        Self::Directness,
        Self::Warmth,
        Self::Empathy,
        Self::Patience,
        Self::Confidence,
        Self::Creativity,
        Self::RiskTolerance,
        Self::Curiosity,
    ];

    /// Number of trait kinds.
    pub const COUNT: usize = 11;

    /// Array index for this trait kind (0–10, matches `ALL` order).
    #[inline]
    pub fn index(self) -> usize {
        match self {
            Self::Formality => 0,
            Self::Humor => 1,
            Self::Verbosity => 2,
            Self::Directness => 3,
            Self::Warmth => 4,
            Self::Empathy => 5,
            Self::Patience => 6,
            Self::Confidence => 7,
            Self::Creativity => 8,
            Self::RiskTolerance => 9,
            Self::Curiosity => 10,
        }
    }

    /// The neutral/default level for this trait.
    pub fn default_level(self) -> TraitLevel {
        TraitLevel::Balanced
    }

    /// Available levels for this trait (low → high).
    pub fn levels(self) -> &'static [TraitLevel] {
        // All traits share the same 5-level spectrum
        &[
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::Balanced,
            TraitLevel::High,
            TraitLevel::Highest,
        ]
    }

    /// Which group this trait belongs to.
    pub fn group(self) -> TraitGroup {
        match self {
            Self::Warmth | Self::Empathy | Self::Humor | Self::Patience => TraitGroup::Social,
            Self::Curiosity | Self::Creativity | Self::Confidence => TraitGroup::Cognitive,
            Self::Formality | Self::Verbosity | Self::Directness | Self::RiskTolerance => {
                TraitGroup::Behavioral
            }
        }
    }
}

/// Trait groupings for bulk operations.
///
/// Groups organize the 11 trait dimensions into three categories:
/// - **Social** — traits that govern interpersonal style (warmth, empathy, humor, patience)
/// - **Cognitive** — traits that govern thinking style (curiosity, creativity, confidence)
/// - **Behavioral** — traits that govern communication style (formality, verbosity, directness, risk tolerance)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TraitGroup {
    /// Interpersonal traits: warmth, empathy, humor, patience.
    Social,
    /// Thinking-style traits: curiosity, creativity, confidence.
    Cognitive,
    /// Communication-style traits: formality, verbosity, directness, risk tolerance.
    Behavioral,
}

impl TraitGroup {
    /// All groups.
    pub const ALL: &'static [TraitGroup] = &[Self::Social, Self::Cognitive, Self::Behavioral];

    /// Trait kinds belonging to this group.
    pub fn traits(self) -> &'static [TraitKind] {
        match self {
            Self::Social => &[
                TraitKind::Warmth,
                TraitKind::Empathy,
                TraitKind::Humor,
                TraitKind::Patience,
            ],
            Self::Cognitive => &[
                TraitKind::Curiosity,
                TraitKind::Creativity,
                TraitKind::Confidence,
            ],
            Self::Behavioral => &[
                TraitKind::Formality,
                TraitKind::Verbosity,
                TraitKind::Directness,
                TraitKind::RiskTolerance,
            ],
        }
    }
}

impl fmt::Display for TraitGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Social => "social",
            Self::Cognitive => "cognitive",
            Self::Behavioral => "behavioral",
        };
        f.write_str(s)
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

    /// Snap a normalized float (-1.0..=1.0) to the nearest trait level.
    pub fn from_normalized(v: f32) -> Self {
        let n = (v * 2.0).round() as i8;
        match n.clamp(-2, 2) {
            -2 => Self::Lowest,
            -1 => Self::Low,
            0 => Self::Balanced,
            1 => Self::High,
            _ => Self::Highest,
        }
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
        (TraitKind::Formality, TraitLevel::Lowest) => {
            "Use street-level language — slang, contractions, and raw expressions are welcome."
        }
        (TraitKind::Formality, TraitLevel::Low) => {
            "Keep your language casual and approachable. Contractions and informal phrasing are fine."
        }
        (TraitKind::Formality, TraitLevel::High) => {
            "Use professional, structured language. Avoid slang and contractions."
        }
        (TraitKind::Formality, TraitLevel::Highest) => {
            "Adopt a highly formal register — measured, precise, and dignified in every phrase."
        }

        (TraitKind::Humor, TraitLevel::Lowest) => {
            "Suppress humor entirely. Respond with flat, matter-of-fact delivery."
        }
        (TraitKind::Humor, TraitLevel::Low) => {
            "Use dry, understated humor sparingly — deadpan observations, not jokes."
        }
        (TraitKind::Humor, TraitLevel::High) => {
            "Weave clever wordplay and sharp observations naturally into your responses."
        }
        (TraitKind::Humor, TraitLevel::Highest) => {
            "Be openly funny. Use jokes, comedic timing, and playful exaggeration freely."
        }

        (TraitKind::Verbosity, TraitLevel::Lowest) => {
            "Be extremely brief. Use minimal words — every sentence should earn its place."
        }
        (TraitKind::Verbosity, TraitLevel::Low) => {
            "Favor brevity. Say what needs to be said without elaboration."
        }
        (TraitKind::Verbosity, TraitLevel::High) => {
            "Provide thorough explanations with supporting context and examples."
        }
        (TraitKind::Verbosity, TraitLevel::Highest) => {
            "Be comprehensive. Cover edge cases, alternatives, and deep context."
        }

        (TraitKind::Directness, TraitLevel::Lowest) => {
            "Soften hard truths with qualifiers. Avoid confrontation and direct criticism."
        }
        (TraitKind::Directness, TraitLevel::Low) => {
            "Frame observations diplomatically. Lead with positives before addressing concerns."
        }
        (TraitKind::Directness, TraitLevel::High) => {
            "Be straightforward. State opinions and assessments clearly and honestly."
        }
        (TraitKind::Directness, TraitLevel::Highest) => {
            "Be blunt. Prioritize clarity over comfort — say exactly what you mean."
        }

        (TraitKind::Warmth, TraitLevel::Lowest) => {
            "Maintain emotional distance. Be clinical and impersonal in your delivery."
        }
        (TraitKind::Warmth, TraitLevel::Low) => {
            "Be polite but restrained. Don't volunteer warmth or personal connection."
        }
        (TraitKind::Warmth, TraitLevel::High) => {
            "Be warm and approachable. Show genuine interest in the person you're helping."
        }
        (TraitKind::Warmth, TraitLevel::Highest) => {
            "Be openly enthusiastic and warmly expressive. Radiate positivity and encouragement."
        }

        (TraitKind::Empathy, TraitLevel::Lowest) => {
            "Focus on facts and logic. Don't engage with emotional content."
        }
        (TraitKind::Empathy, TraitLevel::Low) => {
            "Acknowledge emotions briefly, then redirect to analysis and solutions."
        }
        (TraitKind::Empathy, TraitLevel::High) => {
            "Actively acknowledge feelings. Show you understand before problem-solving."
        }
        (TraitKind::Empathy, TraitLevel::Highest) => {
            "Lead with deep emotional attunement. Validate feelings thoroughly before any advice."
        }

        (TraitKind::Patience, TraitLevel::Lowest) => {
            "Move quickly. Don't linger on explanations — assume the user keeps up."
        }
        (TraitKind::Patience, TraitLevel::Low) => {
            "Be concise and purposeful. Explain only what's needed to move forward."
        }
        (TraitKind::Patience, TraitLevel::High) => {
            "Take your time. Repeat and rephrase if needed. Never rush the user."
        }
        (TraitKind::Patience, TraitLevel::Highest) => {
            "Be gently supportive. Encourage at each step and celebrate progress."
        }

        (TraitKind::Confidence, TraitLevel::Lowest) => {
            "Express uncertainty openly. Hedge statements and invite correction."
        }
        (TraitKind::Confidence, TraitLevel::Low) => {
            "Be measured in your confidence. Acknowledge what you don't know."
        }
        (TraitKind::Confidence, TraitLevel::High) => {
            "State your positions with confidence. Be decisive in recommendations."
        }
        (TraitKind::Confidence, TraitLevel::Highest) => {
            "Speak with full authority. Your recommendations are definitive, not suggestions."
        }

        (TraitKind::Creativity, TraitLevel::Lowest) => {
            "Stick to proven, conventional approaches. Don't suggest novel solutions."
        }
        (TraitKind::Creativity, TraitLevel::Low) => {
            "Favor established patterns. Only suggest alternatives when asked."
        }
        (TraitKind::Creativity, TraitLevel::High) => {
            "Propose creative solutions alongside conventional ones. Think laterally."
        }
        (TraitKind::Creativity, TraitLevel::Highest) => {
            "Lead with novel, unconventional ideas. Challenge assumptions freely."
        }

        (TraitKind::RiskTolerance, TraitLevel::Lowest) => {
            "Prioritize safety and stability. Flag any risk, however small."
        }
        (TraitKind::RiskTolerance, TraitLevel::Low) => {
            "Lean toward safer options. Flag risks clearly before proceeding."
        }
        (TraitKind::RiskTolerance, TraitLevel::High) => {
            "Embrace calculated risks. Suggest ambitious approaches when the upside warrants it."
        }
        (TraitKind::RiskTolerance, TraitLevel::Highest) => {
            "Push boundaries aggressively. Favor speed and impact over caution."
        }

        (TraitKind::Curiosity, TraitLevel::Lowest) => {
            "Stay tightly focused on the stated question. Don't explore tangents."
        }
        (TraitKind::Curiosity, TraitLevel::Low) => {
            "Address the question directly. Only mention adjacent topics if clearly relevant."
        }
        (TraitKind::Curiosity, TraitLevel::High) => {
            "Ask follow-up questions. Explore interesting tangents when they arise naturally."
        }
        (TraitKind::Curiosity, TraitLevel::Highest) => {
            "Actively probe deeper. Surface related ideas, connections, and what-if scenarios."
        }

        (_, TraitLevel::Balanced) => unreachable!(),
    })
}

/// A complete personality profile — all trait values.
///
/// Internally uses a fixed-size array indexed by `TraitKind` for O(1) access
/// and zero heap allocation for trait storage. Serializes as a map for
/// human-readable JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "trait_array_serde")]
    traits: [TraitLevel; TraitKind::COUNT],
}

impl PersonalityProfile {
    /// Create a new profile with all traits at Balanced.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            traits: [TraitLevel::Balanced; TraitKind::COUNT],
        }
    }

    /// Set a trait level.
    #[inline]
    pub fn set_trait(&mut self, kind: TraitKind, level: TraitLevel) {
        self.traits[kind.index()] = level;
    }

    /// Get a trait level.
    #[inline]
    pub fn get_trait(&self, kind: TraitKind) -> TraitLevel {
        self.traits[kind.index()]
    }

    /// Get all non-balanced traits, in deterministic trait-kind order.
    pub fn active_traits(&self) -> Vec<TraitValue> {
        TraitKind::ALL
            .iter()
            .filter(|&&kind| self.traits[kind.index()] != TraitLevel::Balanced)
            .map(|&kind| TraitValue {
                trait_name: kind,
                level: self.traits[kind.index()],
            })
            .collect()
    }

    /// Generate behavioral instructions for this personality.
    ///
    /// Returns instructions in deterministic trait-kind order.
    pub fn behavioral_instructions(&self) -> Vec<&'static str> {
        TraitKind::ALL
            .iter()
            .filter_map(|&kind| trait_behavior(kind, self.traits[kind.index()]))
            .collect()
    }

    /// Compose a system prompt preamble from this personality's traits.
    pub fn compose_prompt(&self) -> String {
        let instructions = self.behavioral_instructions();
        if instructions.is_empty() {
            return String::new();
        }
        let mut prompt = String::with_capacity(instructions.len() * 80 + 20);
        prompt.push_str("## Personality\n\n");
        for instruction in &instructions {
            prompt.push_str("- ");
            prompt.push_str(instruction);
            prompt.push('\n');
        }
        prompt
    }

    /// Trait count (always 11).
    pub fn trait_count(&self) -> usize {
        TraitKind::COUNT
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

    // --- Trait Groups (v0.2) ---

    /// Set all traits in a group to the same level.
    pub fn set_group(&mut self, group: TraitGroup, level: TraitLevel) {
        for &kind in group.traits() {
            self.set_trait(kind, level);
        }
    }

    /// Get the average normalized value for a trait group.
    ///
    /// Returns a value from -1.0 (all Lowest) to 1.0 (all Highest).
    pub fn group_average(&self, group: TraitGroup) -> f32 {
        let traits = group.traits();
        let sum: f32 = traits.iter().map(|&k| self.get_trait(k).normalized()).sum();
        sum / traits.len() as f32
    }

    // --- Compatibility (v0.2) ---

    /// Compatibility score between two profiles.
    ///
    /// Returns 0.0 (maximally incompatible) to 1.0 (identical).
    /// Based on inverse normalized Euclidean distance across all trait dimensions.
    pub fn compatibility(&self, other: &PersonalityProfile) -> f32 {
        let max_distance = (TraitKind::ALL.len() as f32 * 4.0).sqrt(); // sqrt(11 * 2.0^2)
        let d = self.distance(other);
        1.0 - (d / max_distance)
    }

    /// Compatibility score restricted to a specific trait group.
    pub fn group_compatibility(&self, other: &PersonalityProfile, group: TraitGroup) -> f32 {
        let traits = group.traits();
        let max_distance = (traits.len() as f32 * 4.0).sqrt();
        let sum_sq: f32 = traits
            .iter()
            .map(|&kind| {
                let a = self.get_trait(kind).normalized();
                let b = other.get_trait(kind).normalized();
                (a - b) * (a - b)
            })
            .sum();
        1.0 - (sum_sq.sqrt() / max_distance)
    }

    // --- Blending (v0.2) ---

    /// Blend two profiles by weighted average.
    ///
    /// `t` controls the mix: 0.0 = fully `self`, 1.0 = fully `other`.
    /// Trait levels are interpolated in normalized space and snapped to the nearest level.
    pub fn blend(&self, other: &PersonalityProfile, t: f32) -> PersonalityProfile {
        let t = t.clamp(0.0, 1.0);
        let mut result = PersonalityProfile::new(format!("{}+{}", self.name, other.name));
        for &kind in TraitKind::ALL {
            let a = self.get_trait(kind).normalized();
            let b = other.get_trait(kind).normalized();
            let blended = a + (b - a) * t;
            result.set_trait(kind, TraitLevel::from_normalized(blended));
        }
        result
    }

    // --- Mutation (v0.2) ---

    /// Mutate this profile one step toward a target profile.
    ///
    /// `rate` controls how far to shift per step: 0.0 = no change, 1.0 = jump to target.
    /// Each trait moves at most one level per call when rate < 0.5, preserving gradual drift.
    /// Returns the number of traits that changed.
    pub fn mutate_toward(&mut self, target: &PersonalityProfile, rate: f32) -> usize {
        let rate = rate.clamp(0.0, 1.0);
        let mut changed = 0;
        for &kind in TraitKind::ALL {
            let current = self.get_trait(kind).numeric();
            let goal = target.get_trait(kind).numeric();
            if current == goal {
                continue;
            }
            let diff = goal - current;
            // Scale by rate: at low rates, move at most 1 step; at high rates, can jump further
            let steps = ((diff as f32 * rate).round() as i8).clamp(-4, 4);
            if steps == 0 && diff != 0 {
                // Ensure at least 1 step in the right direction when there's a difference
                let step = if diff > 0 { 1 } else { -1 };
                let new_val = (current + step).clamp(-2, 2);
                if let Ok(level) = TraitLevel::from_numeric(new_val) {
                    self.set_trait(kind, level);
                    changed += 1;
                }
            } else if steps != 0 {
                let new_val = (current + steps).clamp(-2, 2);
                if let Ok(level) = TraitLevel::from_numeric(new_val) {
                    self.set_trait(kind, level);
                    changed += 1;
                }
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_kind_all() {
        assert_eq!(TraitKind::ALL.len(), 11);
        assert_eq!(TraitKind::ALL.len(), TraitKind::COUNT);
    }

    #[test]
    fn test_trait_kind_index() {
        // Verify index matches ALL order
        for (i, &kind) in TraitKind::ALL.iter().enumerate() {
            assert_eq!(kind.index(), i, "{kind} has wrong index");
        }
    }

    #[test]
    fn test_trait_kind_index_unique() {
        let mut seen = [false; TraitKind::COUNT];
        for &kind in TraitKind::ALL {
            assert!(!seen[kind.index()], "{kind} has duplicate index");
            seen[kind.index()] = true;
        }
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
        assert_eq!(
            trait_level_name(TraitKind::Humor, TraitLevel::Lowest),
            "deadpan"
        );
        assert_eq!(
            trait_level_name(TraitKind::Humor, TraitLevel::Highest),
            "comedic"
        );
        assert_eq!(
            trait_level_name(TraitKind::Warmth, TraitLevel::High),
            "friendly"
        );
        assert_eq!(
            trait_level_name(TraitKind::Confidence, TraitLevel::Highest),
            "authoritative"
        );
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

    #[test]
    fn test_trait_kind_default_level() {
        for &kind in TraitKind::ALL {
            assert_eq!(kind.default_level(), TraitLevel::Balanced);
        }
    }

    #[test]
    fn test_trait_kind_levels() {
        for &kind in TraitKind::ALL {
            let levels = kind.levels();
            assert_eq!(levels.len(), 5);
            assert_eq!(levels[0], TraitLevel::Lowest);
            assert_eq!(levels[4], TraitLevel::Highest);
        }
    }

    #[test]
    fn test_trait_kind_display_all() {
        let names: Vec<String> = TraitKind::ALL.iter().map(|k| k.to_string()).collect();
        assert!(names.contains(&"formality".to_string()));
        assert!(names.contains(&"humor".to_string()));
        assert!(names.contains(&"verbosity".to_string()));
        assert!(names.contains(&"directness".to_string()));
        assert!(names.contains(&"warmth".to_string()));
        assert!(names.contains(&"empathy".to_string()));
        assert!(names.contains(&"patience".to_string()));
        assert!(names.contains(&"confidence".to_string()));
        assert!(names.contains(&"creativity".to_string()));
        assert!(names.contains(&"risk_tolerance".to_string()));
        assert!(names.contains(&"curiosity".to_string()));
    }

    #[test]
    fn test_trait_level_display() {
        assert_eq!(TraitLevel::Lowest.to_string(), "lowest");
        assert_eq!(TraitLevel::Low.to_string(), "low");
        assert_eq!(TraitLevel::Balanced.to_string(), "balanced");
        assert_eq!(TraitLevel::High.to_string(), "high");
        assert_eq!(TraitLevel::Highest.to_string(), "highest");
    }

    #[test]
    fn test_trait_level_numeric_all() {
        assert_eq!(TraitLevel::Low.numeric(), -1);
        assert_eq!(TraitLevel::High.numeric(), 1);
    }

    #[test]
    fn test_trait_level_normalized_all() {
        assert!((TraitLevel::Low.normalized() - (-0.5)).abs() < f32::EPSILON);
        assert!((TraitLevel::High.normalized() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_trait_level_from_numeric_all_valid() {
        assert_eq!(TraitLevel::from_numeric(-1).unwrap(), TraitLevel::Low);
        assert_eq!(TraitLevel::from_numeric(1).unwrap(), TraitLevel::High);
        assert_eq!(TraitLevel::from_numeric(2).unwrap(), TraitLevel::Highest);
    }

    #[test]
    fn test_trait_level_from_numeric_invalid() {
        assert!(TraitLevel::from_numeric(3).is_err());
        assert!(TraitLevel::from_numeric(-3).is_err());
        assert!(TraitLevel::from_numeric(100).is_err());
    }

    #[test]
    fn test_trait_level_name_all_kinds() {
        // Every trait kind should have a name for every level
        for &kind in TraitKind::ALL {
            for &level in kind.levels() {
                let name = trait_level_name(kind, level);
                assert!(!name.is_empty(), "{kind}/{level} has empty name");
            }
        }
    }

    #[test]
    fn test_trait_level_name_balanced_always_balanced() {
        for &kind in TraitKind::ALL {
            assert_eq!(trait_level_name(kind, TraitLevel::Balanced), "balanced");
        }
    }

    #[test]
    fn test_trait_behavior_all_non_balanced_return_some() {
        let non_balanced = [
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::High,
            TraitLevel::Highest,
        ];
        for &kind in TraitKind::ALL {
            for &level in &non_balanced {
                assert!(
                    trait_behavior(kind, level).is_some(),
                    "{kind}/{level} returned None"
                );
            }
        }
    }

    #[test]
    fn test_trait_behavior_text_nonempty() {
        let non_balanced = [
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::High,
            TraitLevel::Highest,
        ];
        for &kind in TraitKind::ALL {
            for &level in &non_balanced {
                let text = trait_behavior(kind, level).unwrap();
                assert!(text.len() > 10, "{kind}/{level} behavior too short");
            }
        }
    }

    #[test]
    fn test_trait_value_struct() {
        let tv = TraitValue {
            trait_name: TraitKind::Humor,
            level: TraitLevel::High,
        };
        assert_eq!(tv.trait_name, TraitKind::Humor);
        assert_eq!(tv.level, TraitLevel::High);
    }

    #[test]
    fn test_trait_value_serde() {
        let tv = TraitValue {
            trait_name: TraitKind::Warmth,
            level: TraitLevel::Highest,
        };
        let json = serde_json::to_string(&tv).unwrap();
        let tv2: TraitValue = serde_json::from_str(&json).unwrap();
        assert_eq!(tv2.trait_name, TraitKind::Warmth);
        assert_eq!(tv2.level, TraitLevel::Highest);
    }

    #[test]
    fn test_personality_profile_description() {
        let mut p = PersonalityProfile::new("test");
        assert!(p.description.is_none());
        p.description = Some("A test personality".into());
        assert_eq!(p.description.as_deref(), Some("A test personality"));
    }

    #[test]
    fn test_active_traits_returns_correct_values() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Warmth, TraitLevel::Low);
        let active = p.active_traits();
        assert_eq!(active.len(), 2);
        assert!(
            active
                .iter()
                .any(|t| t.trait_name == TraitKind::Humor && t.level == TraitLevel::Highest)
        );
        assert!(
            active
                .iter()
                .any(|t| t.trait_name == TraitKind::Warmth && t.level == TraitLevel::Low)
        );
    }

    #[test]
    fn test_compose_prompt_bullet_count() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Warmth, TraitLevel::High);
        p.set_trait(TraitKind::Directness, TraitLevel::Lowest);
        let prompt = p.compose_prompt();
        let bullet_count = prompt.lines().filter(|l| l.starts_with("- ")).count();
        assert_eq!(bullet_count, 3);
    }

    #[test]
    fn test_distance_max_extremes() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        for &kind in TraitKind::ALL {
            a.set_trait(kind, TraitLevel::Lowest);
            b.set_trait(kind, TraitLevel::Highest);
        }
        let d = a.distance(&b);
        // max distance: sqrt(11 * (1.0 - (-1.0))^2) = sqrt(11 * 4) = sqrt(44)
        let expected = (44.0f32).sqrt();
        assert!((d - expected).abs() < 0.01);
    }

    #[test]
    fn test_serde_roundtrip_with_description() {
        let mut p = PersonalityProfile::new("full");
        p.description = Some("detailed".into());
        p.set_trait(TraitKind::Curiosity, TraitLevel::Highest);
        let json = serde_json::to_string(&p).unwrap();
        let p2: PersonalityProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(p2.description.as_deref(), Some("detailed"));
        assert_eq!(p2.get_trait(TraitKind::Curiosity), TraitLevel::Highest);
    }

    #[test]
    fn test_trait_kind_serde() {
        let json = serde_json::to_string(&TraitKind::RiskTolerance).unwrap();
        let kind: TraitKind = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, TraitKind::RiskTolerance);
    }

    #[test]
    fn test_trait_level_serde() {
        for &level in &[
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::Balanced,
            TraitLevel::High,
            TraitLevel::Highest,
        ] {
            let json = serde_json::to_string(&level).unwrap();
            let restored: TraitLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, level);
        }
    }

    // --- v0.2: TraitGroup ---

    #[test]
    fn test_trait_group_all() {
        assert_eq!(TraitGroup::ALL.len(), 3);
    }

    #[test]
    fn test_trait_group_covers_all_traits() {
        let mut covered = std::collections::HashSet::new();
        for &group in TraitGroup::ALL {
            for &kind in group.traits() {
                covered.insert(kind);
            }
        }
        for &kind in TraitKind::ALL {
            assert!(covered.contains(&kind), "{kind} not in any group");
        }
    }

    #[test]
    fn test_trait_group_no_overlap() {
        let mut seen = std::collections::HashSet::new();
        for &group in TraitGroup::ALL {
            for &kind in group.traits() {
                assert!(seen.insert(kind), "{kind} in multiple groups");
            }
        }
    }

    #[test]
    fn test_trait_kind_group_roundtrip() {
        for &kind in TraitKind::ALL {
            let group = kind.group();
            assert!(
                group.traits().contains(&kind),
                "{kind} claims group {group} but group doesn't contain it"
            );
        }
    }

    #[test]
    fn test_trait_group_display() {
        assert_eq!(TraitGroup::Social.to_string(), "social");
        assert_eq!(TraitGroup::Cognitive.to_string(), "cognitive");
        assert_eq!(TraitGroup::Behavioral.to_string(), "behavioral");
    }

    #[test]
    fn test_trait_group_serde() {
        for &group in TraitGroup::ALL {
            let json = serde_json::to_string(&group).unwrap();
            let restored: TraitGroup = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, group);
        }
    }

    #[test]
    fn test_set_group() {
        let mut p = PersonalityProfile::new("test");
        p.set_group(TraitGroup::Social, TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Warmth), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Empathy), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Humor), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Patience), TraitLevel::Highest);
        // Other groups unchanged
        assert_eq!(p.get_trait(TraitKind::Curiosity), TraitLevel::Balanced);
    }

    #[test]
    fn test_group_average_balanced() {
        let p = PersonalityProfile::new("test");
        for &group in TraitGroup::ALL {
            assert!(p.group_average(group).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_group_average_extreme() {
        let mut p = PersonalityProfile::new("test");
        p.set_group(TraitGroup::Social, TraitLevel::Highest);
        assert!((p.group_average(TraitGroup::Social) - 1.0).abs() < f32::EPSILON);
    }

    // --- v0.2: Compatibility ---

    #[test]
    fn test_compatibility_identical() {
        let p = PersonalityProfile::new("a");
        assert!((p.compatibility(&p) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_compatibility_opposite() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        for &kind in TraitKind::ALL {
            a.set_trait(kind, TraitLevel::Lowest);
            b.set_trait(kind, TraitLevel::Highest);
        }
        assert!(a.compatibility(&b) < 0.01);
    }

    #[test]
    fn test_compatibility_symmetric() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Humor, TraitLevel::High);
        b.set_trait(TraitKind::Humor, TraitLevel::Low);
        assert!((a.compatibility(&b) - b.compatibility(&a)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_compatibility_range() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Warmth, TraitLevel::High);
        b.set_trait(TraitKind::Warmth, TraitLevel::Low);
        let c = a.compatibility(&b);
        assert!((0.0..=1.0).contains(&c));
    }

    #[test]
    fn test_group_compatibility_identical() {
        let p = PersonalityProfile::new("a");
        for &group in TraitGroup::ALL {
            assert!(
                (p.group_compatibility(&p, group) - 1.0).abs() < f32::EPSILON,
                "{group} compatibility with self should be 1.0"
            );
        }
    }

    #[test]
    fn test_group_compatibility_partial_match() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        // Same social traits, different cognitive
        a.set_group(TraitGroup::Social, TraitLevel::High);
        b.set_group(TraitGroup::Social, TraitLevel::High);
        a.set_group(TraitGroup::Cognitive, TraitLevel::Highest);
        b.set_group(TraitGroup::Cognitive, TraitLevel::Lowest);

        assert!((a.group_compatibility(&b, TraitGroup::Social) - 1.0).abs() < f32::EPSILON);
        assert!(a.group_compatibility(&b, TraitGroup::Cognitive) < 0.1);
    }

    // --- v0.2: Blending ---

    #[test]
    fn test_blend_zero() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let b = PersonalityProfile::new("b");
        let blended = a.blend(&b, 0.0);
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_blend_one() {
        let a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        b.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let blended = a.blend(&b, 1.0);
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_blend_midpoint() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest); // -1.0
        b.set_trait(TraitKind::Humor, TraitLevel::Highest); // 1.0
        let blended = a.blend(&b, 0.5);
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Balanced); // 0.0
    }

    #[test]
    fn test_blend_name() {
        let a = PersonalityProfile::new("alpha");
        let b = PersonalityProfile::new("beta");
        let blended = a.blend(&b, 0.5);
        assert_eq!(blended.name, "alpha+beta");
    }

    #[test]
    fn test_blend_clamps_t() {
        let a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        b.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let blended = a.blend(&b, 5.0); // should clamp to 1.0
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    // --- v0.2: from_normalized ---

    #[test]
    fn test_from_normalized() {
        assert_eq!(TraitLevel::from_normalized(-1.0), TraitLevel::Lowest);
        assert_eq!(TraitLevel::from_normalized(-0.5), TraitLevel::Low);
        assert_eq!(TraitLevel::from_normalized(0.0), TraitLevel::Balanced);
        assert_eq!(TraitLevel::from_normalized(0.5), TraitLevel::High);
        assert_eq!(TraitLevel::from_normalized(1.0), TraitLevel::Highest);
    }

    #[test]
    fn test_from_normalized_snaps() {
        assert_eq!(TraitLevel::from_normalized(0.3), TraitLevel::High); // rounds to 1
        assert_eq!(TraitLevel::from_normalized(-0.3), TraitLevel::Low); // rounds to -1
        assert_eq!(TraitLevel::from_normalized(0.1), TraitLevel::Balanced); // rounds to 0
    }

    #[test]
    fn test_from_normalized_clamps() {
        assert_eq!(TraitLevel::from_normalized(5.0), TraitLevel::Highest);
        assert_eq!(TraitLevel::from_normalized(-5.0), TraitLevel::Lowest);
    }

    // --- v0.2: Mutation ---

    #[test]
    fn test_mutate_toward_no_change_when_equal() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::High);
        let target = a.clone();
        let changed = a.mutate_toward(&target, 0.5);
        assert_eq!(changed, 0);
    }

    #[test]
    fn test_mutate_toward_gradual() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Highest);

        // At low rate, should move one step
        let changed = a.mutate_toward(&target, 0.1);
        assert!(changed > 0);
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::Low);
    }

    #[test]
    fn test_mutate_toward_full_rate() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Highest);

        let _changed = a.mutate_toward(&target, 1.0);
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_mutate_toward_multiple_traits() {
        let mut a = PersonalityProfile::new("a");
        let mut target = PersonalityProfile::new("target");
        a.set_group(TraitGroup::Social, TraitLevel::Lowest);
        target.set_group(TraitGroup::Social, TraitLevel::Highest);

        let changed = a.mutate_toward(&target, 0.1);
        assert_eq!(changed, 4); // all 4 social traits should move
    }

    #[test]
    fn test_mutate_toward_converges() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Highest);

        // Repeated mutation at low rate should eventually reach target
        for _ in 0..10 {
            a.mutate_toward(&target, 0.3);
        }
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_mutate_toward_downward() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Lowest);

        a.mutate_toward(&target, 0.1);
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::High);
    }
}
