//! Example: Create an NPC with a personality preset and simulate emotional state.

fn main() {
    // Load BlueShirtGuy preset
    let preset = bhava::presets::get_preset("blue-shirt-guy").unwrap();
    println!("Loaded preset: {} — {}", preset.name, preset.summary);

    // Generate behavioral instructions
    let prompt = preset.profile.compose_prompt();
    println!("\n{prompt}");

    // Generate identity prompt
    let identity_prompt = bhava::archetype::compose_identity_prompt(&preset.identity);
    println!("{identity_prompt}");

    // Simulate emotional state
    let mut state = bhava::mood::EmotionalState::new();
    state.stimulate(bhava::mood::Emotion::Joy, 0.8);
    state.stimulate(bhava::mood::Emotion::Interest, 0.6);
    println!("Mood intensity: {:.2}", state.mood.intensity());
    println!("Dominant emotion: {}", state.mood.dominant_emotion());

    // Analyze some text
    let result = bhava::sentiment::analyze("This is wonderful! I love helping people!");
    println!("\nSentiment: valence={:.2}, positive={}", result.valence, result.is_positive());
}
