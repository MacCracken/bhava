//! Integration tests — cross-module behavior.

use bhava::archetype::{IdentityContent, IdentityLayer, compose_identity_prompt};
use bhava::mood::{Emotion, EmotionalState};
use bhava::presets::{get_preset, list_presets};
use bhava::sentiment;
use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};

// --- preset → prompt composition ---

#[test]
fn test_preset_generates_valid_prompt() {
    let preset = get_preset("blue-shirt-guy").unwrap();
    let prompt = preset.profile.compose_prompt();
    let identity = compose_identity_prompt(&preset.identity);

    assert!(!prompt.is_empty());
    assert!(!identity.is_empty());
    assert!(identity.contains("In Our Image"));
    assert!(prompt.contains("## Personality"));
}

#[test]
fn test_all_presets_generate_prompts() {
    for id in list_presets() {
        let preset = get_preset(id).unwrap();
        let personality_prompt = preset.profile.compose_prompt();
        let identity_prompt = compose_identity_prompt(&preset.identity);

        assert!(
            !personality_prompt.is_empty(),
            "{id} produced empty personality prompt"
        );
        assert!(
            identity_prompt.contains("### Soul"),
            "{id} identity missing Soul layer"
        );
    }
}

#[test]
fn test_preset_identity_includes_preamble_and_layers() {
    let preset = get_preset("t-ron").unwrap();
    let prompt = compose_identity_prompt(&preset.identity);

    assert!(prompt.contains("In Our Image"));
    assert!(prompt.contains("### Soul"));
    assert!(prompt.contains("### Spirit"));
    assert!(prompt.contains("T.Ron"));
}

// --- sentiment → mood ---

#[test]
fn test_sentiment_affects_mood() {
    let result = sentiment::analyze("This is terrible and broken!");
    assert!(result.is_negative());

    let mut state = EmotionalState::new();
    for (emotion, intensity) in &result.emotions {
        state.stimulate(*emotion, *intensity);
    }
    assert!(state.deviation() > 0.0);
}

#[test]
fn test_positive_sentiment_boosts_joy() {
    let result = sentiment::analyze("This is amazing and wonderful, I love it!");
    assert!(result.is_positive());

    let mut state = EmotionalState::new();
    for (emotion, intensity) in &result.emotions {
        state.stimulate(*emotion, *intensity);
    }
    assert!(state.mood.joy > 0.0);
}

#[test]
fn test_trust_sentiment_drives_mood() {
    let result = sentiment::analyze("I trust this system, it feels safe and reliable.");
    let mut state = EmotionalState::new();
    for (emotion, intensity) in &result.emotions {
        state.stimulate(*emotion, *intensity);
    }
    assert!(state.mood.trust > 0.0);
}

#[test]
fn test_frustration_sentiment_drives_mood() {
    let result = sentiment::analyze("I'm frustrated and stuck, this is broken.");
    let mut state = EmotionalState::new();
    for (emotion, intensity) in &result.emotions {
        state.stimulate(*emotion, *intensity);
    }
    assert!(state.mood.frustration > 0.0);
}

// --- personality distance ---

#[test]
fn test_personality_distance_presets() {
    let guy = get_preset("blue-shirt-guy").unwrap();
    let tron = get_preset("t-ron").unwrap();
    let friday = get_preset("friday").unwrap();

    let guy_tron_distance = guy.profile.distance(&tron.profile);
    let friday_tron_distance = friday.profile.distance(&tron.profile);

    assert!(guy_tron_distance > friday_tron_distance);
}

#[test]
fn test_personality_distance_symmetry() {
    let guy = get_preset("blue-shirt-guy").unwrap();
    let tron = get_preset("t-ron").unwrap();

    let d1 = guy.profile.distance(&tron.profile);
    let d2 = tron.profile.distance(&guy.profile);
    assert!((d1 - d2).abs() < f32::EPSILON);
}

#[test]
fn test_personality_distance_self_is_zero() {
    for id in list_presets() {
        let preset = get_preset(id).unwrap();
        assert!(
            preset.profile.distance(&preset.profile).abs() < f32::EPSILON,
            "{id} distance to self is not zero"
        );
    }
}

// --- multi-step emotional state ---

#[test]
fn test_multiple_stimulations_accumulate() {
    let mut state = EmotionalState::new();
    state.stimulate(Emotion::Joy, 0.3);
    state.stimulate(Emotion::Joy, 0.3);
    state.stimulate(Emotion::Joy, 0.3);
    assert!(state.mood.joy > 0.8);
}

#[test]
fn test_opposing_stimulations_cancel() {
    let mut state = EmotionalState::new();
    state.stimulate(Emotion::Joy, 0.5);
    state.stimulate(Emotion::Joy, -0.5);
    assert!(state.mood.joy.abs() < 0.01);
}

#[test]
fn test_decay_reduces_deviation() {
    let mut state = EmotionalState::new();
    state.stimulate(Emotion::Joy, 0.8);
    state.stimulate(Emotion::Frustration, 0.6);
    let before = state.deviation();

    let future = state.last_updated + chrono::Duration::minutes(10);
    state.apply_decay(future);
    let after = state.deviation();
    assert!(after < before, "deviation should decrease after decay");
}

// --- custom profile + identity composition ---

#[test]
fn test_custom_profile_with_full_identity() {
    let mut profile = PersonalityProfile::new("custom");
    profile.set_trait(TraitKind::Humor, TraitLevel::Highest);
    profile.set_trait(TraitKind::Warmth, TraitLevel::High);

    let mut identity = IdentityContent::default();
    identity.set(IdentityLayer::Soul, "The core.");
    identity.set(IdentityLayer::Spirit, "The drive.");
    identity.set(IdentityLayer::Brain, "The mind.");
    identity.set(IdentityLayer::Body, "The form.");
    identity.set(IdentityLayer::Heart, "The pulse.");

    let personality_prompt = profile.compose_prompt();
    let identity_prompt = compose_identity_prompt(&identity);

    assert!(personality_prompt.contains("funny"));
    assert!(identity_prompt.contains("### Soul"));
    assert!(identity_prompt.contains("### Heart"));
    assert_eq!(identity.populated_count(), 5);
}

// --- serde cross-module ---

#[test]
fn test_emotional_state_serde_roundtrip() {
    let mut state = EmotionalState::new();
    state.stimulate(Emotion::Joy, 0.7);
    state.stimulate(Emotion::Trust, -0.3);

    let json = serde_json::to_string(&state).unwrap();
    let restored: EmotionalState = serde_json::from_str(&json).unwrap();
    assert!((restored.mood.joy - state.mood.joy).abs() < 0.01);
    assert!((restored.mood.trust - state.mood.trust).abs() < 0.01);
    assert!((restored.decay_half_life_secs - state.decay_half_life_secs).abs() < 0.01);
}

#[test]
fn test_personality_profile_serde_with_description() {
    let mut profile = PersonalityProfile::new("test");
    profile.description = Some("A test profile".into());
    profile.set_trait(TraitKind::Humor, TraitLevel::Highest);
    profile.set_trait(TraitKind::Confidence, TraitLevel::Low);

    let json = serde_json::to_string(&profile).unwrap();
    let restored: PersonalityProfile = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.name, "test");
    assert_eq!(restored.description.as_deref(), Some("A test profile"));
    assert_eq!(restored.get_trait(TraitKind::Humor), TraitLevel::Highest);
    assert_eq!(restored.get_trait(TraitKind::Confidence), TraitLevel::Low);
}

#[test]
fn test_sentiment_result_serde_preserves_emotions() {
    let result = sentiment::analyze("I trust this and love how interesting it is!");
    let json = serde_json::to_string(&result).unwrap();
    let restored: bhava::sentiment::SentimentResult = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.emotions.len(), result.emotions.len());
    assert!((restored.valence - result.valence).abs() < 0.01);
    assert_eq!(restored.matched_keywords, result.matched_keywords);
}
