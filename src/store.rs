//! Storage trait — abstract persistence interface for bhava state.
//!
//! Implement `BhavaStore` for your database backend (SQLite, Postgres, Redis, etc.).
//! The `sqlite` feature provides a ready-made `SqliteStore` implementation.

use crate::error::Result;
use crate::mood::{EmotionalState, MoodHistory, MoodSnapshot};
use crate::relationship::RelationshipGraph;
use crate::spirit::Spirit;
use crate::traits::PersonalityProfile;

/// Abstract persistence interface for all bhava state.
///
/// Implement this trait to store personality profiles, emotional states,
/// mood history, relationships, and spirit data in your backend of choice.
///
/// All methods use string IDs as keys — the consumer decides the ID scheme
/// (UUID, agent name, etc.).
pub trait BhavaStore {
    /// Save a personality profile.
    fn save_profile(&self, id: &str, profile: &PersonalityProfile) -> Result<()>;

    /// Load a personality profile. Returns `None` if not found.
    fn load_profile(&self, id: &str) -> Option<PersonalityProfile>;

    /// Delete a personality profile. Returns true if it existed.
    fn delete_profile(&self, id: &str) -> Result<bool>;

    /// List all stored profile IDs.
    fn list_profile_ids(&self) -> Vec<String>;

    /// Save an emotional state.
    fn save_emotional_state(&self, id: &str, state: &EmotionalState) -> Result<()>;

    /// Load an emotional state. Returns `None` if not found.
    fn load_emotional_state(&self, id: &str) -> Option<EmotionalState>;

    /// Save a mood history.
    fn save_mood_history(&self, id: &str, history: &MoodHistory) -> Result<()>;

    /// Load a mood history. Returns `None` if not found.
    fn load_mood_history(&self, id: &str) -> Option<MoodHistory>;

    /// Append a single mood snapshot (for incremental writes).
    fn append_snapshot(&self, id: &str, snapshot: &MoodSnapshot) -> Result<()>;

    /// Load recent mood snapshots, oldest first.
    fn load_snapshots(&self, id: &str, limit: usize) -> Vec<MoodSnapshot>;

    /// Save a relationship graph.
    fn save_relationships(&self, id: &str, graph: &RelationshipGraph) -> Result<()>;

    /// Load a relationship graph. Returns `None` if not found.
    fn load_relationships(&self, id: &str) -> Option<RelationshipGraph>;

    /// Save a spirit.
    fn save_spirit(&self, id: &str, spirit: &Spirit) -> Result<()>;

    /// Load a spirit. Returns `None` if not found.
    fn load_spirit(&self, id: &str) -> Option<Spirit>;
}
