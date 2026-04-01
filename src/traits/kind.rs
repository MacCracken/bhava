use serde::{Deserialize, Serialize};

use crate::error::{BhavaError, Result};

/// A personality trait with its current level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraitValue {
    pub trait_name: TraitKind,
    pub level: TraitLevel,
}

/// The available personality trait dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TraitKind {
    // Communication
    Formality,
    Humor,
    Verbosity,
    Directness,
    // Emotional
    Warmth,
    Empathy,
    Patience,
    // Cognitive
    Confidence,
    Creativity,
    RiskTolerance,
    Curiosity,
    Skepticism,
    // Professional
    Autonomy,
    Pedagogy,
    Precision,
}

impl TraitKind {
    /// All trait kinds.
    pub const ALL: &'static [TraitKind] = &[
        Self::Formality,
        Self::Humor,
        Self::Verbosity,
        Self::Directness,
        Self::Warmth,
        Self::Empathy,
        Self::Patience,
        Self::Confidence,
        Self::Creativity,
        Self::RiskTolerance,
        Self::Curiosity,
        Self::Skepticism,
        Self::Autonomy,
        Self::Pedagogy,
        Self::Precision,
    ];

    /// Number of trait kinds.
    pub const COUNT: usize = 15;

    /// Array index for this trait kind (0–14, matches `ALL` order).
    #[inline]
    pub fn index(self) -> usize {
        match self {
            Self::Formality => 0,
            Self::Humor => 1,
            Self::Verbosity => 2,
            Self::Directness => 3,
            Self::Warmth => 4,
            Self::Empathy => 5,
            Self::Patience => 6,
            Self::Confidence => 7,
            Self::Creativity => 8,
            Self::RiskTolerance => 9,
            Self::Curiosity => 10,
            Self::Skepticism => 11,
            Self::Autonomy => 12,
            Self::Pedagogy => 13,
            Self::Precision => 14,
        }
    }

    /// The neutral/default level for this trait.
    pub fn default_level(self) -> TraitLevel {
        TraitLevel::Balanced
    }

    /// Available levels for this trait (low → high).
    pub fn levels(self) -> &'static [TraitLevel] {
        // All traits share the same 5-level spectrum
        &[
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::Balanced,
            TraitLevel::High,
            TraitLevel::Highest,
        ]
    }

    /// Which group this trait belongs to.
    pub fn group(self) -> TraitGroup {
        match self {
            Self::Warmth | Self::Empathy | Self::Humor | Self::Patience => TraitGroup::Social,
            Self::Curiosity | Self::Creativity | Self::Confidence | Self::Skepticism => {
                TraitGroup::Cognitive
            }
            Self::Formality | Self::Verbosity | Self::Directness | Self::RiskTolerance => {
                TraitGroup::Behavioral
            }
            Self::Autonomy | Self::Pedagogy | Self::Precision => TraitGroup::Professional,
        }
    }
}

/// Trait groupings for bulk operations.
///
/// Groups organize the 15 trait dimensions into four categories:
/// - **Social** — interpersonal style (warmth, empathy, humor, patience)
/// - **Cognitive** — thinking style (curiosity, creativity, confidence, skepticism)
/// - **Behavioral** — communication style (formality, verbosity, directness, risk tolerance)
/// - **Professional** — work style (autonomy, pedagogy, precision)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TraitGroup {
    /// Interpersonal traits: warmth, empathy, humor, patience.
    Social,
    /// Thinking-style traits: curiosity, creativity, confidence, skepticism.
    Cognitive,
    /// Communication-style traits: formality, verbosity, directness, risk tolerance.
    Behavioral,
    /// Work-style traits: autonomy, pedagogy, precision.
    Professional,
}

impl TraitGroup {
    /// All groups.
    pub const ALL: &'static [TraitGroup] = &[
        Self::Social,
        Self::Cognitive,
        Self::Behavioral,
        Self::Professional,
    ];

    /// Trait kinds belonging to this group.
    pub fn traits(self) -> &'static [TraitKind] {
        match self {
            Self::Social => &[
                TraitKind::Warmth,
                TraitKind::Empathy,
                TraitKind::Humor,
                TraitKind::Patience,
            ],
            Self::Cognitive => &[
                TraitKind::Curiosity,
                TraitKind::Creativity,
                TraitKind::Confidence,
                TraitKind::Skepticism,
            ],
            Self::Behavioral => &[
                TraitKind::Formality,
                TraitKind::Verbosity,
                TraitKind::Directness,
                TraitKind::RiskTolerance,
            ],
            Self::Professional => &[
                TraitKind::Autonomy,
                TraitKind::Pedagogy,
                TraitKind::Precision,
            ],
        }
    }
}

impl_display!(TraitGroup {
    Social => "social",
    Cognitive => "cognitive",
    Behavioral => "behavioral",
    Professional => "professional",
});

impl_display!(TraitKind {
    Formality => "formality",
    Humor => "humor",
    Verbosity => "verbosity",
    Directness => "directness",
    Warmth => "warmth",
    Empathy => "empathy",
    Patience => "patience",
    Confidence => "confidence",
    Creativity => "creativity",
    RiskTolerance => "risk_tolerance",
    Skepticism => "skepticism",
    Autonomy => "autonomy",
    Pedagogy => "pedagogy",
    Precision => "precision",
    Curiosity => "curiosity",
});

/// Graduated level within a trait spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TraitLevel {
    Lowest,
    Low,
    Balanced,
    High,
    Highest,
}

impl TraitLevel {
    /// Numeric value: -2 (Lowest) to +2 (Highest).
    #[inline]
    #[must_use]
    pub fn numeric(self) -> i8 {
        match self {
            Self::Lowest => -2,
            Self::Low => -1,
            Self::Balanced => 0,
            Self::High => 1,
            Self::Highest => 2,
        }
    }

    /// Normalized to -1.0..=1.0.
    #[inline]
    #[must_use]
    pub fn normalized(self) -> f32 {
        self.numeric() as f32 / 2.0
    }

    /// Snap a normalized float (-1.0..=1.0) to the nearest trait level.
    #[must_use]
    pub fn from_normalized(v: f32) -> Self {
        let n = (v * 2.0).round() as i8;
        match n.clamp(-2, 2) {
            -2 => Self::Lowest,
            -1 => Self::Low,
            0 => Self::Balanced,
            1 => Self::High,
            _ => Self::Highest,
        }
    }

    /// Parse from numeric value.
    ///
    /// # Errors
    /// Returns `BhavaError::InvalidConfig` if `n` is outside -2..=2.
    pub fn from_numeric(n: i8) -> Result<Self> {
        match n {
            -2 => Ok(Self::Lowest),
            -1 => Ok(Self::Low),
            0 => Ok(Self::Balanced),
            1 => Ok(Self::High),
            2 => Ok(Self::Highest),
            _ => Err(BhavaError::InvalidConfig {
                reason: format!("trait level must be -2..=2, got {n}"),
            }),
        }
    }
}

impl_display!(TraitLevel {
    Lowest => "lowest",
    Low => "low",
    Balanced => "balanced",
    High => "high",
    Highest => "highest",
});
