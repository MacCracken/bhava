//! ACT-R activation math — frequency × recency memory activation with Hebbian boost.
//!
//! Implements Anderson's ACT-R base-level activation equation (2007):
//!
//! ```text
//! B_i = ln(n) - d × ln(L)
//! ```
//!
//! where `n` = number of rehearsals, `L` = age since first presentation,
//! `d` = decay parameter (~0.5). Plus a recency bonus for recently accessed
//! chunks and Hebbian associative links between co-activated chunks.
//!
//! This models how memory activation determines retrieval probability:
//! frequently rehearsed, recently accessed, and associatively linked chunks
//! are easier to retrieve.

use serde::{Deserialize, Serialize};

/// A single memory chunk with activation tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationEntry {
    /// Identifier for this memory chunk.
    pub tag: String,
    /// Number of times this chunk has been rehearsed/accessed.
    pub count: u32,
    /// Timestamp of first presentation (seconds, simulation time).
    pub first_seen: f64,
    /// Timestamp of most recent access.
    pub last_seen: f64,
    /// Associative boost from co-activated chunks (Hebbian learning).
    pub hebbian_boost: f64,
}

impl ActivationEntry {
    /// ACT-R base-level activation: B = ln(n) - d × ln(L).
    ///
    /// `L` = age of chunk (now - first_seen), minimum 1.0 to avoid ln(0).
    /// Higher count and younger age both increase activation.
    #[must_use]
    #[inline]
    pub fn base_level(&self, now: f64, decay: f64) -> f64 {
        let age = (now - self.first_seen).max(1.0);
        let n = (self.count.max(1)) as f64;
        n.ln() - decay * age.ln()
    }

    /// Recency bonus: exponential decay from last access.
    ///
    /// bonus = e^(-λ(now - last_seen)), λ = ln(2) / half_life.
    #[must_use]
    #[inline]
    pub fn recency_bonus(&self, now: f64, half_life: f64) -> f64 {
        let elapsed = (now - self.last_seen).max(0.0);
        let lambda = core::f64::consts::LN_2 / half_life.max(1.0);
        (-lambda * elapsed).exp()
    }

    /// Total activation = base_level + hebbian_boost + recency_bonus.
    #[must_use]
    #[inline]
    pub fn activation(&self, now: f64, decay: f64, recency_half_life: f64) -> f64 {
        self.base_level(now, decay)
            + self.hebbian_boost
            + self.recency_bonus(now, recency_half_life)
    }
}

/// Associative link between two memory chunks (Hebbian learning).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HebbianLink {
    /// First chunk tag.
    pub tag_a: String,
    /// Second chunk tag.
    pub tag_b: String,
    /// Association strength: 0.0 (no link) to 1.0 (maximally associated).
    pub strength: f64,
}

/// Capacity-bounded collection of activation entries with Hebbian links.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationStore {
    entries: Vec<ActivationEntry>,
    links: Vec<HebbianLink>,
    capacity: usize,
    /// Decay parameter d (default 0.5, from ACT-R literature).
    pub decay: f64,
    /// Recency bonus half-life in seconds (default 300.0 = 5 minutes).
    pub recency_half_life: f64,
}

impl ActivationStore {
    /// Create an empty store with the given capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            links: Vec::new(),
            capacity: capacity.max(1),
            decay: 0.5,
            recency_half_life: 300.0,
        }
    }

    /// Rehearse a chunk: increment count and update last_seen.
    ///
    /// Creates a new entry if the tag is not found. Evicts the
    /// lowest-activation entry when at capacity.
    pub fn rehearse(&mut self, tag: impl Into<String>, now: f64) {
        let tag = tag.into();
        if let Some(entry) = self.entries.iter_mut().find(|e| e.tag == tag) {
            entry.count = entry.count.saturating_add(1);
            entry.last_seen = now;
            return;
        }

        // New entry — evict if at capacity
        if self.entries.len() >= self.capacity {
            self.evict_lowest(now);
        }

        self.entries.push(ActivationEntry {
            tag,
            count: 1,
            first_seen: now,
            last_seen: now,
            hebbian_boost: 0.0,
        });
    }

    /// Create or strengthen a Hebbian link between two chunks.
    ///
    /// Strength approaches 1.0 asymptotically:
    /// `s_new = s_old + delta × (1.0 - s_old)`.
    pub fn strengthen_link(
        &mut self,
        tag_a: impl Into<String>,
        tag_b: impl Into<String>,
        delta: f64,
    ) {
        let tag_a = tag_a.into();
        let tag_b = tag_b.into();
        let delta = delta.clamp(0.0, 1.0);

        if let Some(link) = self.links.iter_mut().find(|l| {
            (l.tag_a == tag_a && l.tag_b == tag_b) || (l.tag_a == tag_b && l.tag_b == tag_a)
        }) {
            link.strength = (link.strength + delta * (1.0 - link.strength)).min(1.0);
            return;
        }

        self.links.push(HebbianLink {
            tag_a,
            tag_b,
            strength: delta.min(1.0),
        });
    }

    /// Get activation for a specific tag. Returns `None` if not found.
    #[must_use]
    pub fn retrieve(&self, tag: &str, now: f64) -> Option<f64> {
        self.entries
            .iter()
            .find(|e| e.tag == tag)
            .map(|e| e.activation(now, self.decay, self.recency_half_life))
    }

    /// Retrieve all entries with activation above threshold, sorted descending.
    #[must_use]
    pub fn retrieve_above(&self, threshold: f64, now: f64) -> Vec<(&ActivationEntry, f64)> {
        let mut results: Vec<_> = self
            .entries
            .iter()
            .map(|e| (e, e.activation(now, self.decay, self.recency_half_life)))
            .filter(|&(_, a)| a >= threshold)
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Spread activation from a source chunk through Hebbian links.
    ///
    /// For each link from source, adds `source_activation × link_strength × 0.1`
    /// to the linked entry's hebbian_boost. The 0.1 dampening factor prevents
    /// runaway activation. Hebbian boosts clamped to 0.0–5.0.
    pub fn spread_activation(&mut self, source: &str, now: f64) {
        let source_activation = match self.retrieve(source, now) {
            Some(a) => a,
            None => return,
        };

        // Collect link targets and their boost deltas
        let boosts: Vec<(String, f64)> = self
            .links
            .iter()
            .filter_map(|link| {
                let target = if link.tag_a == source {
                    Some(&link.tag_b)
                } else if link.tag_b == source {
                    Some(&link.tag_a)
                } else {
                    None
                }?;
                let boost = source_activation * link.strength * 0.1;
                Some((target.clone(), boost))
            })
            .collect();

        // Apply boosts
        for (target_tag, boost) in boosts {
            if let Some(entry) = self.entries.iter_mut().find(|e| e.tag == target_tag) {
                entry.hebbian_boost = (entry.hebbian_boost + boost).clamp(0.0, 5.0);
            }
        }
    }

    /// Number of stored entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the store is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Number of Hebbian links.
    #[must_use]
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Evict the entry with the lowest activation.
    fn evict_lowest(&mut self, now: f64) {
        if self.entries.is_empty() {
            return;
        }
        let mut min_idx = 0;
        let mut min_act = f64::MAX;
        for (i, e) in self.entries.iter().enumerate() {
            let a = e.activation(now, self.decay, self.recency_half_life);
            if a < min_act {
                min_act = a;
                min_idx = i;
            }
        }
        self.entries.swap_remove(min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_level_increases_with_count() {
        let e = ActivationEntry {
            tag: "test".into(),
            count: 1,
            first_seen: 0.0,
            last_seen: 10.0,
            hebbian_boost: 0.0,
        };
        let e2 = ActivationEntry {
            count: 10,
            ..e.clone()
        };
        assert!(
            e2.base_level(100.0, 0.5) > e.base_level(100.0, 0.5),
            "more rehearsals should increase base level"
        );
    }

    #[test]
    fn test_base_level_decreases_with_age() {
        let e = ActivationEntry {
            tag: "test".into(),
            count: 5,
            first_seen: 0.0,
            last_seen: 0.0,
            hebbian_boost: 0.0,
        };
        let young = e.base_level(10.0, 0.5);
        let old = e.base_level(1000.0, 0.5);
        assert!(young > old, "young={young} old={old}");
    }

    #[test]
    fn test_recency_bonus_decays() {
        let e = ActivationEntry {
            tag: "test".into(),
            count: 1,
            first_seen: 0.0,
            last_seen: 100.0,
            hebbian_boost: 0.0,
        };
        let recent = e.recency_bonus(101.0, 300.0);
        let stale = e.recency_bonus(1000.0, 300.0);
        assert!(recent > stale, "recent={recent} stale={stale}");
    }

    #[test]
    fn test_recency_bonus_at_access_time() {
        let e = ActivationEntry {
            tag: "test".into(),
            count: 1,
            first_seen: 0.0,
            last_seen: 100.0,
            hebbian_boost: 0.0,
        };
        let bonus = e.recency_bonus(100.0, 300.0);
        assert!((bonus - 1.0).abs() < f64::EPSILON, "at access: {bonus}");
    }

    #[test]
    fn test_rehearse_creates_entry() {
        let mut store = ActivationStore::new(10);
        store.rehearse("foo", 1.0);
        assert_eq!(store.len(), 1);
        assert!(store.retrieve("foo", 1.0).is_some());
    }

    #[test]
    fn test_rehearse_increments() {
        let mut store = ActivationStore::new(10);
        store.rehearse("foo", 1.0);
        let a1 = store.retrieve("foo", 2.0).unwrap();
        store.rehearse("foo", 2.0);
        let a2 = store.retrieve("foo", 2.0).unwrap();
        assert!(a2 > a1, "second rehearsal should increase activation");
    }

    #[test]
    fn test_eviction_at_capacity() {
        let mut store = ActivationStore::new(2);
        store.rehearse("a", 1.0);
        store.rehearse("b", 2.0);
        // Rehearse b many times to make it higher activation
        for i in 0..10 {
            store.rehearse("b", 3.0 + i as f64);
        }
        // Add c — should evict a (lowest activation)
        store.rehearse("c", 15.0);
        assert_eq!(store.len(), 2);
        assert!(store.retrieve("a", 15.0).is_none(), "a should be evicted");
        assert!(store.retrieve("b", 15.0).is_some());
        assert!(store.retrieve("c", 15.0).is_some());
    }

    #[test]
    fn test_hebbian_link_creates() {
        let mut store = ActivationStore::new(10);
        store.strengthen_link("a", "b", 0.3);
        assert_eq!(store.link_count(), 1);
    }

    #[test]
    fn test_hebbian_link_strengthens() {
        let mut store = ActivationStore::new(10);
        store.strengthen_link("a", "b", 0.3);
        store.strengthen_link("a", "b", 0.3);
        // s = 0.3, then s = 0.3 + 0.3*(1-0.3) = 0.3 + 0.21 = 0.51
        assert_eq!(store.link_count(), 1, "should not create duplicate link");
        let link = &store.links[0];
        assert!(link.strength > 0.3, "should strengthen: {}", link.strength);
        assert!(link.strength < 1.0, "should not exceed 1.0");
    }

    #[test]
    fn test_hebbian_link_asymptotic() {
        let mut store = ActivationStore::new(10);
        for _ in 0..100 {
            store.strengthen_link("a", "b", 0.5);
        }
        let link = &store.links[0];
        assert!(
            (link.strength - 1.0).abs() < 0.01,
            "should approach 1.0: {}",
            link.strength
        );
    }

    #[test]
    fn test_spread_activation() {
        let mut store = ActivationStore::new(10);
        store.rehearse("source", 1.0);
        store.rehearse("target", 1.0);
        // Rehearse source many times
        for i in 0..10 {
            store.rehearse("source", 2.0 + i as f64);
        }
        store.strengthen_link("source", "target", 0.8);

        let before = store.retrieve("target", 15.0).unwrap();
        store.spread_activation("source", 15.0);
        let after = store.retrieve("target", 15.0).unwrap();
        assert!(
            after > before,
            "spread should boost target: before={before} after={after}"
        );
    }

    #[test]
    fn test_spread_activation_nonexistent_source() {
        let mut store = ActivationStore::new(10);
        store.rehearse("a", 1.0);
        store.spread_activation("nonexistent", 1.0); // should not panic
    }

    #[test]
    fn test_retrieve_above_threshold() {
        let mut store = ActivationStore::new(10);
        store.rehearse("high", 1.0);
        for i in 0..20 {
            store.rehearse("high", 2.0 + i as f64);
        }
        store.rehearse("low", 1.0);

        let results = store.retrieve_above(0.0, 25.0);
        assert!(!results.is_empty());
        // Should be sorted descending
        if results.len() >= 2 {
            assert!(results[0].1 >= results[1].1);
        }
    }

    #[test]
    fn test_empty_store() {
        let store = ActivationStore::new(10);
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(store.retrieve("anything", 1.0).is_none());
    }

    #[test]
    fn test_base_level_min_age() {
        // Age = 0 should use minimum 1.0, not produce -inf from ln(0)
        let e = ActivationEntry {
            tag: "test".into(),
            count: 1,
            first_seen: 10.0,
            last_seen: 10.0,
            hebbian_boost: 0.0,
        };
        let b = e.base_level(10.0, 0.5);
        assert!(b.is_finite(), "base level at age 0 should be finite: {b}");
    }

    #[test]
    fn test_serde_store() {
        let mut store = ActivationStore::new(10);
        store.rehearse("test", 1.0);
        store.strengthen_link("a", "b", 0.5);
        let json = serde_json::to_string(&store).unwrap();
        let store2: ActivationStore = serde_json::from_str(&json).unwrap();
        assert_eq!(store2.len(), store.len());
        assert_eq!(store2.link_count(), store.link_count());
    }

    #[test]
    fn test_serde_entry() {
        let e = ActivationEntry {
            tag: "test".into(),
            count: 5,
            first_seen: 1.0,
            last_seen: 10.0,
            hebbian_boost: 0.3,
        };
        let json = serde_json::to_string(&e).unwrap();
        let e2: ActivationEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(e2.tag, "test");
        assert_eq!(e2.count, 5);
    }
}
