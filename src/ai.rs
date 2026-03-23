//! AI integration — personality-aware prompt building and sentiment feedback.
//!
//! Provides utilities for integrating bhava's personality and mood systems
//! with agnosai (agent orchestration) and hoosh (inference gateway):
//!
//! - **Prompt composition**: Build system prompts from personality + mood + identity
//! - **Sentiment feedback**: Analyze AI responses and feed back into emotional state
//! - **Agent metadata**: Export personality data for agent registration

use serde::{Deserialize, Serialize};

use crate::archetype::{IdentityContent, compose_identity_prompt};
use crate::mood::{EmotionalState, compose_mood_prompt};
use crate::sentiment;
use crate::traits::PersonalityProfile;

/// Configuration for AI integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Hoosh inference endpoint.
    pub hoosh_endpoint: String,
    /// Whether to inject mood context into system prompts.
    pub inject_mood: bool,
    /// Whether to run sentiment feedback on responses.
    pub sentiment_feedback: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            hoosh_endpoint: "http://localhost:8088".into(),
            inject_mood: true,
            sentiment_feedback: true,
        }
    }
}

/// Compose a complete system prompt from personality, identity, mood, and optional spirit.
///
/// This is the main integration point — produces a prompt string suitable for
/// injection into `InferenceRequest.system` (hoosh) or `AgentDefinition.backstory` (agnosai).
///
/// Sections included (in order):
/// 1. Identity prompt (archetype preamble + layer content)
/// 2. Personality disposition (trait behavioral instructions)
/// 3. Current mood state + tone guide (if `inject_mood` is true)
/// 4. Spirit content (if provided)
#[must_use]
pub fn compose_system_prompt(
    profile: &PersonalityProfile,
    identity: &IdentityContent,
    mood: Option<&EmotionalState>,
    spirit: Option<&str>,
) -> String {
    let mut prompt = compose_identity_prompt(identity);

    let disposition = profile.compose_prompt();
    if !disposition.is_empty() {
        prompt.push('\n');
        prompt.push_str(&disposition);
    }

    if let Some(state) = mood {
        prompt.push('\n');
        prompt.push_str(&compose_mood_prompt(state));
    }

    if let Some(spirit_text) = spirit.filter(|s| !s.is_empty()) {
        prompt.push_str("\n## Spirit\n\n");
        prompt.push_str(spirit_text);
        prompt.push('\n');
    }

    prompt
}

/// Analyze an AI response and apply sentiment feedback to the emotional state.
///
/// This closes the sentiment feedback loop:
/// 1. Analyze the response text for sentiment
/// 2. Apply detected emotions as stimuli to the emotional state
/// 3. Return the analysis result for logging/tracking
///
/// The `scale` parameter controls how strongly the feedback affects mood
/// (0.0 = no effect, 1.0 = full strength).
pub fn apply_sentiment_feedback(
    response_text: &str,
    state: &mut EmotionalState,
    scale: f32,
) -> sentiment::SentimentResult {
    let result = sentiment::analyze(response_text);
    let scale = scale.clamp(0.0, 1.0);

    for &(emotion, intensity) in &result.emotions {
        state.stimulate(emotion, intensity * scale);
    }

    result
}

/// Export personality metadata for agent registration.
///
/// Produces a JSON-compatible struct suitable for inclusion in
/// `AgentDefinition.backstory` or task context metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityMetadata {
    /// Agent name.
    pub name: String,
    /// Profile description.
    pub description: Option<String>,
    /// Active (non-balanced) traits with their levels.
    pub active_traits: Vec<(String, String)>,
    /// Current mood state label.
    pub mood_state: Option<String>,
    /// Trait group averages.
    pub group_averages: Vec<(String, f32)>,
}

/// Build personality metadata for agent registration.
#[must_use]
pub fn build_personality_metadata(
    profile: &PersonalityProfile,
    mood: Option<&EmotionalState>,
) -> PersonalityMetadata {
    use crate::traits::TraitGroup;

    let active_traits = profile
        .active_traits()
        .into_iter()
        .map(|tv| (tv.trait_name.to_string(), tv.level.to_string()))
        .collect();

    let group_averages = TraitGroup::ALL
        .iter()
        .map(|&g| (g.to_string(), profile.group_average(g)))
        .collect();

    let mood_state = mood.map(|s| s.classify().to_string());

    PersonalityMetadata {
        name: profile.name.clone(),
        description: profile.description.clone(),
        active_traits,
        mood_state,
        group_averages,
    }
}

/// Apply a mood trigger based on interaction outcome.
///
/// Convenience function that maps common interaction results to mood triggers.
pub fn feedback_from_outcome(state: &mut EmotionalState, outcome: InteractionOutcome) {
    match outcome {
        InteractionOutcome::Praised => state.apply_trigger(&crate::mood::trigger_praised()),
        InteractionOutcome::Criticized => state.apply_trigger(&crate::mood::trigger_criticized()),
        InteractionOutcome::Surprised => state.apply_trigger(&crate::mood::trigger_surprised()),
        InteractionOutcome::Threatened => state.apply_trigger(&crate::mood::trigger_threatened()),
        InteractionOutcome::Neutral => {}
    }
}

/// Common interaction outcomes that affect mood.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InteractionOutcome {
    Praised,
    Criticized,
    Surprised,
    Threatened,
    Neutral,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetype::IdentityLayer;
    use crate::mood::Emotion;
    use crate::traits::{TraitKind, TraitLevel};

    fn test_profile() -> PersonalityProfile {
        let mut p = PersonalityProfile::new("TestBot");
        p.set_trait(TraitKind::Humor, TraitLevel::High);
        p.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        p
    }

    fn test_identity() -> IdentityContent {
        let mut id = IdentityContent::default();
        id.set(IdentityLayer::Soul, "You are a helpful test bot.");
        id
    }

    #[test]
    fn test_ai_config_default() {
        let c = AiConfig::default();
        assert_eq!(c.hoosh_endpoint, "http://localhost:8088");
        assert!(c.inject_mood);
        assert!(c.sentiment_feedback);
    }

    #[test]
    fn test_ai_config_serde() {
        let c = AiConfig::default();
        let json = serde_json::to_string(&c).unwrap();
        let c2: AiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(c2.hoosh_endpoint, c.hoosh_endpoint);
    }

    #[test]
    fn test_compose_system_prompt_basic() {
        let profile = test_profile();
        let identity = test_identity();
        let prompt = compose_system_prompt(&profile, &identity, None, None);
        assert!(prompt.contains("In Our Image"));
        assert!(prompt.contains("### Soul"));
        assert!(prompt.contains("## Personality"));
    }

    #[test]
    fn test_compose_system_prompt_with_mood() {
        let profile = test_profile();
        let identity = test_identity();
        let mut mood = EmotionalState::new();
        mood.stimulate(Emotion::Joy, 0.8);
        let prompt = compose_system_prompt(&profile, &identity, Some(&mood), None);
        assert!(prompt.contains("Current Mood"));
    }

    #[test]
    fn test_compose_system_prompt_with_spirit() {
        let profile = test_profile();
        let identity = test_identity();
        let spirit = "I am driven by curiosity and a love of learning.";
        let prompt = compose_system_prompt(&profile, &identity, None, Some(spirit));
        assert!(prompt.contains("## Spirit"));
        assert!(prompt.contains("curiosity"));
    }

    #[test]
    fn test_compose_system_prompt_full() {
        let profile = test_profile();
        let identity = test_identity();
        let mut mood = EmotionalState::new();
        mood.stimulate(Emotion::Joy, 0.5);
        let spirit = "Passionate about helping.";
        let prompt = compose_system_prompt(&profile, &identity, Some(&mood), Some(spirit));
        assert!(prompt.contains("In Our Image"));
        assert!(prompt.contains("## Personality"));
        assert!(prompt.contains("Current Mood"));
        assert!(prompt.contains("## Spirit"));
    }

    #[test]
    fn test_apply_sentiment_feedback_positive() {
        let mut state = EmotionalState::new();
        let result =
            apply_sentiment_feedback("This is wonderful and amazing work!", &mut state, 1.0);
        assert!(result.is_positive());
        assert!(state.mood.joy > 0.0);
    }

    #[test]
    fn test_apply_sentiment_feedback_negative() {
        let mut state = EmotionalState::new();
        let result = apply_sentiment_feedback("This is terrible and broken.", &mut state, 1.0);
        assert!(result.is_negative());
        assert!(state.mood.joy < 0.0);
    }

    #[test]
    fn test_apply_sentiment_feedback_scaled() {
        let mut full = EmotionalState::new();
        let mut half = EmotionalState::new();
        apply_sentiment_feedback("This is great!", &mut full, 1.0);
        apply_sentiment_feedback("This is great!", &mut half, 0.5);
        assert!(full.mood.joy > half.mood.joy);
    }

    #[test]
    fn test_apply_sentiment_feedback_zero_scale() {
        let mut state = EmotionalState::new();
        apply_sentiment_feedback("Amazing wonderful fantastic!", &mut state, 0.0);
        assert!(state.deviation() < f32::EPSILON);
    }

    #[test]
    fn test_build_personality_metadata() {
        let profile = test_profile();
        let meta = build_personality_metadata(&profile, None);
        assert_eq!(meta.name, "TestBot");
        assert!(!meta.active_traits.is_empty());
        assert!(meta.mood_state.is_none());
        assert_eq!(meta.group_averages.len(), 4);
    }

    #[test]
    fn test_build_personality_metadata_with_mood() {
        let profile = test_profile();
        let mut mood = EmotionalState::new();
        mood.stimulate(Emotion::Joy, 0.8);
        let meta = build_personality_metadata(&profile, Some(&mood));
        assert!(meta.mood_state.is_some());
    }

    #[test]
    fn test_personality_metadata_serde() {
        let profile = test_profile();
        let meta = build_personality_metadata(&profile, None);
        let json = serde_json::to_string(&meta).unwrap();
        let meta2: PersonalityMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(meta2.name, "TestBot");
    }

    #[test]
    fn test_feedback_from_outcome() {
        let mut state = EmotionalState::new();
        feedback_from_outcome(&mut state, InteractionOutcome::Praised);
        assert!(state.mood.joy > 0.0);

        let mut state2 = EmotionalState::new();
        feedback_from_outcome(&mut state2, InteractionOutcome::Criticized);
        assert!(state2.mood.joy < 0.0);
    }

    #[test]
    fn test_feedback_neutral_noop() {
        let mut state = EmotionalState::new();
        feedback_from_outcome(&mut state, InteractionOutcome::Neutral);
        assert!(state.deviation() < f32::EPSILON);
    }

    #[test]
    fn test_interaction_outcome_serde() {
        for outcome in [
            InteractionOutcome::Praised,
            InteractionOutcome::Criticized,
            InteractionOutcome::Surprised,
            InteractionOutcome::Threatened,
            InteractionOutcome::Neutral,
        ] {
            let json = serde_json::to_string(&outcome).unwrap();
            let restored: InteractionOutcome = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, outcome);
        }
    }
}
