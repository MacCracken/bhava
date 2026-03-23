use serde::{Deserialize, Serialize};

use super::types::Emotion;

// --- Mood Triggers (v0.3) ---

/// A stimulus-response mapping: a named event that affects multiple emotions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodTrigger {
    /// Trigger name (e.g., "praised", "criticized", "surprised").
    pub name: String,
    /// Emotion responses: each pair is (emotion, intensity delta).
    pub responses: Vec<(Emotion, f32)>,
}

impl MoodTrigger {
    /// Create a new trigger with the given name and no responses.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            responses: Vec::new(),
        }
    }

    /// Add an emotion response to this trigger.
    pub fn respond(mut self, emotion: Emotion, intensity: f32) -> Self {
        self.responses.push((emotion, intensity));
        self
    }
}

/// Built-in trigger presets for common emotional stimuli.
pub fn trigger_praised() -> MoodTrigger {
    MoodTrigger::new("praised")
        .respond(Emotion::Joy, 0.4)
        .respond(Emotion::Dominance, 0.2)
        .respond(Emotion::Trust, 0.1)
}

/// Built-in trigger: criticized (joy-, dominance-, frustration+).
pub fn trigger_criticized() -> MoodTrigger {
    MoodTrigger::new("criticized")
        .respond(Emotion::Joy, -0.3)
        .respond(Emotion::Dominance, -0.2)
        .respond(Emotion::Frustration, 0.3)
}

/// Built-in trigger: surprised (arousal+, interest+).
pub fn trigger_surprised() -> MoodTrigger {
    MoodTrigger::new("surprised")
        .respond(Emotion::Arousal, 0.5)
        .respond(Emotion::Interest, 0.3)
}

/// Built-in trigger: threatened (arousal+, trust-, dominance-, frustration+).
pub fn trigger_threatened() -> MoodTrigger {
    MoodTrigger::new("threatened")
        .respond(Emotion::Arousal, 0.4)
        .respond(Emotion::Trust, -0.4)
        .respond(Emotion::Dominance, -0.3)
        .respond(Emotion::Frustration, 0.2)
}
