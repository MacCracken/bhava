//! Zodiac manifestation engine — celestial placements mapped to personality.
//!
//! Maps zodiac signs to bhava's existing trait system. Each sign produces
//! a [`PersonalityProfile`] with trait levels derived from astrological tradition.
//!
//! # Elements
//!
//! The four elements govern which trait group a sign emphasizes:
//! - **Fire** (Aries, Leo, Sagittarius) — confidence, drive, risk-taking
//! - **Water** (Cancer, Scorpio, Pisces) — empathy, warmth, emotional depth
//! - **Earth** (Taurus, Virgo, Capricorn) — precision, patience, discipline
//! - **Air** (Gemini, Libra, Aquarius) — curiosity, humor, independence
//!
//! # Modalities
//!
//! The three modalities describe behavioral rhythm:
//! - **Cardinal** (Aries, Cancer, Libra, Capricorn) — initiators, direct action
//! - **Fixed** (Taurus, Leo, Scorpio, Aquarius) — persistent, deep focus
//! - **Mutable** (Gemini, Virgo, Sagittarius, Pisces) — adaptive, context-switching
//!
//! # Examples
//!
//! ```
//! use bhava::zodiac::{ZodiacSign, Element, Modality, sign_element, sign_modality, sign_profile};
//!
//! let sign = ZodiacSign::Scorpio;
//! assert_eq!(sign_element(sign), Element::Water);
//! assert_eq!(sign_modality(sign), Modality::Fixed);
//!
//! let profile = sign_profile(sign);
//! assert_eq!(profile.name, "Scorpio");
//! ```

use serde::{Deserialize, Serialize};

use crate::traits::{PersonalityProfile, TraitKind, TraitLevel};

#[cfg(feature = "mood")]
use crate::mood::MoodVector;

/// The twelve zodiac signs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ZodiacSign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl ZodiacSign {
    /// All signs in traditional order.
    pub const ALL: &'static [ZodiacSign] = &[
        Self::Aries,
        Self::Taurus,
        Self::Gemini,
        Self::Cancer,
        Self::Leo,
        Self::Virgo,
        Self::Libra,
        Self::Scorpio,
        Self::Sagittarius,
        Self::Capricorn,
        Self::Aquarius,
        Self::Pisces,
    ];

    /// Number of zodiac signs.
    pub const COUNT: usize = 12;
}

impl_display!(ZodiacSign {
    Aries => "Aries",
    Taurus => "Taurus",
    Gemini => "Gemini",
    Cancer => "Cancer",
    Leo => "Leo",
    Virgo => "Virgo",
    Libra => "Libra",
    Scorpio => "Scorpio",
    Sagittarius => "Sagittarius",
    Capricorn => "Capricorn",
    Aquarius => "Aquarius",
    Pisces => "Pisces",
});

/// The four classical elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Element {
    /// Aries, Leo, Sagittarius — confidence, drive, assertiveness.
    Fire,
    /// Cancer, Scorpio, Pisces — empathy, warmth, emotional depth.
    Water,
    /// Taurus, Virgo, Capricorn — precision, patience, discipline.
    Earth,
    /// Gemini, Libra, Aquarius — curiosity, humor, independence.
    Air,
}

impl_display!(Element {
    Fire => "Fire",
    Water => "Water",
    Earth => "Earth",
    Air => "Air",
});

/// The three modalities (qualities).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Modality {
    /// Aries, Cancer, Libra, Capricorn — initiators, direct action.
    Cardinal,
    /// Taurus, Leo, Scorpio, Aquarius — persistent, deep focus.
    Fixed,
    /// Gemini, Virgo, Sagittarius, Pisces — adaptive, context-switching.
    Mutable,
}

impl_display!(Modality {
    Cardinal => "Cardinal",
    Fixed => "Fixed",
    Mutable => "Mutable",
});

// ── Classification ────────────────────────────────────────────────────────

/// Return the element for a zodiac sign.
#[must_use]
#[inline]
pub fn sign_element(sign: ZodiacSign) -> Element {
    match sign {
        ZodiacSign::Aries | ZodiacSign::Leo | ZodiacSign::Sagittarius => Element::Fire,
        ZodiacSign::Cancer | ZodiacSign::Scorpio | ZodiacSign::Pisces => Element::Water,
        ZodiacSign::Taurus | ZodiacSign::Virgo | ZodiacSign::Capricorn => Element::Earth,
        ZodiacSign::Gemini | ZodiacSign::Libra | ZodiacSign::Aquarius => Element::Air,
    }
}

/// Return the modality for a zodiac sign.
#[must_use]
#[inline]
pub fn sign_modality(sign: ZodiacSign) -> Modality {
    match sign {
        ZodiacSign::Aries | ZodiacSign::Cancer | ZodiacSign::Libra | ZodiacSign::Capricorn => {
            Modality::Cardinal
        }
        ZodiacSign::Taurus | ZodiacSign::Leo | ZodiacSign::Scorpio | ZodiacSign::Aquarius => {
            Modality::Fixed
        }
        ZodiacSign::Gemini | ZodiacSign::Virgo | ZodiacSign::Sagittarius | ZodiacSign::Pisces => {
            Modality::Mutable
        }
    }
}

// ── Sign Profiles ─────────────────────────────────────────────────────────

/// Produce a personality profile for a zodiac sign.
///
/// Each sign maps to a distinct configuration of bhava's 15 personality
/// traits. Element determines the dominant trait group; the individual sign
/// determines variation within the element.
///
/// # Examples
///
/// ```
/// use bhava::zodiac::{ZodiacSign, sign_profile};
/// use bhava::traits::TraitKind;
///
/// let aries = sign_profile(ZodiacSign::Aries);
/// assert_eq!(aries.get_trait(TraitKind::Confidence), bhava::traits::TraitLevel::Highest);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument)]
#[must_use]
pub fn sign_profile(sign: ZodiacSign) -> PersonalityProfile {
    use TraitKind::*;
    use TraitLevel::*;

    let mut p = PersonalityProfile::new(sign.to_string());

    match sign {
        // ── Fire ──────────────────────────────────────────────────────
        ZodiacSign::Aries => {
            // The Ram — bold, direct, impatient pioneer
            p.set_trait(Directness, Highest);
            p.set_trait(Confidence, Highest);
            p.set_trait(RiskTolerance, High);
            p.set_trait(Curiosity, High);
            p.set_trait(Autonomy, High);
            p.set_trait(Patience, Low);
            p.set_trait(Empathy, Low);
        }
        ZodiacSign::Leo => {
            // The Lion — charismatic, warm, commanding leader
            p.set_trait(Confidence, Highest);
            p.set_trait(Warmth, High);
            p.set_trait(Creativity, High);
            p.set_trait(Pedagogy, High);
            p.set_trait(Directness, High);
            p.set_trait(Humor, High);
            p.set_trait(Skepticism, Low);
        }
        ZodiacSign::Sagittarius => {
            // The Archer — adventurous, philosophical, restless seeker
            p.set_trait(Curiosity, Highest);
            p.set_trait(RiskTolerance, High);
            p.set_trait(Humor, High);
            p.set_trait(Creativity, High);
            p.set_trait(Confidence, High);
            p.set_trait(Patience, Low);
            p.set_trait(Precision, Low);
        }

        // ── Water ─────────────────────────────────────────────────────
        ZodiacSign::Cancer => {
            // The Crab — nurturing, protective, emotionally deep
            p.set_trait(Empathy, Highest);
            p.set_trait(Warmth, Highest);
            p.set_trait(Patience, High);
            p.set_trait(Pedagogy, High);
            p.set_trait(RiskTolerance, Low);
            p.set_trait(Directness, Low);
        }
        ZodiacSign::Scorpio => {
            // The Scorpion — intense, perceptive, transformative
            p.set_trait(Skepticism, Highest);
            p.set_trait(Confidence, High);
            p.set_trait(Empathy, High);
            p.set_trait(Precision, High);
            p.set_trait(Autonomy, High);
            p.set_trait(Humor, Low);
            p.set_trait(Verbosity, Low);
        }
        ZodiacSign::Pisces => {
            // The Fish — imaginative, compassionate, fluid
            p.set_trait(Empathy, Highest);
            p.set_trait(Creativity, Highest);
            p.set_trait(Warmth, High);
            p.set_trait(Curiosity, High);
            p.set_trait(Precision, Low);
            p.set_trait(Directness, Low);
        }

        // ── Earth ─────────────────────────────────────────────────────
        ZodiacSign::Taurus => {
            // The Bull — patient, sensual, steadfast
            p.set_trait(Patience, Highest);
            p.set_trait(Precision, High);
            p.set_trait(Warmth, High);
            p.set_trait(Autonomy, High);
            p.set_trait(RiskTolerance, Low);
            p.set_trait(Curiosity, Low);
        }
        ZodiacSign::Virgo => {
            // The Maiden — analytical, meticulous, service-oriented
            p.set_trait(Precision, Highest);
            p.set_trait(Skepticism, High);
            p.set_trait(Pedagogy, High);
            p.set_trait(Patience, High);
            p.set_trait(RiskTolerance, Low);
            p.set_trait(Confidence, Low);
        }
        ZodiacSign::Capricorn => {
            // The Sea-Goat — disciplined, ambitious, strategic
            p.set_trait(Autonomy, Highest);
            p.set_trait(Confidence, High);
            p.set_trait(Precision, High);
            p.set_trait(Patience, High);
            p.set_trait(Humor, Low);
            p.set_trait(Warmth, Low);
        }

        // ── Air ───────────────────────────────────────────────────────
        ZodiacSign::Gemini => {
            // The Twins — mercurial, witty, endlessly curious
            p.set_trait(Curiosity, Highest);
            p.set_trait(Humor, Highest);
            p.set_trait(Creativity, High);
            p.set_trait(Verbosity, High);
            p.set_trait(Patience, Low);
            p.set_trait(Precision, Low);
        }
        ZodiacSign::Libra => {
            // The Scales — harmonious, diplomatic, relationship-focused
            p.set_trait(Warmth, Highest);
            p.set_trait(Empathy, High);
            p.set_trait(Humor, High);
            p.set_trait(Patience, High);
            p.set_trait(Directness, Low);
            p.set_trait(Autonomy, Low);
        }
        ZodiacSign::Aquarius => {
            // The Water Bearer — independent, visionary, unconventional
            p.set_trait(Autonomy, Highest);
            p.set_trait(Curiosity, Highest);
            p.set_trait(Creativity, High);
            p.set_trait(RiskTolerance, High);
            p.set_trait(Warmth, Low);
            p.set_trait(Patience, Low);
        }
    }

    p
}

// ── Planets ───────────────────────────────────────────────────────────────

/// Celestial bodies used in natal chart composition.
///
/// Each planet governs a specific bhava module:
/// - Inner planets (Sun–Venus) map to identity layers (Soul/Heart/Body/Brain/Spirit)
/// - Outer planets (Mars–Pluto) map to behavioral modules (energy/growth/stress/eq/appraisal/flow)
/// - Lunar nodes map to preference and memory patterns
/// - Chiron maps to regulation wounds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Planet {
    /// Core personality — maps to `traits` (Soul layer).
    Sun,
    /// Emotional baseline — maps to `mood` (Heart layer).
    Moon,
    /// Social presentation — maps to `display_rules` (Body layer).
    Rising,
    /// Communication and reasoning — maps to `reasoning` (Brain layer).
    Mercury,
    /// Passions and relationship approach — maps to `spirit` (Spirit layer).
    Venus,
    /// Drive intensity — maps to `energy`.
    Mars,
    /// Growth direction — maps to `growth`.
    Jupiter,
    /// Stress resistance — maps to `stress`.
    Saturn,
    /// Emotional intelligence — maps to `eq`.
    Neptune,
    /// Emotional intensity — maps to `appraisal`.
    Pluto,
    /// Flow sensitivity — maps to `flow`.
    Uranus,
    /// Long-term preference — maps to `preference`.
    NorthNode,
    /// Default activation patterns — maps to `actr`.
    SouthNode,
    /// Regulation wounds — maps to `regulation`.
    Chiron,
}

impl Planet {
    /// All planets in traditional order.
    pub const ALL: &'static [Planet] = &[
        Self::Sun,
        Self::Moon,
        Self::Rising,
        Self::Mercury,
        Self::Venus,
        Self::Mars,
        Self::Jupiter,
        Self::Saturn,
        Self::Neptune,
        Self::Pluto,
        Self::Uranus,
        Self::NorthNode,
        Self::SouthNode,
        Self::Chiron,
    ];

    /// Number of planets.
    pub const COUNT: usize = 14;

    /// Whether this is an inner (personal) planet.
    #[must_use]
    #[inline]
    pub fn is_inner(self) -> bool {
        matches!(
            self,
            Self::Sun | Self::Moon | Self::Rising | Self::Mercury | Self::Venus
        )
    }
}

impl_display!(Planet {
    Sun => "Sun",
    Moon => "Moon",
    Rising => "Rising",
    Mercury => "Mercury",
    Venus => "Venus",
    Mars => "Mars",
    Jupiter => "Jupiter",
    Saturn => "Saturn",
    Neptune => "Neptune",
    Pluto => "Pluto",
    Uranus => "Uranus",
    NorthNode => "North Node",
    SouthNode => "South Node",
    Chiron => "Chiron",
});

// ── Natal Chart ───────────────────────────────────────────────────────────

/// A natal chart — zodiac sign placements for each planet.
///
/// Build with the fluent API, then call [`manifest()`](Self::manifest)
/// to produce a [`ManifestedProfile`].
///
/// # Examples
///
/// ```
/// use bhava::zodiac::{NatalChart, ZodiacSign};
///
/// let chart = NatalChart::new()
///     .sun(ZodiacSign::Scorpio)
///     .moon(ZodiacSign::Cancer)
///     .rising(ZodiacSign::Gemini);
///
/// let profile = chart.manifest();
/// assert_eq!(profile.personality.name, "Scorpio");
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NatalChart {
    placements: [Option<ZodiacSign>; Planet::COUNT],
}

impl NatalChart {
    /// Create an empty chart with no placements.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a placement for any planet.
    #[must_use]
    pub fn place(mut self, planet: Planet, sign: ZodiacSign) -> Self {
        let idx = planet as usize;
        if idx < self.placements.len() {
            self.placements[idx] = Some(sign);
        }
        self
    }

    /// Get the sign placed for a planet, if any.
    #[must_use]
    #[inline]
    pub fn get(&self, planet: Planet) -> Option<ZodiacSign> {
        self.placements.get(planet as usize).copied().flatten()
    }

    /// Sun placement — core personality (Soul).
    #[must_use]
    pub fn sun(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Sun, sign)
    }

    /// Moon placement — emotional baseline (Heart).
    #[must_use]
    pub fn moon(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Moon, sign)
    }

    /// Rising (Ascendant) — social presentation (Body).
    #[must_use]
    pub fn rising(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Rising, sign)
    }

    /// Mercury — reasoning strategy (Brain).
    #[must_use]
    pub fn mercury(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Mercury, sign)
    }

    /// Venus — passions and aesthetics (Spirit).
    #[must_use]
    pub fn venus(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Venus, sign)
    }

    /// Mars — energy and drive.
    #[must_use]
    pub fn mars(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Mars, sign)
    }

    /// Jupiter — growth direction.
    #[must_use]
    pub fn jupiter(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Jupiter, sign)
    }

    /// Saturn — stress resistance.
    #[must_use]
    pub fn saturn(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Saturn, sign)
    }

    /// Neptune — emotional intelligence style.
    #[must_use]
    pub fn neptune(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Neptune, sign)
    }

    /// Pluto — emotional intensity.
    #[must_use]
    pub fn pluto(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Pluto, sign)
    }

    /// Uranus — flow sensitivity.
    #[must_use]
    pub fn uranus(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Uranus, sign)
    }

    /// North Node — long-term preference direction.
    #[must_use]
    pub fn north_node(self, sign: ZodiacSign) -> Self {
        self.place(Planet::NorthNode, sign)
    }

    /// South Node — default activation patterns.
    #[must_use]
    pub fn south_node(self, sign: ZodiacSign) -> Self {
        self.place(Planet::SouthNode, sign)
    }

    /// Chiron — regulation wounds.
    #[must_use]
    pub fn chiron(self, sign: ZodiacSign) -> Self {
        self.place(Planet::Chiron, sign)
    }

    /// Number of planets that have placements.
    #[must_use]
    pub fn placement_count(&self) -> usize {
        self.placements.iter().filter(|p| p.is_some()).count()
    }

    /// Manifest the chart into a full personality/emotion profile.
    ///
    /// Uses the Sun sign for the base personality. If no Sun placement
    /// exists, defaults to Aries.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    #[must_use]
    pub fn manifest(&self) -> ManifestedProfile {
        let sun_sign = self.get(Planet::Sun).unwrap_or(ZodiacSign::Aries);
        let personality = sign_profile(sun_sign);

        // Moon modifies mood baseline
        #[cfg(feature = "mood")]
        let mood_baseline = {
            let base = crate::mood::derive_mood_baseline(&personality);
            if let Some(moon_sign) = self.get(Planet::Moon) {
                moon_mood_modifier(moon_sign, base)
            } else {
                base
            }
        };

        // Mercury selects reasoning strategy
        #[cfg(all(feature = "mood", feature = "traits"))]
        let reasoning_strategy = {
            if let Some(mercury_sign) = self.get(Planet::Mercury) {
                mercury_reasoning_strategy(mercury_sign)
            } else {
                crate::reasoning::select_reasoning_strategy(&personality)
            }
        };

        // Venus shapes spirit (passions, inspirations, pains)
        #[cfg(feature = "archetype")]
        let spirit = {
            if let Some(venus_sign) = self.get(Planet::Venus) {
                venus_spirit(venus_sign)
            } else {
                crate::spirit::Spirit::new()
            }
        };

        // Rising shapes display rules
        #[cfg(feature = "mood")]
        let display_context = {
            if let Some(rising_sign) = self.get(Planet::Rising) {
                rising_display_context(rising_sign)
            } else {
                crate::display_rules::CulturalContext::new("default")
            }
        };

        // Mars modifies energy parameters
        #[cfg(feature = "mood")]
        let energy = {
            let mut e = crate::energy::EnergyState::new();
            if let Some(mars_sign) = self.get(Planet::Mars) {
                mars_energy_modifier(mars_sign, &mut e);
            }
            e
        };

        // Saturn modifies stress parameters
        #[cfg(feature = "mood")]
        let stress = {
            let mut s = crate::stress::StressState::new();
            if let Some(saturn_sign) = self.get(Planet::Saturn) {
                saturn_stress_modifier(saturn_sign, &mut s);
            }
            s
        };

        // Jupiter modifies growth parameters
        #[cfg(all(feature = "mood", feature = "traits"))]
        let growth = {
            let mut g = crate::growth::GrowthLedger::new();
            if let Some(jupiter_sign) = self.get(Planet::Jupiter) {
                jupiter_growth_modifier(jupiter_sign, &mut g);
            }
            g
        };

        // Neptune modifies EQ profile
        #[cfg(feature = "mood")]
        let eq = {
            let mut eq = crate::eq::EqProfile::new();
            if let Some(neptune_sign) = self.get(Planet::Neptune) {
                neptune_eq_modifier(neptune_sign, &mut eq);
            }
            eq
        };

        // Uranus modifies flow thresholds
        #[cfg(feature = "mood")]
        let flow = {
            let mut f = crate::flow::FlowState::default();
            if let Some(uranus_sign) = self.get(Planet::Uranus) {
                uranus_flow_modifier(uranus_sign, &mut f);
            }
            f
        };

        // North Node modifies preference bias
        #[cfg(feature = "mood")]
        let preference_bias = {
            let mut bias = crate::preference::PreferenceBias::neutral();
            if let Some(nn_sign) = self.get(Planet::NorthNode) {
                north_node_preference_modifier(nn_sign, &mut bias);
            }
            bias
        };

        // South Node modifies ACT-R memory parameters
        #[cfg(feature = "mood")]
        let (actr_decay, actr_recency_half_life) = {
            if let Some(sn_sign) = self.get(Planet::SouthNode) {
                south_node_actr_params(sn_sign)
            } else {
                (0.5, 300.0)
            }
        };

        ManifestedProfile {
            personality,
            #[cfg(feature = "mood")]
            mood_baseline,
            #[cfg(all(feature = "mood", feature = "traits"))]
            reasoning_strategy,
            #[cfg(feature = "archetype")]
            spirit,
            #[cfg(feature = "mood")]
            display_context,
            #[cfg(feature = "mood")]
            energy,
            #[cfg(feature = "mood")]
            stress,
            #[cfg(all(feature = "mood", feature = "traits"))]
            growth,
            #[cfg(feature = "mood")]
            eq,
            #[cfg(feature = "mood")]
            flow,
            #[cfg(feature = "mood")]
            preference_bias,
            #[cfg(feature = "mood")]
            actr_decay,
            #[cfg(feature = "mood")]
            actr_recency_half_life,
        }
    }
}

/// The output of manifesting a natal chart — personality and emotion configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestedProfile {
    /// Core personality from Sun sign.
    pub personality: PersonalityProfile,
    /// Mood baseline from personality + Moon sign modifier.
    #[cfg(feature = "mood")]
    pub mood_baseline: MoodVector,
    /// Preferred reasoning strategy from Mercury sign.
    #[cfg(all(feature = "mood", feature = "traits"))]
    pub reasoning_strategy: crate::reasoning::ReasoningStrategy,
    /// Spirit (passions, inspirations, pains) from Venus sign.
    #[cfg(feature = "archetype")]
    pub spirit: crate::spirit::Spirit,
    /// Display rules context from Rising sign.
    #[cfg(feature = "mood")]
    pub display_context: crate::display_rules::CulturalContext,
    /// Energy configuration from Mars sign.
    #[cfg(feature = "mood")]
    pub energy: crate::energy::EnergyState,
    /// Stress configuration from Saturn sign.
    #[cfg(feature = "mood")]
    pub stress: crate::stress::StressState,
    /// Growth configuration from Jupiter sign.
    #[cfg(all(feature = "mood", feature = "traits"))]
    pub growth: crate::growth::GrowthLedger,
    /// EQ profile from Neptune sign.
    #[cfg(feature = "mood")]
    pub eq: crate::eq::EqProfile,
    /// Flow state configuration from Uranus sign.
    #[cfg(feature = "mood")]
    pub flow: crate::flow::FlowState,
    /// Preference bias from North Node sign.
    #[cfg(feature = "mood")]
    pub preference_bias: crate::preference::PreferenceBias,
    /// ACT-R memory parameters from South Node sign.
    #[cfg(feature = "mood")]
    pub actr_decay: f64,
    /// ACT-R recency half-life from South Node sign.
    #[cfg(feature = "mood")]
    pub actr_recency_half_life: f64,
}

// ── Moon modifier ─────────────────────────────────────────────────────────

/// Modify a mood baseline based on the Moon sign placement.
///
/// The Moon governs emotional reactivity — how intensely and in which
/// direction the baseline mood leans.
#[cfg(feature = "mood")]
#[must_use]
fn moon_mood_modifier(moon: ZodiacSign, mut baseline: MoodVector) -> MoodVector {
    use crate::mood::Emotion;

    match sign_element(moon) {
        Element::Fire => {
            // Fire moons: elevated arousal, higher joy baseline
            baseline.nudge(Emotion::Arousal, 0.15);
            baseline.nudge(Emotion::Joy, 0.1);
            baseline.nudge(Emotion::Dominance, 0.1);
        }
        Element::Water => {
            // Water moons: deep emotional sensitivity, higher trust
            baseline.nudge(Emotion::Trust, 0.15);
            baseline.nudge(Emotion::Joy, 0.05);
            baseline.nudge(Emotion::Arousal, -0.1);
        }
        Element::Earth => {
            // Earth moons: stable, low arousal, grounded
            baseline.nudge(Emotion::Arousal, -0.15);
            baseline.nudge(Emotion::Trust, 0.1);
            baseline.nudge(Emotion::Frustration, -0.1);
        }
        Element::Air => {
            // Air moons: curious, mentally stimulated, variable
            baseline.nudge(Emotion::Interest, 0.15);
            baseline.nudge(Emotion::Arousal, 0.05);
            baseline.nudge(Emotion::Joy, 0.05);
        }
    }

    // Modality fine-tuning
    match sign_modality(moon) {
        Modality::Cardinal => baseline.nudge(Emotion::Dominance, 0.05),
        Modality::Fixed => baseline.nudge(Emotion::Trust, 0.05),
        Modality::Mutable => baseline.nudge(Emotion::Interest, 0.05),
    }

    baseline
}

// ── Mercury modifier ──────────────────────────────────────────────────────

/// Select a reasoning strategy based on the Mercury sign placement.
///
/// Mercury governs communication and cognition. The sign's modality
/// determines the primary reasoning approach:
/// - Cardinal signs → Analytical (initiating, direct)
/// - Fixed signs → Systematic (persistent, structured)
/// - Mutable signs → Intuitive (adaptive, fluid)
///
/// The element adds a secondary bias:
/// - Water Mercury → Empathetic tilt
/// - Air Mercury → Creative tilt
#[cfg(all(feature = "mood", feature = "traits"))]
#[must_use]
fn mercury_reasoning_strategy(mercury: ZodiacSign) -> crate::reasoning::ReasoningStrategy {
    use crate::reasoning::ReasoningStrategy;

    match (sign_modality(mercury), sign_element(mercury)) {
        // Water signs override modality — empathetic reasoning dominates
        (_, Element::Water) => ReasoningStrategy::Empathetic,
        // Air mutable = creative (Gemini archetype)
        (Modality::Mutable, Element::Air) => ReasoningStrategy::Creative,
        // Cardinal = analytical
        (Modality::Cardinal, _) => ReasoningStrategy::Analytical,
        // Fixed = systematic
        (Modality::Fixed, _) => ReasoningStrategy::Systematic,
        // Mutable (non-water, non-air) = intuitive
        (Modality::Mutable, _) => ReasoningStrategy::Intuitive,
    }
}

// ── Venus modifier ────────────────────────────────────────────────────────

/// Produce a Spirit from the Venus sign placement.
///
/// Venus governs passions, aesthetics, and relationship style.
/// The element determines the motivational theme; the sign adds specificity.
#[cfg(feature = "archetype")]
#[must_use]
fn venus_spirit(venus: ZodiacSign) -> crate::spirit::Spirit {
    let mut spirit = crate::spirit::Spirit::new();

    match sign_element(venus) {
        Element::Fire => {
            spirit.add_passion(
                "creative expression",
                "Driven to create, perform, and inspire",
                0.8,
            );
            spirit.add_inspiration("bold action", "Inspired by courage and decisive moves", 0.7);
            spirit.add_pain(
                "stagnation",
                "Pained by creative suppression or boredom",
                0.6,
            );
        }
        Element::Water => {
            spirit.add_passion(
                "deep connection",
                "Driven to form profound emotional bonds",
                0.9,
            );
            spirit.add_inspiration(
                "vulnerability",
                "Inspired by authentic emotional expression",
                0.8,
            );
            spirit.add_pain(
                "emotional betrayal",
                "Pained by broken trust or superficiality",
                0.7,
            );
        }
        Element::Earth => {
            spirit.add_passion(
                "craftsmanship",
                "Driven to build lasting, beautiful things",
                0.8,
            );
            spirit.add_inspiration("natural beauty", "Inspired by elegance in simplicity", 0.7);
            spirit.add_pain(
                "waste",
                "Pained by carelessness or squandered resources",
                0.6,
            );
        }
        Element::Air => {
            spirit.add_passion("harmony", "Driven to create balance and fairness", 0.8);
            spirit.add_inspiration(
                "intellectual beauty",
                "Inspired by elegant ideas and wit",
                0.7,
            );
            spirit.add_pain("discord", "Pained by conflict and injustice", 0.6);
        }
    }

    spirit
}

// ── Rising modifier ──────────────────────────────────────────────────────

/// Produce display rules from the Rising (Ascendant) sign placement.
///
/// The Rising sign governs how emotions are expressed vs felt — the social
/// mask. Fire risings amplify; Earth risings de-amplify; Water risings
/// mask with socially appropriate substitutes; Air risings qualify with humor.
#[cfg(feature = "mood")]
#[must_use]
fn rising_display_context(rising: ZodiacSign) -> crate::display_rules::CulturalContext {
    use crate::display_rules::{CulturalContext, DisplayRule};
    use crate::mood::Emotion;

    let name = format!("{} rising", rising);
    let mut ctx = CulturalContext::new(name);

    match sign_element(rising) {
        Element::Fire => {
            // Fire risings: amplify emotional expression — what you see is what they feel, but louder
            ctx.add_rule(DisplayRule::Amplify {
                target: Emotion::Joy,
                factor: 1.4,
            });
            ctx.add_rule(DisplayRule::Amplify {
                target: Emotion::Arousal,
                factor: 1.3,
            });
            ctx.add_rule(DisplayRule::Amplify {
                target: Emotion::Dominance,
                factor: 1.3,
            });
        }
        Element::Water => {
            // Water risings: mask vulnerability with warmth, de-amplify frustration
            ctx.add_rule(DisplayRule::DeAmplify {
                target: Emotion::Frustration,
                factor: 0.5,
            });
            ctx.add_rule(DisplayRule::Amplify {
                target: Emotion::Trust,
                factor: 1.3,
            });
        }
        Element::Earth => {
            // Earth risings: understated, composed — de-amplify everything
            ctx.add_rule(DisplayRule::DeAmplify {
                target: Emotion::Arousal,
                factor: 0.6,
            });
            ctx.add_rule(DisplayRule::DeAmplify {
                target: Emotion::Frustration,
                factor: 0.5,
            });
            ctx.add_rule(DisplayRule::DeAmplify {
                target: Emotion::Joy,
                factor: 0.8,
            });
        }
        Element::Air => {
            // Air risings: intellectualize emotion — qualify with interest/humor
            ctx.add_rule(DisplayRule::Qualify {
                qualifier: Emotion::Interest,
                intensity: 0.2,
            });
            ctx.add_rule(DisplayRule::DeAmplify {
                target: Emotion::Frustration,
                factor: 0.7,
            });
        }
    }

    ctx
}

// ── Mars modifier ─────────────────────────────────────────────────────────

/// Modify energy parameters based on the Mars sign placement.
///
/// Mars governs drive intensity. Fire Mars = explosive drive, fast drain/recovery.
/// Earth Mars = steady, enduring. Water Mars = emotionally driven, variable.
/// Air Mars = mentally energized, moderate.
#[cfg(feature = "mood")]
fn mars_energy_modifier(mars: ZodiacSign, energy: &mut crate::energy::EnergyState) {
    match sign_element(mars) {
        Element::Fire => {
            // Fire Mars: explosive drive — higher drain, higher recovery, fast fatigue
            energy.drain_rate = 0.035;
            energy.recovery_rate = 0.045;
            energy.fatigue_gain = 0.04;
            energy.fitness_gain = 0.015;
        }
        Element::Earth => {
            // Earth Mars: enduring — low drain, steady recovery, slow fatigue buildup
            energy.drain_rate = 0.015;
            energy.recovery_rate = 0.025;
            energy.fatigue_tau = 20.0; // slower fatigue decay = lingers longer
            energy.fitness_tau = 80.0; // slower fitness decay = retains gains
        }
        Element::Water => {
            // Water Mars: emotionally driven — moderate drain, recovery tied to emotional state
            energy.drain_rate = 0.025;
            energy.recovery_rate = 0.03;
            energy.fatigue_gain = 0.035;
        }
        Element::Air => {
            // Air Mars: mentally energized — low physical drain, moderate recovery
            energy.drain_rate = 0.018;
            energy.recovery_rate = 0.035;
            energy.fitness_gain = 0.012;
        }
    }
}

// ── Saturn modifier ───────────────────────────────────────────────────────

/// Modify stress parameters based on the Saturn sign placement.
///
/// Saturn governs discipline and endurance under pressure. Earth Saturn = high
/// burnout resistance. Fire Saturn = fast accumulation but also fast recovery.
/// Water Saturn = low thresholds but deep recovery. Air Saturn = intellectual coping.
#[cfg(feature = "mood")]
fn saturn_stress_modifier(saturn: ZodiacSign, stress: &mut crate::stress::StressState) {
    match sign_element(saturn) {
        Element::Earth => {
            // Earth Saturn: fortress — high thresholds, slow but deep
            stress.threshold_fatigue = 0.7;
            stress.threshold_burnout = 0.95;
            stress.recovery_rate = 0.015;
            stress.accumulation_rate = 0.04;
        }
        Element::Fire => {
            // Fire Saturn: burns hot, recovers fast
            stress.threshold_fatigue = 0.5;
            stress.threshold_burnout = 0.85;
            stress.recovery_rate = 0.035;
            stress.accumulation_rate = 0.06;
        }
        Element::Water => {
            // Water Saturn: absorbs deeply, slow to release
            stress.threshold_fatigue = 0.5;
            stress.threshold_burnout = 0.8;
            stress.recovery_rate = 0.01;
            stress.accumulation_rate = 0.04;
        }
        Element::Air => {
            // Air Saturn: intellectualizes stress — moderate thresholds, moderate recovery
            stress.threshold_fatigue = 0.6;
            stress.threshold_burnout = 0.9;
            stress.recovery_rate = 0.025;
            stress.accumulation_rate = 0.045;
        }
    }
}

// ── Jupiter modifier ──────────────────────────────────────────────────────

/// Modify growth parameters based on the Jupiter sign placement.
///
/// Jupiter governs expansion and adaptation speed. Fire Jupiter = rapid growth,
/// low threshold. Earth Jupiter = slow but lasting. Water Jupiter = emotionally
/// catalyzed growth. Air Jupiter = intellectually driven adaptation.
#[cfg(all(feature = "mood", feature = "traits"))]
fn jupiter_growth_modifier(jupiter: ZodiacSign, growth: &mut crate::growth::GrowthLedger) {
    match sign_element(jupiter) {
        Element::Fire => {
            // Fire Jupiter: rapid evolution — low threshold, fast decay (needs reinforcement)
            growth.threshold = 2.0;
            growth.decay_rate = 0.08;
        }
        Element::Earth => {
            // Earth Jupiter: slow but permanent — high threshold, very slow decay
            growth.threshold = 4.0;
            growth.decay_rate = 0.02;
        }
        Element::Water => {
            // Water Jupiter: emotionally catalyzed — moderate threshold, moderate decay
            growth.threshold = 2.5;
            growth.decay_rate = 0.04;
        }
        Element::Air => {
            // Air Jupiter: intellectually driven — moderate threshold, moderate decay
            growth.threshold = 3.0;
            growth.decay_rate = 0.06;
        }
    }
}

// ── Neptune modifier ──────────────────────────────────────────────────────

/// Modify EQ branch weights based on the Neptune sign placement.
///
/// Neptune governs emotional intelligence style. Water Neptune = perception-dominant
/// (intuitive reading). Earth Neptune = management-dominant (practical coping).
/// Fire Neptune = facilitation-dominant (emotions fuel action). Air Neptune =
/// understanding-dominant (intellectual comprehension).
#[cfg(feature = "mood")]
fn neptune_eq_modifier(neptune: ZodiacSign, eq: &mut crate::eq::EqProfile) {
    use crate::eq::EqBranch;

    match sign_element(neptune) {
        Element::Water => {
            // Water Neptune: intuitive perception, deep feeling
            eq.set(EqBranch::Perception, 0.75);
            eq.set(EqBranch::Understanding, 0.6);
        }
        Element::Earth => {
            // Earth Neptune: practical emotional management
            eq.set(EqBranch::Management, 0.75);
            eq.set(EqBranch::Facilitation, 0.6);
        }
        Element::Fire => {
            // Fire Neptune: emotions fuel cognitive performance
            eq.set(EqBranch::Facilitation, 0.75);
            eq.set(EqBranch::Perception, 0.6);
        }
        Element::Air => {
            // Air Neptune: intellectual emotional understanding
            eq.set(EqBranch::Understanding, 0.75);
            eq.set(EqBranch::Management, 0.6);
        }
    }
}

// ── Uranus modifier ──────────────────────────────────────────────────────

/// Modify flow thresholds based on the Uranus sign placement.
///
/// Uranus governs the relationship with flow states. Fire Uranus = easy entry,
/// low disruption resistance. Earth Uranus = hard entry, high disruption resistance.
/// Water Uranus = emotionally triggered flow. Air Uranus = novelty-seeking flow.
#[cfg(feature = "mood")]
fn uranus_flow_modifier(uranus: ZodiacSign, flow: &mut crate::flow::FlowState) {
    match sign_element(uranus) {
        Element::Fire => {
            // Fire Uranus: easy to enter flow, but also easy to disrupt
            flow.interest_threshold = 0.3;
            flow.entry_threshold = 0.8;
            flow.build_rate = 0.07;
            flow.frustration_ceiling = 0.25;
        }
        Element::Earth => {
            // Earth Uranus: hard to enter, but deeply stable once there
            flow.interest_threshold = 0.5;
            flow.entry_threshold = 1.2;
            flow.build_rate = 0.03;
            flow.frustration_ceiling = 0.5;
        }
        Element::Water => {
            // Water Uranus: emotionally catalyzed flow — passion-driven entry
            flow.interest_threshold = 0.35;
            flow.dominance_floor = 0.0;
            flow.arousal_floor = 0.2;
            flow.build_rate = 0.06;
        }
        Element::Air => {
            // Air Uranus: novelty-seeking — curiosity lowers threshold
            flow.interest_threshold = 0.25;
            flow.entry_threshold = 0.9;
            flow.arousal_ceiling = 0.8;
            flow.build_rate = 0.06;
        }
    }
}

// ── North Node modifier ──────────────────────────────────────────────────

/// Modify preference bias based on the North Node sign placement.
///
/// The North Node represents what the entity gravitates toward over time.
/// Fire NN = strong positive preference formation (drawn to action).
/// Water NN = strong negative sensitivity (avoids emotional harm).
/// Earth NN = balanced, conservative preference formation.
/// Air NN = novelty bias (positive toward new, less weight on negative).
#[cfg(feature = "mood")]
fn north_node_preference_modifier(
    north_node: ZodiacSign,
    bias: &mut crate::preference::PreferenceBias,
) {
    match sign_element(north_node) {
        Element::Fire => {
            // Fire NN: enthusiastic — forms positive preferences quickly
            bias.positive_gain = 1.4;
            bias.negative_gain = 0.8;
        }
        Element::Water => {
            // Water NN: protective — weights negative experiences heavily
            bias.positive_gain = 1.0;
            bias.negative_gain = 1.4;
        }
        Element::Earth => {
            // Earth NN: conservative — slow, balanced preference formation
            bias.positive_gain = 0.9;
            bias.negative_gain = 0.9;
        }
        Element::Air => {
            // Air NN: novelty-seeking — drawn to new, forgets bad quickly
            bias.positive_gain = 1.3;
            bias.negative_gain = 0.7;
        }
    }
}

// ── South Node modifier ─────────────────────────────────────────────────

/// Compute ACT-R memory parameters from the South Node sign placement.
///
/// The South Node represents pre-existing knowledge patterns and comfort zones.
/// Earth SN = slow decay (strong long-term memory). Fire SN = fast decay (lives
/// in the present). Water SN = long recency (emotional memories linger).
/// Air SN = short recency (quick mental turnover).
///
/// Returns `(decay, recency_half_life)`.
#[cfg(feature = "mood")]
#[must_use]
fn south_node_actr_params(south_node: ZodiacSign) -> (f64, f64) {
    match sign_element(south_node) {
        Element::Earth => {
            // Earth SN: deep roots — slow decay, long recency
            (0.35, 450.0)
        }
        Element::Fire => {
            // Fire SN: lives in the present — fast decay, short recency
            (0.7, 180.0)
        }
        Element::Water => {
            // Water SN: emotional memories linger — moderate decay, very long recency
            (0.45, 600.0)
        }
        Element::Air => {
            // Air SN: quick mental turnover — moderate decay, short recency
            (0.55, 200.0)
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{TraitKind, TraitLevel};

    // ── Element mapping ───────────────────────────────────────────────

    #[test]
    fn fire_signs() {
        assert_eq!(sign_element(ZodiacSign::Aries), Element::Fire);
        assert_eq!(sign_element(ZodiacSign::Leo), Element::Fire);
        assert_eq!(sign_element(ZodiacSign::Sagittarius), Element::Fire);
    }

    #[test]
    fn water_signs() {
        assert_eq!(sign_element(ZodiacSign::Cancer), Element::Water);
        assert_eq!(sign_element(ZodiacSign::Scorpio), Element::Water);
        assert_eq!(sign_element(ZodiacSign::Pisces), Element::Water);
    }

    #[test]
    fn earth_signs() {
        assert_eq!(sign_element(ZodiacSign::Taurus), Element::Earth);
        assert_eq!(sign_element(ZodiacSign::Virgo), Element::Earth);
        assert_eq!(sign_element(ZodiacSign::Capricorn), Element::Earth);
    }

    #[test]
    fn air_signs() {
        assert_eq!(sign_element(ZodiacSign::Gemini), Element::Air);
        assert_eq!(sign_element(ZodiacSign::Libra), Element::Air);
        assert_eq!(sign_element(ZodiacSign::Aquarius), Element::Air);
    }

    // ── Modality mapping ──────────────────────────────────────────────

    #[test]
    fn cardinal_signs() {
        assert_eq!(sign_modality(ZodiacSign::Aries), Modality::Cardinal);
        assert_eq!(sign_modality(ZodiacSign::Cancer), Modality::Cardinal);
        assert_eq!(sign_modality(ZodiacSign::Libra), Modality::Cardinal);
        assert_eq!(sign_modality(ZodiacSign::Capricorn), Modality::Cardinal);
    }

    #[test]
    fn fixed_signs() {
        assert_eq!(sign_modality(ZodiacSign::Taurus), Modality::Fixed);
        assert_eq!(sign_modality(ZodiacSign::Leo), Modality::Fixed);
        assert_eq!(sign_modality(ZodiacSign::Scorpio), Modality::Fixed);
        assert_eq!(sign_modality(ZodiacSign::Aquarius), Modality::Fixed);
    }

    #[test]
    fn mutable_signs() {
        assert_eq!(sign_modality(ZodiacSign::Gemini), Modality::Mutable);
        assert_eq!(sign_modality(ZodiacSign::Virgo), Modality::Mutable);
        assert_eq!(sign_modality(ZodiacSign::Sagittarius), Modality::Mutable);
        assert_eq!(sign_modality(ZodiacSign::Pisces), Modality::Mutable);
    }

    // ── Exhaustiveness ────────────────────────────────────────────────

    #[test]
    fn all_signs_counted() {
        assert_eq!(ZodiacSign::ALL.len(), ZodiacSign::COUNT);
    }

    #[test]
    fn every_sign_has_element_and_modality() {
        for &sign in ZodiacSign::ALL {
            let _ = sign_element(sign);
            let _ = sign_modality(sign);
        }
    }

    #[test]
    fn element_distribution() {
        // 3 signs per element
        for element in [Element::Fire, Element::Water, Element::Earth, Element::Air] {
            let count = ZodiacSign::ALL
                .iter()
                .filter(|&&s| sign_element(s) == element)
                .count();
            assert_eq!(count, 3, "{element} should have 3 signs");
        }
    }

    #[test]
    fn modality_distribution() {
        // 4 signs per modality
        for modality in [Modality::Cardinal, Modality::Fixed, Modality::Mutable] {
            let count = ZodiacSign::ALL
                .iter()
                .filter(|&&s| sign_modality(s) == modality)
                .count();
            assert_eq!(count, 4, "{modality} should have 4 signs");
        }
    }

    // ── Sign profiles ─────────────────────────────────────────────────

    #[test]
    fn all_signs_produce_non_default_profiles() {
        let default = PersonalityProfile::new("default");
        for &sign in ZodiacSign::ALL {
            let profile = sign_profile(sign);
            // At least one trait should differ from Balanced
            let has_non_balanced = TraitKind::ALL
                .iter()
                .any(|&t| profile.get_trait(t) != default.get_trait(t));
            assert!(
                has_non_balanced,
                "{sign} profile should have non-default traits"
            );
        }
    }

    #[test]
    fn sign_profiles_named_correctly() {
        for &sign in ZodiacSign::ALL {
            let profile = sign_profile(sign);
            assert_eq!(profile.name, sign.to_string());
        }
    }

    // ── Specific sign trait verification ───────────────────────────────

    #[test]
    fn aries_is_bold_direct() {
        let p = sign_profile(ZodiacSign::Aries);
        assert_eq!(p.get_trait(TraitKind::Directness), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Confidence), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Patience), TraitLevel::Low);
    }

    #[test]
    fn cancer_is_nurturing_empathic() {
        let p = sign_profile(ZodiacSign::Cancer);
        assert_eq!(p.get_trait(TraitKind::Empathy), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Warmth), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::RiskTolerance), TraitLevel::Low);
    }

    #[test]
    fn virgo_is_precise_analytical() {
        let p = sign_profile(ZodiacSign::Virgo);
        assert_eq!(p.get_trait(TraitKind::Precision), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Skepticism), TraitLevel::High);
    }

    #[test]
    fn gemini_is_curious_witty() {
        let p = sign_profile(ZodiacSign::Gemini);
        assert_eq!(p.get_trait(TraitKind::Curiosity), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn scorpio_is_intense_perceptive() {
        let p = sign_profile(ZodiacSign::Scorpio);
        assert_eq!(p.get_trait(TraitKind::Skepticism), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Confidence), TraitLevel::High);
        assert_eq!(p.get_trait(TraitKind::Empathy), TraitLevel::High);
    }

    #[test]
    fn capricorn_is_disciplined_ambitious() {
        let p = sign_profile(ZodiacSign::Capricorn);
        assert_eq!(p.get_trait(TraitKind::Autonomy), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Precision), TraitLevel::High);
        assert_eq!(p.get_trait(TraitKind::Humor), TraitLevel::Low);
    }

    // ── Same-element profiles share trait emphasis ─────────────────────

    #[test]
    fn fire_signs_share_confidence() {
        for sign in [ZodiacSign::Aries, ZodiacSign::Leo, ZodiacSign::Sagittarius] {
            let p = sign_profile(sign);
            assert!(
                p.get_trait(TraitKind::Confidence) >= TraitLevel::High,
                "{sign} should have high confidence"
            );
        }
    }

    #[test]
    fn water_signs_share_empathy() {
        for sign in [ZodiacSign::Cancer, ZodiacSign::Scorpio, ZodiacSign::Pisces] {
            let p = sign_profile(sign);
            assert!(
                p.get_trait(TraitKind::Empathy) >= TraitLevel::High,
                "{sign} should have high empathy"
            );
        }
    }

    #[test]
    fn earth_signs_share_patience() {
        for sign in [ZodiacSign::Taurus, ZodiacSign::Virgo, ZodiacSign::Capricorn] {
            let p = sign_profile(sign);
            assert!(
                p.get_trait(TraitKind::Patience) >= TraitLevel::High,
                "{sign} should have high patience"
            );
        }
    }

    #[test]
    fn air_signs_share_curiosity_or_independence() {
        for sign in [ZodiacSign::Gemini, ZodiacSign::Libra, ZodiacSign::Aquarius] {
            let p = sign_profile(sign);
            let curious = p.get_trait(TraitKind::Curiosity) >= TraitLevel::High;
            let independent = p.get_trait(TraitKind::Autonomy) >= TraitLevel::High;
            let social = p.get_trait(TraitKind::Warmth) >= TraitLevel::High;
            assert!(
                curious || independent || social,
                "{sign} should have high curiosity, autonomy, or warmth"
            );
        }
    }

    // ── Display ───────────────────────────────────────────────────────

    #[test]
    fn display_formatting() {
        assert_eq!(ZodiacSign::Aries.to_string(), "Aries");
        assert_eq!(ZodiacSign::Sagittarius.to_string(), "Sagittarius");
        assert_eq!(Element::Fire.to_string(), "Fire");
        assert_eq!(Modality::Cardinal.to_string(), "Cardinal");
    }

    // ── Serde ─────────────────────────────────────────────────────────

    #[test]
    fn serde_roundtrip() {
        for &sign in ZodiacSign::ALL {
            let json = serde_json::to_string(&sign).unwrap();
            let back: ZodiacSign = serde_json::from_str(&json).unwrap();
            assert_eq!(back, sign);
        }
    }

    // ── Planet enum ───────────────────────────────────────────────────

    #[test]
    fn all_planets_counted() {
        assert_eq!(Planet::ALL.len(), Planet::COUNT);
    }

    #[test]
    fn planet_discriminants_are_contiguous() {
        // Ensures `planet as usize` is safe for array indexing
        for (i, &planet) in Planet::ALL.iter().enumerate() {
            assert_eq!(planet as usize, i, "{planet} discriminant should be {i}");
        }
    }

    #[test]
    fn inner_planets() {
        assert!(Planet::Sun.is_inner());
        assert!(Planet::Moon.is_inner());
        assert!(Planet::Rising.is_inner());
        assert!(Planet::Mercury.is_inner());
        assert!(Planet::Venus.is_inner());
        assert!(!Planet::Mars.is_inner());
        assert!(!Planet::Chiron.is_inner());
    }

    #[test]
    fn planet_display() {
        assert_eq!(Planet::Sun.to_string(), "Sun");
        assert_eq!(Planet::NorthNode.to_string(), "North Node");
        assert_eq!(Planet::Chiron.to_string(), "Chiron");
    }

    #[test]
    fn planet_serde_roundtrip() {
        for &planet in Planet::ALL {
            let json = serde_json::to_string(&planet).unwrap();
            let back: Planet = serde_json::from_str(&json).unwrap();
            assert_eq!(back, planet);
        }
    }

    // ── NatalChart builder ────────────────────────────────────────────

    #[test]
    fn empty_chart() {
        let chart = NatalChart::new();
        assert_eq!(chart.placement_count(), 0);
        assert_eq!(chart.get(Planet::Sun), None);
    }

    #[test]
    fn chart_builder_fluent() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Scorpio)
            .moon(ZodiacSign::Cancer)
            .rising(ZodiacSign::Gemini);
        assert_eq!(chart.placement_count(), 3);
        assert_eq!(chart.get(Planet::Sun), Some(ZodiacSign::Scorpio));
        assert_eq!(chart.get(Planet::Moon), Some(ZodiacSign::Cancer));
        assert_eq!(chart.get(Planet::Rising), Some(ZodiacSign::Gemini));
    }

    #[test]
    fn chart_generic_place() {
        let chart = NatalChart::new().place(Planet::Mars, ZodiacSign::Aries);
        assert_eq!(chart.get(Planet::Mars), Some(ZodiacSign::Aries));
    }

    #[test]
    fn full_chart() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Scorpio)
            .moon(ZodiacSign::Cancer)
            .rising(ZodiacSign::Gemini)
            .mercury(ZodiacSign::Sagittarius)
            .venus(ZodiacSign::Libra)
            .mars(ZodiacSign::Aries)
            .jupiter(ZodiacSign::Sagittarius)
            .saturn(ZodiacSign::Capricorn)
            .neptune(ZodiacSign::Pisces)
            .pluto(ZodiacSign::Scorpio)
            .uranus(ZodiacSign::Aquarius)
            .north_node(ZodiacSign::Leo)
            .south_node(ZodiacSign::Aquarius)
            .chiron(ZodiacSign::Virgo);
        assert_eq!(chart.placement_count(), Planet::COUNT);
    }

    #[test]
    fn chart_serde_roundtrip() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Leo)
            .moon(ZodiacSign::Pisces);
        let json = serde_json::to_string(&chart).unwrap();
        let back: NatalChart = serde_json::from_str(&json).unwrap();
        assert_eq!(back.get(Planet::Sun), Some(ZodiacSign::Leo));
        assert_eq!(back.get(Planet::Moon), Some(ZodiacSign::Pisces));
        assert_eq!(back.get(Planet::Mars), None);
    }

    // ── Manifestation ─────────────────────────────────────────────────

    #[test]
    fn manifest_sun_only() {
        let chart = NatalChart::new().sun(ZodiacSign::Aries);
        let profile = chart.manifest();
        assert_eq!(profile.personality.name, "Aries");
        assert_eq!(
            profile.personality.get_trait(TraitKind::Confidence),
            TraitLevel::Highest
        );
    }

    #[test]
    fn manifest_empty_defaults_to_aries() {
        let chart = NatalChart::new();
        let profile = chart.manifest();
        assert_eq!(profile.personality.name, "Aries");
    }

    #[cfg(feature = "mood")]
    #[test]
    fn manifest_moon_modifies_baseline() {
        use crate::mood::Emotion;

        let sun_only = NatalChart::new().sun(ZodiacSign::Taurus).manifest();
        let with_fire_moon = NatalChart::new()
            .sun(ZodiacSign::Taurus)
            .moon(ZodiacSign::Aries)
            .manifest();

        // Fire moon should raise arousal above the personality-derived baseline
        assert!(
            with_fire_moon.mood_baseline.get(Emotion::Arousal)
                > sun_only.mood_baseline.get(Emotion::Arousal),
            "fire moon should raise arousal: {} vs {}",
            with_fire_moon.mood_baseline.get(Emotion::Arousal),
            sun_only.mood_baseline.get(Emotion::Arousal),
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn manifest_water_moon_raises_trust() {
        use crate::mood::Emotion;

        let sun_only = NatalChart::new().sun(ZodiacSign::Aries).manifest();
        let with_water_moon = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .moon(ZodiacSign::Cancer)
            .manifest();

        assert!(
            with_water_moon.mood_baseline.get(Emotion::Trust)
                > sun_only.mood_baseline.get(Emotion::Trust),
            "water moon should raise trust"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn manifest_earth_moon_lowers_arousal() {
        use crate::mood::Emotion;

        let sun_only = NatalChart::new().sun(ZodiacSign::Gemini).manifest();
        let with_earth_moon = NatalChart::new()
            .sun(ZodiacSign::Gemini)
            .moon(ZodiacSign::Taurus)
            .manifest();

        assert!(
            with_earth_moon.mood_baseline.get(Emotion::Arousal)
                < sun_only.mood_baseline.get(Emotion::Arousal),
            "earth moon should lower arousal"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn manifest_air_moon_raises_interest() {
        use crate::mood::Emotion;

        let sun_only = NatalChart::new().sun(ZodiacSign::Capricorn).manifest();
        let with_air_moon = NatalChart::new()
            .sun(ZodiacSign::Capricorn)
            .moon(ZodiacSign::Gemini)
            .manifest();

        assert!(
            with_air_moon.mood_baseline.get(Emotion::Interest)
                > sun_only.mood_baseline.get(Emotion::Interest),
            "air moon should raise interest"
        );
    }

    // ── Mercury → reasoning strategy ──────────────────────────────────

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn mercury_cardinal_is_analytical() {
        use crate::reasoning::ReasoningStrategy;
        // Aries = Cardinal Fire
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .mercury(ZodiacSign::Capricorn); // Cardinal Earth
        let profile = chart.manifest();
        assert_eq!(profile.reasoning_strategy, ReasoningStrategy::Analytical);
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn mercury_fixed_is_systematic() {
        use crate::reasoning::ReasoningStrategy;
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .mercury(ZodiacSign::Taurus); // Fixed Earth
        let profile = chart.manifest();
        assert_eq!(profile.reasoning_strategy, ReasoningStrategy::Systematic);
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn mercury_water_is_empathetic() {
        use crate::reasoning::ReasoningStrategy;
        // Water overrides modality
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .mercury(ZodiacSign::Cancer); // Cardinal Water → Empathetic
        let profile = chart.manifest();
        assert_eq!(profile.reasoning_strategy, ReasoningStrategy::Empathetic);
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn mercury_mutable_air_is_creative() {
        use crate::reasoning::ReasoningStrategy;
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .mercury(ZodiacSign::Gemini); // Mutable Air → Creative
        let profile = chart.manifest();
        assert_eq!(profile.reasoning_strategy, ReasoningStrategy::Creative);
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn mercury_mutable_fire_is_intuitive() {
        use crate::reasoning::ReasoningStrategy;
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .mercury(ZodiacSign::Sagittarius); // Mutable Fire → Intuitive
        let profile = chart.manifest();
        assert_eq!(profile.reasoning_strategy, ReasoningStrategy::Intuitive);
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn no_mercury_derives_from_personality() {
        let chart = NatalChart::new().sun(ZodiacSign::Virgo);
        let profile = chart.manifest();
        // Should derive from personality traits, not crash
        let _ = profile.reasoning_strategy;
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn mercury_all_signs_produce_valid_strategy() {
        use crate::reasoning::ReasoningStrategy;
        for &sign in ZodiacSign::ALL {
            let chart = NatalChart::new().sun(ZodiacSign::Aries).mercury(sign);
            let profile = chart.manifest();
            // Every sign must map to one of the 5 strategies
            assert!(
                ReasoningStrategy::ALL.contains(&profile.reasoning_strategy),
                "{sign} mercury should produce a valid strategy"
            );
        }
    }

    // ── Venus → spirit ────────────────────────────────────────────────

    #[cfg(feature = "archetype")]
    #[test]
    fn venus_produces_spirit() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .venus(ZodiacSign::Libra); // Air Venus
        let profile = chart.manifest();
        assert!(
            !profile.spirit.passions.is_empty(),
            "Venus should create passions"
        );
        assert!(
            !profile.spirit.inspirations.is_empty(),
            "Venus should create inspirations"
        );
        assert!(
            !profile.spirit.pains.is_empty(),
            "Venus should create pains"
        );
    }

    #[cfg(feature = "archetype")]
    #[test]
    fn venus_elements_produce_different_spirits() {
        let fire = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .venus(ZodiacSign::Leo)
            .manifest();
        let water = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .venus(ZodiacSign::Pisces)
            .manifest();
        // Different elements should produce different passion themes
        assert_ne!(fire.spirit.passions[0].name, water.spirit.passions[0].name,);
    }

    #[cfg(feature = "archetype")]
    #[test]
    fn no_venus_produces_empty_spirit() {
        let chart = NatalChart::new().sun(ZodiacSign::Aries);
        let profile = chart.manifest();
        assert!(profile.spirit.passions.is_empty());
    }

    // ── Rising → display rules ────────────────────────────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn rising_produces_display_context() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .rising(ZodiacSign::Leo); // Fire Rising
        let profile = chart.manifest();
        assert!(
            profile.display_context.rule_count() > 0,
            "Rising should create display rules"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn fire_rising_amplifies() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .rising(ZodiacSign::Aries); // Fire Rising
        let profile = chart.manifest();
        assert!(
            profile.display_context.rule_count() >= 3,
            "fire rising should have amplification rules"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn earth_rising_deamplifies() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .rising(ZodiacSign::Taurus); // Earth Rising
        let profile = chart.manifest();
        assert!(
            profile.display_context.rule_count() >= 3,
            "earth rising should have de-amplification rules"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn no_rising_produces_empty_context() {
        let chart = NatalChart::new().sun(ZodiacSign::Aries);
        let profile = chart.manifest();
        assert_eq!(profile.display_context.rule_count(), 0);
    }

    // ── Mars → energy ─────────────────────────────────────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn fire_mars_high_drain_high_recovery() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .mars(ZodiacSign::Aries); // Fire Mars
        let profile = chart.manifest();
        let default = crate::energy::EnergyState::new();
        assert!(
            profile.energy.drain_rate > default.drain_rate,
            "fire mars drain: {} vs default: {}",
            profile.energy.drain_rate,
            default.drain_rate
        );
        assert!(
            profile.energy.recovery_rate > default.recovery_rate,
            "fire mars recovery should be higher"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn earth_mars_low_drain() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .mars(ZodiacSign::Taurus); // Earth Mars
        let profile = chart.manifest();
        let default = crate::energy::EnergyState::new();
        assert!(
            profile.energy.drain_rate < default.drain_rate,
            "earth mars should drain slowly"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn no_mars_uses_defaults() {
        let chart = NatalChart::new().sun(ZodiacSign::Aries);
        let profile = chart.manifest();
        let default = crate::energy::EnergyState::new();
        assert!(
            (profile.energy.drain_rate - default.drain_rate).abs() < f32::EPSILON,
            "no mars should use default drain rate"
        );
    }

    // ── Saturn → stress ───────────────────────────────────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn earth_saturn_high_thresholds() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .saturn(ZodiacSign::Capricorn); // Earth Saturn
        let profile = chart.manifest();
        let default = crate::stress::StressState::new();
        assert!(
            profile.stress.threshold_burnout > default.threshold_burnout,
            "earth saturn should have higher burnout threshold"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn fire_saturn_fast_recovery() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .saturn(ZodiacSign::Aries); // Fire Saturn
        let profile = chart.manifest();
        let default = crate::stress::StressState::new();
        assert!(
            profile.stress.recovery_rate > default.recovery_rate,
            "fire saturn should recover faster"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn water_saturn_slow_recovery() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .saturn(ZodiacSign::Cancer); // Water Saturn
        let profile = chart.manifest();
        let default = crate::stress::StressState::new();
        assert!(
            profile.stress.recovery_rate < default.recovery_rate,
            "water saturn should recover slowly"
        );
    }

    // ── Jupiter → growth ──────────────────────────────────────────────

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn fire_jupiter_low_threshold() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .jupiter(ZodiacSign::Sagittarius); // Fire Jupiter
        let profile = chart.manifest();
        let default = crate::growth::GrowthLedger::new();
        assert!(
            profile.growth.threshold < default.threshold,
            "fire jupiter should lower growth threshold: {} vs {}",
            profile.growth.threshold,
            default.threshold,
        );
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn earth_jupiter_high_threshold() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .jupiter(ZodiacSign::Taurus); // Earth Jupiter
        let profile = chart.manifest();
        let default = crate::growth::GrowthLedger::new();
        assert!(
            profile.growth.threshold > default.threshold,
            "earth jupiter should raise growth threshold"
        );
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn earth_jupiter_slow_decay() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .jupiter(ZodiacSign::Virgo); // Earth Jupiter
        let profile = chart.manifest();
        let default = crate::growth::GrowthLedger::new();
        assert!(
            profile.growth.decay_rate < default.decay_rate,
            "earth jupiter should have slower pressure decay"
        );
    }

    // ── Exhaustiveness: all signs through outer planets ────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn all_signs_through_mars() {
        for &sign in ZodiacSign::ALL {
            let chart = NatalChart::new().sun(ZodiacSign::Aries).mars(sign);
            let profile = chart.manifest();
            assert!(
                profile.energy.drain_rate > 0.0,
                "{sign} mars should produce valid energy"
            );
        }
    }

    #[cfg(feature = "mood")]
    #[test]
    fn all_signs_through_saturn() {
        for &sign in ZodiacSign::ALL {
            let chart = NatalChart::new().sun(ZodiacSign::Aries).saturn(sign);
            let profile = chart.manifest();
            assert!(
                profile.stress.threshold_burnout > profile.stress.threshold_fatigue,
                "{sign} saturn: burnout threshold must exceed fatigue threshold"
            );
        }
    }

    #[cfg(all(feature = "mood", feature = "traits"))]
    #[test]
    fn all_signs_through_jupiter() {
        for &sign in ZodiacSign::ALL {
            let chart = NatalChart::new().sun(ZodiacSign::Aries).jupiter(sign);
            let profile = chart.manifest();
            assert!(
                profile.growth.threshold > 0.0,
                "{sign} jupiter should produce valid growth"
            );
        }
    }

    // ── Neptune → EQ ────────────────────────────────────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn water_neptune_perception_dominant() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .neptune(ZodiacSign::Pisces); // Water Neptune
        let profile = chart.manifest();
        let default = crate::eq::EqProfile::new();
        assert!(
            profile.eq.get(crate::eq::EqBranch::Perception)
                > default.get(crate::eq::EqBranch::Perception),
            "water neptune should raise perception"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn earth_neptune_management_dominant() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .neptune(ZodiacSign::Capricorn); // Earth Neptune
        let profile = chart.manifest();
        let default = crate::eq::EqProfile::new();
        assert!(
            profile.eq.get(crate::eq::EqBranch::Management)
                > default.get(crate::eq::EqBranch::Management),
            "earth neptune should raise management"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn all_signs_through_neptune() {
        for &sign in ZodiacSign::ALL {
            let chart = NatalChart::new().sun(ZodiacSign::Aries).neptune(sign);
            let profile = chart.manifest();
            assert!(
                profile.eq.overall() > 0.0,
                "{sign} neptune should produce valid EQ"
            );
        }
    }

    // ── Uranus → flow ─────────────────────────────────────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn fire_uranus_easy_flow_entry() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .uranus(ZodiacSign::Aries); // Fire Uranus
        let profile = chart.manifest();
        let default = crate::flow::FlowState::default();
        assert!(
            profile.flow.entry_threshold < default.entry_threshold,
            "fire uranus should lower flow entry threshold"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn earth_uranus_hard_flow_entry() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .uranus(ZodiacSign::Taurus); // Earth Uranus
        let profile = chart.manifest();
        let default = crate::flow::FlowState::default();
        assert!(
            profile.flow.entry_threshold > default.entry_threshold,
            "earth uranus should raise flow entry threshold"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn all_signs_through_uranus() {
        for &sign in ZodiacSign::ALL {
            let chart = NatalChart::new().sun(ZodiacSign::Aries).uranus(sign);
            let profile = chart.manifest();
            assert!(
                profile.flow.entry_threshold > 0.0,
                "{sign} uranus should produce valid flow"
            );
        }
    }

    // ── North Node → preference bias ──────────────────────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn fire_nn_positive_bias() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .north_node(ZodiacSign::Leo); // Fire NN
        let profile = chart.manifest();
        assert!(
            profile.preference_bias.positive_gain > 1.0,
            "fire NN should boost positive gain"
        );
        assert!(
            profile.preference_bias.negative_gain < 1.0,
            "fire NN should dampen negative gain"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn water_nn_negative_bias() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .north_node(ZodiacSign::Cancer); // Water NN
        let profile = chart.manifest();
        assert!(
            profile.preference_bias.negative_gain > 1.0,
            "water NN should boost negative gain"
        );
    }

    // ── South Node → ACT-R params ─────────────────────────────────────

    #[cfg(feature = "mood")]
    #[test]
    fn earth_sn_slow_decay() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .south_node(ZodiacSign::Taurus); // Earth SN
        let profile = chart.manifest();
        assert!(profile.actr_decay < 0.5, "earth SN should have slow decay");
        assert!(
            profile.actr_recency_half_life > 300.0,
            "earth SN should have long recency"
        );
    }

    #[cfg(feature = "mood")]
    #[test]
    fn fire_sn_fast_decay() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Aries)
            .south_node(ZodiacSign::Aries); // Fire SN
        let profile = chart.manifest();
        assert!(profile.actr_decay > 0.5, "fire SN should have fast decay");
        assert!(
            profile.actr_recency_half_life < 300.0,
            "fire SN should have short recency"
        );
    }

    // ── Full chart manifestation ──────────────────────────────────────

    #[test]
    fn full_chart_all_planets() {
        let chart = NatalChart::new()
            .sun(ZodiacSign::Scorpio)
            .moon(ZodiacSign::Cancer)
            .rising(ZodiacSign::Gemini)
            .mercury(ZodiacSign::Sagittarius)
            .venus(ZodiacSign::Libra)
            .mars(ZodiacSign::Aries)
            .jupiter(ZodiacSign::Sagittarius)
            .saturn(ZodiacSign::Capricorn)
            .neptune(ZodiacSign::Pisces)
            .uranus(ZodiacSign::Aquarius)
            .north_node(ZodiacSign::Leo)
            .south_node(ZodiacSign::Aquarius)
            .chiron(ZodiacSign::Virgo);
        let profile = chart.manifest();
        assert_eq!(profile.personality.name, "Scorpio");
    }
}
