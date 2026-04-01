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

        ManifestedProfile {
            personality,
            #[cfg(feature = "mood")]
            mood_baseline,
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
}
