//! Affective computing metrics — quantitative measures of emotional behavior.
//!
//! Computes metrics from mood history that characterize an entity's emotional
//! profile over time. Based on affective computing literature:
//!
//! - **Complexity** — Number of simultaneously active emotions (Barrett 2004)
//! - **Granularity** — Differentiation between emotions via Shannon entropy
//!   (Tugade, Fredrickson & Barrett 2004)
//! - **Inertia** — Resistance to mood change via lag-1 autocorrelation
//!   (Kuppens et al. 2010)
//! - **Variability** — Overall mood change magnitude over time (Eid & Diener 1999)

use serde::{Deserialize, Serialize};

use crate::mood::{Emotion, MoodHistory, MoodVector};

/// Affective computing metrics computed from mood history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectiveMetrics {
    /// Emotional complexity: average number of active emotions per snapshot.
    /// Range: 0.0 to `Emotion::ALL.len()` (currently 6).
    pub complexity: f32,
    /// Emotional granularity: Shannon entropy of emotion activations.
    /// Higher = more differentiated emotional experience.
    /// Range: 0.0 (flat/undifferentiated) to ~1.79 (maximally differentiated).
    pub granularity: f32,
    /// Emotional inertia: lag-1 autocorrelation of mood intensity.
    /// Higher = mood carries over more from one snapshot to the next.
    /// Range: -1.0 (anti-correlated) to 1.0 (perfectly persistent).
    pub inertia: f32,
    /// Emotional variability: average magnitude of mood change between snapshots.
    /// Higher = more emotionally volatile.
    /// Range: 0.0 (perfectly stable) to unbounded (rapid swings).
    pub variability: f32,
}

impl AffectiveMetrics {
    /// Zero metrics (no data).
    #[must_use]
    pub fn zero() -> Self {
        Self {
            complexity: 0.0,
            granularity: 0.0,
            inertia: 0.0,
            variability: 0.0,
        }
    }
}

/// Threshold for considering an emotion "active".
const ACTIVE_THRESHOLD: f32 = 0.1;

/// Count how many emotions exceed the activation threshold.
#[must_use]
#[inline]
fn emotional_complexity(mood: &MoodVector) -> f32 {
    Emotion::ALL
        .iter()
        .filter(|&&e| mood.get(e).abs() > ACTIVE_THRESHOLD)
        .count() as f32
}

/// Shannon entropy of emotion activations (normalized absolute values).
#[must_use]
#[inline]
fn emotional_granularity(mood: &MoodVector) -> f32 {
    let mut values = [0.0f32; 6];
    for (i, &e) in Emotion::ALL.iter().enumerate() {
        values[i] = mood.get(e).abs();
    }
    let total: f32 = values.iter().sum();
    if total < f32::EPSILON {
        return 0.0;
    }
    let mut entropy = 0.0f32;
    for &v in &values {
        let p = v / total;
        if p > f32::EPSILON {
            entropy -= p * p.ln();
        }
    }
    entropy
}

/// Euclidean distance between two mood vectors.
#[must_use]
fn mood_distance(a: &MoodVector, b: &MoodVector) -> f32 {
    let mut sum = 0.0f32;
    for &e in Emotion::ALL {
        let diff = a.get(e) - b.get(e);
        sum += diff * diff;
    }
    sum.sqrt()
}

/// Compute affective metrics from mood history.
///
/// Requires at least 2 snapshots for inertia and variability.
/// Returns `AffectiveMetrics::zero()` for empty histories.
#[must_use]
pub fn compute_affective_metrics(history: &MoodHistory) -> AffectiveMetrics {
    if history.is_empty() {
        return AffectiveMetrics::zero();
    }

    let snapshots: Vec<_> = history.iter().collect();
    let n = snapshots.len();

    // Complexity: average active emotion count
    let complexity = snapshots
        .iter()
        .map(|s| emotional_complexity(&s.mood))
        .sum::<f32>()
        / n as f32;

    // Granularity: average Shannon entropy
    let granularity = snapshots
        .iter()
        .map(|s| emotional_granularity(&s.mood))
        .sum::<f32>()
        / n as f32;

    if n < 2 {
        return AffectiveMetrics {
            complexity,
            granularity,
            inertia: 0.0,
            variability: 0.0,
        };
    }

    // Variability: average mood-to-mood distance
    let variability = snapshots
        .windows(2)
        .map(|w| mood_distance(&w[0].mood, &w[1].mood))
        .sum::<f32>()
        / (n - 1) as f32;

    // Inertia: lag-1 autocorrelation of mood intensity
    let intensities: Vec<f32> = snapshots.iter().map(|s| s.mood.intensity()).collect();
    let inertia = lag1_autocorrelation(&intensities);

    AffectiveMetrics {
        complexity,
        granularity,
        inertia,
        variability,
    }
}

/// Compute a single-snapshot complexity score.
///
/// Useful for real-time display without needing full history.
#[must_use]
#[inline]
pub fn snapshot_complexity(mood: &MoodVector) -> f32 {
    emotional_complexity(mood)
}

/// Compute a single-snapshot granularity score.
///
/// Useful for real-time display without needing full history.
#[must_use]
#[inline]
pub fn snapshot_granularity(mood: &MoodVector) -> f32 {
    emotional_granularity(mood)
}

/// Lag-1 autocorrelation of a time series.
fn lag1_autocorrelation(values: &[f32]) -> f32 {
    let n = values.len();
    if n < 2 {
        return 0.0;
    }
    let mean = values.iter().sum::<f32>() / n as f32;
    let mut numerator = 0.0f32;
    let mut denominator = 0.0f32;
    for i in 0..n - 1 {
        numerator += (values[i] - mean) * (values[i + 1] - mean);
    }
    for v in values {
        denominator += (v - mean) * (v - mean);
    }
    if denominator.abs() < f32::EPSILON {
        return 0.0;
    }
    (numerator / denominator).clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mood::{MoodHistory, MoodSnapshot, MoodState};
    use chrono::Utc;

    fn make_snapshot(joy: f32, arousal: f32) -> MoodSnapshot {
        let mut mood = MoodVector::neutral();
        mood.set(Emotion::Joy, joy);
        mood.set(Emotion::Arousal, arousal);
        MoodSnapshot {
            mood,
            state: MoodState::Calm,
            deviation: 0.0,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_empty_history() {
        let h = MoodHistory::new(10);
        let m = compute_affective_metrics(&h);
        assert!(m.complexity.abs() < f32::EPSILON);
        assert!(m.granularity.abs() < f32::EPSILON);
        assert!(m.inertia.abs() < f32::EPSILON);
        assert!(m.variability.abs() < f32::EPSILON);
    }

    #[test]
    fn test_single_snapshot() {
        let mut h = MoodHistory::new(10);
        h.record(make_snapshot(0.5, 0.3));
        let m = compute_affective_metrics(&h);
        assert!(m.complexity > 0.0);
        assert!(m.granularity > 0.0);
        assert!(m.inertia.abs() < f32::EPSILON); // need 2+ for inertia
        assert!(m.variability.abs() < f32::EPSILON); // need 2+ for variability
    }

    #[test]
    fn test_stable_mood_low_variability() {
        let mut h = MoodHistory::new(10);
        for _ in 0..5 {
            h.record(make_snapshot(0.5, 0.3));
        }
        let m = compute_affective_metrics(&h);
        assert!(
            m.variability < 0.01,
            "stable mood should have low variability: {}",
            m.variability
        );
    }

    #[test]
    fn test_volatile_mood_high_variability() {
        let mut h = MoodHistory::new(10);
        for i in 0..6 {
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            h.record(make_snapshot(0.8 * sign, 0.5 * sign));
        }
        let m = compute_affective_metrics(&h);
        assert!(
            m.variability > 0.5,
            "volatile mood should have high variability: {}",
            m.variability
        );
    }

    #[test]
    fn test_high_inertia_trending() {
        let mut h = MoodHistory::new(20);
        for i in 0..10 {
            let v = i as f32 * 0.1;
            h.record(make_snapshot(v, v * 0.5));
        }
        let m = compute_affective_metrics(&h);
        assert!(
            m.inertia > 0.5,
            "steadily increasing mood should have high inertia: {}",
            m.inertia
        );
    }

    #[test]
    fn test_complexity_neutral() {
        let mood = MoodVector::neutral();
        assert!(snapshot_complexity(&mood) < f32::EPSILON);
    }

    #[test]
    fn test_complexity_multi_emotion() {
        let mut mood = MoodVector::neutral();
        mood.set(Emotion::Joy, 0.5);
        mood.set(Emotion::Arousal, 0.3);
        mood.set(Emotion::Trust, 0.2);
        assert!((snapshot_complexity(&mood) - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_granularity_single_emotion() {
        let mut mood = MoodVector::neutral();
        mood.set(Emotion::Joy, 0.8);
        let g = snapshot_granularity(&mood);
        // Single active emotion = low granularity (but not zero due to normalization)
        assert!(g < 0.5, "single emotion granularity should be low: {g}");
    }

    #[test]
    fn test_granularity_multi_emotion() {
        let mut mood = MoodVector::neutral();
        for &e in Emotion::ALL {
            mood.set(e, 0.5);
        }
        let g = snapshot_granularity(&mood);
        // All equal = maximum entropy
        assert!(
            g > 1.0,
            "uniform emotions should have high granularity: {g}"
        );
    }

    #[test]
    fn test_zero_metrics() {
        let m = AffectiveMetrics::zero();
        assert!(m.complexity.abs() < f32::EPSILON);
    }

    #[test]
    fn test_serde() {
        let m = AffectiveMetrics {
            complexity: 3.0,
            granularity: 1.5,
            inertia: 0.7,
            variability: 0.3,
        };
        let json = serde_json::to_string(&m).unwrap();
        let m2: AffectiveMetrics = serde_json::from_str(&json).unwrap();
        assert!((m2.complexity - m.complexity).abs() < f32::EPSILON);
    }

    #[test]
    fn test_lag1_autocorrelation_constant() {
        // Constant series = 0 variance = 0 autocorrelation
        assert!(lag1_autocorrelation(&[1.0, 1.0, 1.0, 1.0]).abs() < f32::EPSILON);
    }

    #[test]
    fn test_lag1_autocorrelation_alternating() {
        // Alternating series should have negative autocorrelation
        let r = lag1_autocorrelation(&[1.0, -1.0, 1.0, -1.0, 1.0]);
        assert!(r < -0.5, "alternating should be negatively correlated: {r}");
    }
}
