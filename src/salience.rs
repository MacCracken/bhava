//! Salience classification — somatic marker urgency/importance scoring.
//!
//! Based on Damasio's somatic marker hypothesis (1994): emotionally significant
//! events leave body-state markers that bias future decision-making. Salience
//! is scored on two orthogonal dimensions:
//!
//! - **Urgency** — how time-critical is this? Driven by desirability, likelihood,
//!   and current arousal/deviation.
//! - **Importance** — how much does this matter long-term? Driven by moral
//!   weight (praiseworthiness), prior memory intensity, and causal attribution.
//!
//! Combined salience uses the geometric mean (both must be nonzero for high
//! salience), classified into Background / Notable / Significant / Critical.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::appraisal::Appraisal;
use crate::mood::{EmotionalMemory, MoodVector};

/// Urgency × importance score for an event or memory.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SalienceScore {
    /// How time-critical is this? 0.0 = no urgency, 1.0 = act now.
    pub urgency: f32,
    /// How much does this matter long-term? 0.0 = trivial, 1.0 = life-defining.
    pub importance: f32,
}

impl SalienceScore {
    /// Create a score (values clamped to 0.0–1.0).
    #[must_use]
    pub fn new(urgency: f32, importance: f32) -> Self {
        Self {
            urgency: urgency.clamp(0.0, 1.0),
            importance: importance.clamp(0.0, 1.0),
        }
    }

    /// Zero salience — completely unremarkable.
    #[must_use]
    pub fn zero() -> Self {
        Self {
            urgency: 0.0,
            importance: 0.0,
        }
    }

    /// Combined salience magnitude: sqrt(urgency × importance).
    ///
    /// Geometric mean ensures both dimensions must contribute for high salience.
    /// An urgent but unimportant event scores lower than one that is both.
    #[must_use]
    #[inline]
    pub fn magnitude(&self) -> f32 {
        (self.urgency * self.importance).sqrt()
    }

    /// Classify into a discrete salience level.
    #[must_use]
    pub fn level(&self) -> SalienceLevel {
        let m = self.magnitude();
        if m >= 0.75 {
            SalienceLevel::Critical
        } else if m >= 0.45 {
            SalienceLevel::Significant
        } else if m >= 0.2 {
            SalienceLevel::Notable
        } else {
            SalienceLevel::Background
        }
    }
}

/// Discrete salience classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SalienceLevel {
    /// Below awareness threshold (magnitude < 0.2).
    Background,
    /// Noticed but not prioritized (0.2–0.45).
    Notable,
    /// Demands attention (0.45–0.75).
    Significant,
    /// Immediate action required (>= 0.75).
    Critical,
}

impl fmt::Display for SalienceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Background => "background",
            Self::Notable => "notable",
            Self::Significant => "significant",
            Self::Critical => "critical",
        };
        f.write_str(s)
    }
}

/// Score salience of an appraisal given current mood state.
///
/// - **Urgency** = |desirability| × likelihood × (1 + mood_deviation).
///   Strong desirability + high likelihood + already-aroused agent = urgent.
/// - **Importance** = max(|desirability|, |praiseworthiness|) × (1 + memory_intensity).
///   Morally weighted events matter more; reinforced by prior memories.
///
/// Both dimensions clamped to 0.0–1.0.
#[must_use]
pub fn classify_salience(
    appraisal: &Appraisal,
    mood_deviation: f32,
    memory_intensity: f32,
) -> SalienceScore {
    let urgency = (appraisal.desirability.abs()
        * appraisal.likelihood
        * (1.0 + mood_deviation.clamp(0.0, 1.0)))
    .clamp(0.0, 1.0);

    let importance = (appraisal
        .desirability
        .abs()
        .max(appraisal.praiseworthiness.abs())
        * (1.0 + memory_intensity.clamp(0.0, 1.0)))
    .clamp(0.0, 1.0);

    SalienceScore {
        urgency,
        importance,
    }
}

/// Score salience of a stored emotional memory.
///
/// Synthesizes urgency from arousal intensity and importance from
/// the emotional magnitude of the stored mood.
#[must_use]
pub fn memory_salience(memory: &EmotionalMemory) -> SalienceScore {
    let urgency = (memory.mood.arousal.abs() * memory.intensity).clamp(0.0, 1.0);
    let importance =
        (memory.mood.joy.abs().max(memory.mood.dominance.abs()) * memory.intensity).clamp(0.0, 1.0);

    SalienceScore {
        urgency,
        importance,
    }
}

/// Filter a slice of memories by salience threshold.
///
/// Returns matching memories paired with their salience scores,
/// sorted by magnitude descending. Use with `recall_congruent` or
/// any other source of `EmotionalMemory` references.
#[must_use]
pub fn filter_salient<'a>(
    memories: &[&'a EmotionalMemory],
    threshold: f32,
) -> Vec<(&'a EmotionalMemory, SalienceScore)> {
    let mut results: Vec<_> = memories
        .iter()
        .filter_map(|&mem| {
            let score = memory_salience(mem);
            if score.magnitude() >= threshold {
                Some((mem, score))
            } else {
                None
            }
        })
        .collect();

    results.sort_by(|a, b| {
        b.1.magnitude()
            .partial_cmp(&a.1.magnitude())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results
}

/// Compute a salience-weighted mood from a set of memories.
///
/// More salient memories contribute more to the resulting mood vector.
/// Returns neutral if no memories exceed the threshold.
#[must_use]
pub fn salience_weighted_mood(memories: &[&EmotionalMemory], threshold: f32) -> MoodVector {
    let mut weighted_sum = MoodVector::neutral();
    let mut total_weight = 0.0f32;

    for &mem in memories {
        let score = memory_salience(mem);
        let weight = score.magnitude();
        if weight < threshold {
            continue;
        }
        for &e in crate::mood::Emotion::ALL {
            let current = weighted_sum.get(e);
            weighted_sum.set(e, current + mem.mood.get(e) * weight * mem.intensity);
        }
        total_weight += weight;
    }

    if total_weight > f32::EPSILON {
        for &e in crate::mood::Emotion::ALL {
            let v = weighted_sum.get(e) / total_weight;
            weighted_sum.set(e, v);
        }
    }

    weighted_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::appraisal::Appraisal;
    use crate::mood::{Emotion, EmotionalMemory, MoodVector};

    #[test]
    fn test_neutral_appraisal_is_background() {
        let a = Appraisal::event("nothing", 0.0);
        let score = classify_salience(&a, 0.0, 0.0);
        assert_eq!(score.level(), SalienceLevel::Background);
    }

    #[test]
    fn test_extreme_appraisal_is_critical() {
        let a = Appraisal::event("crisis", 1.0).with_praise(0.9);
        let score = classify_salience(&a, 0.8, 0.5);
        assert_eq!(
            score.level(),
            SalienceLevel::Critical,
            "magnitude={}",
            score.magnitude()
        );
    }

    #[test]
    fn test_magnitude_geometric_mean() {
        let s = SalienceScore::new(0.64, 1.0);
        assert!((s.magnitude() - 0.8).abs() < 0.001, "mag={}", s.magnitude());
    }

    #[test]
    fn test_magnitude_zero_if_either_zero() {
        let s = SalienceScore::new(0.0, 1.0);
        assert!(s.magnitude().abs() < f32::EPSILON);
        let s2 = SalienceScore::new(1.0, 0.0);
        assert!(s2.magnitude().abs() < f32::EPSILON);
    }

    #[test]
    fn test_importance_from_praiseworthiness() {
        let a = Appraisal::event("moral", 0.1).with_praise(0.9);
        let score = classify_salience(&a, 0.0, 0.0);
        // praiseworthiness (0.9) > desirability (0.1), so importance driven by praise
        assert!(score.importance > 0.5, "importance={}", score.importance);
    }

    #[test]
    fn test_urgency_from_deviation() {
        let a = Appraisal::event("urgent", 0.8);
        let calm = classify_salience(&a, 0.0, 0.0);
        let aroused = classify_salience(&a, 0.8, 0.0);
        assert!(
            aroused.urgency > calm.urgency,
            "aroused={} calm={}",
            aroused.urgency,
            calm.urgency
        );
    }

    #[test]
    fn test_salience_level_thresholds() {
        assert_eq!(
            SalienceScore::new(0.01, 0.01).level(),
            SalienceLevel::Background
        );
        assert_eq!(SalienceScore::new(0.3, 0.3).level(), SalienceLevel::Notable);
        assert_eq!(
            SalienceScore::new(0.6, 0.6).level(),
            SalienceLevel::Significant
        );
        assert_eq!(
            SalienceScore::new(0.9, 0.9).level(),
            SalienceLevel::Critical
        );
    }

    #[test]
    fn test_salience_score_clamps() {
        let s = SalienceScore::new(2.0, -1.0);
        assert!((s.urgency - 1.0).abs() < f32::EPSILON);
        assert!(s.importance.abs() < f32::EPSILON);
    }

    #[test]
    fn test_memory_salience() {
        let mut mood = MoodVector::neutral();
        mood.set(Emotion::Arousal, 0.8);
        mood.set(Emotion::Joy, 0.7);
        let mem = EmotionalMemory {
            tag: "intense".into(),
            mood,
            intensity: 0.9,
        };
        let score = memory_salience(&mem);
        assert!(score.urgency > 0.5, "urgency={}", score.urgency);
        assert!(score.importance > 0.5, "importance={}", score.importance);
    }

    #[test]
    fn test_filter_salient() {
        let mut strong_mood = MoodVector::neutral();
        strong_mood.set(Emotion::Arousal, 0.9);
        strong_mood.set(Emotion::Joy, 0.8);
        let strong = EmotionalMemory {
            tag: "strong".into(),
            mood: strong_mood,
            intensity: 0.9,
        };
        let weak = EmotionalMemory {
            tag: "weak".into(),
            mood: MoodVector::neutral(),
            intensity: 0.1,
        };
        let memories: Vec<&EmotionalMemory> = vec![&strong, &weak];
        let results = filter_salient(&memories, 0.3);
        assert_eq!(results.len(), 1, "only strong should pass threshold");
        assert_eq!(results[0].0.tag, "strong");
    }

    #[test]
    fn test_filter_salient_sorted() {
        let mut m1 = MoodVector::neutral();
        m1.set(Emotion::Arousal, 0.5);
        m1.set(Emotion::Joy, 0.5);
        let medium = EmotionalMemory {
            tag: "medium".into(),
            mood: m1,
            intensity: 0.6,
        };
        let mut m2 = MoodVector::neutral();
        m2.set(Emotion::Arousal, 0.9);
        m2.set(Emotion::Joy, 0.9);
        let high = EmotionalMemory {
            tag: "high".into(),
            mood: m2,
            intensity: 0.9,
        };
        let memories: Vec<&EmotionalMemory> = vec![&medium, &high];
        let results = filter_salient(&memories, 0.1);
        assert!(results.len() >= 2);
        assert!(results[0].1.magnitude() >= results[1].1.magnitude());
    }

    #[test]
    fn test_salience_weighted_mood_empty() {
        let memories: Vec<&EmotionalMemory> = vec![];
        let mood = salience_weighted_mood(&memories, 0.0);
        assert!(mood.intensity() < f32::EPSILON);
    }

    #[test]
    fn test_salience_weighted_mood_biased() {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        m.set(Emotion::Arousal, 0.7);
        let mem = EmotionalMemory {
            tag: "happy".into(),
            mood: m,
            intensity: 0.9,
        };
        let memories: Vec<&EmotionalMemory> = vec![&mem];
        let recalled = salience_weighted_mood(&memories, 0.0);
        assert!(recalled.joy > 0.0, "should recall positive joy");
    }

    #[test]
    fn test_level_display() {
        assert_eq!(SalienceLevel::Background.to_string(), "background");
        assert_eq!(SalienceLevel::Critical.to_string(), "critical");
    }

    #[test]
    fn test_zero_score() {
        let s = SalienceScore::zero();
        assert!(s.magnitude().abs() < f32::EPSILON);
        assert_eq!(s.level(), SalienceLevel::Background);
    }

    #[test]
    fn test_serde() {
        let s = SalienceScore::new(0.7, 0.8);
        let json = serde_json::to_string(&s).unwrap();
        let s2: SalienceScore = serde_json::from_str(&json).unwrap();
        assert!((s2.urgency - s.urgency).abs() < f32::EPSILON);
    }

    #[test]
    fn test_serde_level() {
        let l = SalienceLevel::Significant;
        let json = serde_json::to_string(&l).unwrap();
        let l2: SalienceLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(l2, l);
    }
}
