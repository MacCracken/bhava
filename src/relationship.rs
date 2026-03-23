//! Relationship graph — inter-entity affinity, trust, and interaction tracking.
//!
//! Models persistent relationships between entities (agents, NPCs, users) with
//! affinity scores, trust levels, interaction history, and time-based decay.
//! Ported from SecureYeoman's simulation/relationship-graph.ts.

use serde::{Deserialize, Serialize};

/// Type of relationship between two entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum RelationshipType {
    Neutral,
    Ally,
    Rival,
    Mentor,
    Student,
    TradePartner,
    Family,
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Neutral => "neutral",
            Self::Ally => "ally",
            Self::Rival => "rival",
            Self::Mentor => "mentor",
            Self::Student => "student",
            Self::TradePartner => "trade_partner",
            Self::Family => "family",
        };
        f.write_str(s)
    }
}

/// A directional relationship from one entity to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Source entity ID.
    pub source: String,
    /// Target entity ID.
    pub target: String,
    /// Relationship type.
    pub rel_type: RelationshipType,
    /// Affinity: -1.0 (hostile) to 1.0 (devoted).
    pub affinity: f32,
    /// Trust: 0.0 (no trust) to 1.0 (absolute trust).
    pub trust: f32,
    /// How many interactions have occurred.
    pub interaction_count: u32,
    /// Decay rate per tick (0.0 = no decay, 1.0 = instant reset).
    pub decay_rate: f32,
}

impl Relationship {
    /// Create a new neutral relationship.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            rel_type: RelationshipType::Neutral,
            affinity: 0.0,
            trust: 0.5,
            interaction_count: 0,
            decay_rate: 0.01,
        }
    }

    /// Apply affinity and trust deltas from an interaction.
    pub fn interact(&mut self, affinity_delta: f32, trust_delta: f32) {
        self.affinity = (self.affinity + affinity_delta).clamp(-1.0, 1.0);
        self.trust = (self.trust + trust_delta).clamp(0.0, 1.0);
        self.interaction_count += 1;
    }

    /// Decay affinity toward 0 and trust toward 0.5.
    pub fn decay(&mut self) {
        if self.decay_rate <= 0.0 {
            return;
        }
        let r = self.decay_rate.clamp(0.0, 1.0);
        self.affinity += (0.0 - self.affinity) * r;
        self.trust += (0.5 - self.trust) * r;
    }

    /// Is this relationship positive?
    pub fn is_positive(&self) -> bool {
        self.affinity > 0.1
    }

    /// Is this relationship negative?
    pub fn is_negative(&self) -> bool {
        self.affinity < -0.1
    }
}

/// An in-memory relationship graph for an entity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelationshipGraph {
    relationships: Vec<Relationship>,
}

impl RelationshipGraph {
    /// Create an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or update a relationship. If one exists between source→target, updates it.
    pub fn upsert(&mut self, rel: Relationship) {
        if let Some(existing) = self
            .relationships
            .iter_mut()
            .find(|r| r.source == rel.source && r.target == rel.target)
        {
            *existing = rel;
        } else {
            self.relationships.push(rel);
        }
    }

    /// Get a relationship between two entities.
    pub fn get(&self, source: &str, target: &str) -> Option<&Relationship> {
        self.relationships
            .iter()
            .find(|r| r.source == source && r.target == target)
    }

    /// Get a mutable relationship between two entities.
    pub fn get_mut(&mut self, source: &str, target: &str) -> Option<&mut Relationship> {
        self.relationships
            .iter_mut()
            .find(|r| r.source == source && r.target == target)
    }

    /// Record an interaction between two entities.
    ///
    /// Auto-creates the relationship if it doesn't exist.
    /// Returns a mutable reference to the updated relationship.
    pub fn record_interaction(
        &mut self,
        source: &str,
        target: &str,
        affinity_delta: f32,
        trust_delta: f32,
    ) {
        if let Some(rel) = self.get_mut(source, target) {
            rel.interact(affinity_delta, trust_delta);
        } else {
            let mut rel = Relationship::new(source, target);
            rel.interact(affinity_delta, trust_delta);
            self.relationships.push(rel);
        }
    }

    /// Remove a relationship.
    pub fn remove(&mut self, source: &str, target: &str) -> bool {
        let before = self.relationships.len();
        self.relationships
            .retain(|r| !(r.source == source && r.target == target));
        self.relationships.len() < before
    }

    /// All relationships for a given source entity.
    pub fn relationships_for(&self, source: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source == source)
            .collect()
    }

    /// All relationships in the graph.
    pub fn all(&self) -> &[Relationship] {
        &self.relationships
    }

    /// Number of relationships.
    pub fn len(&self) -> usize {
        self.relationships.len()
    }

    /// Whether the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.relationships.is_empty()
    }

    /// Decay all relationships one tick.
    pub fn decay_all(&mut self) {
        for rel in &mut self.relationships {
            rel.decay();
        }
    }

    /// Average affinity across all relationships for a source entity.
    pub fn average_affinity(&self, source: &str) -> f32 {
        let mut sum = 0.0f32;
        let mut count = 0u32;
        for r in &self.relationships {
            if r.source == source {
                sum += r.affinity;
                count += 1;
            }
        }
        if count == 0 { 0.0 } else { sum / count as f32 }
    }

    /// Average trust across all relationships for a source entity.
    pub fn average_trust(&self, source: &str) -> f32 {
        let mut sum = 0.0f32;
        let mut count = 0u32;
        for r in &self.relationships {
            if r.source == source {
                sum += r.trust;
                count += 1;
            }
        }
        if count == 0 { 0.5 } else { sum / count as f32 }
    }

    /// Allies (positive affinity) of a source entity.
    pub fn allies(&self, source: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source == source && r.is_positive())
            .collect()
    }

    /// Rivals (negative affinity) of a source entity.
    pub fn rivals(&self, source: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source == source && r.is_negative())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_new() {
        let r = Relationship::new("alice", "bob");
        assert_eq!(r.source, "alice");
        assert_eq!(r.target, "bob");
        assert_eq!(r.rel_type, RelationshipType::Neutral);
        assert!(r.affinity.abs() < f32::EPSILON);
        assert!((r.trust - 0.5).abs() < f32::EPSILON);
        assert_eq!(r.interaction_count, 0);
    }

    #[test]
    fn test_interact() {
        let mut r = Relationship::new("a", "b");
        r.interact(0.3, 0.1);
        assert!((r.affinity - 0.3).abs() < 0.01);
        assert!((r.trust - 0.6).abs() < 0.01);
        assert_eq!(r.interaction_count, 1);
    }

    #[test]
    fn test_interact_clamps() {
        let mut r = Relationship::new("a", "b");
        r.interact(5.0, 5.0);
        assert!((r.affinity - 1.0).abs() < f32::EPSILON);
        assert!((r.trust - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_decay() {
        let mut r = Relationship::new("a", "b");
        r.affinity = 0.8;
        r.trust = 0.9;
        r.decay_rate = 0.5;
        r.decay();
        // affinity decays toward 0
        assert!(r.affinity < 0.8);
        assert!(r.affinity > 0.0);
        // trust decays toward 0.5
        assert!(r.trust < 0.9);
        assert!(r.trust > 0.5);
    }

    #[test]
    fn test_decay_zero_rate() {
        let mut r = Relationship::new("a", "b");
        r.affinity = 0.8;
        r.decay_rate = 0.0;
        r.decay();
        assert!((r.affinity - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_is_positive_negative() {
        let mut r = Relationship::new("a", "b");
        r.affinity = 0.5;
        assert!(r.is_positive());
        assert!(!r.is_negative());
        r.affinity = -0.5;
        assert!(!r.is_positive());
        assert!(r.is_negative());
        r.affinity = 0.0;
        assert!(!r.is_positive());
        assert!(!r.is_negative());
    }

    #[test]
    fn test_graph_new() {
        let g = RelationshipGraph::new();
        assert!(g.is_empty());
        assert_eq!(g.len(), 0);
    }

    #[test]
    fn test_graph_upsert_and_get() {
        let mut g = RelationshipGraph::new();
        let mut r = Relationship::new("a", "b");
        r.affinity = 0.5;
        g.upsert(r);
        assert_eq!(g.len(), 1);
        let found = g.get("a", "b").unwrap();
        assert!((found.affinity - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_graph_upsert_replaces() {
        let mut g = RelationshipGraph::new();
        let mut r1 = Relationship::new("a", "b");
        r1.affinity = 0.5;
        g.upsert(r1);

        let mut r2 = Relationship::new("a", "b");
        r2.affinity = -0.3;
        g.upsert(r2);

        assert_eq!(g.len(), 1);
        assert!((g.get("a", "b").unwrap().affinity - (-0.3)).abs() < 0.01);
    }

    #[test]
    fn test_record_interaction_creates() {
        let mut g = RelationshipGraph::new();
        g.record_interaction("a", "b", 0.3, 0.1);
        assert_eq!(g.len(), 1);
        let r = g.get("a", "b").unwrap();
        assert!((r.affinity - 0.3).abs() < 0.01);
        assert_eq!(r.interaction_count, 1);
    }

    #[test]
    fn test_record_interaction_updates() {
        let mut g = RelationshipGraph::new();
        g.record_interaction("a", "b", 0.3, 0.1);
        g.record_interaction("a", "b", 0.2, 0.05);
        assert_eq!(g.len(), 1);
        let r = g.get("a", "b").unwrap();
        assert!((r.affinity - 0.5).abs() < 0.01);
        assert_eq!(r.interaction_count, 2);
    }

    #[test]
    fn test_remove() {
        let mut g = RelationshipGraph::new();
        g.record_interaction("a", "b", 0.3, 0.1);
        assert!(g.remove("a", "b"));
        assert!(g.is_empty());
        assert!(!g.remove("a", "b"));
    }

    #[test]
    fn test_relationships_for() {
        let mut g = RelationshipGraph::new();
        g.record_interaction("a", "b", 0.3, 0.1);
        g.record_interaction("a", "c", 0.1, 0.0);
        g.record_interaction("b", "a", -0.2, 0.0);

        let a_rels = g.relationships_for("a");
        assert_eq!(a_rels.len(), 2);
    }

    #[test]
    fn test_decay_all() {
        let mut g = RelationshipGraph::new();
        let mut r = Relationship::new("a", "b");
        r.affinity = 0.8;
        r.decay_rate = 0.5;
        g.upsert(r);
        g.decay_all();
        assert!(g.get("a", "b").unwrap().affinity < 0.8);
    }

    #[test]
    fn test_average_affinity() {
        let mut g = RelationshipGraph::new();
        g.record_interaction("a", "b", 0.6, 0.0);
        g.record_interaction("a", "c", 0.2, 0.0);
        let avg = g.average_affinity("a");
        assert!((avg - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_average_affinity_empty() {
        let g = RelationshipGraph::new();
        assert!(g.average_affinity("nobody").abs() < f32::EPSILON);
    }

    #[test]
    fn test_average_trust_empty() {
        let g = RelationshipGraph::new();
        assert!((g.average_trust("nobody") - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_allies_and_rivals() {
        let mut g = RelationshipGraph::new();
        g.record_interaction("a", "b", 0.5, 0.0);
        g.record_interaction("a", "c", -0.5, 0.0);
        g.record_interaction("a", "d", 0.0, 0.0);
        assert_eq!(g.allies("a").len(), 1);
        assert_eq!(g.rivals("a").len(), 1);
    }

    #[test]
    fn test_relationship_type_display() {
        assert_eq!(RelationshipType::Ally.to_string(), "ally");
        assert_eq!(RelationshipType::Rival.to_string(), "rival");
        assert_eq!(RelationshipType::Mentor.to_string(), "mentor");
    }

    #[test]
    fn test_relationship_serde() {
        let mut r = Relationship::new("a", "b");
        r.rel_type = RelationshipType::Ally;
        r.affinity = 0.8;
        let json = serde_json::to_string(&r).unwrap();
        let r2: Relationship = serde_json::from_str(&json).unwrap();
        assert_eq!(r2.source, "a");
        assert_eq!(r2.rel_type, RelationshipType::Ally);
    }

    #[test]
    fn test_graph_serde() {
        let mut g = RelationshipGraph::new();
        g.record_interaction("a", "b", 0.5, 0.1);
        let json = serde_json::to_string(&g).unwrap();
        let g2: RelationshipGraph = serde_json::from_str(&json).unwrap();
        assert_eq!(g2.len(), 1);
    }

    #[test]
    fn test_relationship_type_serde() {
        for t in [
            RelationshipType::Neutral,
            RelationshipType::Ally,
            RelationshipType::Rival,
            RelationshipType::Mentor,
            RelationshipType::Student,
            RelationshipType::TradePartner,
            RelationshipType::Family,
        ] {
            let json = serde_json::to_string(&t).unwrap();
            let restored: RelationshipType = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, t);
        }
    }
}
