//! Bhava — Emotion and personality engine for AGNOS
//!
//! Sanskrit: भाव (bhava) — emotion, feeling, state of being
//!
//! Provides a unified personality and emotional state system for AI agents,
//! game NPCs, and any entity that needs expressive behavior. Extracted from
//! SecureYeoman's soul/brain architecture.
//!
//! # Modules
//!
//! - [`traits`] — 15-dimension personality spectrums with behavioral instructions
//! - [`mood`] — Emotional state vectors with time-based decay, triggers, history, and mood-aware prompts
//! - [`archetype`] — Identity hierarchy (Soul/Spirit/Brain/Body/Heart) with templates and validation
//! - [`sentiment`] — Keyword-based sentiment analysis with negation, intensity modifiers, and sentence-level analysis
//! - [`presets`] — Built-in personality templates (BlueShirtGuy, T.Ron, Friday, Oracle, Scout)
//! - [`spirit`] — Passions, inspirations, and pains — the animating force
//! - [`relationship`] — Inter-entity affinity, trust, and interaction tracking
//! - [`appraisal`] — OCC appraisal model — goal-aware emotion generation
//! - [`stress`] — Allostatic load / burnout modeling
//! - [`regulation`] — Emotion regulation strategies (suppress, reappraise, distract)
//! - [`growth`] — Experience-driven personality evolution
//! - [`monitor`] — Live sentiment monitoring for streaming text
//! - [`ai`] — System prompt composition, sentiment feedback, and agent metadata
//! - [`store`] — Storage trait for pluggable persistence backends
//! - [`storage`] — SQLite persistence implementation (feature: `sqlite`)
//! - [`rhythm`] — Biological rhythms: ultradian, seasonal, and biorhythm cycles
//! - [`microexpr`] — Micro-expression detection during emotional suppression
//! - [`affective`] — Affective computing metrics (complexity, granularity, inertia, variability)
//! - [`proximity`] — Spatial proximity triggers for location-based mood effects
//! - [`reasoning`] — Personality-driven reasoning strategy selection
//! - [`active_hours`] — Time-of-day personality activation scheduling
//! - [`eq`] — Emotional intelligence (EQ) — Mayer-Salovey four-branch model
//! - [`display_rules`] — Cultural display rules (Matsumoto framework)
//! - [`energy`] — Depletable energy resource with Banister fitness-fatigue model
//! - [`circadian`] — 24-hour alertness cycle with chronotype (Borbély two-process)
//! - [`flow`] — Flow state detection with hysteresis (Csikszentmihalyi)
//! - [`salience`] — Somatic marker urgency/importance scoring (Damasio)
//! - [`actr`] — ACT-R frequency × recency memory activation with Hebbian boost
//! - [`preference`] — Adaptive preference learning from interaction outcomes
//! - [`belief`] — Belief system — memories crystallize into beliefs, beliefs form self-concept, self-understanding deepens into cosmic understanding
//! - [`intuition`] — Subconscious pattern integration — gut feelings from converging subsystems
//! - [`compat`] — Jantu creature behavior integration (feature: `instinct`)
//! - [`error`] — Error types

pub mod error;

#[cfg(feature = "traits")]
pub mod traits;

#[cfg(feature = "mood")]
pub mod mood;

#[cfg(feature = "archetype")]
pub mod archetype;

#[cfg(feature = "sentiment")]
pub mod sentiment;

#[cfg(feature = "presets")]
pub mod presets;

#[cfg(feature = "archetype")]
pub mod spirit;

#[cfg(feature = "mood")]
pub mod relationship;

#[cfg(feature = "mood")]
pub mod appraisal;

#[cfg(feature = "mood")]
pub mod stress;

#[cfg(feature = "mood")]
pub mod regulation;

#[cfg(all(feature = "mood", feature = "traits"))]
pub mod growth;

#[cfg(feature = "sentiment")]
pub mod monitor;

#[cfg(feature = "mood")]
pub mod rhythm;

#[cfg(feature = "mood")]
pub mod microexpr;

#[cfg(feature = "mood")]
pub mod affective;

#[cfg(feature = "mood")]
pub mod proximity;

#[cfg(all(feature = "mood", feature = "traits"))]
pub mod reasoning;

#[cfg(feature = "mood")]
pub mod active_hours;

#[cfg(feature = "mood")]
pub mod eq;

#[cfg(feature = "mood")]
pub mod display_rules;

#[cfg(feature = "mood")]
pub mod energy;

#[cfg(feature = "mood")]
pub mod circadian;

#[cfg(feature = "mood")]
pub mod flow;

#[cfg(feature = "mood")]
pub mod salience;

#[cfg(feature = "mood")]
pub mod actr;

#[cfg(feature = "mood")]
pub mod preference;

#[cfg(all(feature = "mood", feature = "traits"))]
pub mod belief;

#[cfg(all(feature = "mood", feature = "traits"))]
pub mod intuition;

#[cfg(feature = "instinct")]
pub mod compat;

#[cfg(feature = "ai")]
pub mod ai;

// Storage trait (available when core features are on)
#[cfg(all(
    feature = "traits",
    feature = "mood",
    feature = "archetype",
    feature = "sentiment"
))]
pub mod store;

#[cfg(feature = "sqlite")]
pub mod storage;

pub use error::BhavaError;
