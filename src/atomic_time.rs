//! Tanmatra atomic time integration — physical time grounding for bhava.
//!
//! Provides bridge functions between tanmatra's atomic time system and
//! bhava's time-dependent modules (circadian, rhythm, growth, actr, aesthetic).
//! The key distinction: wall-clock time vs simulation time. A game running
//! at 10× speed should produce 10× circadian phase advance, not 10× tick
//! calls at wrong dt.
//!
//! Requires the `atomic_time` feature.
//!
//! # Layer Model
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Bhava (Personality Engine)      │
//! │  circadian, rhythm, growth, actr │
//! ├──────────────────────────────────┤
//! │  This module (bridge)            │
//! │  TimeContext → module parameters │
//! ├──────────────────────────────────┤
//! │  Tanmatra (Atomic Physics)      │
//! │  SimulationClock, TimeContext    │
//! └──────────────────────────────────┘
//! ```
//!
//! # Bridge Functions
//!
//! ## Time Extraction
//! - [`simulation_hours`] — elapsed simulation time in hours (circadian phase)
//! - [`simulation_seconds`] — elapsed simulation seconds (general)
//! - [`time_multiplier`] — current speed multiplier (growth rate scaling)
//! - [`is_paused`] — whether time is frozen (freeze all decay/growth)
//!
//! ## Growth / Decay Scaling
//! - [`growth_rate_scale`] — multiplier for trait pressure accumulation
//! - [`decay_rate_scale`] — multiplier for memory/mood/belief decay
//!
//! ## Circadian
//! - [`circadian_hour_of_day`] — simulation-time hour of day (0.0–24.0)
//!
//! # Example
//!
//! ```
//! use bhava::atomic_time::{simulation_hours, circadian_hour_of_day, is_paused};
//! use tanmatra::bridge::TimeContext;
//!
//! // Real-time: 3600 TAI seconds = 1 hour
//! let ctx = TimeContext::real_time(3600.0);
//! assert!((simulation_hours(&ctx) - 1.0).abs() < 0.001);
//! assert!(!is_paused(&ctx));
//!
//! // Circadian hour wraps to 0–24
//! let hour = circadian_hour_of_day(&ctx);
//! assert!(hour >= 0.0 && hour < 24.0);
//! ```

use tanmatra::bridge::TimeContext;

/// Elapsed simulation time in seconds.
///
/// For `RealTime`, equals TAI seconds. For `Simulated`, equals the
/// simulation-adjusted elapsed time. For `Paused`, returns the frozen value.
#[must_use]
#[inline]
pub fn simulation_seconds(ctx: &TimeContext) -> f64 {
    ctx.effective_elapsed_s()
}

/// Elapsed simulation time in hours.
///
/// Convenience for circadian/rhythm modules that think in hours.
#[must_use]
#[inline]
pub fn simulation_hours(ctx: &TimeContext) -> f64 {
    ctx.effective_elapsed_s() / 3600.0
}

/// Current simulation speed multiplier.
///
/// Returns 1.0 for real-time, 0.0 for paused, >1.0 for fast-forward.
/// Use this to scale per-tick growth/decay rates so that a 10× simulation
/// produces 10× the trait pressure per wall-clock tick.
#[must_use]
#[inline]
pub fn time_multiplier(ctx: &TimeContext) -> f64 {
    ctx.multiplier()
}

/// Whether time is frozen.
///
/// When paused, circadian phase should not advance, growth should not
/// accumulate, decay should not progress, and no time-dependent state
/// should change.
#[must_use]
#[inline]
pub fn is_paused(ctx: &TimeContext) -> bool {
    ctx.is_paused()
}

/// Growth rate scale factor.
///
/// Multiplier for trait pressure, belief reinforcement, preference drift,
/// and any process that accumulates over time. Returns 0.0 when paused
/// (no growth), multiplier value otherwise.
///
/// # Example
///
/// ```
/// use bhava::atomic_time::growth_rate_scale;
/// use tanmatra::bridge::{TimeContext, SimulationClock};
///
/// // 5× speed simulation
/// let clock = SimulationClock::new(0.0).set_multiplier(0.0, 5.0);
/// let ctx = TimeContext::from_simulation_clock(&clock, 10.0);
/// assert!((growth_rate_scale(&ctx) - 5.0).abs() < 0.001);
///
/// // Paused — no growth
/// let paused = SimulationClock::new(0.0).pause(0.0);
/// let ctx = TimeContext::from_simulation_clock(&paused, 10.0);
/// assert!((growth_rate_scale(&ctx) - 0.0).abs() < 0.001);
/// ```
#[must_use]
#[inline]
pub fn growth_rate_scale(ctx: &TimeContext) -> f64 {
    ctx.multiplier()
}

/// Decay rate scale factor.
///
/// Multiplier for mood decay, memory activation decay, belief erosion,
/// and aesthetic preference fading. Same as [`growth_rate_scale`] — both
/// growth and decay scale linearly with simulation speed.
#[must_use]
#[inline]
pub fn decay_rate_scale(ctx: &TimeContext) -> f64 {
    ctx.multiplier()
}

/// Simulation-time hour of day (0.0–24.0).
///
/// Wraps elapsed simulation seconds into a 24-hour cycle for circadian
/// phase computation. The "day" starts at simulation second 0.
///
/// For `RealTime`, this wraps TAI seconds into 24h. For `Simulated`,
/// the accelerated/decelerated time determines the phase.
///
/// # Example
///
/// ```
/// use bhava::atomic_time::circadian_hour_of_day;
/// use tanmatra::bridge::TimeContext;
///
/// // 13 hours into simulation
/// let ctx = TimeContext::real_time(13.0 * 3600.0);
/// let hour = circadian_hour_of_day(&ctx);
/// assert!((hour - 13.0).abs() < 0.01);
///
/// // Wraps past 24h
/// let ctx = TimeContext::real_time(25.0 * 3600.0);
/// let hour = circadian_hour_of_day(&ctx);
/// assert!((hour - 1.0).abs() < 0.01);
/// ```
#[must_use]
#[inline]
pub fn circadian_hour_of_day(ctx: &TimeContext) -> f64 {
    let elapsed_s = ctx.effective_elapsed_s();
    let hours = elapsed_s / 3600.0;
    hours.rem_euclid(24.0)
}

/// Simulation-time day fraction (0.0–1.0).
///
/// Normalized circadian phase for modules that work in fractions rather
/// than hours. 0.0 = midnight, 0.5 = noon.
#[must_use]
#[inline]
pub fn circadian_day_fraction(ctx: &TimeContext) -> f64 {
    circadian_hour_of_day(ctx) / 24.0
}

/// Elapsed simulation days.
///
/// For seasonal rhythm cycles that operate on day-scale periods.
#[must_use]
#[inline]
pub fn simulation_days(ctx: &TimeContext) -> f64 {
    ctx.effective_elapsed_s() / 86400.0
}

/// Elapsed delta-time in simulation seconds between two contexts.
///
/// Returns 0.0 if either context is paused. Useful for modules that
/// need the time step between two consecutive ticks in simulation time.
///
/// # Example
///
/// ```
/// use bhava::atomic_time::delta_seconds;
/// use tanmatra::bridge::TimeContext;
///
/// let prev = TimeContext::real_time(1000.0);
/// let now = TimeContext::real_time(1001.0);
/// assert!((delta_seconds(&prev, &now) - 1.0).abs() < 0.001);
/// ```
#[must_use]
#[inline]
pub fn delta_seconds(prev: &TimeContext, now: &TimeContext) -> f64 {
    if prev.is_paused() || now.is_paused() {
        return 0.0;
    }
    let dt = now.effective_elapsed_s() - prev.effective_elapsed_s();
    dt.max(0.0)
}

/// Elapsed delta-time in simulation hours between two contexts.
#[must_use]
#[inline]
pub fn delta_hours(prev: &TimeContext, now: &TimeContext) -> f64 {
    delta_seconds(prev, now) / 3600.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use tanmatra::bridge::SimulationClock;

    #[test]
    fn real_time_basics() {
        let ctx = TimeContext::real_time(7200.0); // 2 hours
        assert!((simulation_seconds(&ctx) - 7200.0).abs() < 0.001);
        assert!((simulation_hours(&ctx) - 2.0).abs() < 0.001);
        assert!((time_multiplier(&ctx) - 1.0).abs() < 0.001);
        assert!(!is_paused(&ctx));
    }

    #[test]
    fn simulated_time_scaling() {
        let clock = SimulationClock::new(0.0).set_multiplier(0.0, 10.0);
        let ctx = TimeContext::from_simulation_clock(&clock, 100.0);
        // 100 real seconds at 10× = 1000 sim seconds
        assert!((simulation_seconds(&ctx) - 1000.0).abs() < 0.001);
        assert!((time_multiplier(&ctx) - 10.0).abs() < 0.001);
    }

    #[test]
    fn paused_freezes() {
        let clock = SimulationClock::new(0.0).pause(50.0);
        let ctx = TimeContext::from_simulation_clock(&clock, 200.0);
        assert!(is_paused(&ctx));
        assert!((growth_rate_scale(&ctx)).abs() < 0.001);
        assert!((decay_rate_scale(&ctx)).abs() < 0.001);
    }

    #[test]
    fn circadian_wraps_24h() {
        let ctx = TimeContext::real_time(25.0 * 3600.0);
        let hour = circadian_hour_of_day(&ctx);
        assert!((hour - 1.0).abs() < 0.01);
    }

    #[test]
    fn circadian_noon() {
        let ctx = TimeContext::real_time(12.0 * 3600.0);
        assert!((circadian_hour_of_day(&ctx) - 12.0).abs() < 0.01);
        assert!((circadian_day_fraction(&ctx) - 0.5).abs() < 0.01);
    }

    #[test]
    fn circadian_accelerated() {
        // 10× speed, 1 real hour = 10 sim hours
        let clock = SimulationClock::new(0.0).set_multiplier(0.0, 10.0);
        let ctx = TimeContext::from_simulation_clock(&clock, 3600.0);
        assert!((circadian_hour_of_day(&ctx) - 10.0).abs() < 0.01);
    }

    #[test]
    fn simulation_days_count() {
        let ctx = TimeContext::real_time(3.5 * 86400.0);
        assert!((simulation_days(&ctx) - 3.5).abs() < 0.001);
    }

    #[test]
    fn delta_seconds_basic() {
        let prev = TimeContext::real_time(1000.0);
        let now = TimeContext::real_time(1005.0);
        assert!((delta_seconds(&prev, &now) - 5.0).abs() < 0.001);
    }

    #[test]
    fn delta_seconds_paused_is_zero() {
        let prev = TimeContext::real_time(1000.0);
        let clock = SimulationClock::new(0.0).pause(0.0);
        let now = TimeContext::from_simulation_clock(&clock, 2000.0);
        assert!((delta_seconds(&prev, &now)).abs() < 0.001);
    }

    #[test]
    fn delta_seconds_accelerated() {
        // 5× speed: 10 real seconds = 50 sim seconds
        let clock = SimulationClock::new(0.0).set_multiplier(0.0, 5.0);
        let prev = TimeContext::from_simulation_clock(&clock, 100.0);
        let now = TimeContext::from_simulation_clock(&clock, 110.0);
        assert!((delta_seconds(&prev, &now) - 50.0).abs() < 0.001);
    }

    #[test]
    fn delta_hours_basic() {
        let prev = TimeContext::real_time(0.0);
        let now = TimeContext::real_time(7200.0);
        assert!((delta_hours(&prev, &now) - 2.0).abs() < 0.001);
    }

    #[test]
    fn growth_and_decay_match_multiplier() {
        let clock = SimulationClock::new(0.0).set_multiplier(0.0, 3.0);
        let ctx = TimeContext::from_simulation_clock(&clock, 10.0);
        assert!((growth_rate_scale(&ctx) - 3.0).abs() < 0.001);
        assert!((decay_rate_scale(&ctx) - 3.0).abs() < 0.001);
    }

    #[test]
    fn serde_roundtrip_time_context() {
        let ctx = TimeContext::real_time(12345.0);
        let json = serde_json::to_string(&ctx).unwrap();
        let deser: TimeContext = serde_json::from_str(&json).unwrap();
        assert!((ctx.tai_seconds() - deser.tai_seconds()).abs() < f64::EPSILON);
    }

    #[test]
    fn serde_roundtrip_simulation_clock() {
        let clock = SimulationClock::new(100.0).set_multiplier(100.0, 5.0);
        let json = serde_json::to_string(&clock).unwrap();
        let deser: SimulationClock = serde_json::from_str(&json).unwrap();
        assert!((clock.simulation_time(200.0) - deser.simulation_time(200.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn negative_elapsed_clamped() {
        // If contexts are out of order, delta should be 0
        let prev = TimeContext::real_time(2000.0);
        let now = TimeContext::real_time(1000.0);
        assert!((delta_seconds(&prev, &now)).abs() < 0.001);
    }
}
