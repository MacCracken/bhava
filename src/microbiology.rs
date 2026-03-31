//! Jivanu microbiology integration — sickness behavior and immune state pressing on emotion.
//!
//! Provides bridge functions between jivanu's microbial/immune system models and
//! bhava's emotion/personality systems. Infection triggers sickness behavior:
//! fatigue, social withdrawal, anhedonia, irritability. Immune recovery restores
//! baseline mood. Drug effects modulate cognition and energy.
//!
//! Requires the `microbiology` feature.
//!
//! # Layer Model
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Bhava (Personality Engine)      │
//! │  Mood, stress, energy, cognition │
//! ├──────────────────────────────────┤
//! │  This module (bridge)            │
//! │  Immune state → Emotion effects  │
//! ├──────────────────────────────────┤
//! │  Jivanu (Microbiology Engine)    │
//! │  SIR/SEIR, metabolism, pharma    │
//! └──────────────────────────────────┘
//! ```
//!
//! # Bridge Functions
//!
//! ## Infection → Sickness Behavior
//! - [`sickness_mood`] — infected fraction → cytokine-driven mood depression
//! - [`sickness_severity`] — SEIR state → normalized severity (0–1)
//!
//! ## Immune State → Recovery
//! - [`recovery_mood_boost`] — recovered fraction → mood restoration
//! - [`immune_energy_drain`] — infected fraction → energy cost of immune response
//!
//! ## Epidemiology → Social Behavior
//! - [`contagion_avoidance`] — R0 → social withdrawal pressure
//! - [`herd_safety`] — vaccination coverage → trust/safety feeling
//!
//! ## Metabolism → Energy
//! - [`metabolic_efficiency`] — growth rate modifier → energy availability
//! - [`temperature_stress`] — cardinal temperature model → thermal discomfort
//!
//! ## Pharmacology → Cognition
//! - [`drug_cognitive_effect`] — plasma concentration vs EC50 → cognitive modifier
//! - [`drug_sedation`] — concentration above therapeutic → drowsiness

use crate::mood::MoodVector;

// ── Infection → Sickness Behavior ──────────────────────────────────────

/// Convert infected fraction to sickness behavior mood shift.
///
/// Sickness behavior is a conserved adaptive response: cytokines trigger
/// fatigue, anhedonia (reduced joy), social withdrawal (reduced trust),
/// and irritability (increased frustration). Scales with infection severity.
///
/// ```
/// use bhava::microbiology::sickness_mood;
///
/// let healthy = sickness_mood(0.0);
/// assert!(healthy.frustration < 0.01);
///
/// let sick = sickness_mood(0.5);
/// assert!(sick.frustration > 0.2);
/// assert!(sick.joy < 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn sickness_mood(infected_fraction: f32) -> MoodVector {
    let i = infected_fraction.clamp(0.0, 1.0);
    MoodVector {
        joy: (-i * 0.6).clamp(-1.0, 0.0),
        arousal: (-i * 0.4).clamp(-1.0, 0.0),
        dominance: (-i * 0.3).clamp(-1.0, 0.0),
        trust: (-i * 0.5).clamp(-1.0, 0.0),
        interest: (-i * 0.5).clamp(-1.0, 0.0),
        frustration: (i * 0.5).clamp(0.0, 1.0),
    }
}

/// Compute normalized sickness severity from SEIR state.
///
/// Combines exposed + infected fractions, weighted by disease progression.
/// Returns 0.0 (healthy) to 1.0 (critically ill).
///
/// ```
/// use bhava::microbiology::sickness_severity;
///
/// assert!((sickness_severity(0.0, 0.0) - 0.0).abs() < 0.01);
/// assert!(sickness_severity(0.1, 0.3) > 0.2);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn sickness_severity(exposed_fraction: f32, infected_fraction: f32) -> f32 {
    let e = exposed_fraction.clamp(0.0, 1.0);
    let i = infected_fraction.clamp(0.0, 1.0);
    // Infected counts more than exposed (symptomatic vs incubating)
    (e * 0.3 + i * 0.7).clamp(0.0, 1.0)
}

// ── Immune State → Recovery ────────────────────────────────────────────

/// Convert recovered fraction to a mood restoration boost.
///
/// As the immune system wins, energy and mood recover. Higher recovered
/// fraction → more positive mood shift. Represents the relief and
/// vitality of overcoming illness.
///
/// ```
/// use bhava::microbiology::recovery_mood_boost;
///
/// let early = recovery_mood_boost(0.1);
/// let recovered = recovery_mood_boost(0.8);
/// assert!(recovered.joy > early.joy);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn recovery_mood_boost(recovered_fraction: f32) -> MoodVector {
    let r = recovered_fraction.clamp(0.0, 1.0);
    MoodVector {
        joy: (r * 0.4).clamp(0.0, 1.0),
        arousal: (r * 0.2).clamp(0.0, 1.0),
        dominance: (r * 0.2).clamp(0.0, 1.0),
        trust: (r * 0.3).clamp(0.0, 1.0),
        interest: (r * 0.3).clamp(0.0, 1.0),
        frustration: 0.0,
    }
}

/// Compute energy drain from immune response.
///
/// Fighting infection costs energy — fever, immune cell production, cytokine
/// storms. Returns a drain multiplier (1.0 = normal, up to 3.0 = severe).
///
/// ```
/// use bhava::microbiology::immune_energy_drain;
///
/// assert!((immune_energy_drain(0.0) - 1.0).abs() < 0.01);
/// assert!(immune_energy_drain(0.5) > 1.5);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn immune_energy_drain(infected_fraction: f32) -> f32 {
    let i = infected_fraction.clamp(0.0, 1.0);
    // Linear: 1.0 at healthy, up to 3.0 at fully infected
    1.0 + i * 2.0
}

// ── Epidemiology → Social Behavior ─────────────────────────────────────

/// Compute social withdrawal pressure from disease transmissibility.
///
/// Higher R0 → stronger instinct to avoid social contact. Uses jivanu's
/// R0 calculation. Returns withdrawal pressure [0.0, 1.0].
/// Falls back to 0.0 on error.
///
/// ```
/// use bhava::microbiology::contagion_avoidance;
///
/// let mild = contagion_avoidance(1.5, 0.2);
/// let severe = contagion_avoidance(5.0, 0.2);
/// assert!(severe > mild);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn contagion_avoidance(beta: f64, gamma: f64) -> f32 {
    let r0 = jivanu::epidemiology::r_naught(beta, gamma).unwrap_or(1.0);
    // Sigmoid on R0: R0=1 → 0.0, R0=3 → ~0.5, R0=10 → ~0.9
    let pressure = 1.0 - (-0.3 * (r0 - 1.0)).exp();
    (pressure as f32).clamp(0.0, 1.0)
}

/// Convert vaccination/herd immunity coverage to a safety feeling.
///
/// Higher coverage → more trust, less anxiety. Uses jivanu's herd immunity
/// threshold. Returns trust modifier [-0.3, 0.3].
/// Falls back to 0.0 on error.
///
/// ```
/// use bhava::microbiology::herd_safety;
///
/// let low_coverage = herd_safety(0.2, 3.0);
/// let high_coverage = herd_safety(0.8, 3.0);
/// assert!(high_coverage > low_coverage);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn herd_safety(vaccination_coverage: f64, r0: f64) -> f32 {
    let threshold = jivanu::epidemiology::herd_immunity_threshold(r0).unwrap_or(1.0);
    if threshold <= 0.0 {
        return 0.3;
    }
    // How close to herd immunity: 0 = none, 1 = at threshold, >1 = above
    let ratio = (vaccination_coverage / threshold).clamp(0.0, 2.0);
    ((ratio - 0.5) * 0.4) as f32
}

// ── Metabolism → Energy ────────────────────────────────────────────────

/// Convert microbial growth rate to energy availability modifier.
///
/// When gut microbiome is healthy (high growth rate under good conditions),
/// metabolic efficiency is high. Under stress (low rate), energy availability drops.
/// Uses jivanu's cardinal temperature model as an example input.
/// Returns energy modifier [0.5, 1.0].
///
/// ```
/// use bhava::microbiology::metabolic_efficiency;
///
/// let optimal = metabolic_efficiency(1.0);
/// let stressed = metabolic_efficiency(0.2);
/// assert!(optimal > stressed);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn metabolic_efficiency(growth_rate_fraction: f32) -> f32 {
    // Growth fraction of max (0-1) maps to energy availability
    let g = growth_rate_fraction.clamp(0.0, 1.0);
    0.5 + g * 0.5
}

/// Convert temperature deviation from optimal to thermal discomfort.
///
/// Uses jivanu's cardinal temperature model to compute growth rate fraction,
/// then maps to discomfort. Returns stress input [0.0, 1.0].
/// Falls back to 0.5 (moderate discomfort) on error.
///
/// ```
/// use bhava::microbiology::temperature_stress;
///
/// // At optimal temperature, minimal stress
/// let optimal = temperature_stress(37.0, 15.0, 37.0, 45.0);
/// assert!(optimal < 0.1);
///
/// // At extreme temperature, high stress
/// let cold = temperature_stress(20.0, 15.0, 37.0, 45.0);
/// assert!(cold > optimal);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn temperature_stress(temp_c: f64, t_min: f64, t_opt: f64, t_max: f64) -> f32 {
    let growth_fraction =
        jivanu::growth::cardinal_temperature(temp_c, t_min, t_opt, t_max).unwrap_or(0.0);
    // Invert: high growth = low stress, low growth = high stress
    (1.0 - growth_fraction.clamp(0.0, 1.0)) as f32
}

// ── Pharmacology → Cognition ───────────────────────────────────────────

/// Compute cognitive effect of a drug using the Emax model.
///
/// At low concentration, minimal effect. At EC50, half-maximal effect.
/// Returns a cognitive modifier [0.0, 1.0] where 1.0 = maximum drug effect.
/// Falls back to 0.0 on error.
///
/// ```
/// use bhava::microbiology::drug_cognitive_effect;
///
/// let sub_therapeutic = drug_cognitive_effect(0.5, 1.0, 5.0, 1.0);
/// let therapeutic = drug_cognitive_effect(5.0, 1.0, 5.0, 1.0);
/// assert!(therapeutic > sub_therapeutic);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn drug_cognitive_effect(concentration: f64, e_max: f64, ec50: f64, hill_n: f64) -> f32 {
    let effect = jivanu::metabolism::emax_model(concentration, e_max, ec50, hill_n).unwrap_or(0.0);
    (effect as f32).clamp(0.0, 1.0)
}

/// Compute sedation level from drug concentration above therapeutic range.
///
/// Concentration below EC50 → minimal sedation. Above 2×EC50 → significant
/// drowsiness. Returns drowsiness [0.0, 1.0].
///
/// ```
/// use bhava::microbiology::drug_sedation;
///
/// assert!(drug_sedation(1.0, 5.0) < 0.2);
/// assert!(drug_sedation(15.0, 5.0) > 0.5);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn drug_sedation(concentration: f64, ec50: f64) -> f32 {
    if ec50 <= 0.0 {
        return 0.0;
    }
    let ratio = (concentration / ec50).clamp(0.0, 10.0);
    // Sigmoid: ratio 1 → ~0.15, ratio 2 → ~0.4, ratio 5 → ~0.8
    let sedation = 1.0 - (-0.3 * (ratio - 0.5)).exp();
    (sedation as f32).clamp(0.0, 1.0)
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Sickness Behavior ──────────────────────────────────────────────

    #[test]
    fn healthy_no_sickness() {
        let mood = sickness_mood(0.0);
        assert!(mood.frustration < 0.01);
        assert!(mood.joy.abs() < 0.01);
    }

    #[test]
    fn sick_depressed_mood() {
        let mood = sickness_mood(0.6);
        assert!(mood.joy < 0.0);
        assert!(mood.frustration > 0.2);
        assert!(mood.trust < 0.0);
        assert!(mood.interest < 0.0);
    }

    #[test]
    fn severity_zero_when_healthy() {
        assert!((sickness_severity(0.0, 0.0) - 0.0).abs() < 0.01);
    }

    #[test]
    fn severity_infected_worse_than_exposed() {
        let exposed_only = sickness_severity(0.5, 0.0);
        let infected_only = sickness_severity(0.0, 0.5);
        assert!(infected_only > exposed_only);
    }

    // ── Recovery ───────────────────────────────────────────────────────

    #[test]
    fn recovery_boosts_joy() {
        let early = recovery_mood_boost(0.1);
        let late = recovery_mood_boost(0.9);
        assert!(late.joy > early.joy);
        assert!(late.trust > early.trust);
    }

    #[test]
    fn immune_drain_at_rest() {
        assert!((immune_energy_drain(0.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn immune_drain_increases() {
        assert!(immune_energy_drain(0.5) > 1.5);
        assert!(immune_energy_drain(1.0) > 2.5);
    }

    // ── Social Behavior ────────────────────────────────────────────────

    #[test]
    fn low_r0_low_avoidance() {
        let pressure = contagion_avoidance(0.5, 0.3);
        assert!(pressure < 0.3);
    }

    #[test]
    fn high_r0_high_avoidance() {
        let pressure = contagion_avoidance(5.0, 0.2);
        assert!(pressure > 0.3);
    }

    #[test]
    fn high_coverage_more_safety() {
        let low = herd_safety(0.2, 3.0);
        let high = herd_safety(0.8, 3.0);
        assert!(high > low);
    }

    // ── Metabolism ──────────────────────────────────────────────────────

    #[test]
    fn optimal_growth_high_efficiency() {
        assert!(metabolic_efficiency(1.0) > 0.9);
    }

    #[test]
    fn stressed_growth_low_efficiency() {
        assert!(metabolic_efficiency(0.0) < 0.6);
    }

    #[test]
    fn optimal_temperature_low_stress() {
        let stress = temperature_stress(37.0, 15.0, 37.0, 45.0);
        assert!(stress < 0.1);
    }

    #[test]
    fn extreme_temperature_high_stress() {
        let cold = temperature_stress(18.0, 15.0, 37.0, 45.0);
        assert!(cold > 0.5);
    }

    // ── Pharmacology ───────────────────────────────────────────────────

    #[test]
    fn sub_therapeutic_low_effect() {
        let effect = drug_cognitive_effect(0.5, 1.0, 5.0, 1.0);
        assert!(effect < 0.2);
    }

    #[test]
    fn therapeutic_moderate_effect() {
        let effect = drug_cognitive_effect(5.0, 1.0, 5.0, 1.0);
        assert!(effect > 0.3 && effect < 0.7);
    }

    #[test]
    fn low_concentration_low_sedation() {
        assert!(drug_sedation(1.0, 5.0) < 0.3);
    }

    #[test]
    fn high_concentration_high_sedation() {
        assert!(drug_sedation(15.0, 5.0) > 0.5);
    }

    #[test]
    fn zero_ec50_no_sedation() {
        assert!((drug_sedation(5.0, 0.0) - 0.0).abs() < 0.01);
    }
}
