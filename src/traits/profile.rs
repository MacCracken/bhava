use serde::{Deserialize, Serialize};

use super::descriptions::{trait_behavior, trait_level_name};
use super::kind::{TraitGroup, TraitKind, TraitLevel, TraitValue};

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

/// Cosine similarity between two float iterators.
///
/// Returns a value from -1.0 (opposite) to 1.0 (identical direction).
/// Mapped to 0.0–1.0 range: `(cos + 1) / 2`. Returns 1.0 for two zero vectors.
fn cosine_similarity(a: impl Iterator<Item = f32>, b: impl Iterator<Item = f32>) -> f32 {
    let mut dot = 0.0f32;
    let mut mag_a = 0.0f32;
    let mut mag_b = 0.0f32;

    for (va, vb) in a.zip(b) {
        dot += va * vb;
        mag_a += va * va;
        mag_b += vb * vb;
    }

    let denom = mag_a.sqrt() * mag_b.sqrt();
    if denom < f32::EPSILON {
        return 1.0; // two zero vectors are identical by convention
    }

    // Raw cosine is -1..1, map to 0..1
    let cos = dot / denom;
    (cos + 1.0) / 2.0
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn behavioral_instructions(&self) -> Vec<&'static str> {
        TraitKind::ALL
            .iter()
            .filter_map(|&kind| trait_behavior(kind, self.traits[kind.index()]))
            .collect()
    }

    /// Compose a system prompt preamble from this personality's traits.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn group_average(&self, group: TraitGroup) -> f32 {
        let traits = group.traits();
        let sum: f32 = traits.iter().map(|&k| self.get_trait(k).normalized()).sum();
        sum / traits.len() as f32
    }

    // --- Compatibility (v0.2) ---

    /// Compatibility score between two profiles using cosine similarity.
    ///
    /// Returns 0.0 (orthogonal/incompatible) to 1.0 (identical pattern).
    /// Cosine similarity measures the angle between two trait vectors,
    /// capturing behavioral pattern similarity regardless of intensity differences.
    ///
    /// For two all-Balanced profiles (zero vectors), returns 1.0 by convention.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn compatibility(&self, other: &PersonalityProfile) -> f32 {
        cosine_similarity(
            TraitKind::ALL
                .iter()
                .map(|&k| self.get_trait(k).normalized()),
            TraitKind::ALL
                .iter()
                .map(|&k| other.get_trait(k).normalized()),
        )
    }

    /// Compatibility score restricted to a specific trait group.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn group_compatibility(&self, other: &PersonalityProfile, group: TraitGroup) -> f32 {
        let traits = group.traits();
        cosine_similarity(
            traits.iter().map(|&k| self.get_trait(k).normalized()),
            traits.iter().map(|&k| other.get_trait(k).normalized()),
        )
    }

    // --- Blending (v0.2) ---

    /// Blend two profiles by weighted average.
    ///
    /// `t` controls the mix: 0.0 = fully `self`, 1.0 = fully `other`.
    /// Trait levels are interpolated in normalized space and snapped to the nearest level.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn mutate_toward(&mut self, target: &PersonalityProfile, rate: f32) -> usize {
        let rate = rate.clamp(0.0, 1.0);
        if rate < f32::EPSILON {
            return 0;
        }
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
            if steps == 0 {
                // At very low rate, ensure at least 1 step in the right direction
                let step = if diff > 0 { 1 } else { -1 };
                let new_val = (current + step).clamp(-2, 2);
                if let Ok(level) = TraitLevel::from_numeric(new_val) {
                    self.set_trait(kind, level);
                    changed += 1;
                }
            } else {
                let new_val = (current + steps).clamp(-2, 2);
                if let Ok(level) = TraitLevel::from_numeric(new_val) {
                    self.set_trait(kind, level);
                    changed += 1;
                }
            }
        }
        changed
    }

    // --- Serialization (SY parity) ---

    /// Export personality to a portable markdown format.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn to_markdown(&self) -> String {
        use std::fmt::Write;
        let mut md = String::with_capacity(512);
        let _ = writeln!(md, "# {}", self.name);
        if let Some(desc) = &self.description {
            let _ = writeln!(md, "\n{desc}");
        }
        md.push_str("\n## Traits\n\n");
        md.push_str("| Trait | Level | Name |\n");
        md.push_str("|-------|-------|------|\n");
        for &kind in TraitKind::ALL {
            let level = self.get_trait(kind);
            let name = trait_level_name(kind, level);
            let _ = writeln!(md, "| {kind} | {level} | {name} |");
        }
        md
    }

    /// Import personality from markdown format.
    ///
    /// Expects the format produced by `to_markdown()`. Unrecognized traits
    /// default to Balanced. Returns None if the name line is missing.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn from_markdown(md: &str) -> Option<Self> {
        let mut lines = md.lines();

        // First line: "# Name"
        let name_line = lines.next()?.trim();
        let name = name_line.strip_prefix("# ")?.trim();
        if name.is_empty() {
            return None;
        }

        let mut profile = PersonalityProfile::new(name);
        let mut in_description = false;
        let mut description_lines: Vec<&str> = Vec::new();

        for line in lines {
            let trimmed = line.trim();

            if trimmed == "## Traits" {
                if !description_lines.is_empty() {
                    profile.description = Some(description_lines.join("\n").trim().to_string());
                }
                in_description = false;
                continue;
            }

            if trimmed.starts_with("| Trait") || trimmed.starts_with("|---") {
                continue;
            }

            if trimmed.starts_with("| ") {
                // Parse trait row: "| kind | level | name |"
                let parts: Vec<&str> = trimmed.split('|').map(|s| s.trim()).collect();
                if parts.len() >= 4 {
                    let kind_str = parts[1];
                    let level_str = parts[2];
                    if let (Some(kind), Some(level)) =
                        (parse_trait_kind(kind_str), parse_trait_level(level_str))
                    {
                        profile.set_trait(kind, level);
                    }
                }
                continue;
            }

            if trimmed.starts_with("# ") {
                continue; // skip header re-encounters
            }

            // Accumulate description lines (between name and ## Traits)
            if !trimmed.is_empty() || in_description {
                in_description = true;
                description_lines.push(trimmed);
            }
        }

        Some(profile)
    }
}

/// Parse a trait kind from its display string.
pub(super) fn parse_trait_kind(s: &str) -> Option<TraitKind> {
    match s {
        "formality" => Some(TraitKind::Formality),
        "humor" => Some(TraitKind::Humor),
        "verbosity" => Some(TraitKind::Verbosity),
        "directness" => Some(TraitKind::Directness),
        "warmth" => Some(TraitKind::Warmth),
        "empathy" => Some(TraitKind::Empathy),
        "patience" => Some(TraitKind::Patience),
        "confidence" => Some(TraitKind::Confidence),
        "creativity" => Some(TraitKind::Creativity),
        "risk_tolerance" => Some(TraitKind::RiskTolerance),
        "curiosity" => Some(TraitKind::Curiosity),
        "skepticism" => Some(TraitKind::Skepticism),
        "autonomy" => Some(TraitKind::Autonomy),
        "pedagogy" => Some(TraitKind::Pedagogy),
        "precision" => Some(TraitKind::Precision),
        _ => None,
    }
}

/// Parse a trait level from its display string.
pub(super) fn parse_trait_level(s: &str) -> Option<TraitLevel> {
    match s {
        "lowest" => Some(TraitLevel::Lowest),
        "low" => Some(TraitLevel::Low),
        "balanced" => Some(TraitLevel::Balanced),
        "high" => Some(TraitLevel::High),
        "highest" => Some(TraitLevel::Highest),
        _ => None,
    }
}
