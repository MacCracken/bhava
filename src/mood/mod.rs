//! Emotional state vectors with time-based decay.
//!
//! Models emotion as a multidimensional vector that shifts in response to
//! events and decays toward a baseline over time.

mod baseline;
mod contagion;
mod damping;
mod history;
mod memory;
mod plutchik;
mod prompt;
mod triggers;
mod types;

// Re-export all public types and functions
pub use self::baseline::AdaptiveBaseline;
#[cfg(feature = "traits")]
pub use self::baseline::{derive_mood_baseline, emotion_amplifier, mood_trait_influence};
#[cfg(feature = "traits")]
pub use self::contagion::contagion_from_personality;
pub use self::contagion::{ContagionParams, compute_contagion, group_mood};
pub use self::damping::DampedResponse;
pub use self::history::{MoodHistory, MoodSnapshot};
pub use self::memory::{EmotionalMemory, EmotionalMemoryBank};
pub use self::plutchik::{CompoundEmotion, detect_compound_emotions};
pub use self::prompt::{ActionTendency, action_tendency, compose_mood_prompt, mood_tone_guide};
pub use self::triggers::{
    MoodTrigger, trigger_criticized, trigger_praised, trigger_surprised, trigger_threatened,
};
pub use self::types::{ActiveCause, Emotion, EmotionalState, MoodState, MoodVector};

#[cfg(test)]
mod tests;
