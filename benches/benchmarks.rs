use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_trait_behavior(c: &mut Criterion) {
    use bhava::traits::{TraitKind, TraitLevel, trait_behavior};
    let mut group = c.benchmark_group("traits");
    group.bench_function("behavior_lookup", |b| {
        b.iter(|| trait_behavior(black_box(TraitKind::Humor), black_box(TraitLevel::Highest)))
    });
    group.bench_function("level_name", |b| {
        use bhava::traits::trait_level_name;
        b.iter(|| trait_level_name(black_box(TraitKind::Warmth), black_box(TraitLevel::Highest)))
    });
    group.bench_function("level_from_numeric", |b| {
        b.iter(|| TraitLevel::from_numeric(black_box(1)))
    });
    group.finish();
}

fn bench_personality_prompt(c: &mut Criterion) {
    use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
    let mut group = c.benchmark_group("personality");

    let mut p = PersonalityProfile::new("test");
    p.set_trait(TraitKind::Humor, TraitLevel::Highest);
    p.set_trait(TraitKind::Warmth, TraitLevel::High);
    p.set_trait(TraitKind::Directness, TraitLevel::Highest);

    group.bench_function("compose_prompt", |b| {
        b.iter(|| black_box(&p).compose_prompt())
    });
    group.bench_function("behavioral_instructions", |b| {
        b.iter(|| black_box(&p).behavioral_instructions())
    });
    group.bench_function("active_traits", |b| {
        b.iter(|| black_box(&p).active_traits())
    });

    let mut q = PersonalityProfile::new("other");
    q.set_trait(TraitKind::Humor, TraitLevel::Lowest);
    q.set_trait(TraitKind::Warmth, TraitLevel::Low);
    group.bench_function("distance", |b| {
        b.iter(|| black_box(&p).distance(black_box(&q)))
    });
    group.bench_function("compatibility", |b| {
        b.iter(|| black_box(&p).compatibility(black_box(&q)))
    });
    group.bench_function("blend", |b| {
        b.iter(|| black_box(&p).blend(black_box(&q), 0.5))
    });
    group.bench_function("group_average", |b| {
        use bhava::traits::TraitGroup;
        b.iter(|| black_box(&p).group_average(black_box(TraitGroup::Social)))
    });
    group.bench_function("mutate_toward", |b| {
        b.iter_batched(
            || p.clone(),
            |mut profile| profile.mutate_toward(black_box(&q), 0.3),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("group_compatibility", |b| {
        use bhava::traits::TraitGroup;
        b.iter(|| black_box(&p).group_compatibility(black_box(&q), black_box(TraitGroup::Social)))
    });
    group.finish();
}

fn bench_mood_operations(c: &mut Criterion) {
    use bhava::mood::{Emotion, EmotionalState, MoodVector};
    let mut group = c.benchmark_group("mood");

    group.bench_function("stimulate", |b| {
        let mut s = EmotionalState::new();
        b.iter(|| s.stimulate(black_box(Emotion::Joy), black_box(0.5)))
    });
    group.bench_function("intensity", |b| {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.8);
        m.set(Emotion::Trust, 0.3);
        b.iter(|| black_box(&m).intensity())
    });
    group.bench_function("blend", |b| {
        let a = MoodVector::neutral();
        let mut bv = MoodVector::neutral();
        bv.set(Emotion::Joy, 1.0);
        b.iter(|| black_box(&a).blend(black_box(&bv), 0.5))
    });
    group.bench_function("decay", |b| {
        b.iter_batched(
            || {
                let mut m = MoodVector::neutral();
                m.set(Emotion::Joy, 0.8);
                m.set(Emotion::Trust, 0.5);
                m
            },
            |mut m| m.decay(black_box(0.3)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("dominant_emotion", |b| {
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, -0.9);
        m.set(Emotion::Joy, 0.3);
        b.iter(|| black_box(&m).dominant_emotion())
    });
    group.bench_function("nudge", |b| {
        let mut m = MoodVector::neutral();
        b.iter(|| m.nudge(black_box(Emotion::Trust), black_box(0.01)))
    });
    group.bench_function("deviation", |b| {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        s.stimulate(Emotion::Frustration, 0.3);
        b.iter(|| black_box(&s).deviation())
    });
    group.bench_function("apply_decay", |b| {
        b.iter_batched(
            || {
                let mut s = EmotionalState::new();
                s.stimulate(Emotion::Joy, 0.8);
                s
            },
            |mut s| {
                let future = s.last_updated + chrono::Duration::minutes(5);
                s.apply_decay(future);
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("classify", |b| {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        b.iter(|| black_box(&s).classify())
    });
    group.bench_function("apply_trigger", |b| {
        use bhava::mood::trigger_praised;
        let trigger = trigger_praised();
        b.iter_batched(
            || {
                let mut s = EmotionalState::new();
                s.stimulate(Emotion::Joy, 0.2);
                s
            },
            |mut s| s.apply_trigger(black_box(&trigger)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("snapshot", |b| {
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.5);
        s.stimulate(Emotion::Frustration, 0.3);
        b.iter(|| black_box(&s).snapshot())
    });
    group.bench_function("mood_trait_influence", |b| {
        use bhava::mood::mood_trait_influence;
        use bhava::traits::TraitKind;
        let mut m = MoodVector::neutral();
        m.set(Emotion::Frustration, 0.7);
        m.set(Emotion::Joy, 0.3);
        b.iter(|| mood_trait_influence(black_box(&m), black_box(TraitKind::Directness)))
    });
    group.bench_function("history_record", |b| {
        use bhava::mood::MoodHistory;
        b.iter_batched(
            || {
                let mut h = MoodHistory::new(100);
                let s = EmotionalState::new();
                for _ in 0..99 {
                    h.record(s.snapshot());
                }
                (h, s)
            },
            |(mut h, s)| h.record(s.snapshot()),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("history_deviation_trend", |b| {
        use bhava::mood::MoodHistory;
        let mut h = MoodHistory::new(100);
        let mut s = EmotionalState::new();
        for i in 0..50 {
            s.stimulate(Emotion::Joy, 0.01 * i as f32);
            h.record(s.snapshot());
        }
        b.iter(|| black_box(&h).deviation_trend())
    });
    group.bench_function("derive_baseline", |b| {
        use bhava::mood::derive_mood_baseline;
        use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        p.set_trait(TraitKind::Humor, TraitLevel::High);
        p.set_trait(TraitKind::Confidence, TraitLevel::High);
        b.iter(|| derive_mood_baseline(black_box(&p)))
    });
    group.bench_function("compose_mood_prompt", |b| {
        use bhava::mood::compose_mood_prompt;
        let mut s = EmotionalState::new();
        s.stimulate(Emotion::Joy, 0.8);
        b.iter(|| compose_mood_prompt(black_box(&s)))
    });
    group.bench_function("mood_tone_guide", |b| {
        use bhava::mood::{MoodState, mood_tone_guide};
        b.iter(|| mood_tone_guide(black_box(MoodState::Euphoric)))
    });
    group.finish();
}

fn bench_spirit(c: &mut Criterion) {
    use bhava::spirit::Spirit;
    let mut group = c.benchmark_group("spirit");

    let mut s = Spirit::new();
    s.add_passion("coding", "Writing elegant solutions", 0.9);
    s.add_inspiration("open source", "Community collaboration", 0.8);
    s.add_pain("tech debt", "Accumulated shortcuts", 0.6);

    group.bench_function("compose_prompt", |b| {
        b.iter(|| black_box(&s).compose_prompt())
    });
    group.finish();
}

fn bench_sentiment(c: &mut Criterion) {
    use bhava::sentiment;
    let mut group = c.benchmark_group("sentiment");

    group.bench_function("positive_short", |b| {
        b.iter(|| sentiment::analyze(black_box("This is great!")))
    });
    group.bench_function("negative_medium", |b| {
        b.iter(|| {
            sentiment::analyze(black_box(
                "This is terrible and broken, I hate it and it's useless.",
            ))
        })
    });
    group.bench_function("neutral_long", |b| {
        b.iter(|| {
            sentiment::analyze(black_box(
                "The meeting is scheduled for three o'clock in the main conference room on the second floor.",
            ))
        })
    });
    group.bench_function("mixed_emotions", |b| {
        b.iter(|| {
            sentiment::analyze(black_box(
                "I'm curious but frustrated with this broken yet interesting system that I trust.",
            ))
        })
    });
    group.bench_function("keyword_dense", |b| {
        b.iter(|| {
            sentiment::analyze(black_box(
                "good great excellent amazing wonderful fantastic love happy glad pleased awesome perfect",
            ))
        })
    });
    group.bench_function("negation", |b| {
        b.iter(|| sentiment::analyze(black_box("This is not good and not bad at all.")))
    });
    group.bench_function("intensifiers", |b| {
        b.iter(|| {
            sentiment::analyze(black_box(
                "This is very good and extremely helpful but slightly slow.",
            ))
        })
    });
    group.bench_function("sentences_3", |b| {
        b.iter(|| {
            sentiment::analyze_sentences(black_box(
                "This is great! That is terrible. Overall it was fine.",
            ))
        })
    });
    group.finish();
}

fn bench_archetype(c: &mut Criterion) {
    use bhava::archetype::{IdentityContent, IdentityLayer, compose_identity_prompt};
    let mut group = c.benchmark_group("archetype");

    group.bench_function("compose_preamble", |b| {
        b.iter(bhava::archetype::compose_preamble)
    });
    group.bench_function("compose_identity_2_layers", |b| {
        let mut content = IdentityContent::default();
        content.set(IdentityLayer::Soul, "You are a helpful assistant.");
        content.set(IdentityLayer::Spirit, "You are driven by curiosity.");
        b.iter(|| compose_identity_prompt(black_box(&content)))
    });
    group.bench_function("compose_identity_5_layers", |b| {
        let mut content = IdentityContent::default();
        content.set(IdentityLayer::Soul, "The core identity.");
        content.set(IdentityLayer::Spirit, "The driving force.");
        content.set(IdentityLayer::Brain, "Knowledge and memory.");
        content.set(IdentityLayer::Body, "Capabilities and tools.");
        content.set(IdentityLayer::Heart, "Vital rhythms.");
        b.iter(|| compose_identity_prompt(black_box(&content)))
    });
    group.bench_function("validate", |b| {
        use bhava::archetype::ValidationRules;
        let mut content = IdentityContent::default();
        content.set(IdentityLayer::Soul, "I am an agent with purpose.");
        content.set(IdentityLayer::Spirit, "Driven by curiosity.");
        let rules = ValidationRules::strict();
        b.iter(|| black_box(&content).validate(black_box(&rules)))
    });
    group.bench_function("template_apply", |b| {
        use bhava::archetype::template_guardian;
        let t = template_guardian();
        b.iter(|| black_box(&t).apply())
    });
    group.bench_function("crew_prompt_3", |b| {
        use bhava::archetype::{
            CrewMember, compose_crew_prompt, template_assistant, template_expert, template_guardian,
        };
        let members = vec![
            CrewMember {
                name: "Lead".into(),
                identity: template_expert().apply(),
            },
            CrewMember {
                name: "Guard".into(),
                identity: template_guardian().apply(),
            },
            CrewMember {
                name: "Helper".into(),
                identity: template_assistant().apply(),
            },
        ];
        b.iter(|| compose_crew_prompt(black_box(&members)))
    });
    group.bench_function("merge", |b| {
        use bhava::archetype::{template_expert, template_guardian};
        let a = template_expert().apply();
        let g = template_guardian().apply();
        b.iter(|| black_box(&a).merge(black_box(&g), "\n\n"))
    });
    group.finish();
}

fn bench_presets(c: &mut Criterion) {
    use bhava::archetype::compose_identity_prompt;
    use bhava::presets;
    let mut group = c.benchmark_group("presets");

    group.bench_function("get_preset", |b| {
        b.iter(|| presets::get_preset(black_box("blue-shirt-guy")))
    });
    group.bench_function("list_presets", |b| b.iter(presets::list_presets));
    group.bench_function("preset_full_prompt", |b| {
        b.iter(|| {
            let p = presets::get_preset("blue-shirt-guy").unwrap();
            let personality = p.profile.compose_prompt();
            let identity = compose_identity_prompt(&p.identity);
            black_box((personality, identity))
        })
    });
    group.finish();
}

fn bench_serde(c: &mut Criterion) {
    use bhava::mood::{Emotion, EmotionalState, MoodVector};
    use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
    let mut group = c.benchmark_group("serde");

    let mut profile = PersonalityProfile::new("bench");
    profile.set_trait(TraitKind::Humor, TraitLevel::Highest);
    profile.set_trait(TraitKind::Warmth, TraitLevel::High);
    let profile_json = serde_json::to_string(&profile).unwrap();

    group.bench_function("personality_serialize", |b| {
        b.iter(|| serde_json::to_string(black_box(&profile)).unwrap())
    });
    group.bench_function("personality_deserialize", |b| {
        b.iter(|| serde_json::from_str::<PersonalityProfile>(black_box(&profile_json)).unwrap())
    });

    let mut mood = MoodVector::neutral();
    mood.set(Emotion::Joy, 0.7);
    mood.set(Emotion::Trust, -0.3);
    let mood_json = serde_json::to_string(&mood).unwrap();

    group.bench_function("mood_serialize", |b| {
        b.iter(|| serde_json::to_string(black_box(&mood)).unwrap())
    });
    group.bench_function("mood_deserialize", |b| {
        b.iter(|| serde_json::from_str::<MoodVector>(black_box(&mood_json)).unwrap())
    });

    let mut state = EmotionalState::new();
    state.stimulate(Emotion::Joy, 0.5);
    let state_json = serde_json::to_string(&state).unwrap();

    group.bench_function("emotional_state_serialize", |b| {
        b.iter(|| serde_json::to_string(black_box(&state)).unwrap())
    });
    group.bench_function("emotional_state_deserialize", |b| {
        b.iter(|| serde_json::from_str::<EmotionalState>(black_box(&state_json)).unwrap())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_trait_behavior,
    bench_personality_prompt,
    bench_mood_operations,
    bench_sentiment,
    bench_archetype,
    bench_presets,
    bench_spirit,
    bench_serde,
);
criterion_main!(benches);
