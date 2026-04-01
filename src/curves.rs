//! Decay and recovery curve abstractions.
//!
//! Provides a [`DecayCurve`] trait and common implementations for exponential
//! decay and logistic (sigmoid) curves used throughout bhava's time-dependent
//! systems (mood decay, memory activation, energy performance).

use serde::{Deserialize, Serialize};

/// A curve that computes a decay factor for a given elapsed time.
///
/// The decay factor is the fraction of the original value remaining
/// after `elapsed` time units: 1.0 = no decay, 0.0 = fully decayed.
pub trait DecayCurve {
    /// Compute the decay factor for the given elapsed time.
    ///
    /// Returns a value in \[0.0, 1.0\] representing the fraction remaining.
    #[must_use]
    fn decay_factor(&self, elapsed: f64) -> f64;
}

/// Exponential decay: `factor = 2^(-elapsed / half_life)`.
///
/// After one half-life, 50% remains. After two half-lives, 25% remains.
/// Equivalent to `e^(-λt)` where `λ = ln(2) / half_life`.
///
/// # Examples
///
/// ```
/// use bhava::curves::{DecayCurve, ExponentialDecay};
///
/// let decay = ExponentialDecay::new(10.0);
///
/// // At t=0, full value remains
/// assert!((decay.decay_factor(0.0) - 1.0).abs() < 1e-10);
///
/// // At t=half_life, half remains
/// assert!((decay.decay_factor(10.0) - 0.5).abs() < 1e-10);
///
/// // At t=2*half_life, quarter remains
/// assert!((decay.decay_factor(20.0) - 0.25).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExponentialDecay {
    /// Time for the value to halve. Must be positive.
    half_life: f64,
    /// Precomputed `ln(2) / half_life`.
    lambda: f64,
}

impl ExponentialDecay {
    /// Create a new exponential decay with the given half-life.
    ///
    /// # Panics
    ///
    /// This function does not panic. If `half_life` is ≤ 0 or non-finite,
    /// it is silently clamped to 1.0.
    #[inline]
    #[must_use]
    pub fn new(half_life: f64) -> Self {
        let hl = if half_life.is_finite() && half_life > 0.0 {
            half_life
        } else {
            1.0
        };
        Self {
            half_life: hl,
            lambda: core::f64::consts::LN_2 / hl,
        }
    }

    /// The half-life of this decay curve.
    #[inline]
    #[must_use]
    pub fn half_life(&self) -> f64 {
        self.half_life
    }

    /// The decay constant λ = ln(2) / half_life.
    #[inline]
    #[must_use]
    pub fn lambda(&self) -> f64 {
        self.lambda
    }
}

impl DecayCurve for ExponentialDecay {
    #[inline]
    fn decay_factor(&self, elapsed: f64) -> f64 {
        if elapsed <= 0.0 {
            return 1.0;
        }
        (-self.lambda * elapsed).exp()
    }
}

/// Standard logistic (sigmoid) curve: `f(x) = 1 / (1 + e^(-steepness * (x - midpoint)))`.
///
/// Useful for performance curves (energy → cognitive performance),
/// threshold transitions, and smooth step functions.
///
/// # Examples
///
/// ```
/// use bhava::curves::LogisticCurve;
///
/// let curve = LogisticCurve::new(0.0, 4.0);
///
/// // At midpoint, output is 0.5
/// assert!((curve.evaluate(0.0) - 0.5).abs() < 1e-10);
///
/// // Far positive → approaches 1.0
/// assert!(curve.evaluate(5.0) > 0.99);
///
/// // Far negative → approaches 0.0
/// assert!(curve.evaluate(-5.0) < 0.01);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LogisticCurve {
    /// The x-value where the output is 0.5.
    pub midpoint: f64,
    /// Controls the steepness of the transition. Higher = sharper.
    pub steepness: f64,
}

impl LogisticCurve {
    /// Create a new logistic curve.
    #[inline]
    #[must_use]
    pub const fn new(midpoint: f64, steepness: f64) -> Self {
        Self {
            midpoint,
            steepness,
        }
    }

    /// Evaluate the logistic function at `x`.
    ///
    /// Returns a value in (0.0, 1.0).
    #[inline]
    #[must_use]
    pub fn evaluate(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-self.steepness * (x - self.midpoint)).exp())
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── ExponentialDecay ──────────────────────────────────────────────

    #[test]
    fn exponential_no_elapsed() {
        let d = ExponentialDecay::new(10.0);
        assert!((d.decay_factor(0.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn exponential_half_life() {
        let d = ExponentialDecay::new(10.0);
        assert!((d.decay_factor(10.0) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn exponential_two_half_lives() {
        let d = ExponentialDecay::new(10.0);
        assert!((d.decay_factor(20.0) - 0.25).abs() < 1e-10);
    }

    #[test]
    fn exponential_negative_elapsed() {
        let d = ExponentialDecay::new(10.0);
        assert!((d.decay_factor(-5.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn exponential_invalid_half_life_clamped() {
        let d = ExponentialDecay::new(0.0);
        assert_eq!(d.half_life(), 1.0);
        let d = ExponentialDecay::new(-5.0);
        assert_eq!(d.half_life(), 1.0);
        let d = ExponentialDecay::new(f64::NAN);
        assert_eq!(d.half_life(), 1.0);
    }

    #[test]
    fn exponential_lambda() {
        let d = ExponentialDecay::new(10.0);
        assert!((d.lambda() - core::f64::consts::LN_2 / 10.0).abs() < 1e-15);
    }

    #[test]
    fn exponential_serde_roundtrip() {
        let d = ExponentialDecay::new(300.0);
        let json = serde_json::to_string(&d).unwrap();
        let back: ExponentialDecay = serde_json::from_str(&json).unwrap();
        assert_eq!(back.half_life(), 300.0);
    }

    // ── LogisticCurve ────────────────────────────────────────────────

    #[test]
    fn logistic_midpoint() {
        let c = LogisticCurve::new(0.0, 4.0);
        assert!((c.evaluate(0.0) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn logistic_far_positive() {
        let c = LogisticCurve::new(0.0, 4.0);
        assert!(c.evaluate(5.0) > 0.99);
    }

    #[test]
    fn logistic_far_negative() {
        let c = LogisticCurve::new(0.0, 4.0);
        assert!(c.evaluate(-5.0) < 0.01);
    }

    #[test]
    fn logistic_shifted_midpoint() {
        let c = LogisticCurve::new(5.0, 2.0);
        assert!((c.evaluate(5.0) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn logistic_steepness_effect() {
        let shallow = LogisticCurve::new(0.0, 1.0);
        let steep = LogisticCurve::new(0.0, 10.0);
        // At x=1, steeper curve should be closer to 1.0
        assert!(steep.evaluate(1.0) > shallow.evaluate(1.0));
    }

    #[test]
    fn logistic_serde_roundtrip() {
        let c = LogisticCurve::new(3.0, 2.5);
        let json = serde_json::to_string(&c).unwrap();
        let back: LogisticCurve = serde_json::from_str(&json).unwrap();
        assert_eq!(back, c);
    }
}
