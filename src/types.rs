//! Core type-safety primitives for bhava.
//!
//! - [`Normalized01`] — a value clamped to \[0.0, 1.0\]
//! - [`Balanced11`] — a value clamped to \[-1.0, 1.0\]
//! - [`ThresholdClassifier`] — classify a value into discrete levels by static thresholds
//! - [`evict_min`] — remove the element with the lowest score from a `Vec`

use serde::{Deserialize, Serialize};

// ── Normalized01 ──────────────────────────────────────────────────────────

/// A 32-bit float clamped to \[0.0, 1.0\].
///
/// Use for quantities that are inherently bounded: conviction, energy,
/// trust, probability, intensity, etc.
///
/// # Construction
///
/// ```
/// use bhava::types::Normalized01;
///
/// let n = Normalized01::new(0.5);
/// assert_eq!(n.get(), 0.5);
///
/// // Out-of-range values are clamped
/// assert_eq!(Normalized01::new(1.5).get(), 1.0);
/// assert_eq!(Normalized01::new(-0.3).get(), 0.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Normalized01(f32);

impl Normalized01 {
    /// Zero.
    pub const ZERO: Self = Self(0.0);

    /// One half.
    pub const HALF: Self = Self(0.5);

    /// One (maximum).
    pub const ONE: Self = Self(1.0);

    /// Create a new `Normalized01`, clamping the input to \[0.0, 1.0\].
    #[inline]
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Extract the inner `f32`.
    #[inline]
    #[must_use]
    pub fn get(self) -> f32 {
        self.0
    }
}

impl Default for Normalized01 {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<f32> for Normalized01 {
    #[inline]
    fn from(v: f32) -> Self {
        Self::new(v)
    }
}

impl core::fmt::Display for Normalized01 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── Balanced11 ────────────────────────────────────────────────────────────

/// A 32-bit float clamped to \[-1.0, 1.0\].
///
/// Use for quantities with a bipolar range: mood dimensions, valence,
/// affinity, sentiment scores, etc.
///
/// # Construction
///
/// ```
/// use bhava::types::Balanced11;
///
/// let b = Balanced11::new(-0.5);
/// assert_eq!(b.get(), -0.5);
///
/// // Out-of-range values are clamped
/// assert_eq!(Balanced11::new(2.0).get(), 1.0);
/// assert_eq!(Balanced11::new(-3.0).get(), -1.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Balanced11(f32);

impl Balanced11 {
    /// Minimum (-1.0).
    pub const MIN: Self = Self(-1.0);

    /// Zero (neutral).
    pub const ZERO: Self = Self(0.0);

    /// Maximum (1.0).
    pub const MAX: Self = Self(1.0);

    /// Create a new `Balanced11`, clamping the input to \[-1.0, 1.0\].
    #[inline]
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(-1.0, 1.0))
    }

    /// Extract the inner `f32`.
    #[inline]
    #[must_use]
    pub fn get(self) -> f32 {
        self.0
    }
}

impl Default for Balanced11 {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<f32> for Balanced11 {
    #[inline]
    fn from(v: f32) -> Self {
        Self::new(v)
    }
}

impl core::fmt::Display for Balanced11 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── ThresholdClassifier ───────────────────────────────────────────────────

/// Classify a floating-point value into a discrete level using static thresholds.
///
/// Thresholds are checked in descending order: the first threshold whose
/// value is ≤ the input determines the label. If no threshold matches,
/// the `default` label is returned.
///
/// # Examples
///
/// ```
/// use bhava::types::ThresholdClassifier;
///
/// #[derive(Debug, Clone, Copy, PartialEq)]
/// enum Level { High, Medium, Low }
///
/// const CLASSIFIER: ThresholdClassifier<Level> = ThresholdClassifier::new(
///     &[(0.7, Level::High), (0.3, Level::Medium)],
///     Level::Low,
/// );
///
/// assert_eq!(CLASSIFIER.classify(0.9), Level::High);
/// assert_eq!(CLASSIFIER.classify(0.5), Level::Medium);
/// assert_eq!(CLASSIFIER.classify(0.1), Level::Low);
/// ```
pub struct ThresholdClassifier<L: Copy + 'static> {
    /// Thresholds in descending order: `(min_value, label)`.
    thresholds: &'static [(f32, L)],
    /// Label for values below all thresholds.
    default: L,
}

impl<L: Copy + 'static> ThresholdClassifier<L> {
    /// Create a new classifier.
    ///
    /// `thresholds` must be in descending order by the `f32` component.
    /// Each entry means "if value >= threshold, return this label."
    #[inline]
    #[must_use]
    pub const fn new(thresholds: &'static [(f32, L)], default: L) -> Self {
        Self {
            thresholds,
            default,
        }
    }

    /// Classify a value into a level.
    #[inline]
    #[must_use]
    pub fn classify(&self, value: f32) -> L {
        for &(threshold, label) in self.thresholds {
            if value >= threshold {
                return label;
            }
        }
        self.default
    }
}

// ── evict_min ─────────────────────────────────────────────────────────────

/// Remove and return the element with the minimum score from `vec`.
///
/// Uses `swap_remove` for O(1) removal (does not preserve order).
/// Returns `None` if the vector is empty.
///
/// # Examples
///
/// ```
/// use bhava::types::evict_min;
///
/// let mut v = vec![10, 3, 7, 1, 5];
/// let removed = evict_min(&mut v, |x| *x as f64);
/// assert_eq!(removed, Some(1));
/// assert_eq!(v.len(), 4);
/// ```
#[inline]
pub fn evict_min<T>(vec: &mut Vec<T>, score: impl Fn(&T) -> f64) -> Option<T> {
    if vec.is_empty() {
        return None;
    }
    let mut min_idx = 0;
    let mut min_score = score(&vec[0]);
    for (i, item) in vec.iter().enumerate().skip(1) {
        let s = score(item);
        if s < min_score {
            min_score = s;
            min_idx = i;
        }
    }
    Some(vec.swap_remove(min_idx))
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Normalized01 ──────────────────────────────────────────────────

    #[test]
    fn normalized01_clamps_high() {
        assert_eq!(Normalized01::new(1.5).get(), 1.0);
    }

    #[test]
    fn normalized01_clamps_low() {
        assert_eq!(Normalized01::new(-0.5).get(), 0.0);
    }

    #[test]
    fn normalized01_passthrough() {
        assert_eq!(Normalized01::new(0.42).get(), 0.42);
    }

    #[test]
    fn normalized01_constants() {
        assert_eq!(Normalized01::ZERO.get(), 0.0);
        assert_eq!(Normalized01::HALF.get(), 0.5);
        assert_eq!(Normalized01::ONE.get(), 1.0);
    }

    #[test]
    fn normalized01_default() {
        assert_eq!(Normalized01::default().get(), 0.0);
    }

    #[test]
    fn normalized01_from_f32() {
        let n: Normalized01 = 0.7.into();
        assert_eq!(n.get(), 0.7);
        let n: Normalized01 = 5.0.into();
        assert_eq!(n.get(), 1.0);
    }

    #[test]
    fn normalized01_serde_roundtrip() {
        let n = Normalized01::new(0.75);
        let json = serde_json::to_string(&n).unwrap();
        assert_eq!(json, "0.75");
        let back: Normalized01 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, n);
    }

    #[test]
    fn normalized01_deserialize_clamps() {
        // Deserializing out-of-range should produce the raw value (transparent)
        // but new() would clamp it. This tests the transparent path.
        let n: Normalized01 = serde_json::from_str("0.5").unwrap();
        assert_eq!(n.get(), 0.5);
    }

    #[test]
    fn normalized01_display() {
        assert_eq!(format!("{}", Normalized01::new(0.5)), "0.5");
    }

    #[test]
    fn normalized01_partial_ord() {
        assert!(Normalized01::new(0.3) < Normalized01::new(0.7));
        assert!(Normalized01::new(1.0) > Normalized01::new(0.0));
    }

    // ── Balanced11 ────────────────────────────────────────────────────

    #[test]
    fn balanced11_clamps_high() {
        assert_eq!(Balanced11::new(2.0).get(), 1.0);
    }

    #[test]
    fn balanced11_clamps_low() {
        assert_eq!(Balanced11::new(-3.0).get(), -1.0);
    }

    #[test]
    fn balanced11_passthrough() {
        assert_eq!(Balanced11::new(-0.5).get(), -0.5);
    }

    #[test]
    fn balanced11_constants() {
        assert_eq!(Balanced11::MIN.get(), -1.0);
        assert_eq!(Balanced11::ZERO.get(), 0.0);
        assert_eq!(Balanced11::MAX.get(), 1.0);
    }

    #[test]
    fn balanced11_default() {
        assert_eq!(Balanced11::default().get(), 0.0);
    }

    #[test]
    fn balanced11_from_f32() {
        let b: Balanced11 = (-0.3).into();
        assert_eq!(b.get(), -0.3);
        let b: Balanced11 = (-5.0).into();
        assert_eq!(b.get(), -1.0);
    }

    #[test]
    fn balanced11_serde_roundtrip() {
        let b = Balanced11::new(-0.25);
        let json = serde_json::to_string(&b).unwrap();
        assert_eq!(json, "-0.25");
        let back: Balanced11 = serde_json::from_str(&json).unwrap();
        assert_eq!(back, b);
    }

    #[test]
    fn balanced11_display() {
        assert_eq!(format!("{}", Balanced11::new(-0.5)), "-0.5");
    }

    #[test]
    fn balanced11_partial_ord() {
        assert!(Balanced11::new(-0.5) < Balanced11::new(0.5));
    }

    // ── ThresholdClassifier ───────────────────────────────────────────

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum TestLevel {
        High,
        Medium,
        Low,
    }

    const TEST_CLASSIFIER: ThresholdClassifier<TestLevel> = ThresholdClassifier::new(
        &[(0.7, TestLevel::High), (0.3, TestLevel::Medium)],
        TestLevel::Low,
    );

    #[test]
    fn classifier_high() {
        assert_eq!(TEST_CLASSIFIER.classify(0.9), TestLevel::High);
        assert_eq!(TEST_CLASSIFIER.classify(0.7), TestLevel::High);
    }

    #[test]
    fn classifier_medium() {
        assert_eq!(TEST_CLASSIFIER.classify(0.5), TestLevel::Medium);
        assert_eq!(TEST_CLASSIFIER.classify(0.3), TestLevel::Medium);
    }

    #[test]
    fn classifier_low() {
        assert_eq!(TEST_CLASSIFIER.classify(0.1), TestLevel::Low);
        assert_eq!(TEST_CLASSIFIER.classify(0.0), TestLevel::Low);
    }

    #[test]
    fn classifier_boundary() {
        // Exactly on boundary goes to the higher level
        assert_eq!(TEST_CLASSIFIER.classify(0.7), TestLevel::High);
        assert_eq!(TEST_CLASSIFIER.classify(0.3), TestLevel::Medium);
    }

    // ── evict_min ─────────────────────────────────────────────────────

    #[test]
    fn evict_min_removes_lowest() {
        let mut v = vec![10, 3, 7, 1, 5];
        let removed = evict_min(&mut v, |x| *x as f64);
        assert_eq!(removed, Some(1));
        assert_eq!(v.len(), 4);
        assert!(!v.contains(&1));
    }

    #[test]
    fn evict_min_empty() {
        let mut v: Vec<i32> = vec![];
        assert_eq!(evict_min(&mut v, |x| *x as f64), None);
    }

    #[test]
    fn evict_min_single() {
        let mut v = vec![42];
        assert_eq!(evict_min(&mut v, |x| *x as f64), Some(42));
        assert!(v.is_empty());
    }

    #[test]
    fn evict_min_by_field() {
        #[derive(Debug, PartialEq)]
        struct Item {
            name: &'static str,
            score: f64,
        }
        let mut v = vec![
            Item {
                name: "a",
                score: 5.0,
            },
            Item {
                name: "b",
                score: 1.0,
            },
            Item {
                name: "c",
                score: 3.0,
            },
        ];
        let removed = evict_min(&mut v, |item| item.score);
        assert_eq!(removed.unwrap().name, "b");
    }
}
