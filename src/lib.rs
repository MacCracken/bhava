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
//! - [`traits`] — Personality trait spectrums (formality, humor, warmth, etc.)
//! - [`mood`] — Emotional state vectors with time-based decay
//! - [`archetype`] — Identity hierarchy (Soul/Spirit/Brain/Body/Heart)
//! - [`sentiment`] — Basic emotion classification from text
//! - [`presets`] — Built-in personality templates (BlueShirtGuy, T.Ron, etc.)
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

#[cfg(feature = "ai")]
pub mod ai;

pub use error::BhavaError;
