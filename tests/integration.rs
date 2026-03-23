//! Integration tests — cross-module behavior.

use bhava::archetype::{
    CrewMember, IdentityContent, IdentityLayer, ValidationRules, compose_crew_prompt,
    compose_identity_prompt, get_template,
};
use bhava::mood::{
    Emotion, EmotionalState, MoodHistory, MoodState, mood_trait_influence, trigger_praised,
};
use bhava::presets::{get_preset, list_presets};
use bhava::sentiment;
use bhava::traits::{PersonalityProfile, TraitGroup, TraitKind, TraitLevel};

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

// --- v0.2: trait groups + compatibility ---

#[test]
fn test_preset_group_averages() {
    let guy = get_preset("blue-shirt-guy").unwrap();
    let tron = get_preset("t-ron").unwrap();
    // Guy should have higher social average than T.Ron
    assert!(
        guy.profile.group_average(TraitGroup::Social)
            > tron.profile.group_average(TraitGroup::Social)
    );
}

#[test]
fn test_blend_presets() {
    let guy = get_preset("blue-shirt-guy").unwrap();
    let tron = get_preset("t-ron").unwrap();
    let blended = guy.profile.blend(&tron.profile, 0.5);
    // Blended warmth should be between Guy's Highest and T.Ron's Low
    let warmth = blended.get_trait(TraitKind::Warmth);
    assert!(warmth > TraitLevel::Low);
    assert!(warmth < TraitLevel::Highest);
}

#[test]
fn test_compatibility_same_group_presets() {
    let guy = get_preset("blue-shirt-guy").unwrap();
    let oracle = get_preset("oracle").unwrap();
    let tron = get_preset("t-ron").unwrap();
    // Guy and Oracle are both warm/patient — higher social compatibility than Guy+T.Ron
    assert!(
        guy.profile
            .group_compatibility(&oracle.profile, TraitGroup::Social)
            > guy
                .profile
                .group_compatibility(&tron.profile, TraitGroup::Social)
    );
}

// --- v0.3: triggers → classify → history ---

#[test]
fn test_trigger_classify_history_pipeline() {
    let mut state = EmotionalState::new();
    let mut history = MoodHistory::new(10);

    // Record calm baseline
    history.record(state.snapshot());
    assert_eq!(state.classify(), MoodState::Calm);

    // Apply praise trigger
    state.apply_trigger(&trigger_praised());
    history.record(state.snapshot());

    // Should now be in a positive state
    let latest = history.latest_state().unwrap();
    assert!(
        latest == MoodState::Content || latest == MoodState::Euphoric,
        "expected positive state, got {latest}"
    );

    // Deviation trend should be escalating
    assert!(history.deviation_trend() > 0.0);
}

// --- v0.3: mood influence on traits ---

#[test]
fn test_mood_influences_trait_expression() {
    let mut state = EmotionalState::new();
    state.stimulate(Emotion::Frustration, 0.8);

    // Frustrated mood should boost directness and reduce patience
    let directness_boost = mood_trait_influence(&state.mood, TraitKind::Directness);
    let patience_penalty = mood_trait_influence(&state.mood, TraitKind::Patience);
    assert!(directness_boost > 0.0);
    assert!(patience_penalty < 0.0);
}

// --- v0.4: sentiment negation → mood ---

#[test]
fn test_negated_sentiment_drives_mood() {
    let result = sentiment::analyze("This is not good at all.");
    let mut state = EmotionalState::new();
    for (emotion, intensity) in &result.emotions {
        state.stimulate(*emotion, *intensity);
    }
    // "not good" should produce negative/neutral mood
    assert!(state.mood.joy <= 0.0);
}

#[test]
fn test_sentence_analysis_mixed_mood() {
    let doc = sentiment::analyze_sentences("I love this! But I hate the bugs.");
    assert_eq!(doc.sentences.len(), 2);
    assert!(doc.sentences[0].sentiment.is_positive());
    assert!(doc.sentences[1].sentiment.is_negative());
}

// --- v0.5: templates + validation ---

#[test]
fn test_all_templates_pass_default_validation() {
    use bhava::archetype::list_templates;
    let rules = ValidationRules::default();
    for name in list_templates() {
        let template = get_template(name).unwrap();
        let content = template.apply();
        assert!(
            content.is_valid(&rules),
            "{name} template fails default validation"
        );
    }
}

#[test]
fn test_guardian_template_passes_strict_validation() {
    let content = get_template("guardian").unwrap().apply();
    let rules = ValidationRules::strict();
    assert!(content.is_valid(&rules));
}

// --- v0.5: crew with presets ---

#[test]
fn test_crew_from_presets() {
    let members: Vec<CrewMember> = list_presets()
        .iter()
        .map(|id| {
            let p = get_preset(id).unwrap();
            CrewMember {
                name: p.name.to_string(),
                identity: p.identity,
            }
        })
        .collect();
    let prompt = compose_crew_prompt(&members);
    assert!(prompt.contains("Crew (5 members)"));
    assert!(prompt.contains("### BlueShirtGuy"));
    assert!(prompt.contains("### T.Ron"));
}

// --- v0.5: merge identities ---

#[test]
fn test_merge_preset_identities() {
    let guy = get_preset("blue-shirt-guy").unwrap();
    let tron = get_preset("t-ron").unwrap();
    let merged = guy.identity.merge(&tron.identity, "\n\n");
    let soul = merged.get(IdentityLayer::Soul).unwrap();
    assert!(soul.contains("Guy"));
    assert!(soul.contains("T.Ron"));
}

// --- Relationship graph ---

#[test]
fn test_relationship_interaction_flow() {
    use bhava::relationship::RelationshipGraph;
    let mut graph = RelationshipGraph::new();

    // Build relationships over multiple interactions
    graph.record_interaction("player", "npc_a", 0.2, 0.1);
    graph.record_interaction("player", "npc_a", 0.2, 0.1);
    graph.record_interaction("player", "npc_b", -0.3, -0.1);

    assert_eq!(graph.allies("player").len(), 1);
    assert_eq!(graph.rivals("player").len(), 1);
    assert!(graph.average_affinity("player") > -0.1);
}

#[test]
fn test_relationship_decay_over_time() {
    use bhava::relationship::{Relationship, RelationshipGraph};
    let mut graph = RelationshipGraph::new();
    let mut r = Relationship::new("a", "b");
    r.affinity = 0.8;
    r.trust = 0.9;
    r.decay_rate = 0.1;
    graph.upsert(r);

    // Multiple decay ticks
    for _ in 0..20 {
        graph.decay_all();
    }

    let rel = graph.get("a", "b").unwrap();
    assert!(rel.affinity.abs() < 0.1, "affinity should decay toward 0");
    assert!(
        (rel.trust - 0.5).abs() < 0.1,
        "trust should decay toward 0.5"
    );
}

// --- Personality markdown serialization ---

#[test]
fn test_preset_markdown_roundtrip() {
    for id in list_presets() {
        let preset = get_preset(id).unwrap();
        let md = preset.profile.to_markdown();
        let restored = PersonalityProfile::from_markdown(&md).unwrap();
        // Every trait should survive the roundtrip
        for &kind in bhava::traits::TraitKind::ALL {
            assert_eq!(
                restored.get_trait(kind),
                preset.profile.get_trait(kind),
                "{id}: {kind} didn't roundtrip"
            );
        }
    }
}

// --- Spirit + archetype composition ---

#[test]
fn test_spirit_into_identity() {
    use bhava::spirit::Spirit;
    let mut spirit = Spirit::new();
    spirit.add_passion("helping", "Serving users effectively", 0.9);
    spirit.add_pain("errors", "When things go wrong", 0.6);

    let prompt = spirit.compose_prompt();
    assert!(!prompt.is_empty());

    // Spirit prompt can be set as the Spirit identity layer
    let mut identity = IdentityContent::default();
    identity.set(IdentityLayer::Soul, "I am an assistant.");
    identity.set(IdentityLayer::Spirit, &prompt);

    let full = compose_identity_prompt(&identity);
    assert!(full.contains("### Spirit"));
    assert!(full.contains("helping"));
}

// --- Mood baseline from preset ---

#[test]
fn test_preset_mood_baseline() {
    use bhava::mood::derive_mood_baseline;
    let guy = get_preset("blue-shirt-guy").unwrap();
    let tron = get_preset("t-ron").unwrap();

    let guy_baseline = derive_mood_baseline(&guy.profile);
    let tron_baseline = derive_mood_baseline(&tron.profile);

    // Guy should have higher joy baseline than T.Ron
    assert!(
        guy_baseline.joy > tron_baseline.joy,
        "Guy should be happier at rest than T.Ron"
    );
}
