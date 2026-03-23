use serde::{Deserialize, Serialize};

use super::core::{Emotion, MoodVector};

// --- Emotional Memory ---

/// A somatic marker — an emotional memory associated with a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalMemory {
    /// What this memory is associated with (entity_id, location, event_type).
    pub tag: String,
    /// The emotional state recorded.
    pub mood: MoodVector,
    /// Strength of the memory (decays over time, 0.0–1.0).
    pub intensity: f32,
}

/// Bank of emotional memories — agents remember how things made them feel.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmotionalMemoryBank {
    memories: Vec<EmotionalMemory>,
    capacity: usize,
}

impl EmotionalMemoryBank {
    /// Create a memory bank with the given capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            memories: Vec::new(),
            capacity: capacity.max(1),
        }
    }

    /// Record an emotional memory. Overwrites if tag already exists.
    pub fn record(&mut self, tag: impl Into<String>, mood: &MoodVector, intensity: f32) {
        let tag = tag.into();
        if let Some(existing) = self.memories.iter_mut().find(|m| m.tag == tag) {
            existing.mood = mood.clone();
            existing.intensity = intensity.clamp(0.0, 1.0);
        } else {
            if self.memories.len() >= self.capacity {
                // Evict weakest memory
                if let Some(weakest) = self
                    .memories
                    .iter()
                    .enumerate()
                    .min_by(|a, b| a.1.intensity.partial_cmp(&b.1.intensity).unwrap())
                    .map(|(i, _)| i)
                {
                    self.memories.swap_remove(weakest);
                }
            }
            self.memories.push(EmotionalMemory {
                tag,
                mood: mood.clone(),
                intensity: intensity.clamp(0.0, 1.0),
            });
        }
    }

    /// Recall the emotional memory for a tag, attenuated by intensity.
    #[must_use]
    pub fn recall(&self, tag: &str) -> Option<MoodVector> {
        self.memories.iter().find(|m| m.tag == tag).map(|m| {
            let mut recalled = m.mood.clone();
            for &e in Emotion::ALL {
                recalled.set(e, recalled.get(e) * m.intensity);
            }
            recalled
        })
    }

    /// Decay all memory intensities.
    pub fn decay(&mut self, rate: f32) {
        let r = rate.clamp(0.0, 1.0);
        self.memories.retain_mut(|m| {
            m.intensity *= 1.0 - r;
            m.intensity > 0.01
        });
    }

    /// Number of stored memories.
    #[must_use]
    pub fn len(&self) -> usize {
        self.memories.len()
    }

    /// Whether the bank is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.memories.is_empty()
    }
}
