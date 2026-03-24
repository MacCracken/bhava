use super::*;

#[test]
fn test_neutral_mood() {
    let m = MoodVector::neutral();
    assert!((m.intensity()).abs() < f32::EPSILON);
}

#[test]
fn test_get_set() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.8);
    assert!((m.get(Emotion::Joy) - 0.8).abs() < f32::EPSILON);
}

#[test]
fn test_clamp() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 5.0);
    assert!((m.get(Emotion::Joy) - 1.0).abs() < f32::EPSILON);
    m.set(Emotion::Joy, -5.0);
    assert!((m.get(Emotion::Joy) - (-1.0)).abs() < f32::EPSILON);
}

#[test]
fn test_nudge() {
    let mut m = MoodVector::neutral();
    m.nudge(Emotion::Trust, 0.3);
    m.nudge(Emotion::Trust, 0.3);
    assert!((m.get(Emotion::Trust) - 0.6).abs() < 0.01);
}

#[test]
fn test_intensity() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 1.0);
    assert!((m.intensity() - 1.0).abs() < 0.01);
}

#[test]
fn test_dominant_emotion() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Frustration, -0.9);
    m.set(Emotion::Joy, 0.3);
    assert_eq!(m.dominant_emotion(), Emotion::Frustration);
}

#[test]
fn test_decay() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 1.0);
    m.decay(0.5);
    assert!((m.get(Emotion::Joy) - 0.5).abs() < 0.01);
}

#[test]
fn test_blend() {
    let a = MoodVector::neutral();
    let mut b = MoodVector::neutral();
    b.set(Emotion::Joy, 1.0);
    let c = a.blend(&b, 0.5);
    assert!((c.joy - 0.5).abs() < 0.01);
}

#[test]
fn test_emotional_state_new() {
    let s = EmotionalState::new();
    assert!(s.deviation().abs() < f32::EPSILON);
}

#[test]
fn test_stimulate() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.5);
    assert!(s.mood.joy > 0.0);
    assert!(s.deviation() > 0.0);
}

#[test]
fn test_invalid_decay_rate() {
    let mut s = EmotionalState::new();
    assert!(s.set_decay_half_life(-1.0).is_err());
    assert!(s.set_decay_half_life(0.0).is_err());
    assert!(s.set_decay_half_life(60.0).is_ok());
}

#[test]
fn test_emotion_display() {
    assert_eq!(Emotion::Joy.to_string(), "joy");
    assert_eq!(Emotion::Frustration.to_string(), "frustration");
}

#[test]
fn test_mood_serde() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.7);
    m.set(Emotion::Trust, -0.3);
    let json = serde_json::to_string(&m).unwrap();
    let m2: MoodVector = serde_json::from_str(&json).unwrap();
    assert!((m2.joy - 0.7).abs() < 0.01);
    assert!((m2.trust - (-0.3)).abs() < 0.01);
}

#[test]
fn test_emotion_all() {
    assert_eq!(Emotion::ALL.len(), 6);
}

#[test]
fn test_emotion_display_all() {
    let names: Vec<String> = Emotion::ALL.iter().map(|e| e.to_string()).collect();
    assert!(names.contains(&"joy".to_string()));
    assert!(names.contains(&"arousal".to_string()));
    assert!(names.contains(&"dominance".to_string()));
    assert!(names.contains(&"trust".to_string()));
    assert!(names.contains(&"interest".to_string()));
    assert!(names.contains(&"frustration".to_string()));
}

#[test]
fn test_set_all_dimensions() {
    let mut m = MoodVector::neutral();
    for (i, &e) in Emotion::ALL.iter().enumerate() {
        let val = (i as f32 + 1.0) * 0.15;
        m.set(e, val);
        assert!((m.get(e) - val).abs() < f32::EPSILON);
    }
}

#[test]
fn test_nudge_clamps() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.9);
    m.nudge(Emotion::Joy, 0.5);
    assert!((m.get(Emotion::Joy) - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_dominant_emotion_neutral() {
    let m = MoodVector::neutral();
    // When all zero, returns Joy (first checked)
    let _ = m.dominant_emotion(); // just ensure no panic
}

#[test]
fn test_dominant_emotion_negative() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, -0.2);
    m.set(Emotion::Frustration, -0.9);
    assert_eq!(m.dominant_emotion(), Emotion::Frustration);
}

#[test]
fn test_decay_zero() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.8);
    m.decay(0.0);
    assert!((m.get(Emotion::Joy) - 0.8).abs() < f32::EPSILON);
}

#[test]
fn test_decay_full() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.8);
    m.set(Emotion::Trust, -0.5);
    m.decay(1.0);
    assert!(m.get(Emotion::Joy).abs() < f32::EPSILON);
    assert!(m.get(Emotion::Trust).abs() < f32::EPSILON);
}

#[test]
fn test_decay_clamps_factor() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.8);
    m.decay(5.0); // should clamp to 1.0
    assert!(m.get(Emotion::Joy).abs() < f32::EPSILON);
}

#[test]
fn test_blend_zero() {
    let mut a = MoodVector::neutral();
    a.set(Emotion::Joy, 0.5);
    let b = MoodVector::neutral();
    let c = a.blend(&b, 0.0);
    assert!((c.joy - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_blend_one() {
    let a = MoodVector::neutral();
    let mut b = MoodVector::neutral();
    b.set(Emotion::Joy, 1.0);
    let c = a.blend(&b, 1.0);
    assert!((c.joy - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_blend_clamps_t() {
    let a = MoodVector::neutral();
    let mut b = MoodVector::neutral();
    b.set(Emotion::Joy, 1.0);
    let c = a.blend(&b, 5.0); // should clamp to 1.0
    assert!((c.joy - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_intensity_multiple_dimensions() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.6);
    m.set(Emotion::Trust, 0.8);
    let expected = (0.6f32 * 0.6 + 0.8 * 0.8).sqrt();
    assert!((m.intensity() - expected).abs() < 0.01);
}

#[test]
fn test_emotional_state_default() {
    let s = EmotionalState::default();
    assert!(s.deviation().abs() < f32::EPSILON);
    assert!((s.decay_half_life_secs - 300.0).abs() < f64::EPSILON);
}

#[test]
fn test_emotional_state_with_baseline() {
    let mut baseline = MoodVector::neutral();
    baseline.set(Emotion::Joy, 0.5);
    let s = EmotionalState::with_baseline(baseline);
    assert!((s.mood.joy - 0.5).abs() < f32::EPSILON);
    assert!((s.baseline.joy - 0.5).abs() < f32::EPSILON);
    assert!(s.deviation().abs() < f32::EPSILON);
}

#[test]
fn test_set_decay_half_life_valid() {
    let mut s = EmotionalState::new();
    assert!(s.set_decay_half_life(60.0).is_ok());
    assert!((s.decay_half_life_secs - 60.0).abs() < f64::EPSILON);
}

#[test]
fn test_apply_decay_no_time() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.8);
    let before = s.mood.joy;
    s.apply_decay(s.last_updated); // zero elapsed
    assert!((s.mood.joy - before).abs() < f32::EPSILON);
}

#[test]
fn test_apply_decay_toward_baseline() {
    let mut baseline = MoodVector::neutral();
    baseline.set(Emotion::Joy, 0.3);
    let mut s = EmotionalState::with_baseline(baseline);
    s.stimulate(Emotion::Joy, 0.5); // mood.joy now ~0.8

    let future = s.last_updated + chrono::Duration::hours(1);
    s.apply_decay(future);
    // After long decay, should approach baseline (0.3)
    assert!((s.mood.joy - 0.3).abs() < 0.05);
}

#[test]
fn test_apply_decay_negative_elapsed() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.8);
    let before = s.mood.joy;
    let past = s.last_updated - chrono::Duration::minutes(5);
    s.apply_decay(past); // negative elapsed, should be no-op
    assert!((s.mood.joy - before).abs() < f32::EPSILON);
}

#[test]
fn test_deviation_with_baseline() {
    let mut baseline = MoodVector::neutral();
    baseline.set(Emotion::Joy, 0.5);
    let mut s = EmotionalState::with_baseline(baseline);
    // mood starts at baseline, deviation is 0
    assert!(s.deviation().abs() < f32::EPSILON);
    s.stimulate(Emotion::Joy, 0.3);
    assert!(s.deviation() > 0.0);
}

#[test]
fn test_emotional_state_serde() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.5);
    s.stimulate(Emotion::Frustration, -0.3);
    let json = serde_json::to_string(&s).unwrap();
    let s2: EmotionalState = serde_json::from_str(&json).unwrap();
    assert!((s2.mood.joy - s.mood.joy).abs() < 0.01);
    assert!((s2.mood.frustration - s.mood.frustration).abs() < 0.01);
    assert!((s2.decay_half_life_secs - s.decay_half_life_secs).abs() < 0.01);
}

#[test]
fn test_emotion_serde() {
    for &e in Emotion::ALL {
        let json = serde_json::to_string(&e).unwrap();
        let e2: Emotion = serde_json::from_str(&json).unwrap();
        assert_eq!(e2, e);
    }
}

// --- v0.3: MoodState ---

#[test]
fn test_classify_calm() {
    let s = EmotionalState::new();
    assert_eq!(s.classify(), MoodState::Calm);
}

#[test]
fn test_classify_euphoric() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.8);
    assert_eq!(s.classify(), MoodState::Euphoric);
}

#[test]
fn test_classify_frustrated() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Frustration, 0.7);
    assert_eq!(s.classify(), MoodState::Frustrated);
}

#[test]
fn test_classify_guarded() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Trust, -0.6);
    assert_eq!(s.classify(), MoodState::Guarded);
}

#[test]
fn test_classify_curious() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Interest, 0.5);
    assert_eq!(s.classify(), MoodState::Curious);
}

#[test]
fn test_classify_agitated() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Arousal, 0.6);
    assert_eq!(s.classify(), MoodState::Agitated);
}

#[test]
fn test_mood_state_display() {
    assert_eq!(MoodState::Calm.to_string(), "calm");
    assert_eq!(MoodState::Euphoric.to_string(), "euphoric");
    assert_eq!(MoodState::Frustrated.to_string(), "frustrated");
}

#[test]
fn test_mood_state_serde() {
    let json = serde_json::to_string(&MoodState::Euphoric).unwrap();
    let restored: MoodState = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, MoodState::Euphoric);
}

// --- v0.3: MoodTrigger ---

#[test]
fn test_trigger_builder() {
    let t = MoodTrigger::new("test")
        .respond(Emotion::Joy, 0.5)
        .respond(Emotion::Trust, 0.3);
    assert_eq!(t.name, "test");
    assert_eq!(t.responses.len(), 2);
}

#[test]
fn test_apply_trigger() {
    let mut s = EmotionalState::new();
    let t = trigger_praised();
    s.apply_trigger(&t);
    assert!(s.mood.joy > 0.0);
    assert!(s.mood.dominance > 0.0);
    assert!(s.mood.trust > 0.0);
}

#[test]
fn test_trigger_criticized() {
    let mut s = EmotionalState::new();
    s.apply_trigger(&trigger_criticized());
    assert!(s.mood.joy < 0.0);
    assert!(s.mood.frustration > 0.0);
}

#[test]
fn test_trigger_surprised() {
    let mut s = EmotionalState::new();
    s.apply_trigger(&trigger_surprised());
    assert!(s.mood.arousal > 0.0);
    assert!(s.mood.interest > 0.0);
}

#[test]
fn test_trigger_threatened() {
    let mut s = EmotionalState::new();
    s.apply_trigger(&trigger_threatened());
    assert!(s.mood.trust < 0.0);
    assert!(s.mood.dominance < 0.0);
}

#[test]
fn test_trigger_serde() {
    let t = trigger_praised();
    let json = serde_json::to_string(&t).unwrap();
    let t2: MoodTrigger = serde_json::from_str(&json).unwrap();
    assert_eq!(t2.name, "praised");
    assert_eq!(t2.responses.len(), t.responses.len());
}

// --- v0.3: MoodHistory ---

#[test]
fn test_history_new() {
    let h = MoodHistory::new(10);
    assert!(h.is_empty());
    assert_eq!(h.len(), 0);
}

#[test]
fn test_history_record() {
    let mut h = MoodHistory::new(10);
    let s = EmotionalState::new();
    h.record(s.snapshot());
    assert_eq!(h.len(), 1);
    assert!(!h.is_empty());
}

#[test]
fn test_history_capacity() {
    let mut h = MoodHistory::new(3);
    let s = EmotionalState::new();
    for _ in 0..5 {
        h.record(s.snapshot());
    }
    assert_eq!(h.len(), 3);
}

#[test]
fn test_history_average_deviation() {
    let mut h = MoodHistory::new(10);
    let s = EmotionalState::new();
    h.record(s.snapshot());
    assert!(h.average_deviation().abs() < f32::EPSILON);
}

#[test]
fn test_history_average_deviation_empty() {
    let h = MoodHistory::new(10);
    assert!(h.average_deviation().abs() < f32::EPSILON);
}

#[test]
fn test_history_latest_state() {
    let mut h = MoodHistory::new(10);
    assert!(h.latest_state().is_none());

    let s = EmotionalState::new();
    h.record(s.snapshot());
    assert_eq!(h.latest_state(), Some(MoodState::Calm));
}

#[test]
fn test_history_state_distribution() {
    let mut h = MoodHistory::new(10);
    let mut s = EmotionalState::new();
    h.record(s.snapshot()); // calm

    s.stimulate(Emotion::Joy, 0.8);
    h.record(s.snapshot()); // euphoric

    let dist = h.state_distribution();
    assert_eq!(dist.len(), 2);
}

#[test]
fn test_history_deviation_trend_stable() {
    let mut h = MoodHistory::new(10);
    let s = EmotionalState::new();
    for _ in 0..4 {
        h.record(s.snapshot());
    }
    assert!(h.deviation_trend().abs() < f32::EPSILON);
}

#[test]
fn test_history_deviation_trend_escalating() {
    let mut h = MoodHistory::new(10);
    let mut s = EmotionalState::new();
    // First half: calm
    h.record(s.snapshot());
    h.record(s.snapshot());
    // Second half: stimulated
    s.stimulate(Emotion::Frustration, 0.8);
    h.record(s.snapshot());
    h.record(s.snapshot());
    assert!(h.deviation_trend() > 0.0);
}

#[test]
fn test_history_serde() {
    let mut h = MoodHistory::new(5);
    let s = EmotionalState::new();
    h.record(s.snapshot());
    let json = serde_json::to_string(&h).unwrap();
    let h2: MoodHistory = serde_json::from_str(&json).unwrap();
    assert_eq!(h2.len(), 1);
}

#[test]
fn test_snapshot_fields() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.8);
    let snap = s.snapshot();
    assert_eq!(snap.state, MoodState::Euphoric);
    assert!(snap.deviation > 0.0);
    assert!((snap.mood.joy - s.mood.joy).abs() < f32::EPSILON);
}

// --- v0.3: Mood Influence ---

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_neutral() {
    let m = MoodVector::neutral();
    for &kind in crate::traits::TraitKind::ALL {
        let inf = mood_trait_influence(&m, kind);
        assert!(
            inf.abs() < f32::EPSILON,
            "{kind} influence should be 0 for neutral mood"
        );
    }
}

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_frustration_boosts_directness() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Frustration, 0.8);
    let inf = mood_trait_influence(&m, crate::traits::TraitKind::Directness);
    assert!(inf > 0.0);
}

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_joy_boosts_warmth() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.8);
    let inf = mood_trait_influence(&m, crate::traits::TraitKind::Warmth);
    assert!(inf > 0.0);
}

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_clamped() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Frustration, 1.0);
    m.set(Emotion::Dominance, 1.0);
    m.set(Emotion::Arousal, 1.0);
    for &kind in crate::traits::TraitKind::ALL {
        let inf = mood_trait_influence(&m, kind);
        assert!(
            ((-1.0)..=1.0).contains(&inf),
            "{kind} influence {inf} out of range"
        );
    }
}

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_all_traits_covered() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.5);
    m.set(Emotion::Frustration, 0.3);
    m.set(Emotion::Interest, 0.4);
    // Just ensure no panic for every trait
    for &kind in crate::traits::TraitKind::ALL {
        let _ = mood_trait_influence(&m, kind);
    }
}

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_frustration_reduces_patience() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Frustration, 0.8);
    let inf = mood_trait_influence(&m, crate::traits::TraitKind::Patience);
    assert!(inf < 0.0);
}

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_interest_boosts_curiosity() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Interest, 0.8);
    let inf = mood_trait_influence(&m, crate::traits::TraitKind::Curiosity);
    assert!(inf > 0.0);
}

#[cfg(feature = "traits")]
#[test]
fn test_mood_trait_influence_dominance_boosts_confidence() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Dominance, 0.8);
    let inf = mood_trait_influence(&m, crate::traits::TraitKind::Confidence);
    assert!(inf > 0.0);
}

// --- Additional MoodState coverage ---

#[test]
fn test_classify_melancholy() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, -0.5);
    assert_eq!(s.classify(), MoodState::Melancholy);
}

#[test]
fn test_classify_assertive() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Dominance, 0.6);
    assert_eq!(s.classify(), MoodState::Assertive);
}

#[test]
fn test_classify_overwhelmed() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Dominance, -0.6);
    assert_eq!(s.classify(), MoodState::Overwhelmed);
}

#[test]
fn test_classify_trusting() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Trust, 0.5);
    assert_eq!(s.classify(), MoodState::Trusting);
}

#[test]
fn test_classify_disengaged() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Interest, -0.5);
    assert_eq!(s.classify(), MoodState::Disengaged);
}

#[test]
fn test_classify_content() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.3); // positive but not euphoric
    assert_eq!(s.classify(), MoodState::Content);
}

// --- Additional MoodHistory coverage ---

#[test]
fn test_history_state_distribution_empty() {
    let h = MoodHistory::new(10);
    assert!(h.state_distribution().is_empty());
}

#[test]
fn test_history_deviation_trend_two_snapshots() {
    let mut h = MoodHistory::new(10);
    let s = EmotionalState::new();
    h.record(s.snapshot());
    let mut s2 = EmotionalState::new();
    s2.stimulate(Emotion::Joy, 0.8);
    h.record(s2.snapshot());
    // With 2 snapshots, half=1, first=[0..1], second=[1..]
    assert!(h.deviation_trend() > 0.0);
}

#[test]
fn test_history_deviation_trend_calming() {
    let mut h = MoodHistory::new(10);
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Frustration, 0.9);
    h.record(s.snapshot());
    h.record(s.snapshot());
    // Second half: calm
    let calm = EmotionalState::new();
    h.record(calm.snapshot());
    h.record(calm.snapshot());
    assert!(h.deviation_trend() < 0.0);
}

#[test]
fn test_history_iter() {
    let mut h = MoodHistory::new(10);
    let s = EmotionalState::new();
    h.record(s.snapshot());
    h.record(s.snapshot());
    assert_eq!(h.iter().count(), 2);
}

#[test]
fn test_history_capacity_zero_becomes_one() {
    let h = MoodHistory::new(0);
    // capacity should be clamped to 1
    assert!(h.is_empty());
}

// --- MoodState Display completeness ---

#[test]
fn test_mood_state_display_all() {
    let states = [
        MoodState::Calm,
        MoodState::Content,
        MoodState::Euphoric,
        MoodState::Melancholy,
        MoodState::Agitated,
        MoodState::Assertive,
        MoodState::Overwhelmed,
        MoodState::Trusting,
        MoodState::Guarded,
        MoodState::Curious,
        MoodState::Disengaged,
        MoodState::Frustrated,
    ];
    for state in states {
        assert!(!state.to_string().is_empty());
    }
}

#[test]
fn test_mood_state_serde_all() {
    let states = [
        MoodState::Calm,
        MoodState::Content,
        MoodState::Euphoric,
        MoodState::Melancholy,
        MoodState::Agitated,
        MoodState::Assertive,
        MoodState::Overwhelmed,
        MoodState::Trusting,
        MoodState::Guarded,
        MoodState::Curious,
        MoodState::Disengaged,
        MoodState::Frustrated,
    ];
    for state in states {
        let json = serde_json::to_string(&state).unwrap();
        let restored: MoodState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, state);
    }
}

// --- MoodTrigger edge cases ---

#[test]
fn test_trigger_empty_responses() {
    let t = MoodTrigger::new("empty");
    let mut s = EmotionalState::new();
    s.apply_trigger(&t);
    assert!(s.deviation().abs() < f32::EPSILON);
}

#[test]
fn test_trigger_multiple_same_emotion() {
    let t = MoodTrigger::new("double")
        .respond(Emotion::Joy, 0.3)
        .respond(Emotion::Joy, 0.3);
    let mut s = EmotionalState::new();
    s.apply_trigger(&t);
    assert!((s.mood.joy - 0.6).abs() < 0.01);
}

// --- Trait-to-mood baseline ---

#[cfg(feature = "traits")]
#[test]
fn test_derive_baseline_balanced_near_zero() {
    let profile = crate::traits::PersonalityProfile::new("neutral");
    let baseline = derive_mood_baseline(&profile);
    // All balanced traits → near-zero baseline
    assert!(baseline.joy.abs() < 0.01);
    assert!(baseline.arousal.abs() < 0.01);
}

#[cfg(feature = "traits")]
#[test]
fn test_derive_baseline_warm_positive() {
    let mut profile = crate::traits::PersonalityProfile::new("warm");
    profile.set_trait(
        crate::traits::TraitKind::Warmth,
        crate::traits::TraitLevel::Highest,
    );
    profile.set_trait(
        crate::traits::TraitKind::Humor,
        crate::traits::TraitLevel::Highest,
    );
    let baseline = derive_mood_baseline(&profile);
    assert!(
        baseline.joy > 0.0,
        "warm+funny should have positive valence"
    );
}

#[cfg(feature = "traits")]
#[test]
fn test_derive_baseline_cold_negative() {
    let mut profile = crate::traits::PersonalityProfile::new("cold");
    profile.set_trait(
        crate::traits::TraitKind::Warmth,
        crate::traits::TraitLevel::Lowest,
    );
    profile.set_trait(
        crate::traits::TraitKind::Empathy,
        crate::traits::TraitLevel::Lowest,
    );
    let baseline = derive_mood_baseline(&profile);
    assert!(
        baseline.joy < 0.0,
        "cold+detached should have negative valence"
    );
}

#[cfg(feature = "traits")]
#[test]
fn test_derive_baseline_with_compound_effects() {
    let mut profile = crate::traits::PersonalityProfile::new("playful");
    profile.set_trait(
        crate::traits::TraitKind::Warmth,
        crate::traits::TraitLevel::Highest,
    );
    profile.set_trait(
        crate::traits::TraitKind::Humor,
        crate::traits::TraitLevel::Highest,
    );

    let mut baseline_profile = crate::traits::PersonalityProfile::new("just_warm");
    baseline_profile.set_trait(
        crate::traits::TraitKind::Warmth,
        crate::traits::TraitLevel::Highest,
    );

    let playful = derive_mood_baseline(&profile);
    let just_warm = derive_mood_baseline(&baseline_profile);
    // Compound "playful" effect should boost valence beyond just warmth
    assert!(playful.joy > just_warm.joy);
}

#[cfg(feature = "traits")]
#[test]
fn test_derive_baseline_clamped() {
    use crate::traits::{TraitKind, TraitLevel};
    let mut profile = crate::traits::PersonalityProfile::new("extreme");
    for &kind in TraitKind::ALL {
        profile.set_trait(kind, TraitLevel::Highest);
    }
    let baseline = derive_mood_baseline(&profile);
    assert!(((-1.0)..=1.0).contains(&baseline.joy));
    assert!(((-1.0)..=1.0).contains(&baseline.arousal));
}

// --- Mood tone guides ---

#[test]
fn test_mood_tone_guide_all_states() {
    let states = [
        MoodState::Calm,
        MoodState::Content,
        MoodState::Euphoric,
        MoodState::Melancholy,
        MoodState::Agitated,
        MoodState::Assertive,
        MoodState::Overwhelmed,
        MoodState::Trusting,
        MoodState::Guarded,
        MoodState::Curious,
        MoodState::Disengaged,
        MoodState::Frustrated,
    ];
    for state in states {
        let guide = mood_tone_guide(state);
        assert!(!guide.is_empty(), "{state} has empty tone guide");
    }
}

#[test]
fn test_compose_mood_prompt() {
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Joy, 0.8);
    let prompt = compose_mood_prompt(&s);
    assert!(prompt.contains("## Current Mood:"));
    assert!(prompt.contains("euphoric") || prompt.contains("content"));
}

#[test]
fn test_compose_mood_prompt_calm() {
    let s = EmotionalState::new();
    let prompt = compose_mood_prompt(&s);
    assert!(prompt.contains("calm"));
}

// --- Action Tendencies ---

#[test]
fn test_action_tendency_positive() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.8);
    m.set(Emotion::Trust, 0.5);
    match action_tendency(&m) {
        ActionTendency::Approach { intensity } => assert!(intensity > 0.1),
        other => panic!("expected Approach, got {other:?}"),
    }
}

#[test]
fn test_action_tendency_frustrated() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Frustration, 0.8);
    m.set(Emotion::Dominance, 0.5);
    m.set(Emotion::Arousal, 0.6);
    match action_tendency(&m) {
        ActionTendency::Confront { intensity } => assert!(intensity > 0.1),
        other => panic!("expected Confront, got {other:?}"),
    }
}

#[test]
fn test_action_tendency_withdraw() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, -0.8);
    m.set(Emotion::Arousal, -0.5);
    match action_tendency(&m) {
        ActionTendency::Withdraw { intensity } => assert!(intensity > 0.1),
        other => panic!("expected Withdraw, got {other:?}"),
    }
}

#[test]
fn test_action_tendency_neutral() {
    let m = MoodVector::neutral();
    assert!(matches!(action_tendency(&m), ActionTendency::Neutral));
}

// --- Emotional Contagion ---

#[test]
fn test_contagion_basic() {
    let mut sender = MoodVector::neutral();
    sender.set(Emotion::Joy, 0.8);
    let sp = ContagionParams {
        expressiveness: 0.8,
        susceptibility: 0.0,
    };
    let rp = ContagionParams {
        expressiveness: 0.0,
        susceptibility: 0.8,
    };
    let delta = compute_contagion(&sender, &sp, &rp, 0.7);
    assert!(delta.joy > 0.0);
}

#[test]
fn test_contagion_rival_inverts() {
    let mut sender = MoodVector::neutral();
    sender.set(Emotion::Joy, 0.8);
    let sp = ContagionParams {
        expressiveness: 0.8,
        susceptibility: 0.0,
    };
    let rp = ContagionParams {
        expressiveness: 0.0,
        susceptibility: 0.8,
    };
    let delta = compute_contagion(&sender, &sp, &rp, -0.5);
    assert!(delta.joy < 0.0); // rival's joy → receiver's sadness
}

#[test]
fn test_contagion_zero_affinity() {
    let mut sender = MoodVector::neutral();
    sender.set(Emotion::Joy, 0.8);
    let sp = ContagionParams::default();
    let rp = ContagionParams::default();
    let delta = compute_contagion(&sender, &sp, &rp, 0.0);
    assert!(delta.joy.abs() < f32::EPSILON);
}

#[test]
fn test_group_mood_single() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.6);
    let result = group_mood(&[&m]);
    assert!((result.joy - 0.6).abs() < f32::EPSILON);
}

#[test]
fn test_group_mood_average() {
    let mut a = MoodVector::neutral();
    a.set(Emotion::Joy, 0.8);
    let mut b = MoodVector::neutral();
    b.set(Emotion::Joy, 0.2);
    let result = group_mood(&[&a, &b]);
    assert!((result.joy - 0.5).abs() < 0.01);
}

#[test]
fn test_group_mood_empty() {
    let result = group_mood(&[]);
    assert!(result.intensity() < f32::EPSILON);
}

#[cfg(feature = "traits")]
#[test]
fn test_contagion_from_personality() {
    let mut profile = crate::traits::PersonalityProfile::new("warm");
    profile.set_trait(
        crate::traits::TraitKind::Warmth,
        crate::traits::TraitLevel::Highest,
    );
    profile.set_trait(
        crate::traits::TraitKind::Empathy,
        crate::traits::TraitLevel::Highest,
    );
    let params = contagion_from_personality(&profile);
    assert!(params.expressiveness > 0.5);
    assert!(params.susceptibility > 0.5);
}

// --- Adaptive Baselines ---

#[test]
fn test_adaptive_baseline_new() {
    let core = MoodVector::neutral();
    let ab = AdaptiveBaseline::new(core);
    assert!(ab.drift() < f32::EPSILON);
}

#[test]
fn test_adaptive_baseline_adapts() {
    let core = MoodVector::neutral();
    let mut ab = AdaptiveBaseline::new(core);
    let mut positive = MoodVector::neutral();
    positive.set(Emotion::Joy, 0.8);
    // Adapt many times toward positive mood
    for _ in 0..100 {
        ab.adapt(&positive);
    }
    assert!(
        ab.adapted.joy > 0.0,
        "baseline should shift toward positive"
    );
    assert!(ab.drift() > 0.0);
}

#[test]
fn test_adaptive_baseline_recovery() {
    let core = MoodVector::neutral();
    let mut ab = AdaptiveBaseline::new(core);
    ab.adapted.joy = 0.5; // artificially shift
    // Recovery pulls back toward core (0.0)
    for _ in 0..200 {
        ab.adapt(&MoodVector::neutral());
    }
    assert!(
        ab.adapted.joy.abs() < 0.1,
        "baseline should recover toward core"
    );
}

#[test]
fn test_adaptive_baseline_serde() {
    let ab = AdaptiveBaseline::new(MoodVector::neutral());
    let json = serde_json::to_string(&ab).unwrap();
    let ab2: AdaptiveBaseline = serde_json::from_str(&json).unwrap();
    assert!((ab2.adaptation_rate - ab.adaptation_rate).abs() < f32::EPSILON);
}

// --- Volatility + Momentum ---

#[test]
fn test_volatility_stable() {
    let mut h = MoodHistory::new(10);
    let s = EmotionalState::new();
    for _ in 0..5 {
        h.record(s.snapshot());
    }
    assert!(h.volatility() < 0.01);
}

#[test]
fn test_volatility_varying() {
    let mut h = MoodHistory::new(10);
    let mut s = EmotionalState::new();
    h.record(s.snapshot()); // calm
    s.stimulate(Emotion::Joy, 0.8);
    h.record(s.snapshot()); // excited
    let mut s2 = EmotionalState::new();
    h.record(s2.snapshot()); // calm again
    s2.stimulate(Emotion::Frustration, 0.9);
    h.record(s2.snapshot()); // frustrated
    assert!(h.volatility() > 0.1);
}

#[test]
fn test_momentum_escalating() {
    let mut h = MoodHistory::new(10);
    let mut s = EmotionalState::new();
    for i in 0..5 {
        s.stimulate(Emotion::Joy, 0.1 * (i as f32 + 1.0));
        h.record(s.snapshot());
    }
    assert!(h.momentum() > 0.0);
}

#[test]
fn test_momentum_calming() {
    let mut h = MoodHistory::new(10);
    let mut s = EmotionalState::new();
    s.stimulate(Emotion::Frustration, 0.9);
    h.record(s.snapshot());
    for _ in 0..4 {
        s.mood.decay(0.3);
        h.record(s.snapshot());
    }
    assert!(h.momentum() < 0.0);
}

// --- Cause-tagged decay ---

#[test]
fn test_active_cause() {
    let mut s = EmotionalState::new();
    s.add_active_cause("threat", vec![Emotion::Frustration, Emotion::Arousal]);
    assert!(s.is_cause_active(Emotion::Frustration));
    assert!(!s.is_cause_active(Emotion::Joy));
}

#[test]
fn test_resolve_cause() {
    let mut s = EmotionalState::new();
    s.add_active_cause("threat", vec![Emotion::Frustration]);
    assert!(s.resolve_cause("threat"));
    assert!(!s.is_cause_active(Emotion::Frustration));
    assert!(!s.resolve_cause("threat"));
}

// --- Plutchik Compound Emotions ---

#[test]
fn test_compound_love() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.7);
    m.set(Emotion::Trust, 0.6);
    let compounds = detect_compound_emotions(&m, 0.3);
    assert!(compounds.iter().any(|(e, _)| *e == CompoundEmotion::Love));
}

#[test]
fn test_compound_contempt() {
    let mut m = MoodVector::neutral();
    m.set(Emotion::Frustration, 0.7);
    m.set(Emotion::Trust, -0.6);
    let compounds = detect_compound_emotions(&m, 0.3);
    assert!(
        compounds
            .iter()
            .any(|(e, _)| *e == CompoundEmotion::Contempt)
    );
}

#[test]
fn test_compound_none_neutral() {
    let m = MoodVector::neutral();
    assert!(detect_compound_emotions(&m, 0.3).is_empty());
}

#[test]
fn test_compound_display() {
    assert_eq!(CompoundEmotion::Love.to_string(), "love");
    assert_eq!(
        CompoundEmotion::Aggressiveness.to_string(),
        "aggressiveness"
    );
}

// --- Second-Order Damping ---

#[test]
fn test_damped_underdamped() {
    let mut d = DampedResponse::new(0.3, 2.0);
    d.impulse(1.0);
    let mut crossed_zero = false;
    for _ in 0..100 {
        d.step(0.05);
        if d.position < 0.0 {
            crossed_zero = true;
        }
    }
    assert!(crossed_zero, "underdamped should oscillate");
}

#[test]
fn test_damped_overdamped_settles() {
    let mut d = DampedResponse::new(2.0, 1.0);
    d.impulse(1.0);
    for _ in 0..500 {
        d.step(0.05);
    }
    assert!(d.is_settled(0.05));
}

#[test]
fn test_damped_serde() {
    let d = DampedResponse::new(0.7, 1.5);
    let json = serde_json::to_string(&d).unwrap();
    let d2: DampedResponse = serde_json::from_str(&json).unwrap();
    assert!((d2.zeta - 0.7).abs() < f32::EPSILON);
}

// --- Emotional Memory ---

#[test]
fn test_memory_record_recall() {
    let mut bank = EmotionalMemoryBank::new(10);
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.8);
    bank.record("happy_place", &m, 0.9);
    let recalled = bank.recall("happy_place").unwrap();
    assert!(recalled.joy > 0.5);
}

#[test]
fn test_memory_recall_missing() {
    let bank = EmotionalMemoryBank::new(10);
    assert!(bank.recall("unknown").is_none());
}

#[test]
fn test_memory_overwrite() {
    let mut bank = EmotionalMemoryBank::new(10);
    let mut m1 = MoodVector::neutral();
    m1.set(Emotion::Joy, 0.5);
    bank.record("place", &m1, 0.8);
    let mut m2 = MoodVector::neutral();
    m2.set(Emotion::Frustration, 0.7);
    bank.record("place", &m2, 0.9);
    let recalled = bank.recall("place").unwrap();
    assert!(recalled.frustration > 0.0);
    assert_eq!(bank.len(), 1);
}

#[test]
fn test_memory_capacity_eviction() {
    let mut bank = EmotionalMemoryBank::new(2);
    let m = MoodVector::neutral();
    bank.record("a", &m, 0.5);
    bank.record("b", &m, 0.9);
    bank.record("c", &m, 0.8); // should evict "a" (weakest)
    assert_eq!(bank.len(), 2);
    assert!(bank.recall("a").is_none());
}

#[test]
fn test_memory_decay() {
    let mut bank = EmotionalMemoryBank::new(10);
    let m = MoodVector::neutral();
    bank.record("fading", &m, 0.1);
    bank.decay(0.5); // 0.1 * 0.5 = 0.05
    bank.decay(0.5); // 0.05 * 0.5 = 0.025
    bank.decay(0.5); // 0.025 * 0.5 = 0.0125
    bank.decay(0.5); // 0.00625 → below 0.01 threshold
    assert!(bank.is_empty());
}

#[test]
fn test_memory_serde() {
    let mut bank = EmotionalMemoryBank::new(10);
    let mut m = MoodVector::neutral();
    m.set(Emotion::Joy, 0.5);
    bank.record("test", &m, 0.8);
    let json = serde_json::to_string(&bank).unwrap();
    let bank2: EmotionalMemoryBank = serde_json::from_str(&json).unwrap();
    assert_eq!(bank2.len(), 1);
}

// --- Emotion Amplifier ---

#[cfg(feature = "traits")]
#[test]
fn test_amplifier_neurotic_amplifies_negative() {
