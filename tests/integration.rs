//! Integration tests — cross-module behavior.

#[test]
fn test_preset_generates_valid_prompt() {
    let preset = bhava::presets::get_preset("blue-shirt-guy").unwrap();
    let prompt = preset.profile.compose_prompt();
    let identity = bhava::archetype::compose_identity_prompt(&preset.identity);

    // Both should produce non-empty output
    assert!(!prompt.is_empty());
    assert!(!identity.is_empty());

    // Identity should contain the archetype preamble
    assert!(identity.contains("In Our Image"));

    // Personality prompt should have behavioral instructions
    assert!(prompt.contains("## Personality"));
}

#[test]
fn test_sentiment_affects_mood() {
    let result = bhava::sentiment::analyze("This is terrible and broken!");
    assert!(result.is_negative());

    let mut state = bhava::mood::EmotionalState::new();
    // Apply sentiment to mood
    for (emotion, intensity) in &result.emotions {
        state.stimulate(*emotion, *intensity);
    }
    assert!(state.deviation() > 0.0);
}

#[test]
fn test_personality_distance_presets() {
    let guy = bhava::presets::get_preset("blue-shirt-guy").unwrap();
    let tron = bhava::presets::get_preset("t-ron").unwrap();
    let friday = bhava::presets::get_preset("friday").unwrap();

    // BlueShirtGuy and T.Ron should be far apart (opposite personalities)
    let guy_tron_distance = guy.profile.distance(&tron.profile);
    // Friday should be closer to T.Ron than BlueShirtGuy is
    let friday_tron_distance = friday.profile.distance(&tron.profile);

    assert!(guy_tron_distance > friday_tron_distance);
}
