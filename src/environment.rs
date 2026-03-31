//! Environmental reactivity — the physical world pressing on emotion.
//!
//! Temperature, light, noise, air quality, altitude, weather — these press on
//! mood, energy, stress, and behavior. Bhava doesn't simulate the environment
//! (that's kiran/joshua with ushma, pravash, bijli, prakash). Bhava *reacts* to it.
//!
//! The consumer (game loop, simulation, chat agent) provides an [`Environment`]
//! struct each tick. [`environmental_modifiers`] maps it through personality to
//! produce an [`EnvironmentalEffect`] — multipliers and offsets for all affected
//! modules. No new emotional systems. Just the physical world pressing on the
//! modules we already have.
//!
//! # Example
//!
//! ```
//! use bhava::environment::{Environment, environmental_modifiers};
//!
//! let env = Environment::comfortable_indoor();
//! // Without personality: neutral modifiers
//! let effect = environmental_modifiers(&env, None);
//! assert!((effect.energy_drain_multiplier - 1.0).abs() < 0.1);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Weather condition for mood and stress modulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WeatherCondition {
    /// Clear sky — baseline joy nudge, full circadian light entrainment.
    #[default]
    Clear,
    /// Partial cloud cover — mild light reduction.
    Overcast,
    /// Fog — reduced salience, proximity trigger ranges shortened.
    Fog,
    /// Rain — personality-dependent: neuroticism → anxiety, openness → calm.
    Rain,
    /// Snowfall — quieting effect, mild energy drain from cold.
    Snow,
    /// Storm (thunder, high wind, heavy rain) — stress spike, arousal elevation.
    Storm,
}

impl fmt::Display for WeatherCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clear => write!(f, "Clear"),
            Self::Overcast => write!(f, "Overcast"),
            Self::Fog => write!(f, "Fog"),
            Self::Rain => write!(f, "Rain"),
            Self::Snow => write!(f, "Snow"),
            Self::Storm => write!(f, "Storm"),
        }
    }
}

/// Environmental state passed by the consumer each tick.
///
/// Bhava receives plain `f32` values — it doesn't care where they come from.
/// In a game, kiran provides them from ushma/pravash/prakash. In a chat agent,
/// [`Environment::comfortable_indoor()`] is a sensible default.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Ambient temperature in Celsius.
    pub temperature_c: f32,
    /// Relative humidity: 0–100%.
    pub humidity_pct: f32,
    /// Barometric pressure in hectopascals (standard ≈ 1013.25).
    pub pressure_hpa: f32,
    /// Ambient light in lux (0 = dark, 500 = office, 80_000 = direct sun).
    pub light_lux: f32,
    /// Ambient noise in decibels (30 = quiet room, 70 = traffic, 100 = concert).
    pub noise_db: f32,
    /// Wind speed in m/s.
    pub wind_speed_ms: f32,
    /// Air quality index: 0 (pristine) to 500 (hazardous).
    pub air_quality_aqi: f32,
    /// Elevation in meters above sea level.
    pub altitude_m: f32,
    /// Current weather condition.
    pub weather: WeatherCondition,
}

impl Default for Environment {
    fn default() -> Self {
        Self::comfortable_indoor()
    }
}

impl Environment {
    /// Comfortable indoor environment (22°C, 45% humidity, 500 lux, 30 dB).
    #[must_use]
    pub fn comfortable_indoor() -> Self {
        Self {
            temperature_c: 22.0,
            humidity_pct: 45.0,
            pressure_hpa: 1013.25,
            light_lux: 500.0,
            noise_db: 30.0,
            wind_speed_ms: 0.0,
            air_quality_aqi: 20.0,
            altitude_m: 100.0,
            weather: WeatherCondition::Clear,
        }
    }

    /// Hot summer day (38°C, 70% humidity, 80k lux, clear).
    #[must_use]
    pub fn hot_summer_day() -> Self {
        Self {
            temperature_c: 38.0,
            humidity_pct: 70.0,
            pressure_hpa: 1010.0,
            light_lux: 80_000.0,
            noise_db: 40.0,
            wind_speed_ms: 2.0,
            air_quality_aqi: 60.0,
            altitude_m: 100.0,
            weather: WeatherCondition::Clear,
        }
    }

    /// Cold winter night (−10°C, 30% humidity, 0.1 lux, clear).
    #[must_use]
    pub fn cold_winter_night() -> Self {
        Self {
            temperature_c: -10.0,
            humidity_pct: 30.0,
            pressure_hpa: 1020.0,
            light_lux: 0.1,
            noise_db: 20.0,
            wind_speed_ms: 3.0,
            air_quality_aqi: 15.0,
            altitude_m: 100.0,
            weather: WeatherCondition::Clear,
        }
    }

    /// Thunderstorm (15°C, 90% humidity, 200 lux, 75 dB wind, low pressure).
    #[must_use]
    pub fn storm() -> Self {
        Self {
            temperature_c: 15.0,
            humidity_pct: 90.0,
            pressure_hpa: 990.0,
            light_lux: 200.0,
            noise_db: 75.0,
            wind_speed_ms: 15.0,
            air_quality_aqi: 30.0,
            altitude_m: 100.0,
            weather: WeatherCondition::Storm,
        }
    }

    /// Typical office (21°C, fluorescent 400 lux, 45 dB HVAC hum).
    #[must_use]
    pub fn office() -> Self {
        Self {
            temperature_c: 21.0,
            humidity_pct: 40.0,
            pressure_hpa: 1013.25,
            light_lux: 400.0,
            noise_db: 45.0,
            wind_speed_ms: 0.0,
            air_quality_aqi: 25.0,
            altitude_m: 100.0,
            weather: WeatherCondition::Clear,
        }
    }

    /// Forest environment (18°C, 60% humidity, dappled 2k lux, 25 dB ambient).
    #[must_use]
    pub fn forest() -> Self {
        Self {
            temperature_c: 18.0,
            humidity_pct: 60.0,
            pressure_hpa: 1015.0,
            light_lux: 2000.0,
            noise_db: 25.0,
            wind_speed_ms: 1.0,
            air_quality_aqi: 10.0,
            altitude_m: 300.0,
            weather: WeatherCondition::Clear,
        }
    }

    /// Compute heat index from temperature and humidity.
    ///
    /// Simplified Steadman (1979) heat index — how hot it *feels* when humidity
    /// traps body heat. Only meaningful above 27°C; below that, returns temperature.
    #[must_use]
    #[inline]
    pub fn heat_index(&self) -> f32 {
        if self.temperature_c < 27.0 {
            return self.temperature_c;
        }
        let t = self.temperature_c;
        let rh = self.humidity_pct;
        // Rothfusz regression (NWS simplified)
        let hi =
            -8.785 + 1.611 * t + 2.339 * rh - 0.1461 * t * rh - 0.01231 * t * t - 0.01642 * rh * rh
                + 0.002212 * t * t * rh
                + 0.000725 * t * rh * rh
                - 0.000003582 * t * t * rh * rh;
        hi.max(t)
    }

    /// Wind chill temperature in Celsius.
    ///
    /// North American wind chill formula (Environment Canada / NWS). Only
    /// meaningful below 10°C with wind > 4.8 km/h; otherwise returns temperature.
    #[must_use]
    #[inline]
    pub fn wind_chill(&self) -> f32 {
        let wind_kmh = self.wind_speed_ms * 3.6;
        if self.temperature_c > 10.0 || wind_kmh < 4.8 {
            return self.temperature_c;
        }
        let v016 = wind_kmh.powf(0.16);
        13.12 + 0.6215 * self.temperature_c - 11.37 * v016 + 0.3965 * self.temperature_c * v016
    }

    /// Apparent temperature accounting for both heat index and wind chill.
    #[must_use]
    #[inline]
    pub fn apparent_temperature(&self) -> f32 {
        if self.temperature_c >= 27.0 {
            self.heat_index()
        } else if self.temperature_c <= 10.0 {
            self.wind_chill()
        } else {
            self.temperature_c
        }
    }
}

/// Environmental effect modifiers for all affected bhava modules.
///
/// Multipliers default to `1.0` (no effect). Offsets default to `0.0`.
/// Consumers apply these to their tick loops:
///
/// ```
/// use bhava::environment::{Environment, EnvironmentalEffect, environmental_modifiers};
///
/// let effect = environmental_modifiers(&Environment::office(), None);
/// // energy.drain_rate *= effect.energy_drain_multiplier;
/// // stress.accumulation_rate *= effect.stress_accumulation_multiplier;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalEffect {
    // ── Energy modifiers ──
    /// Multiplier on energy drain rate (>1.0 = drains faster).
    pub energy_drain_multiplier: f32,
    /// Multiplier on energy recovery rate (<1.0 = recovers slower).
    pub energy_recovery_multiplier: f32,

    // ── Stress modifiers ──
    /// Multiplier on stress accumulation rate.
    pub stress_accumulation_multiplier: f32,

    // ── Circadian modifiers ──
    /// Alertness offset: positive = more alert, negative = drowsy.
    pub alertness_offset: f32,

    // ── Flow modifiers ──
    /// Multiplier on flow disruption probability (>1.0 = easier to disrupt).
    pub flow_disruption_multiplier: f32,

    // ── Mood baseline nudges ──
    /// Joy baseline offset (clear sky +, storm − for sensitive).
    pub mood_joy_offset: f32,
    /// Arousal baseline offset (heat/noise → up, calm → down).
    pub mood_arousal_offset: f32,
    /// Trust baseline offset (low pressure → down, stable → up).
    pub mood_trust_offset: f32,

    // ── Salience modifiers ──
    /// Multiplier on salience detection range (<1.0 = reduced, fog).
    pub salience_range_multiplier: f32,
}

impl Default for EnvironmentalEffect {
    fn default() -> Self {
        Self {
            energy_drain_multiplier: 1.0,
            energy_recovery_multiplier: 1.0,
            stress_accumulation_multiplier: 1.0,
            alertness_offset: 0.0,
            flow_disruption_multiplier: 1.0,
            mood_joy_offset: 0.0,
            mood_arousal_offset: 0.0,
            mood_trust_offset: 0.0,
            salience_range_multiplier: 1.0,
        }
    }
}

impl EnvironmentalEffect {
    /// Neutral effect — no environmental influence.
    #[must_use]
    #[inline]
    pub fn neutral() -> Self {
        Self::default()
    }
}

/// Personality sensitivity weights for environmental modulation.
///
/// Not everyone reacts the same way to a hot day. These weights scale
/// environmental effects based on personality traits. Without personality
/// data, a neutral (1.0) sensitivity is used.
#[derive(Debug, Clone)]
struct PersonalitySensitivity {
    /// High patience → heat/noise tolerance (dampens stress).
    patience: f32,
    /// High sensitivity (neuroticism proxy) → weather-reactive.
    sensitivity: f32,
    /// High resilience (confidence) → dampens all environmental stress.
    resilience: f32,
    /// High curiosity → rain/fog as interesting, not stressful.
    curiosity: f32,
}

impl Default for PersonalitySensitivity {
    fn default() -> Self {
        Self {
            patience: 0.0,
            sensitivity: 0.0,
            resilience: 0.0,
            curiosity: 0.0,
        }
    }
}

#[cfg(feature = "traits")]
fn extract_sensitivity(profile: &crate::traits::PersonalityProfile) -> PersonalitySensitivity {
    use crate::traits::TraitKind;
    PersonalitySensitivity {
        patience: profile.get_trait(TraitKind::Patience).normalized(),
        // Sensitivity = inverse of patience + empathy proxy
        sensitivity: -profile.get_trait(TraitKind::Patience).normalized() * 0.5
            + profile.get_trait(TraitKind::Empathy).normalized() * 0.5,
        resilience: profile.get_trait(TraitKind::Confidence).normalized(),
        curiosity: profile.get_trait(TraitKind::Curiosity).normalized(),
    }
}

/// Compute environmental effect modifiers from environment and optional personality.
///
/// Returns multipliers and offsets that consumers apply to their module states.
/// Without personality (`None`), uses neutral sensitivity — everyone reacts
/// the same. With personality, traits modulate the response:
///
/// - **High Patience** → heat/noise tolerance, slower stress accumulation
/// - **High Sensitivity** → weather-reactive, barometric headaches, storm anxiety
/// - **High Resilience (Confidence)** → environmental stress dampened
/// - **High Curiosity** → rain/fog as *interesting* not *stressful*
///
/// # Example
///
/// ```
/// use bhava::environment::{Environment, environmental_modifiers};
///
/// let storm = Environment::storm();
/// let effect = environmental_modifiers(&storm, None);
/// assert!(effect.stress_accumulation_multiplier > 1.0);
/// assert!(effect.mood_arousal_offset > 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn environmental_modifiers(
    env: &Environment,
    #[cfg(feature = "traits")] personality: Option<&crate::traits::PersonalityProfile>,
    #[cfg(not(feature = "traits"))] _personality: Option<&()>,
) -> EnvironmentalEffect {
    #[cfg(feature = "traits")]
    let sens = personality.map(extract_sensitivity).unwrap_or_default();
    #[cfg(not(feature = "traits"))]
    let sens = PersonalitySensitivity::default();

    let mut effect = EnvironmentalEffect::neutral();

    // Resilience dampens all negative environmental effects.
    // Range: 0.7 (very resilient) to 1.3 (very fragile).
    let resilience_factor = 1.0 - sens.resilience * 0.3;

    // ── Temperature ──
    let apparent = env.apparent_temperature();
    if apparent > 35.0 {
        // Heat stress: increased drain, reduced recovery.
        let heat_severity = ((apparent - 35.0) / 15.0).clamp(0.0, 1.0);
        let patience_dampen = 1.0 - sens.patience * 0.3; // patient → tolerant
        effect.energy_drain_multiplier += 0.5 * heat_severity * patience_dampen * resilience_factor;
        effect.energy_recovery_multiplier -= 0.3 * heat_severity * resilience_factor;
        effect.stress_accumulation_multiplier +=
            0.4 * heat_severity * patience_dampen * resilience_factor;
        effect.mood_arousal_offset += 0.1 * heat_severity;
    } else if apparent < 0.0 {
        // Cold stress: increased drain (shivering), allostatic load.
        let cold_severity = ((-apparent) / 20.0).clamp(0.0, 1.0);
        effect.energy_drain_multiplier += 0.4 * cold_severity * resilience_factor;
        effect.stress_accumulation_multiplier += 0.3 * cold_severity * resilience_factor;
    }

    // ── Humidity + heat compound ──
    if env.temperature_c > 30.0 && env.humidity_pct > 60.0 {
        let compound = ((env.humidity_pct - 60.0) / 40.0).clamp(0.0, 1.0)
            * ((env.temperature_c - 30.0) / 10.0).clamp(0.0, 1.0);
        effect.stress_accumulation_multiplier += 0.3 * compound * resilience_factor;
    }

    // ── Barometric pressure ──
    let pressure_deviation = (env.pressure_hpa - 1013.25) / 1013.25;
    if pressure_deviation < -0.015 {
        // Low pressure → anxiety nudge. Sensitive individuals amplified.
        let low_p = ((-pressure_deviation - 0.015) / 0.03).clamp(0.0, 1.0);
        let sensitivity_amp = 1.0 + sens.sensitivity * 0.5;
        effect.mood_trust_offset -= 0.1 * low_p * sensitivity_amp * resilience_factor;
        effect.mood_arousal_offset += 0.1 * low_p * sensitivity_amp * resilience_factor;
        effect.stress_accumulation_multiplier += 0.2 * low_p * sensitivity_amp * resilience_factor;
    }

    // ── Light ──
    if env.light_lux < 100.0 {
        // Low light → drowsiness (melatonin proxy).
        let dark = (1.0 - env.light_lux / 100.0).clamp(0.0, 1.0);
        effect.alertness_offset -= 0.2 * dark;
    } else if env.light_lux > 10_000.0 {
        // Bright light → alertness boost, circadian entrainment.
        let bright = ((env.light_lux - 10_000.0) / 70_000.0).clamp(0.0, 1.0);
        effect.alertness_offset += 0.15 * bright;
    }

    // ── Noise ──
    if env.noise_db > 70.0 {
        // High noise → chronic stress, flow disruption.
        let noise_severity = ((env.noise_db - 70.0) / 30.0).clamp(0.0, 1.0);
        let patience_dampen = 1.0 - sens.patience * 0.3;
        effect.stress_accumulation_multiplier +=
            0.3 * noise_severity * patience_dampen * resilience_factor;
        effect.flow_disruption_multiplier += 0.6 * noise_severity;
    }
    // Sustained moderate noise also impairs flow.
    if env.noise_db > 55.0 {
        let sustained = ((env.noise_db - 55.0) / 15.0).clamp(0.0, 1.0);
        effect.flow_disruption_multiplier += 0.2 * sustained;
    }

    // ── Wind ──
    if env.wind_speed_ms > 10.0 {
        // High wind → energy cost, exposure stress.
        let wind_severity = ((env.wind_speed_ms - 10.0) / 20.0).clamp(0.0, 1.0);
        effect.energy_drain_multiplier += 0.2 * wind_severity * resilience_factor;
    }

    // ── Air quality ──
    if env.air_quality_aqi > 150.0 {
        // Poor air → reduced recovery, elevated stress baseline.
        let aqi_severity = ((env.air_quality_aqi - 150.0) / 350.0).clamp(0.0, 1.0);
        effect.energy_recovery_multiplier -= 0.2 * aqi_severity * resilience_factor;
        effect.stress_accumulation_multiplier += 0.2 * aqi_severity * resilience_factor;
    }

    // ── Altitude ──
    if env.altitude_m > 2500.0 {
        // High altitude → reduced peak performance (O₂).
        let alt_severity = ((env.altitude_m - 2500.0) / 5000.0).clamp(0.0, 1.0);
        effect.energy_drain_multiplier += 0.3 * alt_severity * resilience_factor;
        effect.energy_recovery_multiplier -= 0.15 * alt_severity * resilience_factor;
    }

    // ── Weather condition ──
    match env.weather {
        WeatherCondition::Clear => {
            effect.mood_joy_offset += 0.05;
        }
        WeatherCondition::Overcast => {
            // Mild light reduction handled by light_lux; slight mood damping.
            effect.mood_joy_offset -= 0.02;
        }
        WeatherCondition::Fog => {
            // Reduced environmental salience.
            effect.salience_range_multiplier *= 0.5;
            // Curious entities find fog interesting, not oppressive.
            let curiosity_flip = sens.curiosity * 0.05;
            effect.mood_joy_offset += curiosity_flip - 0.03;
        }
        WeatherCondition::Rain => {
            // Personality-dependent: sensitive → anxiety; curious → calm/reflective.
            let sensitivity_reaction = sens.sensitivity * 0.08 * resilience_factor;
            let curiosity_reaction = sens.curiosity * 0.05;
            effect.mood_joy_offset += curiosity_reaction - sensitivity_reaction;
            effect.mood_arousal_offset -= 0.05; // rain generally calming
        }
        WeatherCondition::Snow => {
            effect.mood_joy_offset += 0.02; // mild novelty
            effect.energy_drain_multiplier += 0.1 * resilience_factor; // cold exertion
            effect.salience_range_multiplier *= 0.8; // reduced visibility
        }
        WeatherCondition::Storm => {
            let sensitivity_amp = 1.0 + sens.sensitivity * 0.5;
            effect.stress_accumulation_multiplier += 0.3 * sensitivity_amp * resilience_factor;
            effect.mood_arousal_offset += 0.15;
            effect.mood_trust_offset -= 0.1 * sensitivity_amp * resilience_factor;
            effect.flow_disruption_multiplier += 0.4;
            effect.salience_range_multiplier *= 0.6;
        }
    }

    // Clamp all multipliers to sane ranges.
    effect.energy_drain_multiplier = effect.energy_drain_multiplier.clamp(0.5, 3.0);
    effect.energy_recovery_multiplier = effect.energy_recovery_multiplier.clamp(0.3, 1.5);
    effect.stress_accumulation_multiplier = effect.stress_accumulation_multiplier.clamp(0.5, 3.0);
    effect.alertness_offset = effect.alertness_offset.clamp(-0.3, 0.3);
    effect.flow_disruption_multiplier = effect.flow_disruption_multiplier.clamp(0.5, 3.0);
    effect.mood_joy_offset = effect.mood_joy_offset.clamp(-0.2, 0.2);
    effect.mood_arousal_offset = effect.mood_arousal_offset.clamp(-0.2, 0.2);
    effect.mood_trust_offset = effect.mood_trust_offset.clamp(-0.2, 0.2);
    effect.salience_range_multiplier = effect.salience_range_multiplier.clamp(0.2, 1.5);

    effect
}

/// Convenience: apply environmental effects to module states in one call.
///
/// Adjusts energy drain/recovery rates, stress accumulation, and nudges mood
/// baselines. This mutates the states directly — call once per tick with the
/// current environment.
///
/// # Example
///
/// ```
/// use bhava::environment::{Environment, apply_environment};
/// use bhava::energy::EnergyState;
/// use bhava::stress::StressState;
/// use bhava::mood::MoodVector;
///
/// let mut energy = EnergyState::new();
/// let mut stress = StressState::new();
/// let mut mood = MoodVector::neutral();
///
/// let env = Environment::hot_summer_day();
/// apply_environment(&env, &mut energy, &mut stress, &mut mood, None);
/// assert!(energy.drain_rate > 0.02); // heat increased drain
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
pub fn apply_environment(
    env: &Environment,
    energy: &mut crate::energy::EnergyState,
    stress: &mut crate::stress::StressState,
    mood: &mut crate::mood::MoodVector,
    #[cfg(feature = "traits")] personality: Option<&crate::traits::PersonalityProfile>,
    #[cfg(not(feature = "traits"))] personality: Option<&()>,
) {
    let effect = environmental_modifiers(env, personality);

    // Energy
    energy.drain_rate *= effect.energy_drain_multiplier;
    energy.recovery_rate *= effect.energy_recovery_multiplier;

    // Stress
    stress.accumulation_rate *= effect.stress_accumulation_multiplier;

    // Mood baseline nudges
    use crate::mood::Emotion;
    mood.nudge(Emotion::Joy, effect.mood_joy_offset);
    mood.nudge(Emotion::Arousal, effect.mood_arousal_offset);
    mood.nudge(Emotion::Trust, effect.mood_trust_offset);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comfortable_indoor_is_neutral() {
        let env = Environment::comfortable_indoor();
        let effect = environmental_modifiers(&env, None);
        assert!((effect.energy_drain_multiplier - 1.0).abs() < 0.01);
        assert!((effect.energy_recovery_multiplier - 1.0).abs() < 0.01);
        assert!((effect.stress_accumulation_multiplier - 1.0).abs() < 0.01);
        assert!((effect.flow_disruption_multiplier - 1.0).abs() < 0.01);
        assert!((effect.salience_range_multiplier - 1.0).abs() < 0.01);
    }

    #[test]
    fn hot_day_increases_drain() {
        let env = Environment::hot_summer_day();
        let effect = environmental_modifiers(&env, None);
        assert!(effect.energy_drain_multiplier > 1.0);
        assert!(effect.energy_recovery_multiplier < 1.0);
        assert!(effect.stress_accumulation_multiplier > 1.0);
    }

    #[test]
    fn cold_night_increases_drain() {
        let env = Environment::cold_winter_night();
        let effect = environmental_modifiers(&env, None);
        assert!(effect.energy_drain_multiplier > 1.0);
        assert!(effect.stress_accumulation_multiplier > 1.0);
    }

    #[test]
    fn storm_elevates_stress_and_arousal() {
        let env = Environment::storm();
        let effect = environmental_modifiers(&env, None);
        assert!(effect.stress_accumulation_multiplier > 1.0);
        assert!(effect.mood_arousal_offset > 0.0);
        assert!(effect.flow_disruption_multiplier > 1.0);
        assert!(effect.salience_range_multiplier < 1.0);
    }

    #[test]
    fn fog_reduces_salience() {
        let mut env = Environment::comfortable_indoor();
        env.weather = WeatherCondition::Fog;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.salience_range_multiplier < 0.6);
    }

    #[test]
    fn low_light_causes_drowsiness() {
        let mut env = Environment::comfortable_indoor();
        env.light_lux = 10.0;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.alertness_offset < 0.0);
    }

    #[test]
    fn bright_light_boosts_alertness() {
        let mut env = Environment::comfortable_indoor();
        env.light_lux = 50_000.0;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.alertness_offset > 0.0);
    }

    #[test]
    fn high_noise_disrupts_flow() {
        let mut env = Environment::comfortable_indoor();
        env.noise_db = 85.0;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.flow_disruption_multiplier >= 1.5);
        assert!(effect.stress_accumulation_multiplier > 1.0);
    }

    #[test]
    fn high_altitude_drains_energy() {
        let mut env = Environment::comfortable_indoor();
        env.altitude_m = 4000.0;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.energy_drain_multiplier > 1.0);
        assert!(effect.energy_recovery_multiplier < 1.0);
    }

    #[test]
    fn poor_air_quality_reduces_recovery() {
        let mut env = Environment::comfortable_indoor();
        env.air_quality_aqi = 300.0;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.energy_recovery_multiplier < 1.0);
        assert!(effect.stress_accumulation_multiplier > 1.0);
    }

    #[test]
    fn clear_sky_joy_nudge() {
        let env = Environment::comfortable_indoor(); // clear weather
        let effect = environmental_modifiers(&env, None);
        assert!(effect.mood_joy_offset > 0.0);
    }

    #[test]
    fn heat_index_below_threshold_returns_temp() {
        let env = Environment {
            temperature_c: 20.0,
            humidity_pct: 80.0,
            ..Default::default()
        };
        assert!((env.heat_index() - 20.0).abs() < 0.01);
    }

    #[test]
    fn heat_index_above_threshold() {
        let env = Environment {
            temperature_c: 35.0,
            humidity_pct: 80.0,
            ..Default::default()
        };
        assert!(env.heat_index() > 35.0); // feels hotter
    }

    #[test]
    fn wind_chill_below_threshold() {
        let env = Environment {
            temperature_c: -5.0,
            wind_speed_ms: 10.0,
            ..Default::default()
        };
        assert!(env.wind_chill() < -5.0); // feels colder
    }

    #[test]
    fn wind_chill_warm_returns_temp() {
        let env = Environment {
            temperature_c: 15.0,
            wind_speed_ms: 10.0,
            ..Default::default()
        };
        assert!((env.wind_chill() - 15.0).abs() < 0.01);
    }

    #[test]
    fn low_pressure_anxiety() {
        let mut env = Environment::comfortable_indoor();
        env.pressure_hpa = 980.0;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.mood_trust_offset < 0.0);
        assert!(effect.mood_arousal_offset > 0.0);
    }

    #[test]
    fn multipliers_clamped() {
        // Extreme environment should not produce unbounded values.
        let env = Environment {
            temperature_c: 55.0,
            humidity_pct: 100.0,
            pressure_hpa: 900.0,
            light_lux: 0.0,
            noise_db: 120.0,
            wind_speed_ms: 40.0,
            air_quality_aqi: 500.0,
            altitude_m: 8000.0,
            weather: WeatherCondition::Storm,
        };
        let effect = environmental_modifiers(&env, None);
        assert!(effect.energy_drain_multiplier <= 3.0);
        assert!(effect.energy_recovery_multiplier >= 0.3);
        assert!(effect.stress_accumulation_multiplier <= 3.0);
        assert!(effect.flow_disruption_multiplier <= 3.0);
        assert!(effect.salience_range_multiplier >= 0.2);
    }

    #[test]
    fn serde_roundtrip_environment() {
        let env = Environment::storm();
        let json = serde_json::to_string(&env).unwrap();
        let deser: Environment = serde_json::from_str(&json).unwrap();
        assert!((env.temperature_c - deser.temperature_c).abs() < f32::EPSILON);
        assert_eq!(env.weather, deser.weather);
    }

    #[test]
    fn serde_roundtrip_effect() {
        let env = Environment::hot_summer_day();
        let effect = environmental_modifiers(&env, None);
        let json = serde_json::to_string(&effect).unwrap();
        let deser: EnvironmentalEffect = serde_json::from_str(&json).unwrap();
        assert!(
            (effect.energy_drain_multiplier - deser.energy_drain_multiplier).abs() < f32::EPSILON
        );
    }

    #[test]
    fn serde_roundtrip_weather() {
        let conditions = [
            WeatherCondition::Clear,
            WeatherCondition::Overcast,
            WeatherCondition::Fog,
            WeatherCondition::Rain,
            WeatherCondition::Snow,
            WeatherCondition::Storm,
        ];
        for cond in &conditions {
            let json = serde_json::to_string(cond).unwrap();
            let deser: WeatherCondition = serde_json::from_str(&json).unwrap();
            assert_eq!(*cond, deser);
        }
    }

    #[test]
    fn apply_environment_mutates_states() {
        let mut energy = crate::energy::EnergyState::new();
        let mut stress = crate::stress::StressState::new();
        let mut mood = crate::mood::MoodVector::neutral();
        let base_drain = energy.drain_rate;
        let base_stress = stress.accumulation_rate;

        let env = Environment::hot_summer_day();
        apply_environment(&env, &mut energy, &mut stress, &mut mood, None);

        assert!(energy.drain_rate > base_drain);
        assert!(stress.accumulation_rate > base_stress);
    }

    #[test]
    fn display_weather() {
        assert_eq!(WeatherCondition::Storm.to_string(), "Storm");
        assert_eq!(WeatherCondition::Clear.to_string(), "Clear");
    }

    #[test]
    fn rain_calms_arousal() {
        let mut env = Environment::comfortable_indoor();
        env.weather = WeatherCondition::Rain;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.mood_arousal_offset < 0.0);
    }

    #[test]
    fn snow_mild_novelty() {
        let mut env = Environment::comfortable_indoor();
        env.weather = WeatherCondition::Snow;
        let effect = environmental_modifiers(&env, None);
        assert!(effect.mood_joy_offset > 0.0);
        assert!(effect.salience_range_multiplier < 1.0);
    }

    #[test]
    fn forest_is_pleasant() {
        let env = Environment::forest();
        let effect = environmental_modifiers(&env, None);
        // Forest: clear sky, low noise, good air, moderate light.
        assert!(effect.mood_joy_offset > 0.0);
        assert!((effect.stress_accumulation_multiplier - 1.0).abs() < 0.1);
        assert!((effect.flow_disruption_multiplier - 1.0).abs() < 0.1);
    }

    #[test]
    fn humidity_heat_compound() {
        let env = Environment {
            temperature_c: 38.0,
            humidity_pct: 90.0,
            ..Default::default()
        };
        let effect = environmental_modifiers(&env, None);
        // Both heat AND humidity compound → extra stress.
        assert!(effect.stress_accumulation_multiplier > 1.5);
    }
}
