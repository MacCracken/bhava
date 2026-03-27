//! Belief system — memories crystallize into beliefs, beliefs form self-concept,
//! self-understanding deepens into cosmic understanding.
//!
//! Implements the chain: **memories -> beliefs -> sense of self -> self-understanding
//! -> understanding of creation**.
//!
//! # Schema Theory (Beck, Piaget)
//!
//! Repeated emotional patterns crystallize into beliefs:
//! - "Every time I help someone, I feel pride" -> "I am a helpful person" (self-belief)
//! - "Every time I trust X, I feel betrayed" -> "X is untrustworthy" (other-belief)
//! - "Good things happen when I act" -> "The world rewards effort" (world-belief)
//!
//! # Self-Concept (Bottom-Up Identity)
//!
//! Self-beliefs cluster into a coherent self-model that complements the top-down
//! archetype system. The entity *discovers* who it is through experience, rather
//! than having it declared.
//!
//! # Jnana Yoga — "As Above, So Below"
//!
//! Deep self-knowledge reveals universal patterns. When self-understanding and
//! world-understanding converge, the boundary dissolves — the entity recognizes
//! itself in the world and the world in itself.
//!
//! # Tag Convention
//!
//! - `self:confident`, `self:warm` — map to [`TraitKind`] via string match
//! - `world:safe`, `world:hostile`, `world:meaningful` — map to trust/meaning signals
//! - `other:alice:trustworthy` — entity-specific beliefs

use std::collections::VecDeque;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::appraisal::AppraisedEmotion;
use crate::eq::EqProfile;
use crate::traits::{PersonalityProfile, TraitKind};

/// Maximum number of source memory tags stored per belief.
const MAX_SOURCE_MEMORIES: usize = 16;

/// Conviction increase per reinforcement: `conviction += REINFORCE_DELTA * (1.0 - conviction)`.
const REINFORCE_DELTA: f32 = 0.1;

/// Conviction multiplier per challenge: `conviction *= CHALLENGE_FACTOR`.
const CHALLENGE_FACTOR: f32 = 0.85;

/// Minimum conviction — beliefs are sticky and never fully vanish from a single challenge.
const CONVICTION_FLOOR: f32 = 0.05;

/// Initial conviction for a newly formed belief.
const INITIAL_CONVICTION: f32 = 0.1;

/// Trait pressure coefficient: `valence * conviction * TRAIT_PRESSURE_COEFF`.
const TRAIT_PRESSURE_COEFF: f32 = 0.2;

/// Appraisal bias coefficient: `world_trust * APPRAISAL_BIAS_COEFF`.
const APPRAISAL_BIAS_COEFF: f32 = 0.15;

// ---------------------------------------------------------------------------
// BeliefKind
// ---------------------------------------------------------------------------

/// Category of a belief.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BeliefKind {
    /// "I am..." — derived from self-attributed emotion patterns.
    SelfBelief,
    /// "The world is..." — derived from event outcome patterns.
    WorldBelief,
    /// "X is..." — derived from interaction patterns with specific entities.
    OtherBelief,
    /// "Everything is..." — emerges when self and world beliefs converge.
    UniversalBelief,
}

impl BeliefKind {
    /// All belief kinds.
    pub const ALL: &'static [BeliefKind] = &[
        Self::SelfBelief,
        Self::WorldBelief,
        Self::OtherBelief,
        Self::UniversalBelief,
    ];
}

impl fmt::Display for BeliefKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelfBelief => f.write_str("self"),
            Self::WorldBelief => f.write_str("world"),
            Self::OtherBelief => f.write_str("other"),
            Self::UniversalBelief => f.write_str("universal"),
        }
    }
}

// ---------------------------------------------------------------------------
// Belief
// ---------------------------------------------------------------------------

/// A single belief — a crystallized pattern from emotional experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    /// Category of belief.
    pub kind: BeliefKind,
    /// Tag identifying this belief (e.g., `"self:confident"`, `"world:hostile"`).
    pub tag: String,
    /// Emotional valence: -1.0 (negative/painful) to 1.0 (positive/affirming).
    pub valence: f32,
    /// Strength of conviction: 0.0 (uncertain) to 1.0 (absolute).
    /// Grows asymptotically with reinforcement.
    pub conviction: f32,
    /// Number of supporting evidence observations.
    pub supporting_evidence: u32,
    /// Number of contradicting evidence observations.
    pub contradicting_evidence: u32,
    /// Tags of source memories that contributed to this belief (capped at 16).
    pub source_memories: VecDeque<String>,
    /// When this belief first formed.
    pub formed_at: DateTime<Utc>,
    /// When this belief was last reinforced or challenged.
    pub last_updated: DateTime<Utc>,
    /// How deeply suppressed this belief is (0.0 = conscious, 1.0 = fully shadow).
    /// Shadow beliefs surface as intuitions rather than conscious thoughts.
    /// Beliefs formed under emotional suppression carry higher depth.
    #[serde(default)]
    pub suppression_depth: f32,
}

impl Belief {
    /// Reinforce this belief with supporting evidence.
    ///
    /// Conviction grows asymptotically toward 1.0.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn reinforce(&mut self, evidence_tag: &str, now: DateTime<Utc>) {
        self.supporting_evidence = self.supporting_evidence.saturating_add(1);
        self.conviction =
            (self.conviction + REINFORCE_DELTA * (1.0 - self.conviction)).clamp(0.0, 1.0);
        self.last_updated = now;
        self.add_source_memory(evidence_tag);
    }

    /// Challenge this belief with contradicting evidence.
    ///
    /// Conviction decreases but never drops below 0.05 (beliefs are sticky).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn challenge(&mut self, evidence_tag: &str, now: DateTime<Utc>) {
        self.contradicting_evidence = self.contradicting_evidence.saturating_add(1);
        self.conviction = (self.conviction * CHALLENGE_FACTOR).max(CONVICTION_FLOOR);
        self.last_updated = now;
        self.add_source_memory(evidence_tag);
    }

    /// Confidence in this belief — supporting evidence ratio weighted by conviction.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    #[inline]
    pub fn confidence(&self) -> f32 {
        let total = self.supporting_evidence + self.contradicting_evidence;
        if total == 0 {
            return 0.0;
        }
        self.conviction * (self.supporting_evidence as f32 / total as f32)
    }

    /// Seconds since this belief formed.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    #[inline]
    pub fn age_seconds(&self, now: DateTime<Utc>) -> f64 {
        (now - self.formed_at).num_milliseconds().max(0) as f64 / 1000.0
    }

    /// Seconds since last reinforcement or challenge.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    #[inline]
    pub fn staleness_seconds(&self, now: DateTime<Utc>) -> f64 {
        (now - self.last_updated).num_milliseconds().max(0) as f64 / 1000.0
    }

    /// Add a source memory tag, evicting the oldest if at capacity.
    fn add_source_memory(&mut self, tag: &str) {
        if self.source_memories.len() >= MAX_SOURCE_MEMORIES {
            self.source_memories.pop_front();
        }
        self.source_memories.push_back(tag.to_owned());
    }
}

// ---------------------------------------------------------------------------
// BeliefSystem
// ---------------------------------------------------------------------------

/// Capacity-bounded collection of beliefs.
///
/// Beliefs are reinforced or created from emotional evidence. When capacity is
/// reached, the belief with the lowest conviction is evicted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefSystem {
    beliefs: Vec<Belief>,
    capacity: usize,
}

impl Default for BeliefSystem {
    fn default() -> Self {
        Self {
            beliefs: Vec::new(),
            capacity: 64,
        }
    }
}

impl BeliefSystem {
    /// Create an empty belief system with the given capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            beliefs: Vec::new(),
            capacity: capacity.max(1),
        }
    }

    /// Reinforce an existing belief or create a new one.
    ///
    /// If a belief with the given tag exists, it is reinforced.
    /// Otherwise, a new belief is created with initial conviction.
    /// At capacity, the weakest belief is evicted.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn reinforce_or_create(
        &mut self,
        kind: BeliefKind,
        tag: impl Into<String>,
        valence: f32,
        evidence_tag: &str,
        now: DateTime<Utc>,
    ) {
        let tag = tag.into();
        if let Some(belief) = self.beliefs.iter_mut().find(|b| b.tag == tag) {
            belief.reinforce(evidence_tag, now);
            return;
        }
        // Evict if at capacity
        if self.beliefs.len() >= self.capacity {
            self.evict_weakest();
        }
        self.beliefs.push(Belief {
            kind,
            tag,
            valence: valence.clamp(-1.0, 1.0),
            conviction: INITIAL_CONVICTION,
            supporting_evidence: 1,
            contradicting_evidence: 0,
            source_memories: VecDeque::from([evidence_tag.to_owned()]),
            formed_at: now,
            last_updated: now,
            suppression_depth: 0.0,
        });
    }

    /// Challenge a belief by tag with contradicting evidence.
    ///
    /// No-op if the tag does not exist.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn challenge(&mut self, tag: &str, evidence_tag: &str, now: DateTime<Utc>) {
        if let Some(belief) = self.beliefs.iter_mut().find(|b| b.tag == tag) {
            belief.challenge(evidence_tag, now);
        }
    }

    /// Look up a belief by tag.
    #[must_use]
    #[inline]
    pub fn get(&self, tag: &str) -> Option<&Belief> {
        self.beliefs.iter().find(|b| b.tag == tag)
    }

    /// All beliefs of a given kind.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn beliefs_of_kind(&self, kind: BeliefKind) -> impl Iterator<Item = &Belief> {
        self.beliefs.iter().filter(move |b| b.kind == kind)
    }

    /// Decay all convictions. Removes beliefs with conviction < 0.01
    /// AND fewer than 2 supporting evidence observations.
    ///
    /// Shadow beliefs (high `suppression_depth`) decay at half rate —
    /// what you deny persists longer than what you acknowledge.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn decay(&mut self, rate: f32) {
        let rate = rate.clamp(0.0, 1.0);
        for belief in &mut self.beliefs {
            let effective_rate = rate * (1.0 - belief.suppression_depth * 0.5);
            belief.conviction *= 1.0 - effective_rate;
        }
        self.beliefs
            .retain(|b| b.conviction >= 0.01 || b.supporting_evidence >= 2);
    }

    /// Internal consistency of the belief system.
    ///
    /// For each pair of same-kind beliefs with opposing valence (sign differs),
    /// that counts as a contradiction. Returns 1.0 (fully coherent) to 0.0.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn coherence(&self) -> f32 {
        if self.beliefs.len() < 2 {
            return 1.0;
        }

        // O(n) single-pass: count positive/negative/total per kind.
        const N_KINDS: usize = 4;
        let mut kind_counts = [0u32; N_KINDS];
        let mut pos_counts = [0u32; N_KINDS];
        let mut neg_counts = [0u32; N_KINDS];

        for belief in &self.beliefs {
            let idx = match belief.kind {
                BeliefKind::SelfBelief => 0,
                BeliefKind::WorldBelief => 1,
                BeliefKind::OtherBelief => 2,
                BeliefKind::UniversalBelief => 3,
            };
            kind_counts[idx] += 1;
            if belief.valence > 0.0 {
                pos_counts[idx] += 1;
            } else if belief.valence < 0.0 {
                neg_counts[idx] += 1;
            }
        }

        let mut contradictions = 0u32;
        let mut total_pairs = 0u32;

        for i in 0..N_KINDS {
            let n = kind_counts[i];
            if n < 2 {
                continue;
            }
            total_pairs += n * (n - 1) / 2;
            contradictions += pos_counts[i] * neg_counts[i];
        }

        if total_pairs == 0 {
            return 1.0;
        }
        1.0 - (contradictions as f32 / total_pairs as f32).min(1.0)
    }

    /// Number of beliefs in the system.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.beliefs.len()
    }

    /// Whether the system has no beliefs.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.beliefs.is_empty()
    }

    /// Top N beliefs by conviction, sorted descending.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn strongest_beliefs(&self, n: usize) -> Vec<&Belief> {
        let mut sorted: Vec<&Belief> = self.beliefs.iter().collect();
        sorted.sort_by(|a, b| {
            b.conviction
                .partial_cmp(&a.conviction)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted.truncate(n);
        sorted
    }

    /// Evict the belief with the lowest conviction.
    fn evict_weakest(&mut self) {
        if self.beliefs.is_empty() {
            return;
        }
        let mut min_idx = 0;
        let mut min_conviction = f32::MAX;
        for (i, b) in self.beliefs.iter().enumerate() {
            if b.conviction < min_conviction {
                min_conviction = b.conviction;
                min_idx = i;
            }
        }
        self.beliefs.swap_remove(min_idx);
    }
}

// ---------------------------------------------------------------------------
// SelfModel
// ---------------------------------------------------------------------------

/// Emergent self-concept derived from self-beliefs.
///
/// Maps self-beliefs to trait dimensions, providing a bottom-up identity
/// that complements the top-down archetype system. The entity *discovers*
/// who it is through accumulated experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    /// Trait-aligned self-perception. Each dimension stores the conviction-weighted
    /// valence of self-beliefs that map to that trait. 0.0 means no signal.
    perceived_traits: [f32; TraitKind::COUNT],
    /// Number of self-beliefs contributing to each dimension.
    evidence_counts: [u32; TraitKind::COUNT],
}

impl Default for SelfModel {
    fn default() -> Self {
        Self {
            perceived_traits: [0.0; TraitKind::COUNT],
            evidence_counts: [0; TraitKind::COUNT],
        }
    }
}

impl SelfModel {
    /// Create an empty self-model with no signal.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the self-model from the current belief system.
    ///
    /// Scans all [`BeliefKind::SelfBelief`] entries, maps their tags to trait
    /// dimensions, and updates perceived traits as conviction-weighted averages.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn update_from_beliefs(&mut self, beliefs: &BeliefSystem) {
        // Reset
        self.perceived_traits = [0.0; TraitKind::COUNT];
        self.evidence_counts = [0; TraitKind::COUNT];

        let mut weight_sums = [0.0f32; TraitKind::COUNT];

        for belief in beliefs.beliefs_of_kind(BeliefKind::SelfBelief) {
            if let Some(trait_kind) = tag_to_trait(&belief.tag) {
                let idx = trait_kind.index();
                self.perceived_traits[idx] += belief.valence * belief.conviction;
                weight_sums[idx] += belief.conviction;
                self.evidence_counts[idx] += 1;
            }
        }

        // Normalize by total conviction weight
        for (i, &w) in weight_sums.iter().enumerate() {
            if w > 0.0 {
                self.perceived_traits[i] /= w;
            }
        }
    }

    /// Self-clarity: proportion of trait dimensions with non-zero signal.
    ///
    /// Range 0.0 (no self-knowledge) to 1.0 (every dimension understood).
    #[must_use]
    #[inline]
    pub fn self_clarity(&self) -> f32 {
        let populated = self.evidence_counts.iter().filter(|&&c| c > 0).count();
        populated as f32 / TraitKind::COUNT as f32
    }

    /// Self-consistency: alignment between perceived self and actual personality.
    ///
    /// For each dimension with evidence, computes sign agreement weighted by
    /// the absolute value of perception signal. Returns 0.0 (contradictory)
    /// to 1.0 (perfectly aligned).
    #[must_use]
    pub fn self_consistency(&self, profile: &PersonalityProfile) -> f32 {
        let mut agreement = 0.0f32;
        let mut total_weight = 0.0f32;

        for &kind in TraitKind::ALL {
            let idx = kind.index();
            if self.evidence_counts[idx] == 0 {
                continue;
            }
            let perceived = self.perceived_traits[idx];
            let actual = profile.get_trait(kind).normalized();
            let weight = perceived.abs().max(0.01);
            total_weight += weight;

            // Sign agreement: +1 if same sign, -1 if opposite, 0 if either is 0
            if (perceived > 0.0 && actual > 0.0) || (perceived < 0.0 && actual < 0.0) {
                agreement += weight;
            } else if (perceived > 0.0 && actual < 0.0) || (perceived < 0.0 && actual > 0.0) {
                // Opposing signs reduce agreement
            } else {
                // One is zero — neutral, count half
                agreement += weight * 0.5;
            }
        }

        if total_weight == 0.0 {
            return 0.0;
        }
        (agreement / total_weight).clamp(0.0, 1.0)
    }

    /// Get self-perception for a specific trait dimension.
    #[must_use]
    #[inline]
    pub fn perceived_trait(&self, kind: TraitKind) -> f32 {
        self.perceived_traits[kind.index()]
    }

    /// Number of evidence observations for a trait dimension.
    #[must_use]
    #[inline]
    pub fn evidence_count(&self, kind: TraitKind) -> u32 {
        self.evidence_counts[kind.index()]
    }
}

// ---------------------------------------------------------------------------
// WorldModel
// ---------------------------------------------------------------------------

/// Emergent worldview derived from world-beliefs and other-beliefs.
///
/// Tracks two fundamental axes of world perception:
/// - **Trust**: Is the world safe or hostile?
/// - **Meaning**: Is the world meaningful or random?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModel {
    /// Average valence of trust-related beliefs. -1.0 (hostile) to 1.0 (safe).
    trust_signal: f32,
    /// Average valence of meaning-related beliefs. -1.0 (random) to 1.0 (purposeful).
    meaning_signal: f32,
    /// Total conviction weight for trust signal.
    trust_evidence: f32,
    /// Total conviction weight for meaning signal.
    meaning_evidence: f32,
}

impl Default for WorldModel {
    fn default() -> Self {
        Self {
            trust_signal: 0.0,
            meaning_signal: 0.0,
            trust_evidence: 0.0,
            meaning_evidence: 0.0,
        }
    }
}

/// Keywords that map beliefs to the trust axis.
const TRUST_KEYWORDS: &[&str] = &[
    "trust",
    "safe",
    "safety",
    "danger",
    "threat",
    "hostile",
    "trustworthy",
    "untrustworthy",
    "reliable",
    "unreliable",
    "secure",
    "insecure",
    "betrayal",
    "loyal",
    "disloyal",
];

/// Keywords that map beliefs to the meaning axis.
const MEANING_KEYWORDS: &[&str] = &[
    "meaning",
    "meaningful",
    "purpose",
    "purposeful",
    "random",
    "chaos",
    "chaotic",
    "order",
    "orderly",
    "just",
    "unjust",
    "fair",
    "unfair",
    "absurd",
];

impl WorldModel {
    /// Create an empty world model with no signal.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the world model from the current belief system.
    ///
    /// Scans all [`BeliefKind::WorldBelief`] and [`BeliefKind::OtherBelief`] entries.
    /// Beliefs with trust-related tags contribute to the trust signal;
    /// beliefs with meaning-related tags contribute to the meaning signal.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn update_from_beliefs(&mut self, beliefs: &BeliefSystem) {
        let mut trust_sum = 0.0f32;
        let mut trust_weight = 0.0f32;
        let mut meaning_sum = 0.0f32;
        let mut meaning_weight = 0.0f32;

        let relevant = beliefs
            .beliefs_of_kind(BeliefKind::WorldBelief)
            .chain(beliefs.beliefs_of_kind(BeliefKind::OtherBelief));

        for belief in relevant {
            let tag_lower = belief.tag.to_lowercase();
            if TRUST_KEYWORDS.iter().any(|kw| tag_lower.contains(kw)) {
                trust_sum += belief.valence * belief.conviction;
                trust_weight += belief.conviction;
            }
            if MEANING_KEYWORDS.iter().any(|kw| tag_lower.contains(kw)) {
                meaning_sum += belief.valence * belief.conviction;
                meaning_weight += belief.conviction;
            }
        }

        self.trust_signal = if trust_weight > 0.0 {
            (trust_sum / trust_weight).clamp(-1.0, 1.0)
        } else {
            0.0
        };
        self.trust_evidence = trust_weight;

        self.meaning_signal = if meaning_weight > 0.0 {
            (meaning_sum / meaning_weight).clamp(-1.0, 1.0)
        } else {
            0.0
        };
        self.meaning_evidence = meaning_weight;
    }

    /// How safe or hostile the entity perceives the world.
    ///
    /// -1.0 (hostile) to 1.0 (safe). Returns 0.0 if no trust-related evidence.
    #[must_use]
    #[inline]
    pub fn world_trust(&self) -> f32 {
        self.trust_signal
    }

    /// How meaningful or random the entity perceives the world.
    ///
    /// -1.0 (random/meaningless) to 1.0 (meaningful/purposeful).
    /// Returns 0.0 if no meaning-related evidence.
    #[must_use]
    #[inline]
    pub fn world_meaning(&self) -> f32 {
        self.meaning_signal
    }

    /// Whether any trust-related evidence exists.
    #[must_use]
    #[inline]
    pub fn has_trust_evidence(&self) -> bool {
        self.trust_evidence > 0.0
    }

    /// Whether any meaning-related evidence exists.
    #[must_use]
    #[inline]
    pub fn has_meaning_evidence(&self) -> bool {
        self.meaning_evidence > 0.0
    }
}

// ---------------------------------------------------------------------------
// InsightEvent
// ---------------------------------------------------------------------------

/// An insight — a moment when self-knowledge and world-knowledge resonate.
///
/// Occurs when cosmic understanding crosses a threshold and matching
/// self-belief / world-belief pairs exist — the entity sees itself
/// reflected in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightEvent {
    /// The self-belief tag that bridged to the world.
    pub self_belief_tag: String,
    /// The world-belief tag that resonated.
    pub world_belief_tag: String,
    /// Depth of insight (0.0-1.0), based on cosmic understanding level.
    pub depth: f32,
}

// ---------------------------------------------------------------------------
// Understanding functions — the philosophical chain
// ---------------------------------------------------------------------------

/// How deeply the entity knows itself.
///
/// Combines EQ understanding capacity with self-model clarity and belief coherence.
///
/// `self_understanding = eq.understanding * 0.4 + self_clarity * 0.3 + coherence * 0.3`
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn self_understanding(eq: &EqProfile, self_model: &SelfModel, coherence: f32) -> f32 {
    (eq.understanding * 0.4 + self_model.self_clarity() * 0.3 + coherence.clamp(0.0, 1.0) * 0.3)
        .clamp(0.0, 1.0)
}

/// Cosmic understanding — when self-knowledge and world-knowledge converge.
///
/// Requires high self-understanding AND coherent world model. When both are
/// present, the boundary between "knowing yourself" and "knowing the world"
/// dissolves.
///
/// Only meaningful when `self_understanding > 0` and the world model has signal.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn cosmic_understanding(
    self_understanding: f32,
    world_model: &WorldModel,
    coherence: f32,
) -> f32 {
    let self_u = self_understanding.clamp(0.0, 1.0);
    let coh = coherence.clamp(0.0, 1.0);
    let base = self_u * coh;

    // World meaning (positive or negative) provides signal; absence provides none
    let world_factor = (1.0 + world_model.world_meaning().abs()) / 2.0;

    (base * world_factor).clamp(0.0, 1.0)
}

/// Check if conditions are ripe for an insight event.
///
/// An insight occurs when cosmic understanding exceeds a threshold and there
/// exist both a self-belief and a world-belief with matching valence sign —
/// the entity sees itself reflected in the world.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn check_insight(beliefs: &BeliefSystem, cosmic: f32, threshold: f32) -> Option<InsightEvent> {
    if cosmic < threshold {
        return None;
    }

    let self_beliefs: Vec<&Belief> = beliefs.beliefs_of_kind(BeliefKind::SelfBelief).collect();
    let world_beliefs: Vec<&Belief> = beliefs.beliefs_of_kind(BeliefKind::WorldBelief).collect();

    // Find the strongest matching pair (same valence sign)
    let mut best: Option<InsightEvent> = None;
    let mut best_strength = 0.0f32;

    for sb in &self_beliefs {
        for wb in &world_beliefs {
            // Same valence sign = resonance
            if (sb.valence > 0.0 && wb.valence > 0.0) || (sb.valence < 0.0 && wb.valence < 0.0) {
                let strength = sb.conviction * wb.conviction;
                if strength > best_strength {
                    best_strength = strength;
                    best = Some(InsightEvent {
                        self_belief_tag: sb.tag.clone(),
                        world_belief_tag: wb.tag.clone(),
                        depth: cosmic,
                    });
                }
            }
        }
    }

    best
}

// ---------------------------------------------------------------------------
// Integration functions
// ---------------------------------------------------------------------------

/// Generate trait pressures from self-beliefs.
///
/// Strong self-beliefs push corresponding traits toward the believed direction.
/// "I am brave" (high conviction, positive valence) pushes Confidence and
/// RiskTolerance up. The output can be fed into
/// [`GrowthLedger::apply_pressure()`](crate::growth::GrowthLedger::apply_pressure).
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn belief_trait_pressure(beliefs: &BeliefSystem) -> [f32; TraitKind::COUNT] {
    let mut pressure = [0.0f32; TraitKind::COUNT];

    for belief in beliefs.beliefs_of_kind(BeliefKind::SelfBelief) {
        if let Some(trait_kind) = tag_to_trait(&belief.tag) {
            let p = belief.valence * belief.conviction * TRAIT_PRESSURE_COEFF;
            pressure[trait_kind.index()] += p;
        }
    }

    pressure
}

/// Compute an appraisal bias from the current world model.
///
/// World-trust shifts desirability expectations (confirmation bias):
/// - Negative world-trust -> events appraised more negatively (pessimism)
/// - Positive world-trust -> events appraised more positively (optimism)
///
/// Returns a small bias to add to appraisal desirability scores.
#[must_use]
#[inline]
pub fn appraisal_bias(world_model: &WorldModel) -> f32 {
    world_model.world_trust() * APPRAISAL_BIAS_COEFF
}

/// Compare the emergent self-model (bottom-up from beliefs) with the
/// declared identity (top-down from archetype).
///
/// Checks if soul-layer content keywords match self-belief tags.
/// Returns 0.0 (no alignment) to 1.0 (perfect alignment).
#[must_use]
pub fn identity_alignment(
    self_model: &SelfModel,
    identity: &crate::archetype::IdentityContent,
) -> f32 {
    let soul_text = match identity.get(crate::archetype::IdentityLayer::Soul) {
        Some(text) => text.to_lowercase(),
        None => return 0.0,
    };

    let mut matches = 0u32;
    let mut total = 0u32;

    for &kind in TraitKind::ALL {
        let idx = kind.index();
        if self_model.evidence_counts[idx] == 0 {
            continue;
        }
        total += 1;
        // Check if the trait name appears in the soul description
        let trait_name = trait_keyword(kind);
        if soul_text.contains(trait_name) {
            matches += 1;
        }
    }

    if total == 0 {
        return 0.0;
    }
    matches as f32 / total as f32
}

// ---------------------------------------------------------------------------
// Emotion classification and belief formation
// ---------------------------------------------------------------------------

/// Whether an emotion is about the self or about social relationships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EmotionCategory {
    /// About events relative to self: Joy, Distress, Hope, Fear, Relief, Disappointment.
    Personal,
    /// About self/others relative to standards: Pride, Shame, Admiration, Reproach, Gratitude, Anger.
    Social,
}

impl fmt::Display for EmotionCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Personal => f.write_str("personal"),
            Self::Social => f.write_str("social"),
        }
    }
}

/// Classify an appraised emotion as personal or social.
#[must_use]
#[inline]
pub fn classify_emotion(emotion: AppraisedEmotion) -> EmotionCategory {
    match emotion {
        AppraisedEmotion::Joy
        | AppraisedEmotion::Distress
        | AppraisedEmotion::Hope
        | AppraisedEmotion::Fear
        | AppraisedEmotion::Relief
        | AppraisedEmotion::Disappointment => EmotionCategory::Personal,

        AppraisedEmotion::Pride
        | AppraisedEmotion::Shame
        | AppraisedEmotion::Admiration
        | AppraisedEmotion::Reproach
        | AppraisedEmotion::Gratitude
        | AppraisedEmotion::Anger => EmotionCategory::Social,
    }
}

/// Map an appraised emotion to the kind of belief it forms.
///
/// Personal emotions form self-beliefs or world-beliefs depending on attribution.
/// Social emotions form self-beliefs (pride/shame) or other-beliefs (admiration/anger).
#[must_use]
pub fn emotion_to_belief_kind(emotion: AppraisedEmotion, is_self: bool) -> BeliefKind {
    match emotion {
        // Personal emotions: self-attributed → self-belief, other → world-belief
        AppraisedEmotion::Joy => {
            if is_self {
                BeliefKind::SelfBelief
            } else {
                BeliefKind::WorldBelief
            }
        }
        AppraisedEmotion::Distress => {
            if is_self {
                BeliefKind::SelfBelief
            } else {
                BeliefKind::WorldBelief
            }
        }
        AppraisedEmotion::Fear => {
            if is_self {
                BeliefKind::SelfBelief
            } else {
                BeliefKind::WorldBelief
            }
        }
        AppraisedEmotion::Disappointment => {
            if is_self {
                BeliefKind::SelfBelief
            } else {
                BeliefKind::WorldBelief
            }
        }
        // Prospect/relief → always world-beliefs (about how things work out)
        AppraisedEmotion::Hope | AppraisedEmotion::Relief => BeliefKind::WorldBelief,
        // Attribution emotions about self → self-beliefs
        AppraisedEmotion::Pride | AppraisedEmotion::Shame => BeliefKind::SelfBelief,
        // Attribution/compound emotions about others → other-beliefs
        AppraisedEmotion::Admiration
        | AppraisedEmotion::Reproach
        | AppraisedEmotion::Gratitude
        | AppraisedEmotion::Anger => BeliefKind::OtherBelief,
    }
}

/// Generate a belief tag from an appraised emotion.
///
/// Tags follow the convention: `"self:capable"`, `"world:hostile"`, `"other:alice:trustworthy"`.
#[must_use]
pub fn emotion_to_belief_tag(
    emotion: AppraisedEmotion,
    is_self: bool,
    agent_tag: Option<&str>,
) -> String {
    let (prefix, keyword) = match emotion {
        AppraisedEmotion::Joy if is_self => ("self", "blessed"),
        AppraisedEmotion::Joy => ("world", "benevolent"),
        AppraisedEmotion::Distress if is_self => ("self", "suffering"),
        AppraisedEmotion::Distress => ("world", "hostile"),
        AppraisedEmotion::Hope => ("world", "promising"),
        AppraisedEmotion::Fear if is_self => ("self", "vulnerable"),
        AppraisedEmotion::Fear => ("world", "dangerous"),
        AppraisedEmotion::Relief => ("world", "survivable"),
        AppraisedEmotion::Disappointment if is_self => ("self", "unlucky"),
        AppraisedEmotion::Disappointment => ("world", "disappointing"),
        AppraisedEmotion::Pride => ("self", "capable"),
        AppraisedEmotion::Shame => ("self", "flawed"),
        AppraisedEmotion::Admiration => ("other", "admirable"),
        AppraisedEmotion::Reproach => ("other", "blameworthy"),
        AppraisedEmotion::Gratitude if is_self => ("other", "trustworthy"),
        AppraisedEmotion::Gratitude => ("other", "generous"),
        AppraisedEmotion::Anger => ("other", "harmful"),
    };

    if prefix == "other" {
        let agent = agent_tag.unwrap_or("unknown");
        format!("{prefix}:{agent}:{keyword}")
    } else {
        format!("{prefix}:{keyword}")
    }
}

/// Determine the valence of a belief formed from an emotion.
///
/// Positive emotions produce positive valence, negative emotions produce negative.
#[must_use]
#[inline]
fn emotion_valence(emotion: AppraisedEmotion) -> f32 {
    match emotion {
        AppraisedEmotion::Joy
        | AppraisedEmotion::Hope
        | AppraisedEmotion::Relief
        | AppraisedEmotion::Pride
        | AppraisedEmotion::Admiration
        | AppraisedEmotion::Gratitude => 1.0,
        AppraisedEmotion::Distress
        | AppraisedEmotion::Fear
        | AppraisedEmotion::Disappointment
        | AppraisedEmotion::Shame
        | AppraisedEmotion::Reproach
        | AppraisedEmotion::Anger => -1.0,
    }
}

/// Modulate belief conviction based on emotional suppression.
///
/// Suppressed emotions still form beliefs, but with reduced conscious conviction.
/// Minimum 0.05 — beliefs always form, even when suppressed.
#[must_use]
#[inline]
fn suppression_modulated_conviction(base_conviction: f32, suppression_gap: f32) -> f32 {
    let suppression = suppression_gap.clamp(0.0, 1.0);
    (base_conviction * (1.0 - suppression * 0.6)).max(0.05)
}

/// Compute the suppression depth for a belief formed under emotional suppression.
///
/// High suppression gap + high emotional intensity = deep shadow belief.
#[must_use]
#[inline]
fn compute_suppression_depth(suppression_gap: f32, intensity: f32) -> f32 {
    (suppression_gap.clamp(0.0, 1.0) * intensity.clamp(0.0, 1.0) * 0.8).clamp(0.0, 1.0)
}

impl BeliefSystem {
    /// Reinforce or create a belief with suppression tracking.
    ///
    /// Like [`reinforce_or_create`](Self::reinforce_or_create) but sets
    /// `suppression_depth` on creation and blends it on reinforcement.
    /// When `suppression_gap > 0`, conviction is modulated downward:
    /// suppressed emotions form weaker conscious beliefs.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn reinforce_or_create_with_suppression(
        &mut self,
        kind: BeliefKind,
        tag: impl Into<String>,
        valence: f32,
        evidence_tag: &str,
        suppression_depth: f32,
        now: DateTime<Utc>,
    ) {
        let tag = tag.into();
        let supp = suppression_depth.clamp(0.0, 1.0);
        if let Some(belief) = self.beliefs.iter_mut().find(|b| b.tag == tag) {
            belief.reinforce(evidence_tag, now);
            // Modulate conviction: suppression reduces what reinforcement gained
            if supp > 0.0 {
                belief.conviction = suppression_modulated_conviction(belief.conviction, supp);
            }
            // Blend suppression depth: weighted average favoring existing
            belief.suppression_depth = belief.suppression_depth * 0.7 + supp * 0.3;
            return;
        }
        // Evict if at capacity
        if self.beliefs.len() >= self.capacity {
            self.evict_weakest();
        }
        let conviction = suppression_modulated_conviction(INITIAL_CONVICTION, supp);
        self.beliefs.push(Belief {
            kind,
            tag,
            valence: valence.clamp(-1.0, 1.0),
            conviction,
            supporting_evidence: 1,
            contradicting_evidence: 0,
            source_memories: VecDeque::from([evidence_tag.to_owned()]),
            formed_at: now,
            last_updated: now,
            suppression_depth: supp,
        });
    }
}

/// Apply an appraised emotion to the belief system, creating or reinforcing beliefs.
///
/// - Classifies the emotion as personal or social
/// - Maps it to the appropriate [`BeliefKind`] based on attribution
/// - Generates the belief tag
/// - Modulates conviction by emotional suppression gap
/// - Tracks suppression depth for shadow belief formation
///
/// Returns the tag of the affected belief.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
pub fn apply_emotion_to_beliefs(
    beliefs: &mut BeliefSystem,
    emotion: AppraisedEmotion,
    intensity: f32,
    is_self: bool,
    agent_tag: Option<&str>,
    suppression_gap: f32,
    now: DateTime<Utc>,
) -> String {
    let kind = emotion_to_belief_kind(emotion, is_self);
    let tag = emotion_to_belief_tag(emotion, is_self, agent_tag);
    let valence = emotion_valence(emotion) * intensity.clamp(0.0, 1.0);
    let supp_depth = compute_suppression_depth(suppression_gap, intensity);

    beliefs.reinforce_or_create_with_suppression(kind, &tag, valence, "emotion", supp_depth, now);

    tag
}

/// Extract shadow beliefs for intuition synthesis.
///
/// Returns `(tag, valence, suppression_depth)` tuples for beliefs with
/// `suppression_depth` above the given threshold. Shadow beliefs are
/// what the entity knows but won't consciously acknowledge — they
/// surface as gut feelings in the intuition module.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn shadow_beliefs(beliefs: &BeliefSystem, threshold: f32) -> Vec<(String, f32, f32)> {
    beliefs
        .beliefs
        .iter()
        .filter(|b| b.suppression_depth > threshold)
        .map(|b| (b.tag.clone(), b.valence, b.suppression_depth))
        .collect()
}

// ---------------------------------------------------------------------------
// Tag-to-trait mapping helpers
// ---------------------------------------------------------------------------

/// Map a belief tag to a trait kind.
///
/// Tags follow the convention `"self:<trait_keyword>"`, e.g., `"self:confident"`.
/// Returns `None` if the tag doesn't map to any known trait.
#[must_use]
fn tag_to_trait(tag: &str) -> Option<TraitKind> {
    // Strip "self:" prefix if present
    let keyword = tag.strip_prefix("self:").unwrap_or(tag).to_lowercase();

    TraitKind::ALL
        .iter()
        .find(|&&kind| keyword == trait_keyword(kind))
        .copied()
}

/// Canonical keyword for a trait kind, used in tag matching.
#[must_use]
#[inline]
fn trait_keyword(kind: TraitKind) -> &'static str {
    match kind {
        TraitKind::Formality => "formal",
        TraitKind::Humor => "humorous",
        TraitKind::Verbosity => "verbose",
        TraitKind::Directness => "direct",
        TraitKind::Warmth => "warm",
        TraitKind::Empathy => "empathetic",
        TraitKind::Patience => "patient",
        TraitKind::Confidence => "confident",
        TraitKind::Creativity => "creative",
        TraitKind::RiskTolerance => "brave",
        TraitKind::Curiosity => "curious",
        TraitKind::Skepticism => "skeptical",
        TraitKind::Autonomy => "autonomous",
        TraitKind::Pedagogy => "pedagogical",
        TraitKind::Precision => "precise",
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PersonalityProfile, TraitKind, TraitLevel};

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn make_belief(tag: &str, kind: BeliefKind, valence: f32, conviction: f32) -> Belief {
        Belief {
            kind,
            tag: tag.to_owned(),
            valence,
            conviction,
            supporting_evidence: 1,
            contradicting_evidence: 0,
            source_memories: VecDeque::from(["initial".to_owned()]),
            formed_at: now(),
            last_updated: now(),
            suppression_depth: 0.0,
        }
    }

    // ---- Belief struct ----

    #[test]
    fn test_reinforce_increases_conviction() {
        let mut b = make_belief("self:confident", BeliefKind::SelfBelief, 0.8, 0.3);
        let before = b.conviction;
        b.reinforce("evidence_1", now());
        assert!(b.conviction > before);
        assert_eq!(b.supporting_evidence, 2);
    }

    #[test]
    fn test_challenge_decreases_conviction() {
        let mut b = make_belief("self:confident", BeliefKind::SelfBelief, 0.8, 0.5);
        let before = b.conviction;
        b.challenge("counter_1", now());
        assert!(b.conviction < before);
        assert_eq!(b.contradicting_evidence, 1);
    }

    #[test]
    fn test_conviction_approaches_one() {
        let mut b = make_belief("self:warm", BeliefKind::SelfBelief, 0.9, 0.1);
        for i in 0..100 {
            b.reinforce(&format!("ev_{i}"), now());
        }
        assert!(b.conviction > 0.95);
        assert!(b.conviction <= 1.0);
    }

    #[test]
    fn test_conviction_floor() {
        let mut b = make_belief("self:warm", BeliefKind::SelfBelief, 0.9, 0.5);
        for i in 0..100 {
            b.challenge(&format!("counter_{i}"), now());
        }
        assert!(b.conviction >= CONVICTION_FLOOR);
    }

    #[test]
    fn test_confidence_ratio() {
        let mut b = make_belief("self:warm", BeliefKind::SelfBelief, 0.9, 0.8);
        b.supporting_evidence = 8;
        b.contradicting_evidence = 2;
        let conf = b.confidence();
        // 0.8 * (8/10) = 0.64
        assert!((conf - 0.64).abs() < 0.001);
    }

    #[test]
    fn test_confidence_zero_evidence() {
        let mut b = make_belief("test", BeliefKind::SelfBelief, 0.5, 0.5);
        b.supporting_evidence = 0;
        b.contradicting_evidence = 0;
        assert_eq!(b.confidence(), 0.0);
    }

    #[test]
    fn test_source_memory_cap() {
        let mut b = make_belief("test", BeliefKind::SelfBelief, 0.5, 0.5);
        for i in 0..20 {
            b.reinforce(&format!("ev_{i}"), now());
        }
        assert!(b.source_memories.len() <= MAX_SOURCE_MEMORIES);
    }

    // ---- BeliefSystem ----

    #[test]
    fn test_new_empty() {
        let bs = BeliefSystem::new(32);
        assert!(bs.is_empty());
        assert_eq!(bs.len(), 0);
    }

    #[test]
    fn test_reinforce_or_create_new() {
        let mut bs = BeliefSystem::new(32);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev1", now());
        assert_eq!(bs.len(), 1);
        let b = bs.get("self:warm").unwrap();
        assert_eq!(b.kind, BeliefKind::SelfBelief);
        assert!((b.valence - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_reinforce_or_create_existing() {
        let mut bs = BeliefSystem::new(32);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev1", now());
        let c1 = bs.get("self:warm").unwrap().conviction;
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev2", now());
        assert_eq!(bs.len(), 1); // Still one belief
        assert!(bs.get("self:warm").unwrap().conviction > c1); // But stronger
    }

    #[test]
    fn test_challenge_existing() {
        let mut bs = BeliefSystem::new(32);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev1", now());
        let c1 = bs.get("self:warm").unwrap().conviction;
        bs.challenge("self:warm", "counter1", now());
        assert!(bs.get("self:warm").unwrap().conviction < c1);
    }

    #[test]
    fn test_challenge_nonexistent() {
        let mut bs = BeliefSystem::new(32);
        // Should not panic
        bs.challenge("nonexistent", "counter1", now());
        assert!(bs.is_empty());
    }

    #[test]
    fn test_eviction_at_capacity() {
        let mut bs = BeliefSystem::new(3);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "a", 0.5, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "b", 0.5, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "c", 0.5, "ev", t);
        // Reinforce "b" and "c" to make "a" the weakest
        bs.reinforce_or_create(BeliefKind::SelfBelief, "b", 0.5, "ev2", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "c", 0.5, "ev2", t);
        // Adding a 4th should evict "a" (lowest conviction)
        bs.reinforce_or_create(BeliefKind::SelfBelief, "d", 0.5, "ev", t);
        assert_eq!(bs.len(), 3);
        assert!(bs.get("a").is_none()); // "a" was evicted
        assert!(bs.get("d").is_some()); // "d" was added
    }

    #[test]
    fn test_decay() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "strong", 0.5, "ev", t);
        // Reinforce to build conviction
        for i in 0..10 {
            bs.reinforce_or_create(BeliefKind::SelfBelief, "strong", 0.5, &format!("ev{i}"), t);
        }
        bs.reinforce_or_create(BeliefKind::SelfBelief, "weak", 0.3, "ev", t);
        // Heavy decay
        bs.decay(0.99);
        // "strong" should survive (supporting_evidence >= 2)
        assert!(bs.get("strong").is_some());
        // "weak" had only 1 evidence and conviction < 0.01 after 99% decay
        assert!(bs.get("weak").is_none());
    }

    #[test]
    fn test_coherence_consistent() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:confident", 0.7, "ev", t);
        // Same sign → no contradictions → coherence = 1.0
        assert!((bs.coherence() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_coherence_contradictory() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:cold", -0.7, "ev", t);
        // Opposing signs → 1 contradiction out of 1 pair → coherence = 0.0
        assert!(bs.coherence() < 0.01);
    }

    #[test]
    fn test_coherence_single_belief() {
        let mut bs = BeliefSystem::new(32);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", now());
        assert!((bs.coherence() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_coherence_mixed_kinds() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        // Different kinds → not compared → no contradictions
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:hostile", -0.7, "ev", t);
        assert!((bs.coherence() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_coherence_stress_matches_brute_force() {
        // Verify O(n) algorithm matches O(n^2) brute-force for 200+ beliefs.
        let mut bs = BeliefSystem::new(256);
        let t = now();
        // Mix all four kinds with positive and negative valence.
        let kinds = [
            BeliefKind::SelfBelief,
            BeliefKind::WorldBelief,
            BeliefKind::OtherBelief,
            BeliefKind::UniversalBelief,
        ];
        for i in 0..200 {
            let kind = kinds[i % 4];
            let valence = if i % 3 == 0 {
                -0.5
            } else if i % 3 == 1 {
                0.5
            } else {
                0.0 // neutral
            };
            bs.reinforce_or_create(kind, format!("tag{i}"), valence, "ev", t);
        }

        // Brute-force O(n^2) reference
        let beliefs = &bs.beliefs;
        let mut contradictions = 0u32;
        let mut total_pairs = 0u32;
        for i in 0..beliefs.len() {
            for j in (i + 1)..beliefs.len() {
                if beliefs[i].kind == beliefs[j].kind {
                    total_pairs += 1;
                    if (beliefs[i].valence > 0.0 && beliefs[j].valence < 0.0)
                        || (beliefs[i].valence < 0.0 && beliefs[j].valence > 0.0)
                    {
                        contradictions += 1;
                    }
                }
            }
        }
        let expected = if total_pairs == 0 {
            1.0
        } else {
            1.0 - (contradictions as f32 / total_pairs as f32).min(1.0)
        };

        assert!(
            (bs.coherence() - expected).abs() < 0.0001,
            "O(n) coherence {} != brute-force {}",
            bs.coherence(),
            expected
        );
    }

    #[test]
    fn test_strongest_beliefs() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "weak", 0.5, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "strong", 0.5, "ev", t);
        // Reinforce "strong" many times
        for i in 0..20 {
            bs.reinforce_or_create(BeliefKind::SelfBelief, "strong", 0.5, &format!("ev{i}"), t);
        }
        let top = bs.strongest_beliefs(1);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].tag, "strong");
    }

    #[test]
    fn test_beliefs_of_kind() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:safe", 0.6, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:brave", 0.7, "ev", t);
        assert_eq!(bs.beliefs_of_kind(BeliefKind::SelfBelief).count(), 2);
        assert_eq!(bs.beliefs_of_kind(BeliefKind::WorldBelief).count(), 1);
        assert_eq!(bs.beliefs_of_kind(BeliefKind::OtherBelief).count(), 0);
    }

    // ---- SelfModel ----

    #[test]
    fn test_self_clarity_empty() {
        let sm = SelfModel::new();
        assert_eq!(sm.self_clarity(), 0.0);
    }

    #[test]
    fn test_self_clarity_partial() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:confident", 0.7, "ev", t);
        let mut sm = SelfModel::new();
        sm.update_from_beliefs(&bs);
        // 2 out of 15 dimensions populated
        assert!((sm.self_clarity() - 2.0 / 15.0).abs() < 0.001);
    }

    #[test]
    fn test_self_consistency_aligned() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:confident", 0.7, "ev", t);

        let mut sm = SelfModel::new();
        sm.update_from_beliefs(&bs);

        let mut profile = PersonalityProfile::new("test");
        profile.set_trait(TraitKind::Warmth, TraitLevel::High);
        profile.set_trait(TraitKind::Confidence, TraitLevel::High);

        let consistency = sm.self_consistency(&profile);
        assert!(
            consistency > 0.5,
            "Expected high consistency, got {consistency}"
        );
    }

    #[test]
    fn test_self_consistency_misaligned() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        // Believes warm but actually cold
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);

        let mut sm = SelfModel::new();
        sm.update_from_beliefs(&bs);

        let mut profile = PersonalityProfile::new("test");
        profile.set_trait(TraitKind::Warmth, TraitLevel::Lowest);

        let consistency = sm.self_consistency(&profile);
        assert!(
            consistency < 0.5,
            "Expected low consistency, got {consistency}"
        );
    }

    #[test]
    fn test_update_from_beliefs() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);

        let mut sm = SelfModel::new();
        sm.update_from_beliefs(&bs);

        assert!(sm.perceived_trait(TraitKind::Warmth) > 0.0);
        assert_eq!(sm.evidence_count(TraitKind::Warmth), 1);
        assert_eq!(sm.evidence_count(TraitKind::Confidence), 0);
    }

    // ---- WorldModel ----

    #[test]
    fn test_world_trust_no_evidence() {
        let wm = WorldModel::new();
        assert_eq!(wm.world_trust(), 0.0);
        assert!(!wm.has_trust_evidence());
    }

    #[test]
    fn test_world_trust_positive() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:safe", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:trustworthy", 0.6, "ev", t);

        let mut wm = WorldModel::new();
        wm.update_from_beliefs(&bs);
        assert!(wm.world_trust() > 0.0);
        assert!(wm.has_trust_evidence());
    }

    #[test]
    fn test_world_trust_negative() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:danger", -0.8, "ev", t);

        let mut wm = WorldModel::new();
        wm.update_from_beliefs(&bs);
        assert!(wm.world_trust() < 0.0);
    }

    #[test]
    fn test_world_meaning_positive() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:meaningful", 0.9, "ev", t);
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:purpose", 0.7, "ev", t);

        let mut wm = WorldModel::new();
        wm.update_from_beliefs(&bs);
        assert!(wm.world_meaning() > 0.0);
        assert!(wm.has_meaning_evidence());
    }

    // ---- Understanding / Insight ----

    #[test]
    fn test_self_understanding_high_eq() {
        let eq = EqProfile::with_scores(0.8, 0.8, 0.9, 0.8);
        let mut bs = BeliefSystem::new(32);
        let t = now();
        // Populate all trait dimensions for high clarity
        for &kind in TraitKind::ALL {
            let tag = format!("self:{}", trait_keyword(kind));
            bs.reinforce_or_create(BeliefKind::SelfBelief, &tag, 0.7, "ev", t);
        }
        let mut sm = SelfModel::new();
        sm.update_from_beliefs(&bs);

        let su = self_understanding(&eq, &sm, 1.0);
        assert!(su > 0.7, "Expected high self-understanding, got {su}");
    }

    #[test]
    fn test_self_understanding_low_eq() {
        let eq = EqProfile::with_scores(0.1, 0.1, 0.1, 0.1);
        let sm = SelfModel::new(); // Empty

        let su = self_understanding(&eq, &sm, 0.5);
        assert!(su < 0.3, "Expected low self-understanding, got {su}");
    }

    #[test]
    fn test_cosmic_understanding_requires_convergence() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:meaningful", 0.9, "ev", t);
        let mut wm = WorldModel::new();
        wm.update_from_beliefs(&bs);

        // High self-understanding + meaningful world → high cosmic
        let cosmic_high = cosmic_understanding(0.9, &wm, 1.0);
        // Low self-understanding → low cosmic regardless of world
        let cosmic_low = cosmic_understanding(0.1, &wm, 1.0);

        assert!(cosmic_high > cosmic_low);
        assert!(cosmic_high > 0.4);
    }

    #[test]
    fn test_cosmic_understanding_zero_when_no_self() {
        let wm = WorldModel::new();
        let cosmic = cosmic_understanding(0.0, &wm, 1.0);
        assert!(cosmic < 0.01);
    }

    #[test]
    fn test_check_insight_below_threshold() {
        let bs = BeliefSystem::new(32);
        assert!(check_insight(&bs, 0.3, 0.5).is_none());
    }

    #[test]
    fn test_check_insight_above_threshold() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:safe", 0.7, "ev", t);

        let insight = check_insight(&bs, 0.8, 0.5);
        assert!(insight.is_some());
        let event = insight.unwrap();
        assert_eq!(event.self_belief_tag, "self:warm");
        assert_eq!(event.world_belief_tag, "world:safe");
        assert!((event.depth - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_check_insight_no_matching_beliefs() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        // Positive self, negative world — no resonance
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:hostile", -0.7, "ev", t);

        assert!(check_insight(&bs, 0.8, 0.5).is_none());
    }

    // ---- Integration functions ----

    #[test]
    fn test_belief_trait_pressure() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:confident", 0.8, "ev", t);
        // Reinforce to increase conviction
        for i in 0..5 {
            bs.reinforce_or_create(
                BeliefKind::SelfBelief,
                "self:confident",
                0.8,
                &format!("ev{i}"),
                t,
            );
        }

        let pressure = belief_trait_pressure(&bs);
        let confidence_idx = TraitKind::Confidence.index();
        assert!(
            pressure[confidence_idx] > 0.0,
            "Expected positive confidence pressure, got {}",
            pressure[confidence_idx]
        );
        // Other dimensions should be zero
        assert_eq!(pressure[TraitKind::Warmth.index()], 0.0);
    }

    #[test]
    fn test_appraisal_bias() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:safe", 0.8, "ev", t);
        let mut wm = WorldModel::new();
        wm.update_from_beliefs(&bs);

        let bias = appraisal_bias(&wm);
        assert!(bias > 0.0, "Expected positive bias for safe world");

        // Hostile world → negative bias
        let mut bs2 = BeliefSystem::new(32);
        bs2.reinforce_or_create(BeliefKind::WorldBelief, "world:danger", -0.8, "ev", t);
        let mut wm2 = WorldModel::new();
        wm2.update_from_beliefs(&bs2);
        assert!(appraisal_bias(&wm2) < 0.0);
    }

    #[test]
    fn test_identity_alignment_match() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:creative", 0.7, "ev", t);

        let mut sm = SelfModel::new();
        sm.update_from_beliefs(&bs);

        let mut identity = crate::archetype::IdentityContent::default();
        identity.set(
            crate::archetype::IdentityLayer::Soul,
            "A warm and creative being",
        );

        let alignment = identity_alignment(&sm, &identity);
        assert!(alignment > 0.5, "Expected high alignment, got {alignment}");
    }

    #[test]
    fn test_identity_alignment_mismatch() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);

        let mut sm = SelfModel::new();
        sm.update_from_beliefs(&bs);

        let mut identity = crate::archetype::IdentityContent::default();
        identity.set(
            crate::archetype::IdentityLayer::Soul,
            "A cold calculating machine",
        );

        let alignment = identity_alignment(&sm, &identity);
        assert!(alignment < 0.5, "Expected low alignment, got {alignment}");
    }

    #[test]
    fn test_identity_alignment_no_soul() {
        let sm = SelfModel::new();
        let identity = crate::archetype::IdentityContent::default();
        assert_eq!(identity_alignment(&sm, &identity), 0.0);
    }

    // ---- Serde round-trips ----

    #[test]
    fn test_serde_belief() {
        let b = make_belief("self:warm", BeliefKind::SelfBelief, 0.8, 0.5);
        let json = serde_json::to_string(&b).unwrap();
        let b2: Belief = serde_json::from_str(&json).unwrap();
        assert_eq!(b.tag, b2.tag);
        assert!((b.valence - b2.valence).abs() < 0.001);
    }

    #[test]
    fn test_serde_belief_system() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "self:warm", 0.8, "ev", t);
        bs.reinforce_or_create(BeliefKind::WorldBelief, "world:safe", 0.6, "ev", t);
        let json = serde_json::to_string(&bs).unwrap();
        let bs2: BeliefSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(bs.len(), bs2.len());
    }

    #[test]
    fn test_serde_self_model() {
        let sm = SelfModel::new();
        let json = serde_json::to_string(&sm).unwrap();
        let sm2: SelfModel = serde_json::from_str(&json).unwrap();
        assert_eq!(sm.self_clarity(), sm2.self_clarity());
    }

    #[test]
    fn test_serde_world_model() {
        let wm = WorldModel::new();
        let json = serde_json::to_string(&wm).unwrap();
        let wm2: WorldModel = serde_json::from_str(&json).unwrap();
        assert_eq!(wm.world_trust(), wm2.world_trust());
    }

    #[test]
    fn test_serde_belief_kind() {
        for &kind in BeliefKind::ALL {
            let json = serde_json::to_string(&kind).unwrap();
            let kind2: BeliefKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, kind2);
        }
    }

    #[test]
    fn test_serde_insight_event() {
        let event = InsightEvent {
            self_belief_tag: "self:warm".to_owned(),
            world_belief_tag: "world:safe".to_owned(),
            depth: 0.75,
        };
        let json = serde_json::to_string(&event).unwrap();
        let event2: InsightEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event.self_belief_tag, event2.self_belief_tag);
        assert!((event.depth - event2.depth).abs() < 0.001);
    }

    // ---- Tag mapping ----

    #[test]
    fn test_tag_to_trait_valid() {
        assert_eq!(tag_to_trait("self:confident"), Some(TraitKind::Confidence));
        assert_eq!(tag_to_trait("self:warm"), Some(TraitKind::Warmth));
        assert_eq!(tag_to_trait("self:brave"), Some(TraitKind::RiskTolerance));
    }

    #[test]
    fn test_tag_to_trait_invalid() {
        assert_eq!(tag_to_trait("self:unknown"), None);
        assert_eq!(tag_to_trait("world:safe"), None);
    }

    #[test]
    fn test_tag_to_trait_case_insensitive() {
        assert_eq!(tag_to_trait("self:Confident"), Some(TraitKind::Confidence));
        assert_eq!(tag_to_trait("self:WARM"), Some(TraitKind::Warmth));
    }

    // ---- Emotion classification ----

    #[test]
    fn test_classify_emotion_personal() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            classify_emotion(AppraisedEmotion::Joy),
            EmotionCategory::Personal
        );
        assert_eq!(
            classify_emotion(AppraisedEmotion::Distress),
            EmotionCategory::Personal
        );
        assert_eq!(
            classify_emotion(AppraisedEmotion::Hope),
            EmotionCategory::Personal
        );
        assert_eq!(
            classify_emotion(AppraisedEmotion::Fear),
            EmotionCategory::Personal
        );
    }

    #[test]
    fn test_classify_emotion_social() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            classify_emotion(AppraisedEmotion::Pride),
            EmotionCategory::Social
        );
        assert_eq!(
            classify_emotion(AppraisedEmotion::Shame),
            EmotionCategory::Social
        );
        assert_eq!(
            classify_emotion(AppraisedEmotion::Admiration),
            EmotionCategory::Social
        );
        assert_eq!(
            classify_emotion(AppraisedEmotion::Anger),
            EmotionCategory::Social
        );
    }

    #[test]
    fn test_emotion_to_belief_kind_pride_self() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            emotion_to_belief_kind(AppraisedEmotion::Pride, true),
            BeliefKind::SelfBelief
        );
    }

    #[test]
    fn test_emotion_to_belief_kind_admiration_other() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            emotion_to_belief_kind(AppraisedEmotion::Admiration, false),
            BeliefKind::OtherBelief
        );
    }

    #[test]
    fn test_emotion_to_belief_kind_distress_world() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            emotion_to_belief_kind(AppraisedEmotion::Distress, false),
            BeliefKind::WorldBelief
        );
    }

    #[test]
    fn test_emotion_to_belief_tag_pride() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            emotion_to_belief_tag(AppraisedEmotion::Pride, true, None),
            "self:capable"
        );
    }

    #[test]
    fn test_emotion_to_belief_tag_admiration_with_agent() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            emotion_to_belief_tag(AppraisedEmotion::Admiration, false, Some("alice")),
            "other:alice:admirable"
        );
    }

    #[test]
    fn test_emotion_to_belief_tag_distress_not_self() {
        use crate::appraisal::AppraisedEmotion;
        assert_eq!(
            emotion_to_belief_tag(AppraisedEmotion::Distress, false, None),
            "world:hostile"
        );
    }

    // ---- Suppression mechanics ----

    #[test]
    fn test_suppression_modulated_conviction() {
        // No suppression → full conviction
        assert!((suppression_modulated_conviction(0.5, 0.0) - 0.5).abs() < 0.001);
        // Full suppression → reduced but above floor
        let conv = suppression_modulated_conviction(0.5, 1.0);
        assert!(conv < 0.5);
        assert!(conv >= 0.05);
    }

    #[test]
    fn test_shadow_belief_decay_slower() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "conscious", 0.5, "ev", t);
        bs.reinforce_or_create_with_suppression(
            BeliefKind::SelfBelief,
            "shadow",
            0.5,
            "ev",
            0.9,
            t,
        );
        // Shadow starts with lower conviction due to suppression modulation
        let c_before = bs.get("conscious").unwrap().conviction;
        let s_before = bs.get("shadow").unwrap().conviction;
        assert!(c_before > s_before, "Suppressed belief should start weaker");

        // Set both to same conviction to isolate decay rate difference
        // by reinforcing shadow until it matches
        for i in 0..20 {
            bs.reinforce_or_create_with_suppression(
                BeliefKind::SelfBelief,
                "shadow",
                0.5,
                &format!("ev{i}"),
                0.9,
                t,
            );
            bs.reinforce_or_create(
                BeliefKind::SelfBelief,
                "conscious",
                0.5,
                &format!("ev{i}"),
                t,
            );
        }
        // Now both have substantial conviction. Apply decay.
        let c_pre = bs.get("conscious").unwrap().conviction;
        let s_pre = bs.get("shadow").unwrap().conviction;

        bs.decay(0.5);
        let c_after = bs.get("conscious").unwrap().conviction;
        let s_after = bs.get("shadow").unwrap().conviction;

        // Shadow should retain a higher % of its pre-decay conviction
        let c_ratio = c_after / c_pre;
        let s_ratio = s_after / s_pre;
        assert!(
            s_ratio > c_ratio,
            "Shadow decay ratio ({s_ratio:.4}) should be higher than conscious ({c_ratio:.4})"
        );
    }

    #[test]
    fn test_apply_emotion_creates_belief() {
        use crate::appraisal::AppraisedEmotion;
        let mut bs = BeliefSystem::new(32);
        let t = now();
        let tag = apply_emotion_to_beliefs(&mut bs, AppraisedEmotion::Joy, 0.8, true, None, 0.0, t);
        assert_eq!(tag, "self:blessed");
        let belief = bs.get(&tag).unwrap();
        assert!(belief.valence > 0.0);
        assert_eq!(belief.kind, BeliefKind::SelfBelief);
        assert!((belief.suppression_depth - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_apply_emotion_with_suppression() {
        use crate::appraisal::AppraisedEmotion;
        let mut bs = BeliefSystem::new(32);
        let t = now();
        let tag = apply_emotion_to_beliefs(
            &mut bs,
            AppraisedEmotion::Anger,
            0.9,
            false,
            Some("bob"),
            0.8, // high suppression
            t,
        );
        assert_eq!(tag, "other:bob:harmful");
        let belief = bs.get(&tag).unwrap();
        assert!(belief.valence < 0.0);
        assert!(belief.suppression_depth > 0.5, "Should be deeply shadow");
        assert!(
            belief.conviction < INITIAL_CONVICTION,
            "Suppressed conviction should be below initial"
        );
    }

    #[test]
    fn test_shadow_beliefs_query() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        bs.reinforce_or_create(BeliefKind::SelfBelief, "conscious", 0.5, "ev", t);
        bs.reinforce_or_create_with_suppression(
            BeliefKind::SelfBelief,
            "shadow",
            -0.5,
            "ev",
            0.8,
            t,
        );

        let shadows = shadow_beliefs(&bs, 0.3);
        assert_eq!(shadows.len(), 1);
        assert_eq!(shadows[0].0, "shadow");
        assert!(shadows[0].2 > 0.3); // suppression_depth above threshold
    }

    #[test]
    fn test_reinforce_with_suppression_blends() {
        let mut bs = BeliefSystem::new(32);
        let t = now();
        // Create with high suppression
        bs.reinforce_or_create_with_suppression(
            BeliefKind::SelfBelief,
            "belief",
            0.5,
            "ev1",
            0.9,
            t,
        );
        let depth1 = bs.get("belief").unwrap().suppression_depth;
        // Reinforce with low suppression → should blend down
        bs.reinforce_or_create_with_suppression(
            BeliefKind::SelfBelief,
            "belief",
            0.5,
            "ev2",
            0.1,
            t,
        );
        let depth2 = bs.get("belief").unwrap().suppression_depth;
        assert!(
            depth2 < depth1,
            "Blended suppression ({depth2}) should be less than original ({depth1})"
        );
    }

    #[test]
    fn test_serde_emotion_category() {
        let json = serde_json::to_string(&EmotionCategory::Personal).unwrap();
        let ec: EmotionCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(ec, EmotionCategory::Personal);
    }

    #[test]
    fn test_serde_belief_with_suppression_depth() {
        let mut b = make_belief("test", BeliefKind::SelfBelief, 0.5, 0.5);
        b.suppression_depth = 0.7;
        let json = serde_json::to_string(&b).unwrap();
        let b2: Belief = serde_json::from_str(&json).unwrap();
        assert!((b2.suppression_depth - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_serde_belief_backward_compat() {
        // Old serialized belief without suppression_depth should deserialize with 0.0
        let json = r#"{"kind":"SelfBelief","tag":"test","valence":0.5,"conviction":0.5,"supporting_evidence":1,"contradicting_evidence":0,"source_memories":["ev"],"formed_at":"2026-03-25T00:00:00Z","last_updated":"2026-03-25T00:00:00Z"}"#;
        let b: Belief = serde_json::from_str(json).unwrap();
        assert!((b.suppression_depth - 0.0).abs() < 0.001);
    }
}
