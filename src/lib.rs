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

#[cfg(feature = "ai")]
pub mod ai;

pub use error::BhavaError;
