//! SQLite persistence — `BhavaStore` implementation backed by SQLite.
//!
//! Provides `SqliteStore` for persisting personality profiles, emotional states,
//! mood history, relationships, and spirit data to a SQLite database.
//!
//! For other backends, implement the `BhavaStore` trait from the `store` module.

use rusqlite::{Connection, params};
use serde_json;

use crate::error::{BhavaError, Result};
use crate::mood::{EmotionalState, MoodHistory, MoodSnapshot};
use crate::relationship::RelationshipGraph;
use crate::spirit::Spirit;
use crate::store::BhavaStore;
use crate::traits::PersonalityProfile;

/// SQLite-backed persistence store for bhava state.
pub struct SqliteStore {
    conn: Connection,
}

impl SqliteStore {
    /// Open or create a SQLite database at the given path.
    ///
    /// # Errors
    /// Returns `BhavaError::Storage` if the database cannot be opened or tables cannot be created.
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path).map_err(|e| BhavaError::Storage(e.to_string()))?;
        let store = Self { conn };
        store.init_tables()?;
        Ok(store)
    }

    /// Create an in-memory database (useful for testing).
    ///
    /// # Errors
    /// Returns `BhavaError::Storage` if tables cannot be created.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().map_err(|e| BhavaError::Storage(e.to_string()))?;
        let store = Self { conn };
        store.init_tables()?;
        Ok(store)
    }

    fn init_tables(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS personality_profiles (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL,
                    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                CREATE TABLE IF NOT EXISTS emotional_states (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL,
                    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                CREATE TABLE IF NOT EXISTS mood_histories (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL,
                    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                CREATE TABLE IF NOT EXISTS relationship_graphs (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL,
                    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                CREATE TABLE IF NOT EXISTS spirits (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL,
                    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
                );
                CREATE TABLE IF NOT EXISTS mood_snapshots (
                    id TEXT NOT NULL,
                    seq INTEGER NOT NULL,
                    data TEXT NOT NULL,
                    created_at TEXT NOT NULL DEFAULT (datetime('now')),
                    PRIMARY KEY (id, seq)
                );",
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))
    }

    // --- PersonalityProfile ---

    /// Save a personality profile.
    pub fn save_profile(&self, id: &str, profile: &PersonalityProfile) -> Result<()> {
        let data =
            serde_json::to_string(profile).map_err(|e| BhavaError::Storage(e.to_string()))?;
        self.conn
            .execute(
                "INSERT OR REPLACE INTO personality_profiles (id, data, updated_at) VALUES (?1, ?2, datetime('now'))",
                params![id, data],
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Load a personality profile.
    #[must_use]
    pub fn load_profile(&self, id: &str) -> Option<PersonalityProfile> {
        self.conn
            .query_row(
                "SELECT data FROM personality_profiles WHERE id = ?1",
                params![id],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
    }

    /// Delete a personality profile.
    pub fn delete_profile(&self, id: &str) -> Result<bool> {
        let affected = self
            .conn
            .execute(
                "DELETE FROM personality_profiles WHERE id = ?1",
                params![id],
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        Ok(affected > 0)
    }

    /// List all stored profile IDs.
    #[must_use]
    pub fn list_profile_ids(&self) -> Vec<String> {
        let mut stmt = match self
            .conn
            .prepare("SELECT id FROM personality_profiles ORDER BY id")
        {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        stmt.query_map([], |row| row.get(0))
            .ok()
            .map(|rows| rows.filter_map(|r| r.ok()).collect())
            .unwrap_or_default()
    }

    // --- EmotionalState ---

    /// Save an emotional state.
    pub fn save_emotional_state(&self, id: &str, state: &EmotionalState) -> Result<()> {
        let data = serde_json::to_string(state).map_err(|e| BhavaError::Storage(e.to_string()))?;
        self.conn
            .execute(
                "INSERT OR REPLACE INTO emotional_states (id, data, updated_at) VALUES (?1, ?2, datetime('now'))",
                params![id, data],
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Load an emotional state.
    #[must_use]
    pub fn load_emotional_state(&self, id: &str) -> Option<EmotionalState> {
        self.conn
            .query_row(
                "SELECT data FROM emotional_states WHERE id = ?1",
                params![id],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
    }

    // --- MoodHistory ---

    /// Save a mood history.
    pub fn save_mood_history(&self, id: &str, history: &MoodHistory) -> Result<()> {
        let data =
            serde_json::to_string(history).map_err(|e| BhavaError::Storage(e.to_string()))?;
        self.conn
            .execute(
                "INSERT OR REPLACE INTO mood_histories (id, data, updated_at) VALUES (?1, ?2, datetime('now'))",
                params![id, data],
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Load a mood history.
    #[must_use]
    pub fn load_mood_history(&self, id: &str) -> Option<MoodHistory> {
        self.conn
            .query_row(
                "SELECT data FROM mood_histories WHERE id = ?1",
                params![id],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
    }

    /// Append a single mood snapshot (for streaming/incremental writes).
    pub fn append_snapshot(&self, id: &str, snapshot: &MoodSnapshot) -> Result<()> {
        let data =
            serde_json::to_string(snapshot).map_err(|e| BhavaError::Storage(e.to_string()))?;
        let seq: i64 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(seq), -1) + 1 FROM mood_snapshots WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        self.conn
            .execute(
                "INSERT INTO mood_snapshots (id, seq, data, created_at) VALUES (?1, ?2, ?3, datetime('now'))",
                params![id, seq, data],
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Load recent mood snapshots.
    pub fn load_snapshots(&self, id: &str, limit: usize) -> Vec<MoodSnapshot> {
        let mut stmt = match self
            .conn
            .prepare("SELECT data FROM mood_snapshots WHERE id = ?1 ORDER BY seq DESC LIMIT ?2")
        {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let mut snapshots: Vec<MoodSnapshot> = stmt
            .query_map(params![id, limit as i64], |row| row.get::<_, String>(0))
            .ok()
            .map(|rows| {
                rows.filter_map(|r| r.ok())
                    .filter_map(|data| serde_json::from_str(&data).ok())
                    .collect()
            })
            .unwrap_or_default();
        snapshots.reverse(); // oldest first
        snapshots
    }

    // --- RelationshipGraph ---

    /// Save a relationship graph.
    pub fn save_relationships(&self, id: &str, graph: &RelationshipGraph) -> Result<()> {
        let data = serde_json::to_string(graph).map_err(|e| BhavaError::Storage(e.to_string()))?;
        self.conn
            .execute(
                "INSERT OR REPLACE INTO relationship_graphs (id, data, updated_at) VALUES (?1, ?2, datetime('now'))",
                params![id, data],
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Load a relationship graph.
    #[must_use]
    pub fn load_relationships(&self, id: &str) -> Option<RelationshipGraph> {
        self.conn
            .query_row(
                "SELECT data FROM relationship_graphs WHERE id = ?1",
                params![id],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
    }

    // --- Spirit ---

    /// Save a spirit.
    pub fn save_spirit(&self, id: &str, spirit: &Spirit) -> Result<()> {
        let data = serde_json::to_string(spirit).map_err(|e| BhavaError::Storage(e.to_string()))?;
        self.conn
            .execute(
                "INSERT OR REPLACE INTO spirits (id, data, updated_at) VALUES (?1, ?2, datetime('now'))",
                params![id, data],
            )
            .map_err(|e| BhavaError::Storage(e.to_string()))?;
        Ok(())
    }

    /// Load a spirit.
    #[must_use]
    pub fn load_spirit(&self, id: &str) -> Option<Spirit> {
        self.conn
            .query_row(
                "SELECT data FROM spirits WHERE id = ?1",
                params![id],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
    }
}

impl BhavaStore for SqliteStore {
    fn save_profile(&self, id: &str, profile: &PersonalityProfile) -> Result<()> {
        SqliteStore::save_profile(self, id, profile)
    }
    fn load_profile(&self, id: &str) -> Option<PersonalityProfile> {
        SqliteStore::load_profile(self, id)
    }
    fn delete_profile(&self, id: &str) -> Result<bool> {
        SqliteStore::delete_profile(self, id)
    }
    fn list_profile_ids(&self) -> Vec<String> {
        SqliteStore::list_profile_ids(self)
    }
    fn save_emotional_state(&self, id: &str, state: &EmotionalState) -> Result<()> {
        SqliteStore::save_emotional_state(self, id, state)
    }
    fn load_emotional_state(&self, id: &str) -> Option<EmotionalState> {
        SqliteStore::load_emotional_state(self, id)
    }
    fn save_mood_history(&self, id: &str, history: &MoodHistory) -> Result<()> {
        SqliteStore::save_mood_history(self, id, history)
    }
    fn load_mood_history(&self, id: &str) -> Option<MoodHistory> {
        SqliteStore::load_mood_history(self, id)
    }
    fn append_snapshot(&self, id: &str, snapshot: &MoodSnapshot) -> Result<()> {
        SqliteStore::append_snapshot(self, id, snapshot)
    }
    fn load_snapshots(&self, id: &str, limit: usize) -> Vec<MoodSnapshot> {
        SqliteStore::load_snapshots(self, id, limit)
    }
    fn save_relationships(&self, id: &str, graph: &RelationshipGraph) -> Result<()> {
        SqliteStore::save_relationships(self, id, graph)
    }
    fn load_relationships(&self, id: &str) -> Option<RelationshipGraph> {
        SqliteStore::load_relationships(self, id)
    }
    fn save_spirit(&self, id: &str, spirit: &Spirit) -> Result<()> {
        SqliteStore::save_spirit(self, id, spirit)
    }
    fn load_spirit(&self, id: &str) -> Option<Spirit> {
        SqliteStore::load_spirit(self, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mood::{Emotion, MoodHistory};
    use crate::traits::{TraitKind, TraitLevel};

    fn test_store() -> SqliteStore {
        SqliteStore::open_in_memory().unwrap()
    }

    // --- Profile ---

    #[test]
    fn test_save_load_profile() {
        let store = test_store();
        let mut p = PersonalityProfile::new("TestBot");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        store.save_profile("bot1", &p).unwrap();

        let loaded = store.load_profile("bot1").unwrap();
        assert_eq!(loaded.name, "TestBot");
        assert_eq!(loaded.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_load_missing_profile() {
        let store = test_store();
        assert!(store.load_profile("nonexistent").is_none());
    }

    #[test]
    fn test_delete_profile() {
        let store = test_store();
        let p = PersonalityProfile::new("test");
        store.save_profile("x", &p).unwrap();
        assert!(store.delete_profile("x").unwrap());
        assert!(store.load_profile("x").is_none());
        assert!(!store.delete_profile("x").unwrap());
    }

    #[test]
    fn test_list_profile_ids() {
        let store = test_store();
        store
            .save_profile("b", &PersonalityProfile::new("B"))
            .unwrap();
        store
            .save_profile("a", &PersonalityProfile::new("A"))
            .unwrap();
        let ids = store.list_profile_ids();
        assert_eq!(ids, vec!["a", "b"]);
    }

    #[test]
    fn test_overwrite_profile() {
        let store = test_store();
        let mut p1 = PersonalityProfile::new("v1");
        p1.set_trait(TraitKind::Humor, TraitLevel::Low);
        store.save_profile("x", &p1).unwrap();

        let mut p2 = PersonalityProfile::new("v2");
        p2.set_trait(TraitKind::Humor, TraitLevel::Highest);
        store.save_profile("x", &p2).unwrap();

        let loaded = store.load_profile("x").unwrap();
        assert_eq!(loaded.name, "v2");
        assert_eq!(loaded.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    // --- EmotionalState ---

    #[test]
    fn test_save_load_emotional_state() {
        let store = test_store();
        let mut state = EmotionalState::new();
        state.stimulate(Emotion::Joy, 0.7);
        store.save_emotional_state("agent1", &state).unwrap();

        let loaded = store.load_emotional_state("agent1").unwrap();
        assert!((loaded.mood.joy - state.mood.joy).abs() < 0.01);
    }

    // --- MoodHistory ---

    #[test]
    fn test_save_load_mood_history() {
        let store = test_store();
        let mut history = MoodHistory::new(10);
        let state = EmotionalState::new();
        history.record(state.snapshot());
        history.record(state.snapshot());
        store.save_mood_history("agent1", &history).unwrap();

        let loaded = store.load_mood_history("agent1").unwrap();
        assert_eq!(loaded.len(), 2);
    }

    // --- Snapshots ---

    #[test]
    fn test_append_and_load_snapshots() {
        let store = test_store();
        let mut state = EmotionalState::new();
        store.append_snapshot("a", &state.snapshot()).unwrap();
        state.stimulate(Emotion::Joy, 0.5);
        store.append_snapshot("a", &state.snapshot()).unwrap();
        state.stimulate(Emotion::Frustration, 0.3);
        store.append_snapshot("a", &state.snapshot()).unwrap();

        let snaps = store.load_snapshots("a", 10);
        assert_eq!(snaps.len(), 3);
        // First should be calm, last should have frustration
        assert!(snaps[2].deviation > snaps[0].deviation);
    }

    #[test]
    fn test_load_snapshots_with_limit() {
        let store = test_store();
        let state = EmotionalState::new();
        for _ in 0..10 {
            store.append_snapshot("a", &state.snapshot()).unwrap();
        }
        let snaps = store.load_snapshots("a", 3);
        assert_eq!(snaps.len(), 3);
    }

    // --- RelationshipGraph ---

    #[test]
    fn test_save_load_relationships() {
        let store = test_store();
        let mut graph = RelationshipGraph::new();
        graph.record_interaction("alice", "bob", 0.5, 0.2);
        store.save_relationships("world1", &graph).unwrap();

        let loaded = store.load_relationships("world1").unwrap();
        assert_eq!(loaded.len(), 1);
        assert!(loaded.get("alice", "bob").is_some());
    }

    // --- Spirit ---

    #[test]
    fn test_save_load_spirit() {
        let store = test_store();
        let mut spirit = Spirit::new();
        spirit.add_passion("coding", "Writing elegant code", 0.9);
        store.save_spirit("agent1", &spirit).unwrap();

        let loaded = store.load_spirit("agent1").unwrap();
        assert_eq!(loaded.passions.len(), 1);
        assert_eq!(loaded.passions[0].name, "coding");
    }

    // --- Cross-type ---

    #[test]
    fn test_full_agent_persistence() {
        let store = test_store();
        let id = "agent42";

        // Save everything
        let mut profile = PersonalityProfile::new("Agent42");
        profile.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        store.save_profile(id, &profile).unwrap();

        let mut state = EmotionalState::new();
        state.stimulate(Emotion::Joy, 0.6);
        store.save_emotional_state(id, &state).unwrap();

        let mut history = MoodHistory::new(100);
        history.record(state.snapshot());
        store.save_mood_history(id, &history).unwrap();

        let mut graph = RelationshipGraph::new();
        graph.record_interaction(id, "user", 0.3, 0.1);
        store.save_relationships(id, &graph).unwrap();

        let mut spirit = Spirit::new();
        spirit.add_passion("helping", "Serving users", 0.8);
        store.save_spirit(id, &spirit).unwrap();

        // Load everything back
        assert!(store.load_profile(id).is_some());
        assert!(store.load_emotional_state(id).is_some());
        assert!(store.load_mood_history(id).is_some());
        assert!(store.load_relationships(id).is_some());
        assert!(store.load_spirit(id).is_some());
    }
}
