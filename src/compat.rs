//! Jantu creature behavior integration — bridging animal instincts to human personality.
//!
//! Provides conversion functions between jantu's ethological types and bhava's
//! personality/emotion types. Jantu models the animal brain (instincts, survival,
//! stress); bhava models the human mind built on top.
//!
//! Requires the `instinct` feature.
//!
//! # Layer Model
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Bhava (Human Personality)       │
//! │  Traits, emotions, reasoning     │
//! ├──────────────────────────────────┤
//! │  This module (bridge)            │
//! │  Instinct → Mood/Drive mapping   │
//! ├──────────────────────────────────┤
//! │  Jantu (Animal Instincts)        │
//! │  Drives, survival, stress        │
//! └──────────────────────────────────┘
//! ```
//!
//! # Bridge Functions
//!
//! ## Core (v1.1.1)
//! - [`mood_from_threat_response`] — fight/flight/freeze/fawn → PAD mood
//! - [`load_from_stress`] — jantu stress → bhava allostatic load
//! - [`mood_shift_from_instinct`] — instinct urgency → emotion dimension
//! - [`dominance_from_rank`] — hierarchy position → dominance
//! - [`instinct_layer_score`] — dominant instinct → intuition layer 1
//!
//! ## Contagion & Social (v1.2.0)
//! - [`mood_from_contagion`] — emotional contagion pressure → mood shift
//! - [`trust_from_cohesion`] — group cohesion → trust modifier
//! - [`mood_from_territorial`] — territorial aggression → dominance/frustration
//!
//! ## Learning & Memory (v1.2.0)
//! - [`reactivity_from_habituation`] — habituation → emotional reactivity scalar
//! - [`actr_seed_from_memory`] — jantu memory trace → ACT-R activation seed
//!
//! ## Environment & Body (v1.2.0)
//! - [`stress_from_landscape`] — perceived risk → stress accumulation input
//! - [`energy_drain_from_drives`] — active instinct drives → energy exertion
//! - [`alertness_from_activity`] — jantu circadian activity → bhava alertness
//!
//! ## Genetics → Personality (v1.2.0)
//! - [`trait_seeds_from_genome`] — behavioral genome → personality trait seeds
//!
//! ## Signals (v1.2.0)
//! - [`mood_from_signal`] — received signal → mood shift

use crate::mood::{Emotion, MoodVector};

// ── Core bridges (v1.1.1) ──────────────────────────────────────────────

/// Convert a jantu threat response into a bhava mood shift.
///
/// Maps the four F's (fight/flight/freeze/fawn) to PAD-model mood dimensions.
///
/// ```
/// use jantu::survival::ThreatResponse;
/// use bhava::compat::mood_from_threat_response;
///
/// let mood = mood_from_threat_response(ThreatResponse::Fight);
/// assert!(mood.arousal > 0.5);
/// assert!(mood.dominance > 0.0);
/// ```
#[must_use]
pub fn mood_from_threat_response(response: jantu::ThreatResponse) -> MoodVector {
    match response {
        jantu::ThreatResponse::Fight => MoodVector {
            joy: -0.3,
            arousal: 0.9,
            dominance: 0.6,
            trust: -0.4,
            interest: -0.2,
            frustration: 0.5,
        },
        jantu::ThreatResponse::Flight => MoodVector {
            joy: -0.5,
            arousal: 0.8,
            dominance: -0.6,
            trust: -0.5,
            interest: -0.3,
            frustration: 0.3,
        },
        jantu::ThreatResponse::Freeze => MoodVector {
            joy: -0.4,
            arousal: 0.3,
            dominance: -0.8,
            trust: -0.3,
            interest: -0.5,
            frustration: 0.2,
        },
        jantu::ThreatResponse::Fawn => MoodVector {
            joy: -0.2,
            arousal: 0.4,
            dominance: -0.7,
            trust: 0.2,
            interest: 0.1,
            frustration: 0.1,
        },
        _ => MoodVector {
            joy: -0.3,
            arousal: 0.5,
            dominance: -0.3,
            trust: -0.2,
            interest: -0.2,
            frustration: 0.3,
        },
    }
}

/// Convert jantu stress state to a bhava allostatic load input.
///
/// Maps jantu's two-tier stress model (acute + chronic + resilience)
/// to a single 0.0–1.0 load value suitable for bhava's stress system.
///
/// ```
/// use jantu::stress::StressState;
/// use bhava::compat::load_from_stress;
///
/// let mut s = StressState::new();
/// assert!(load_from_stress(&s) < 0.01);
///
/// // Repeated stress builds chronic load
/// for _ in 0..10 {
///     s.apply_stressor(0.8);
/// }
/// assert!(load_from_stress(&s) > 0.1);
/// ```
#[must_use]
pub fn load_from_stress(stress: &jantu::stress::StressState) -> f32 {
    // Chronic stress is the primary driver of allostatic load;
    // acute stress contributes when resilience is low.
    let acute_contribution = stress.acute * (1.0 - stress.resilience) * 0.3;
    let chronic_contribution = stress.chronic * 0.7;
    (acute_contribution + chronic_contribution).clamp(0.0, 1.0)
}

/// Map a jantu instinct's urgency to a bhava emotion dimension and magnitude.
///
/// Returns `(emotion, magnitude)` where magnitude is in [-1.0, 1.0].
/// Higher instinct priority → stronger mood effect.
///
/// ```
/// use jantu::instinct::{Instinct, InstinctType, DriveLevel};
/// use bhava::compat::mood_shift_from_instinct;
/// use bhava::mood::Emotion;
///
/// let mut fear = Instinct::new(InstinctType::Fear);
/// fear.drive = DriveLevel::new(0.9);
/// fear.update_priority();
///
/// let (emotion, magnitude) = mood_shift_from_instinct(&fear);
/// assert_eq!(emotion, Emotion::Arousal);
/// assert!(magnitude > 0.5);
/// ```
#[must_use]
pub fn mood_shift_from_instinct(instinct: &jantu::Instinct) -> (Emotion, f32) {
    let intensity = instinct.priority.clamp(0.0, 1.0);
    match instinct.instinct_type {
        jantu::InstinctType::Fear => (Emotion::Arousal, intensity),
        jantu::InstinctType::Aggression => (Emotion::Frustration, intensity),
        jantu::InstinctType::Hunger | jantu::InstinctType::Thirst => {
            (Emotion::Frustration, intensity * 0.5)
        }
        jantu::InstinctType::Curiosity => (Emotion::Interest, intensity),
        jantu::InstinctType::Social => (Emotion::Trust, intensity),
        jantu::InstinctType::Nurturing => (Emotion::Joy, intensity * 0.6),
        jantu::InstinctType::Reproduction => (Emotion::Arousal, intensity * 0.4),
        jantu::InstinctType::Rest => (Emotion::Arousal, -intensity * 0.5),
        // Forward-compatible: unknown instinct types map to mild interest
        _ => (Emotion::Interest, intensity * 0.2),
    }
}

/// Convert jantu hierarchy position to bhava dominance dimension.
///
/// ```
/// use jantu::social::HierarchyPosition;
/// use bhava::compat::dominance_from_rank;
///
/// let alpha = dominance_from_rank(HierarchyPosition::new(0.9));
/// let omega = dominance_from_rank(HierarchyPosition::new(0.1));
/// assert!(alpha > omega);
/// assert!(alpha > 0.0);  // dominant → positive
/// assert!(omega < 0.0);  // subordinate → negative
/// ```
#[must_use]
pub fn dominance_from_rank(position: jantu::HierarchyPosition) -> f32 {
    // Map [0.0, 1.0] rank to [-1.0, 1.0] dominance
    position.value() * 2.0 - 1.0
}

/// Compute the instinct layer score for the intuition system.
///
/// When jantu instincts are available, this provides the "hardwired,
/// species-level" layer 1 score for [`active_layer`](crate::intuition::active_layer).
///
/// High instinct urgency (dominant drive priority > 0.7) means the
/// creature is operating on instinct, overriding higher cognitive layers.
///
/// ```
/// use jantu::instinct::{Instinct, InstinctType, DriveLevel, dominant_instinct};
/// use bhava::compat::instinct_layer_score;
///
/// let mut fear = Instinct::new(InstinctType::Fear);
/// fear.drive = DriveLevel::new(0.9);
/// fear.update_priority();
///
/// let instincts = [fear];
/// let score = instinct_layer_score(&instincts);
/// assert!(score > 0.7); // strong instinct → high layer score
/// ```
#[must_use]
pub fn instinct_layer_score(instincts: &[jantu::Instinct]) -> f32 {
    jantu::instinct::dominant_instinct(instincts)
        .map(|i| i.priority.clamp(0.0, 1.0))
        .unwrap_or(0.0)
}

// ── Contagion & Social bridges (v1.2.0) ─────────────────────────────────

/// Convert jantu emotional contagion aggregate pressure into a bhava mood shift.
///
/// Maps jantu's four emotional states (Fear, Aggression, Calm, Excitement)
/// to bhava mood dimensions. The magnitude scales the shift — higher group
/// pressure produces stronger mood effects.
///
/// ```
/// use jantu::contagion::EmotionalState;
/// use bhava::compat::mood_from_contagion;
///
/// let mood = mood_from_contagion(EmotionalState::Fear, 0.8);
/// assert!(mood.arousal > 0.0);
/// assert!(mood.trust < 0.0);
/// ```
#[must_use]
pub fn mood_from_contagion(state: jantu::contagion::EmotionalState, magnitude: f32) -> MoodVector {
    let mag = magnitude.clamp(0.0, 1.0);
    match state {
        jantu::contagion::EmotionalState::Fear => MoodVector {
            joy: -0.3 * mag,
            arousal: 0.7 * mag,
            dominance: -0.4 * mag,
            trust: -0.3 * mag,
            interest: -0.2 * mag,
            frustration: 0.2 * mag,
        },
        jantu::contagion::EmotionalState::Aggression => MoodVector {
            joy: -0.2 * mag,
            arousal: 0.6 * mag,
            dominance: 0.3 * mag,
            trust: -0.4 * mag,
            interest: -0.1 * mag,
            frustration: 0.6 * mag,
        },
        jantu::contagion::EmotionalState::Calm => MoodVector {
            joy: 0.2 * mag,
            arousal: -0.4 * mag,
            dominance: 0.1 * mag,
            trust: 0.3 * mag,
            interest: 0.1 * mag,
            frustration: -0.3 * mag,
        },
        jantu::contagion::EmotionalState::Excitement => MoodVector {
            joy: 0.4 * mag,
            arousal: 0.6 * mag,
            dominance: 0.1 * mag,
            trust: 0.2 * mag,
            interest: 0.5 * mag,
            frustration: -0.1 * mag,
        },
        _ => MoodVector {
            joy: 0.0,
            arousal: 0.1 * mag,
            dominance: 0.0,
            trust: 0.0,
            interest: 0.0,
            frustration: 0.0,
        },
    }
}

/// Convert jantu group cohesion into a bhava trust modifier.
///
/// High group cohesion (animals staying close together) maps to increased
/// trust in the bhava relationship system. Low cohesion (scattered group)
/// maps to decreased trust.
///
/// Returns a trust delta in [-0.5, 0.5] suitable for
/// [`Relationship::interact`](crate::relationship::Relationship).
///
/// ```
/// use bhava::compat::trust_from_cohesion;
///
/// let tight_group = trust_from_cohesion(0.9);
/// let scattered = trust_from_cohesion(0.2);
/// assert!(tight_group > 0.0);
/// assert!(scattered < 0.0);
/// ```
#[must_use]
#[inline]
pub fn trust_from_cohesion(cohesion: f32) -> f32 {
    // Map [0.0, 1.0] cohesion to [-0.5, 0.5] trust delta.
    // Midpoint at 0.5 cohesion → neutral.
    cohesion.clamp(0.0, 1.0) - 0.5
}

/// Convert jantu territorial aggression response into a bhava mood shift.
///
/// Territorial encounters trigger dominance and frustration responses.
/// The response value comes from [`jantu::territory::territorial_response`].
///
/// ```
/// use bhava::compat::mood_from_territorial;
///
/// let strong_defense = mood_from_territorial(0.9);
/// assert!(strong_defense.dominance > 0.0);
/// assert!(strong_defense.frustration > 0.0);
///
/// let weak_response = mood_from_territorial(0.1);
/// assert!(weak_response.dominance < strong_defense.dominance);
/// ```
#[must_use]
#[inline]
pub fn mood_from_territorial(response_intensity: f32) -> MoodVector {
    let r = response_intensity.clamp(0.0, 1.0);
    MoodVector {
        joy: -0.1 * r,
        arousal: 0.5 * r,
        dominance: 0.6 * r,
        trust: -0.2 * r,
        interest: 0.2 * r,
        frustration: 0.4 * r,
    }
}

// ── Learning & Memory bridges (v1.2.0) ──────────────────────────────────

/// Convert jantu habituation response multiplier to bhava emotional reactivity.
///
/// Habituated stimuli produce dampened emotional responses; sensitized stimuli
/// produce amplified responses. This scalar can be applied to mood shift
/// magnitudes before feeding them into bhava's mood system.
///
/// Returns a multiplier in [0.0, 2.0]:
/// - < 1.0 → habituated (dampened emotional response)
/// - 1.0 → neutral
/// - > 1.0 → sensitized (amplified emotional response)
///
/// ```
/// use jantu::habituation::{StimulusResponse, HabituationParams};
/// use bhava::compat::reactivity_from_habituation;
///
/// let fresh = StimulusResponse::new();
/// assert!((reactivity_from_habituation(&fresh) - 1.0).abs() < 0.01);
///
/// let params = HabituationParams::default();
/// let mut habituated = StimulusResponse::new();
/// for _ in 0..20 {
///     habituated.expose(0.3, &params);
/// }
/// assert!(reactivity_from_habituation(&habituated) < 1.0);
/// ```
#[must_use]
#[inline]
pub fn reactivity_from_habituation(response: &jantu::habituation::StimulusResponse) -> f32 {
    response.response_multiplier().clamp(0.0, 2.0)
}

/// Seed data from a jantu memory trace for bhava's ACT-R activation system.
///
/// Maps jantu's biological memory (food sources, threats, individuals) into
/// values suitable for creating or reinforcing entries in bhava's
/// [`ActivationStore`](crate::actr::ActivationStore).
///
/// Returns `(valence, strength)` where:
/// - `valence` is in [-1.0, 1.0] (emotional coloring of the memory)
/// - `strength` is in [0.0, 1.0] (how accessible the memory is)
///
/// ```
/// use jantu::memory::{MemoryTrace, MemoryType};
/// use bhava::compat::actr_seed_from_memory;
///
/// let threat = MemoryTrace::new(MemoryType::Threat, 0.9, -0.8);
/// let (valence, strength) = actr_seed_from_memory(&threat);
/// assert!(valence < 0.0);  // negative memory
/// assert!(strength > 0.5); // strong trace
/// ```
#[must_use]
#[inline]
pub fn actr_seed_from_memory(trace: &jantu::memory::MemoryTrace) -> (f32, f32) {
    (
        trace.valence.clamp(-1.0, 1.0),
        trace.strength.clamp(0.0, 1.0),
    )
}

// ── Environment & Body bridges (v1.2.0) ─────────────────────────────────

/// Convert jantu landscape perceived risk into a bhava stress accumulation input.
///
/// High perceived risk from the landscape of fear maps to stress load that
/// can be fed into bhava's [`StressState`](crate::stress::StressState).
///
/// Returns a stress input in [0.0, 1.0] suitable for adjusting
/// `StressState.load` or `accumulation_rate`.
///
/// ```
/// use bhava::compat::stress_from_landscape;
///
/// let dangerous = stress_from_landscape(0.9);
/// let safe = stress_from_landscape(0.1);
/// assert!(dangerous > safe);
/// assert!(dangerous > 0.5);
/// ```
#[must_use]
#[inline]
pub fn stress_from_landscape(perceived_risk: f32) -> f32 {
    // Nonlinear mapping: low risk is mostly ignored, high risk escalates fast.
    // Quadratic curve so background danger doesn't cause constant stress.
    let r = perceived_risk.clamp(0.0, 1.0);
    (r * r).clamp(0.0, 1.0)
}

/// Estimate bhava energy exertion from active jantu instinct drives.
///
/// Maps total active drive pressure to an exertion value for bhava's
/// [`EnergyState::tick`](crate::energy::EnergyState). More urgent drives
/// mean the creature is spending more energy on survival behaviors.
///
/// Returns exertion in [0.0, 1.0].
///
/// ```
/// use jantu::instinct::{Instinct, InstinctType, DriveLevel};
/// use bhava::compat::energy_drain_from_drives;
///
/// let mut fear = Instinct::new(InstinctType::Fear);
/// fear.drive = DriveLevel::new(0.9);
/// fear.update_priority();
///
/// let mut rest = Instinct::new(InstinctType::Rest);
/// rest.drive = DriveLevel::new(0.8);
/// rest.update_priority();
///
/// let active = energy_drain_from_drives(&[fear]);
/// let resting = energy_drain_from_drives(&[rest]);
/// assert!(active > resting);
/// ```
#[must_use]
pub fn energy_drain_from_drives(instincts: &[jantu::Instinct]) -> f32 {
    if instincts.is_empty() {
        return 0.0;
    }
    // Rest drive contributes negative exertion (recovery).
    // All other drives contribute proportional to their priority.
    let mut total = 0.0_f32;
    let mut count = 0_u32;
    for inst in instincts {
        let p = inst.priority.clamp(0.0, 1.0);
        if matches!(inst.instinct_type, jantu::InstinctType::Rest) {
            // Rest reduces exertion — active rest drive means the creature
            // is trying to conserve energy.
            total -= p * 0.3;
        } else {
            total += p;
        }
        count += 1;
    }
    // Average across drives, clamped to valid exertion range.
    (total / count as f32).clamp(0.0, 1.0)
}

/// Convert jantu circadian activity level to a bhava alertness value.
///
/// Bridges jantu's [`CircadianClock::activity_level`](jantu::circadian::CircadianClock::activity_level)
/// output to a value compatible with bhava's circadian system.
///
/// Returns alertness in [0.0, 1.0] that can modulate bhava's
/// mood decay rate and energy recovery.
///
/// ```
/// use jantu::circadian::{CircadianClock, ActivityPattern};
/// use bhava::compat::alertness_from_activity;
///
/// let clock = CircadianClock::new(ActivityPattern::Diurnal);
/// let midday = alertness_from_activity(clock.activity_level(12.0));
/// let midnight = alertness_from_activity(clock.activity_level(0.0));
/// assert!(midday > midnight);
/// ```
#[must_use]
#[inline]
pub fn alertness_from_activity(activity_level: f32) -> f32 {
    // Direct mapping — jantu activity [0.0, 1.0] maps to bhava alertness [0.0, 1.0].
    // Apply a slight sigmoid to soften extremes.
    let a = activity_level.clamp(0.0, 1.0);
    // Smoothstep: 3a² - 2a³ — keeps 0 and 1 fixed, smooths transitions.
    a * a * (3.0 - 2.0 * a)
}

// ── Genetics → Personality bridge (v1.2.0) ──────────────────────────────

/// Trait seed values derived from a jantu behavioral genome.
///
/// Each field is a raw [-1.0, 1.0] value that can be quantized to a
/// [`TraitLevel`](crate::traits::TraitLevel) for building a
/// [`PersonalityProfile`](crate::traits::PersonalityProfile).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraitSeeds {
    /// Warmth — derived from sociability (social animals are warmer).
    pub warmth: f32,
    /// Empathy — derived from sociability with nurturing contribution.
    pub empathy: f32,
    /// Patience — inverse of aggression (aggressive animals are impatient).
    pub patience: f32,
    /// Confidence — derived from boldness.
    pub confidence: f32,
    /// Curiosity — direct mapping from exploration drive.
    pub curiosity: f32,
    /// Risk tolerance — derived from boldness with aggression contribution.
    pub risk_tolerance: f32,
    /// Directness — derived from aggression (assertive communication style).
    pub directness: f32,
}

/// Convert a jantu behavioral genome into bhava personality trait seeds.
///
/// Maps jantu's 5-axis behavioral genome (aggression, boldness, sociability,
/// activity, exploration) to 7 of bhava's 15 personality dimensions. The
/// remaining 8 traits (Formality, Humor, Verbosity, Skepticism, Autonomy,
/// Pedagogy, Precision, Creativity) are higher-cognitive and have no
/// animal instinct basis — they default to `Balanced` and must be set by
/// other systems.
///
/// ```
/// use jantu::genetics::{BehavioralGenome, HeritableTrait};
/// use bhava::compat::trait_seeds_from_genome;
///
/// let bold_social = BehavioralGenome {
///     aggression: HeritableTrait::new(0.3, 0.4),
///     boldness: HeritableTrait::new(0.8, 0.35),
///     sociability: HeritableTrait::new(0.9, 0.3),
///     activity: HeritableTrait::new(0.5, 0.45),
///     exploration: HeritableTrait::new(0.7, 0.3),
/// };
/// let seeds = trait_seeds_from_genome(&bold_social);
/// assert!(seeds.confidence > 0.0);
/// assert!(seeds.warmth > 0.0);
/// assert!(seeds.curiosity > 0.0);
/// ```
#[must_use]
pub fn trait_seeds_from_genome(genome: &jantu::genetics::BehavioralGenome) -> TraitSeeds {
    // Use genotype values — the raw genetic potential, not environmentally expressed.
    // Normalize genome axes from [0.0, 1.0] to [-1.0, 1.0] for trait space.
    let agg = genome.aggression.genotype.clamp(0.0, 1.0) * 2.0 - 1.0;
    let bold = genome.boldness.genotype.clamp(0.0, 1.0) * 2.0 - 1.0;
    let soc = genome.sociability.genotype.clamp(0.0, 1.0) * 2.0 - 1.0;
    let expl = genome.exploration.genotype.clamp(0.0, 1.0) * 2.0 - 1.0;

    TraitSeeds {
        warmth: (soc * 0.8).clamp(-1.0, 1.0),
        empathy: (soc * 0.6 + (1.0 - agg.abs()) * 0.4).clamp(-1.0, 1.0),
        patience: (-agg * 0.7).clamp(-1.0, 1.0),
        confidence: (bold * 0.8).clamp(-1.0, 1.0),
        curiosity: (expl * 0.9).clamp(-1.0, 1.0),
        risk_tolerance: (bold * 0.6 + agg * 0.3).clamp(-1.0, 1.0),
        directness: (agg * 0.5 + bold * 0.3).clamp(-1.0, 1.0),
    }
}

// ── Signal bridge (v1.2.0) ──────────────────────────────────────────────

/// Convert a received jantu signal into a bhava mood shift.
///
/// Maps jantu signal functions (Alarm, MatingCall, Submission, Threat, etc.)
/// to mood dimensions. The signal's intensity and honesty modulate the
/// magnitude — dishonest signals produce weaker effects.
///
/// ```
/// use jantu::signals::{Signal, SignalModality, SignalFunction};
/// use bhava::compat::mood_from_signal;
///
/// let alarm = Signal::new(SignalModality::Acoustic, SignalFunction::Alarm, 0.9);
/// let mood = mood_from_signal(&alarm);
/// assert!(mood.arousal > 0.0);
/// assert!(mood.trust < 0.0);
/// ```
#[must_use]
pub fn mood_from_signal(signal: &jantu::signals::Signal) -> MoodVector {
    // Effective intensity: raw intensity modulated by honesty.
    // Dishonest signals (bluffs, false alarms) have reduced impact.
    let eff = (signal.intensity * (0.5 + signal.honesty * 0.5)).clamp(0.0, 1.0);

    match signal.function {
        jantu::signals::SignalFunction::Alarm => MoodVector {
            joy: -0.3 * eff,
            arousal: 0.8 * eff,
            dominance: -0.2 * eff,
            trust: -0.2 * eff,
            interest: 0.3 * eff,
            frustration: 0.1 * eff,
        },
        jantu::signals::SignalFunction::MatingCall => MoodVector {
            joy: 0.3 * eff,
            arousal: 0.4 * eff,
            dominance: 0.0,
            trust: 0.2 * eff,
            interest: 0.5 * eff,
            frustration: -0.1 * eff,
        },
        jantu::signals::SignalFunction::TerritorialDisplay => MoodVector {
            joy: -0.1 * eff,
            arousal: 0.5 * eff,
            dominance: 0.4 * eff,
            trust: -0.3 * eff,
            interest: 0.2 * eff,
            frustration: 0.3 * eff,
        },
        jantu::signals::SignalFunction::Submission => MoodVector {
            joy: 0.1 * eff,
            arousal: -0.2 * eff,
            dominance: 0.5 * eff,
            trust: 0.3 * eff,
            interest: 0.0,
            frustration: -0.2 * eff,
        },
        jantu::signals::SignalFunction::Threat => MoodVector {
            joy: -0.4 * eff,
            arousal: 0.7 * eff,
            dominance: -0.3 * eff,
            trust: -0.5 * eff,
            interest: 0.1 * eff,
            frustration: 0.4 * eff,
        },
        jantu::signals::SignalFunction::Begging => MoodVector {
            joy: -0.1 * eff,
            arousal: 0.2 * eff,
            dominance: 0.2 * eff,
            trust: 0.1 * eff,
            interest: 0.1 * eff,
            frustration: 0.1 * eff,
        },
        jantu::signals::SignalFunction::Contact => MoodVector {
            joy: 0.2 * eff,
            arousal: 0.1 * eff,
            dominance: 0.0,
            trust: 0.3 * eff,
            interest: 0.2 * eff,
            frustration: -0.1 * eff,
        },
        jantu::signals::SignalFunction::FoodCall => MoodVector {
            joy: 0.3 * eff,
            arousal: 0.2 * eff,
            dominance: 0.0,
            trust: 0.2 * eff,
            interest: 0.4 * eff,
            frustration: -0.2 * eff,
        },
        _ => MoodVector {
            joy: 0.0,
            arousal: 0.1 * eff,
            dominance: 0.0,
            trust: 0.0,
            interest: 0.1 * eff,
            frustration: 0.0,
        },
    }
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Core bridge tests ───────────────────────────────────────────

    #[test]
    fn fight_response_high_arousal() {
        let mood = mood_from_threat_response(jantu::ThreatResponse::Fight);
        assert!(mood.arousal > 0.5);
        assert!(mood.dominance > 0.0);
    }

    #[test]
    fn flight_response_low_dominance() {
        let mood = mood_from_threat_response(jantu::ThreatResponse::Flight);
        assert!(mood.dominance < 0.0);
        assert!(mood.arousal > 0.5);
    }

    #[test]
    fn freeze_response_lowest_dominance() {
        let mood = mood_from_threat_response(jantu::ThreatResponse::Freeze);
        assert!(mood.dominance < -0.5);
    }

    #[test]
    fn fawn_response_slight_trust() {
        let mood = mood_from_threat_response(jantu::ThreatResponse::Fawn);
        assert!(mood.trust > 0.0);
    }

    #[test]
    fn unstressed_low_load() {
        let s = jantu::stress::StressState::new();
        assert!(load_from_stress(&s) < 0.01);
    }

    #[test]
    fn chronic_stress_high_load() {
        let mut s = jantu::stress::StressState::new();
        for _ in 0..20 {
            s.apply_stressor(0.7);
        }
        assert!(load_from_stress(&s) > 0.1);
    }

    #[test]
    fn fear_instinct_maps_to_arousal() {
        let mut fear = jantu::Instinct::new(jantu::InstinctType::Fear);
        fear.drive = jantu::DriveLevel::new(0.8);
        fear.update_priority();
        let (emotion, mag) = mood_shift_from_instinct(&fear);
        assert_eq!(emotion, Emotion::Arousal);
        assert!(mag > 0.5);
    }

    #[test]
    fn curiosity_instinct_maps_to_interest() {
        let mut curiosity = jantu::Instinct::new(jantu::InstinctType::Curiosity);
        curiosity.drive = jantu::DriveLevel::new(0.9);
        curiosity.update_priority();
        let (emotion, _) = mood_shift_from_instinct(&curiosity);
        assert_eq!(emotion, Emotion::Interest);
    }

    #[test]
    fn alpha_positive_dominance() {
        assert!(dominance_from_rank(jantu::HierarchyPosition::new(0.9)) > 0.0);
    }

    #[test]
    fn omega_negative_dominance() {
        assert!(dominance_from_rank(jantu::HierarchyPosition::new(0.1)) < 0.0);
    }

    #[test]
    fn instinct_layer_score_high_fear() {
        let mut fear = jantu::Instinct::new(jantu::InstinctType::Fear);
        fear.drive = jantu::DriveLevel::new(0.9);
        fear.update_priority();
        let score = instinct_layer_score(&[fear]);
        assert!(score > 0.7);
    }

    #[test]
    fn instinct_layer_score_empty() {
        assert_eq!(instinct_layer_score(&[]), 0.0);
    }

    #[test]
    fn serde_roundtrip_mood_from_threat() {
        let mood = mood_from_threat_response(jantu::ThreatResponse::Fight);
        let json = serde_json::to_string(&mood).unwrap();
        let mood2: MoodVector = serde_json::from_str(&json).unwrap();
        assert!((mood.arousal - mood2.arousal).abs() < f32::EPSILON);
    }

    // ── Contagion & Social tests ────────────────────────────────────

    #[test]
    fn fear_contagion_raises_arousal() {
        let mood = mood_from_contagion(jantu::contagion::EmotionalState::Fear, 0.8);
        assert!(mood.arousal > 0.3);
        assert!(mood.trust < 0.0);
    }

    #[test]
    fn calm_contagion_positive_trust() {
        let mood = mood_from_contagion(jantu::contagion::EmotionalState::Calm, 0.7);
        assert!(mood.trust > 0.0);
        assert!(mood.arousal < 0.0);
    }

    #[test]
    fn excitement_contagion_joy_and_interest() {
        let mood = mood_from_contagion(jantu::contagion::EmotionalState::Excitement, 0.9);
        assert!(mood.joy > 0.0);
        assert!(mood.interest > 0.0);
    }

    #[test]
    fn aggression_contagion_high_frustration() {
        let mood = mood_from_contagion(jantu::contagion::EmotionalState::Aggression, 0.8);
        assert!(mood.frustration > 0.3);
    }

    #[test]
    fn zero_magnitude_contagion_is_neutral() {
        let mood = mood_from_contagion(jantu::contagion::EmotionalState::Fear, 0.0);
        assert!(mood.arousal.abs() < f32::EPSILON);
        assert!(mood.joy.abs() < f32::EPSILON);
    }

    #[test]
    fn high_cohesion_positive_trust() {
        assert!(trust_from_cohesion(0.9) > 0.0);
    }

    #[test]
    fn low_cohesion_negative_trust() {
        assert!(trust_from_cohesion(0.1) < 0.0);
    }

    #[test]
    fn mid_cohesion_neutral_trust() {
        assert!(trust_from_cohesion(0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn strong_territorial_high_dominance() {
        let mood = mood_from_territorial(0.9);
        assert!(mood.dominance > 0.4);
        assert!(mood.frustration > 0.3);
    }

    #[test]
    fn weak_territorial_low_effect() {
        let mood = mood_from_territorial(0.1);
        assert!(mood.dominance < 0.1);
    }

    // ── Learning & Memory tests ─────────────────────────────────────

    #[test]
    fn fresh_stimulus_neutral_reactivity() {
        let fresh = jantu::habituation::StimulusResponse::new();
        let r = reactivity_from_habituation(&fresh);
        assert!((r - 1.0).abs() < 0.01);
    }

    #[test]
    fn habituated_stimulus_reduced_reactivity() {
        let params = jantu::habituation::HabituationParams::default();
        let mut resp = jantu::habituation::StimulusResponse::new();
        for _ in 0..30 {
            resp.expose(0.3, &params);
        }
        assert!(reactivity_from_habituation(&resp) < 1.0);
    }

    #[test]
    fn threat_memory_negative_valence() {
        let threat = jantu::memory::MemoryTrace::new(jantu::memory::MemoryType::Threat, 0.9, -0.8);
        let (valence, strength) = actr_seed_from_memory(&threat);
        assert!(valence < 0.0);
        assert!(strength > 0.5);
    }

    #[test]
    fn food_memory_positive_valence() {
        let food = jantu::memory::MemoryTrace::new(jantu::memory::MemoryType::FoodSource, 0.7, 0.6);
        let (valence, _) = actr_seed_from_memory(&food);
        assert!(valence > 0.0);
    }

    // ── Environment & Body tests ────────────────────────────────────

    #[test]
    fn high_risk_high_stress() {
        assert!(stress_from_landscape(0.9) > 0.5);
    }

    #[test]
    fn low_risk_low_stress() {
        assert!(stress_from_landscape(0.1) < 0.1);
    }

    #[test]
    fn stress_from_landscape_quadratic() {
        // Verify nonlinear: doubling risk more than doubles stress.
        let low = stress_from_landscape(0.3);
        let high = stress_from_landscape(0.6);
        assert!(high > low * 2.0);
    }

    #[test]
    fn fear_drive_high_exertion() {
        let mut fear = jantu::Instinct::new(jantu::InstinctType::Fear);
        fear.drive = jantu::DriveLevel::new(0.9);
        fear.update_priority();
        let exertion = energy_drain_from_drives(&[fear]);
        assert!(exertion > 0.3);
    }

    #[test]
    fn rest_drive_low_exertion() {
        let mut rest = jantu::Instinct::new(jantu::InstinctType::Rest);
        rest.drive = jantu::DriveLevel::new(0.9);
        rest.update_priority();
        let exertion = energy_drain_from_drives(&[rest]);
        assert!(exertion < 0.1);
    }

    #[test]
    fn empty_drives_zero_exertion() {
        assert_eq!(energy_drain_from_drives(&[]), 0.0);
    }

    #[test]
    fn diurnal_midday_more_alert_than_midnight() {
        let clock =
            jantu::circadian::CircadianClock::new(jantu::circadian::ActivityPattern::Diurnal);
        let midday = alertness_from_activity(clock.activity_level(12.0));
        let midnight = alertness_from_activity(clock.activity_level(0.0));
        assert!(midday > midnight);
    }

    #[test]
    fn alertness_smoothstep_bounds() {
        assert!((alertness_from_activity(0.0)).abs() < f32::EPSILON);
        assert!((alertness_from_activity(1.0) - 1.0).abs() < f32::EPSILON);
    }

    // ── Genetics → Personality tests ────────────────────────────────

    fn genome(
        agg: f32,
        bold: f32,
        soc: f32,
        act: f32,
        expl: f32,
    ) -> jantu::genetics::BehavioralGenome {
        use jantu::genetics::HeritableTrait;
        jantu::genetics::BehavioralGenome {
            aggression: HeritableTrait::new(agg, 0.4),
            boldness: HeritableTrait::new(bold, 0.35),
            sociability: HeritableTrait::new(soc, 0.3),
            activity: HeritableTrait::new(act, 0.45),
            exploration: HeritableTrait::new(expl, 0.3),
        }
    }

    #[test]
    fn social_genome_warm_personality() {
        let g = genome(0.2, 0.5, 0.9, 0.5, 0.5);
        let seeds = trait_seeds_from_genome(&g);
        assert!(seeds.warmth > 0.0);
        assert!(seeds.empathy > 0.0);
    }

    #[test]
    fn aggressive_genome_low_patience() {
        let g = genome(0.9, 0.5, 0.3, 0.5, 0.5);
        let seeds = trait_seeds_from_genome(&g);
        assert!(seeds.patience < 0.0);
        assert!(seeds.directness > 0.0);
    }

    #[test]
    fn bold_explorer_curious_confident() {
        let g = genome(0.3, 0.9, 0.5, 0.7, 0.9);
        let seeds = trait_seeds_from_genome(&g);
        assert!(seeds.confidence > 0.0);
        assert!(seeds.curiosity > 0.0);
        assert!(seeds.risk_tolerance > 0.0);
    }

    #[test]
    fn trait_seeds_bounded() {
        let g = genome(1.0, 1.0, 1.0, 1.0, 1.0);
        let seeds = trait_seeds_from_genome(&g);
        assert!(seeds.warmth >= -1.0 && seeds.warmth <= 1.0);
        assert!(seeds.empathy >= -1.0 && seeds.empathy <= 1.0);
        assert!(seeds.patience >= -1.0 && seeds.patience <= 1.0);
        assert!(seeds.confidence >= -1.0 && seeds.confidence <= 1.0);
        assert!(seeds.curiosity >= -1.0 && seeds.curiosity <= 1.0);
        assert!(seeds.risk_tolerance >= -1.0 && seeds.risk_tolerance <= 1.0);
        assert!(seeds.directness >= -1.0 && seeds.directness <= 1.0);
    }

    // ── Signal tests ────────────────────────────────────────────────

    #[test]
    fn alarm_signal_raises_arousal() {
        let alarm = jantu::signals::Signal::new(
            jantu::signals::SignalModality::Acoustic,
            jantu::signals::SignalFunction::Alarm,
            0.9,
        );
        let mood = mood_from_signal(&alarm);
        assert!(mood.arousal > 0.3);
    }

    #[test]
    fn submission_signal_raises_receiver_dominance() {
        let sub = jantu::signals::Signal::new(
            jantu::signals::SignalModality::Visual,
            jantu::signals::SignalFunction::Submission,
            0.8,
        );
        let mood = mood_from_signal(&sub);
        assert!(mood.dominance > 0.0);
        assert!(mood.trust > 0.0);
    }

    #[test]
    fn threat_signal_negative_trust() {
        let threat = jantu::signals::Signal::new(
            jantu::signals::SignalModality::Acoustic,
            jantu::signals::SignalFunction::Threat,
            0.9,
        );
        let mood = mood_from_signal(&threat);
        assert!(mood.trust < 0.0);
        assert!(mood.frustration > 0.0);
    }

    #[test]
    fn contact_signal_positive_trust() {
        let contact = jantu::signals::Signal::new(
            jantu::signals::SignalModality::Tactile,
            jantu::signals::SignalFunction::Contact,
            0.7,
        );
        let mood = mood_from_signal(&contact);
        assert!(mood.trust > 0.0);
        assert!(mood.joy > 0.0);
    }

    #[test]
    fn food_call_positive_mood() {
        let food = jantu::signals::Signal::new(
            jantu::signals::SignalModality::Acoustic,
            jantu::signals::SignalFunction::FoodCall,
            0.8,
        );
        let mood = mood_from_signal(&food);
        assert!(mood.joy > 0.0);
        assert!(mood.interest > 0.0);
    }

    #[test]
    fn dishonest_signal_reduced_effect() {
        let mut honest = jantu::signals::Signal::new(
            jantu::signals::SignalModality::Acoustic,
            jantu::signals::SignalFunction::Alarm,
            0.9,
        );
        // Default honesty is 1.0 per Signal::new
        let honest_mood = mood_from_signal(&honest);

        honest.honesty = 0.1;
        let dishonest_mood = mood_from_signal(&honest);

        assert!(honest_mood.arousal > dishonest_mood.arousal);
    }

    #[test]
    fn serde_roundtrip_trait_seeds() {
        let genome = jantu::genetics::BehavioralGenome::default_genome();
        let seeds = trait_seeds_from_genome(&genome);
        let json = serde_json::to_string(&seeds).unwrap();
        let seeds2: TraitSeeds = serde_json::from_str(&json).unwrap();
        assert!((seeds.warmth - seeds2.warmth).abs() < f32::EPSILON);
    }
}
