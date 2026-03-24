//! Preference learning — adaptive feedback patterns from interaction history.
//!
//! Learns preferences from repeated interaction outcomes using an exponential
//! moving average with decreasing learning rate. Early experiences have more
//! impact (rapid initial learning), while later experiences refine the
//! preference (stabilization).
//!
//! Preferences are tagged with identifiers (entity names, topics, action types)
//! and carry a valence from -1.0 (strong aversion) to 1.0 (strong preference).
//! Optional personality bias: Warm agents form positive preferences faster,
//! Skeptical agents weight negative experiences more heavily.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A learned preference for a tagged stimulus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceEntry {
    /// What this preference is about (entity, topic, action type).
    pub tag: String,
    /// Learned valence: -1.0 (strong aversion) to 1.0 (strong preference).
    pub valence: f32,
    /// Number of outcome observations.
    pub exposure_count: u32,
    /// When this preference was last updated.
    pub last_exposure: DateTime<Utc>,
}

impl PreferenceEntry {
    /// Learning rate alpha decreases with exposure.
    ///
    /// `alpha = 1 / (1 + exposure_count)`:
    /// First exposure → 0.5, second → 0.33, tenth → 0.09.
    /// Early experiences dominate; later ones refine.
    #[must_use]
    #[inline]
    fn alpha(&self) -> f32 {
        1.0 / (1.0 + self.exposure_count as f32)
    }

    /// Update valence with a new outcome observation.
    fn update(&mut self, outcome: f32, bias: &PreferenceBias, now: DateTime<Utc>) {
        let alpha = self.alpha();
        let biased_outcome = if outcome >= 0.0 {
            outcome * bias.positive_gain
        } else {
            outcome * bias.negative_gain
        };
        self.valence = (self.valence * (1.0 - alpha) + biased_outcome * alpha).clamp(-1.0, 1.0);
        self.exposure_count = self.exposure_count.saturating_add(1);
        self.last_exposure = now;
    }
}

/// Personality bias for preference learning rate.
///
/// Modulates how quickly positive vs negative preferences form.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PreferenceBias {
    /// Multiplier on positive outcomes. Default: 1.0.
    pub positive_gain: f32,
    /// Multiplier on negative outcomes. Default: 1.0.
    pub negative_gain: f32,
}

impl Default for PreferenceBias {
    fn default() -> Self {
        Self {
            positive_gain: 1.0,
            negative_gain: 1.0,
        }
    }
}

impl PreferenceBias {
    /// Neutral bias — no personality modulation.
    #[must_use]
    pub fn neutral() -> Self {
        Self::default()
    }
}

/// Derive preference bias from personality traits.
///
/// - Warm agents: positive_gain boosted (form positive preferences faster)
/// - Skeptical agents: negative_gain boosted (weight negative experiences more)
#[cfg(feature = "traits")]
#[must_use]
pub fn bias_from_personality(profile: &crate::traits::PersonalityProfile) -> PreferenceBias {
    use crate::traits::TraitKind;
    let warmth = profile.get_trait(TraitKind::Warmth).normalized();
    let skepticism = profile.get_trait(TraitKind::Skepticism).normalized();

    PreferenceBias {
        positive_gain: (1.0 + warmth * 0.3).clamp(0.5, 1.5),
        negative_gain: (1.0 + skepticism * 0.3).clamp(0.5, 1.5),
    }
}

/// Capacity-bounded collection of learned preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceStore {
    entries: Vec<PreferenceEntry>,
    capacity: usize,
    /// Personality-driven bias for learning rate.
    pub bias: PreferenceBias,
}

impl PreferenceStore {
    /// Create an empty store with the given capacity and neutral bias.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            capacity: capacity.max(1),
            bias: PreferenceBias::neutral(),
        }
    }

    /// Create with a specific bias.
    #[must_use]
    pub fn with_bias(capacity: usize, bias: PreferenceBias) -> Self {
        Self {
            entries: Vec::new(),
            capacity: capacity.max(1),
            bias,
        }
    }

    /// Record an outcome for a tag.
    ///
    /// `outcome` ranges from -1.0 (terrible) to 1.0 (excellent).
    /// Creates a new entry if the tag is not found. Evicts the entry
    /// with the weakest |valence| when at capacity.
    pub fn record_outcome(&mut self, tag: impl Into<String>, outcome: f32, now: DateTime<Utc>) {
        let tag = tag.into();
        let outcome = outcome.clamp(-1.0, 1.0);

        if let Some(entry) = self.entries.iter_mut().find(|e| e.tag == tag) {
            entry.update(outcome, &self.bias, now);
            return;
        }

        // New entry — evict if at capacity
        if self.entries.len() >= self.capacity {
            self.evict_weakest();
        }

        let mut entry = PreferenceEntry {
            tag,
            valence: 0.0,
            exposure_count: 0,
            last_exposure: now,
        };
        entry.update(outcome, &self.bias, now);
        self.entries.push(entry);
    }

    /// Get current preference valence for a tag.
    #[must_use]
    pub fn preference_for(&self, tag: &str) -> Option<f32> {
        self.entries
            .iter()
            .find(|e| e.tag == tag)
            .map(|e| e.valence)
    }

    /// Get the full entry for a tag.
    #[must_use]
    pub fn get(&self, tag: &str) -> Option<&PreferenceEntry> {
        self.entries.iter().find(|e| e.tag == tag)
    }

    /// Decay all preferences toward neutral.
    ///
    /// `valence *= (1.0 - rate)`. Removes entries where |valence| < 0.01
    /// and exposure_count < 2 (weak, barely-formed preferences).
    pub fn decay(&mut self, rate: f32) {
        let rate = rate.clamp(0.0, 1.0);
        for entry in &mut self.entries {
            entry.valence *= 1.0 - rate;
        }
        self.entries
            .retain(|e| e.valence.abs() >= 0.01 || e.exposure_count >= 2);
    }

    /// Top N strongest positive preferences, sorted by valence descending.
    #[must_use]
    pub fn top_preferences(&self, n: usize) -> Vec<(&str, f32)> {
        let mut positive: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.valence > 0.0)
            .map(|e| (e.tag.as_str(), e.valence))
            .collect();
        positive.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        positive.truncate(n);
        positive
    }

    /// Top N strongest negative preferences (aversions), sorted by valence ascending.
    #[must_use]
    pub fn bottom_preferences(&self, n: usize) -> Vec<(&str, f32)> {
        let mut negative: Vec<_> = self
            .entries
            .iter()
            .filter(|e| e.valence < 0.0)
            .map(|e| (e.tag.as_str(), e.valence))
            .collect();
        negative.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        negative.truncate(n);
        negative
    }

    /// Number of stored preferences.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the store is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Evict the entry with the weakest absolute valence.
    fn evict_weakest(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let mut min_idx = 0;
        let mut min_val = f32::MAX;
        for (i, e) in self.entries.iter().enumerate() {
            if e.valence.abs() < min_val {
                min_val = e.valence.abs();
                min_idx = i;
            }
        }
        self.entries.swap_remove(min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    #[test]
    fn test_new_entry_moves_toward_outcome() {
        let mut store = PreferenceStore::new(10);
        store.record_outcome("agent_a", 0.8, now());
        let v = store.preference_for("agent_a").unwrap();
        assert!(
            v > 0.0,
            "positive outcome should produce positive valence: {v}"
        );
    }

    #[test]
    fn test_alpha_decreases_with_exposure() {
        let e0 = PreferenceEntry {
            tag: "test".into(),
            valence: 0.0,
            exposure_count: 0,
            last_exposure: now(),
        };
        let e10 = PreferenceEntry {
            exposure_count: 10,
            ..e0.clone()
        };
        assert!(e0.alpha() > e10.alpha(), "alpha should decrease");
        assert!((e0.alpha() - 1.0).abs() < f32::EPSILON); // 1/(1+0) = 1.0
        assert!((e10.alpha() - 1.0 / 11.0).abs() < 0.01);
    }

    #[test]
    fn test_repeated_positive_converges() {
        let mut store = PreferenceStore::new(10);
        for _ in 0..20 {
            store.record_outcome("liked", 0.9, now());
        }
        let v = store.preference_for("liked").unwrap();
        assert!(v > 0.7, "20 positive outcomes should converge high: {v}");
    }

    #[test]
    fn test_repeated_negative_converges() {
        let mut store = PreferenceStore::new(10);
        for _ in 0..20 {
            store.record_outcome("disliked", -0.9, now());
        }
        let v = store.preference_for("disliked").unwrap();
        assert!(v < -0.7, "20 negative outcomes should converge low: {v}");
    }

    #[test]
    fn test_mixed_outcomes_near_zero() {
        let mut store = PreferenceStore::new(10);
        for i in 0..20 {
            let outcome = if i % 2 == 0 { 0.5 } else { -0.5 };
            store.record_outcome("mixed", outcome, now());
        }
        let v = store.preference_for("mixed").unwrap();
        assert!(v.abs() < 0.3, "mixed outcomes should be near neutral: {v}");
    }

    #[test]
    fn test_early_experience_dominates() {
        // First positive outcome should have more impact than later negative ones
        let mut store = PreferenceStore::new(10);
        store.record_outcome("test", 1.0, now()); // alpha=1.0 → valence=1.0
        store.record_outcome("test", -0.5, now()); // alpha=0.5 → smaller shift
        let v = store.preference_for("test").unwrap();
        assert!(v > 0.0, "early strong positive should still dominate: {v}");
    }

    #[test]
    fn test_decay_toward_neutral() {
        let mut store = PreferenceStore::new(10);
        store.record_outcome("test", 0.8, now());
        let before = store.preference_for("test").unwrap();
        store.decay(0.3);
        let after = store.preference_for("test").unwrap();
        assert!(after.abs() < before.abs(), "decay should reduce |valence|");
    }

    #[test]
    fn test_decay_removes_weak() {
        let mut store = PreferenceStore::new(10);
        store.record_outcome("weak", 0.005, now());
        // exposure_count is 1, valence ~0.005 → below 0.01 and count < 2
        store.decay(0.5);
        assert!(
            store.preference_for("weak").is_none(),
            "weak preference should be removed"
        );
    }

    #[test]
    fn test_top_preferences_sorted() {
        let mut store = PreferenceStore::new(10);
        for _ in 0..10 {
            store.record_outcome("best", 0.9, now());
        }
        for _ in 0..10 {
            store.record_outcome("good", 0.5, now());
        }
        for _ in 0..10 {
            store.record_outcome("bad", -0.5, now());
        }
        let top = store.top_preferences(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "best");
        assert_eq!(top[1].0, "good");
    }

    #[test]
    fn test_bottom_preferences_sorted() {
        let mut store = PreferenceStore::new(10);
        for _ in 0..10 {
            store.record_outcome("worst", -0.9, now());
        }
        for _ in 0..10 {
            store.record_outcome("bad", -0.3, now());
        }
        let bottom = store.bottom_preferences(2);
        assert_eq!(bottom.len(), 2);
        assert_eq!(bottom[0].0, "worst");
    }

    #[test]
    fn test_eviction_weakest() {
        let mut store = PreferenceStore::new(2);
        // Strong preference
        for _ in 0..10 {
            store.record_outcome("strong", 0.9, now());
        }
        // Weak preference
        store.record_outcome("weak", 0.1, now());
        assert_eq!(store.len(), 2);
        // Third entry should evict weakest
        for _ in 0..5 {
            store.record_outcome("medium", 0.5, now());
        }
        assert_eq!(store.len(), 2);
        assert!(
            store.preference_for("weak").is_none(),
            "weak should be evicted"
        );
        assert!(store.preference_for("strong").is_some());
        assert!(store.preference_for("medium").is_some());
    }

    #[test]
    fn test_valence_clamped() {
        let mut store = PreferenceStore::new(10);
        store.bias = PreferenceBias {
            positive_gain: 5.0,
            negative_gain: 5.0,
        };
        for _ in 0..50 {
            store.record_outcome("extreme", 1.0, now());
        }
        let v = store.preference_for("extreme").unwrap();
        assert!(v <= 1.0, "valence should be clamped: {v}");
    }

    #[test]
    fn test_empty_store() {
        let store = PreferenceStore::new(10);
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(store.preference_for("anything").is_none());
        assert!(store.top_preferences(5).is_empty());
    }

    #[test]
    fn test_with_bias() {
        let bias = PreferenceBias {
            positive_gain: 1.5,
            negative_gain: 0.5,
        };
        let store = PreferenceStore::with_bias(10, bias);
        assert!((store.bias.positive_gain - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_get_entry() {
        let mut store = PreferenceStore::new(10);
        store.record_outcome("test", 0.5, now());
        let entry = store.get("test").unwrap();
        assert_eq!(entry.exposure_count, 1);
    }

    #[test]
    fn test_serde_store() {
        let mut store = PreferenceStore::new(10);
        store.record_outcome("test", 0.7, now());
        let json = serde_json::to_string(&store).unwrap();
        let store2: PreferenceStore = serde_json::from_str(&json).unwrap();
        assert_eq!(store2.len(), store.len());
    }

    #[test]
    fn test_serde_bias() {
        let b = PreferenceBias {
            positive_gain: 1.3,
            negative_gain: 0.8,
        };
        let json = serde_json::to_string(&b).unwrap();
        let b2: PreferenceBias = serde_json::from_str(&json).unwrap();
        assert!((b2.positive_gain - 1.3).abs() < f32::EPSILON);
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_bias_from_personality_warm() {
        let mut p = crate::traits::PersonalityProfile::new("warm");
        p.set_trait(
            crate::traits::TraitKind::Warmth,
            crate::traits::TraitLevel::Highest,
        );
        let bias = bias_from_personality(&p);
        assert!(
            bias.positive_gain > 1.0,
            "warm should boost positive: {}",
            bias.positive_gain
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_bias_from_personality_skeptical() {
        let mut p = crate::traits::PersonalityProfile::new("skeptic");
        p.set_trait(
            crate::traits::TraitKind::Skepticism,
            crate::traits::TraitLevel::Highest,
        );
        let bias = bias_from_personality(&p);
        assert!(
            bias.negative_gain > 1.0,
            "skeptic should boost negative: {}",
            bias.negative_gain
        );
    }

    #[cfg(feature = "traits")]
    #[test]
    fn test_bias_warmth_forms_positive_faster() {
        let mut warm_store = PreferenceStore::with_bias(
            10,
            PreferenceBias {
                positive_gain: 1.3,
                negative_gain: 1.0,
            },
        );
        let mut neutral_store = PreferenceStore::new(10);
        for _ in 0..5 {
            warm_store.record_outcome("agent", 0.5, now());
            neutral_store.record_outcome("agent", 0.5, now());
        }
        let warm_v = warm_store.preference_for("agent").unwrap();
        let neutral_v = neutral_store.preference_for("agent").unwrap();
        assert!(
            warm_v > neutral_v,
            "warm={warm_v} should > neutral={neutral_v}"
        );
    }
}
