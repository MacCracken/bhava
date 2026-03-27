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
    group.bench_function("action_tendency", |b| {
        use bhava::mood::action_tendency;
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.7);
        m.set(Emotion::Trust, 0.4);
        b.iter(|| action_tendency(black_box(&m)))
    });
    group.bench_function("contagion", |b| {
        use bhava::mood::{ContagionParams, compute_contagion};
        let mut sender = MoodVector::neutral();
        sender.set(Emotion::Joy, 0.8);
        sender.set(Emotion::Frustration, 0.3);
        let sp = ContagionParams {
            expressiveness: 0.7,
            susceptibility: 0.0,
        };
        let rp = ContagionParams {
            expressiveness: 0.0,
            susceptibility: 0.6,
        };
        b.iter(|| compute_contagion(black_box(&sender), black_box(&sp), black_box(&rp), 0.5))
    });
    group.bench_function("compound_emotions", |b| {
        use bhava::mood::detect_compound_emotions;
        let mut m = MoodVector::neutral();
        m.set(Emotion::Joy, 0.7);
        m.set(Emotion::Trust, 0.6);
        m.set(Emotion::Frustration, 0.3);
        b.iter(|| detect_compound_emotions(black_box(&m), 0.2))
    });
    group.bench_function("damped_step", |b| {
        use bhava::mood::DampedResponse;
        b.iter_batched(
            || {
                let mut d = DampedResponse::new(0.5, 2.0);
                d.impulse(1.0);
                d
            },
            |mut d| d.step(black_box(0.05)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("memory_recall", |b| {
        use bhava::mood::EmotionalMemoryBank;
        let mut bank = EmotionalMemoryBank::new(100);
        let m = MoodVector::neutral();
        for i in 0..50 {
            bank.record(format!("tag_{i}"), &m, 0.5 + (i as f32) * 0.01);
        }
        b.iter(|| bank.recall(black_box("tag_25")))
    });
    group.bench_function("adaptive_baseline_adapt", |b| {
        use bhava::mood::AdaptiveBaseline;
        let mut positive = MoodVector::neutral();
        positive.set(Emotion::Joy, 0.6);
        b.iter_batched(
            || AdaptiveBaseline::new(MoodVector::neutral()),
            |mut ab| ab.adapt(black_box(&positive)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_appraisal(c: &mut Criterion) {
    use bhava::appraisal::{Appraisal, appraise};
    let mut group = c.benchmark_group("appraisal");

    group.bench_function("positive_event", |b| {
        let a = Appraisal::event("good news", 0.8);
        b.iter(|| appraise(black_box(&a), None))
    });
    group.bench_function("complex_appraisal", |b| {
        let a = Appraisal::event("rival sabotaged", -0.7)
            .with_praise(-0.8)
            .caused_by("rival");
        b.iter(|| appraise(black_box(&a), Some(-0.5)))
    });
    group.finish();
}

fn bench_ocean(c: &mut Criterion) {
    use bhava::traits::{
        OceanScores, PersonalityProfile, TraitKind, TraitLevel, personality_entropy,
        profile_from_ocean,
    };
    let mut group = c.benchmark_group("ocean");

    let mut p = PersonalityProfile::new("test");
    p.set_trait(TraitKind::Warmth, TraitLevel::Highest);
    p.set_trait(TraitKind::Creativity, TraitLevel::High);

    group.bench_function("to_ocean", |b| b.iter(|| black_box(&p).to_ocean()));
    group.bench_function("from_ocean", |b| {
        let o = OceanScores {
            openness: 0.7,
            conscientiousness: 0.3,
            extraversion: 0.5,
            agreeableness: 0.6,
            neuroticism: -0.2,
        };
        b.iter(|| profile_from_ocean("test", black_box(&o)))
    });
    group.bench_function("entropy", |b| b.iter(|| personality_entropy(black_box(&p))));
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

fn bench_relationship(c: &mut Criterion) {
    use bhava::relationship::RelationshipGraph;
    let mut group = c.benchmark_group("relationship");

    group.bench_function("record_interaction", |b| {
        b.iter_batched(
            RelationshipGraph::new,
            |mut g| g.record_interaction("a", "b", 0.3, 0.1),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("decay_10", |b| {
        b.iter_batched(
            || {
                let mut g = RelationshipGraph::new();
                for i in 0..10 {
                    g.record_interaction("a", &format!("b{i}"), 0.5, 0.3);
                }
                g
            },
            |mut g| g.decay_all(),
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_markdown(c: &mut Criterion) {
    use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
    let mut group = c.benchmark_group("markdown");

    let mut p = PersonalityProfile::new("BenchBot");
    p.description = Some("A benchmark personality".into());
    p.set_trait(TraitKind::Humor, TraitLevel::Highest);
    p.set_trait(TraitKind::Warmth, TraitLevel::High);
    p.set_trait(TraitKind::Precision, TraitLevel::High);
    let md = p.to_markdown();

    group.bench_function("to_markdown", |b| b.iter(|| black_box(&p).to_markdown()));
    group.bench_function("from_markdown", |b| {
        b.iter(|| PersonalityProfile::from_markdown(black_box(&md)))
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

fn bench_ai(c: &mut Criterion) {
    use bhava::ai::{
        InteractionOutcome, apply_sentiment_feedback, build_personality_metadata,
        compose_system_prompt, feedback_from_outcome,
    };
    use bhava::archetype::{IdentityContent, IdentityLayer};
    use bhava::mood::{Emotion, EmotionalState};
    use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};

    let mut group = c.benchmark_group("ai");

    let mut profile = PersonalityProfile::new("BenchBot");
    profile.set_trait(TraitKind::Humor, TraitLevel::High);
    profile.set_trait(TraitKind::Warmth, TraitLevel::Highest);
    let mut identity = IdentityContent::default();
    identity.set(IdentityLayer::Soul, "You are a helpful assistant.");
    identity.set(IdentityLayer::Spirit, "Driven by curiosity.");
    let mut mood = EmotionalState::new();
    mood.stimulate(Emotion::Joy, 0.5);

    group.bench_function("compose_system_prompt", |b| {
        b.iter(|| {
            compose_system_prompt(
                black_box(&profile),
                black_box(&identity),
                Some(black_box(&mood)),
                Some("Passionate about helping."),
            )
        })
    });
    group.bench_function("compose_system_prompt_minimal", |b| {
        b.iter(|| compose_system_prompt(black_box(&profile), black_box(&identity), None, None))
    });
    group.bench_function("apply_sentiment_feedback", |b| {
        b.iter_batched(
            EmotionalState::new,
            |mut state| {
                apply_sentiment_feedback(
                    black_box("This is wonderful and amazing work!"),
                    &mut state,
                    1.0,
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("build_personality_metadata", |b| {
        b.iter(|| build_personality_metadata(black_box(&profile), Some(black_box(&mood))))
    });
    group.bench_function("feedback_from_outcome", |b| {
        b.iter_batched(
            EmotionalState::new,
            |mut state| feedback_from_outcome(&mut state, black_box(InteractionOutcome::Praised)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_monitor(c: &mut Criterion) {
    use bhava::monitor::SentimentMonitor;
    use bhava::mood::EmotionalState;
    let mut group = c.benchmark_group("monitor");

    group.bench_function("feed_sentence", |b| {
        b.iter_batched(
            || SentimentMonitor::new(1.0),
            |mut m| m.feed(black_box("This is wonderful work!")),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("feed_and_apply", |b| {
        b.iter_batched(
            || (SentimentMonitor::new(0.5), EmotionalState::new()),
            |(mut m, mut s)| m.feed_and_apply(black_box("Great job!"), &mut s),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("streaming_10_tokens", |b| {
        let tokens = [
            "I ",
            "really ",
            "love ",
            "this ",
            "project! ",
            "But ",
            "the ",
            "bugs ",
            "are ",
            "terrible.",
        ];
        b.iter_batched(
            || (SentimentMonitor::new(0.5), EmotionalState::new()),
            |(mut m, mut s)| {
                for t in &tokens {
                    m.feed_and_apply(black_box(t), &mut s);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_rhythm(c: &mut Criterion) {
    use bhava::mood::MoodVector;
    use bhava::rhythm::{SeasonalRhythm, UltradianRhythm, apply_rhythms, default_biorhythm};
    let mut group = c.benchmark_group("rhythm");

    let now = chrono::Utc::now();
    let ultradian = UltradianRhythm::new();
    let seasonal = SeasonalRhythm::new();
    let biorhythm = default_biorhythm(now - chrono::Duration::hours(100));

    group.bench_function("ultradian_modulate", |b| {
        b.iter(|| ultradian.modulate(black_box(now)))
    });
    group.bench_function("seasonal_modulate", |b| {
        b.iter(|| seasonal.modulate(black_box(now)))
    });
    group.bench_function("biorhythm_modulate", |b| {
        b.iter(|| biorhythm.modulate(black_box(now)))
    });
    group.bench_function("apply_all_rhythms", |b| {
        b.iter_batched(
            MoodVector::neutral,
            |mut mood| {
                apply_rhythms(
                    &mut mood,
                    now,
                    Some(&ultradian),
                    Some(&seasonal),
                    Some(&biorhythm),
                )
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_energy(c: &mut Criterion) {
    use bhava::energy::{EnergyState, exertion_from_mood};
    use bhava::mood::{Emotion, MoodVector};
    let mut group = c.benchmark_group("energy");

    group.bench_function("tick_exertion", |b| {
        b.iter_batched(
            EnergyState::new,
            |mut e| e.tick(black_box(0.6)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("performance", |b| {
        let e = EnergyState::new();
        b.iter(|| black_box(&e).performance())
    });
    group.bench_function("exertion_from_mood", |b| {
        let mut mood = MoodVector::neutral();
        mood.set(Emotion::Arousal, 0.7);
        mood.set(Emotion::Joy, 0.5);
        b.iter(|| exertion_from_mood(black_box(&mood)))
    });
    group.finish();
}

fn bench_circadian(c: &mut Criterion) {
    use bhava::circadian::{Chronotype, CircadianRhythm};
    let mut group = c.benchmark_group("circadian");

    let now = chrono::Utc::now();
    let c_rhythm = CircadianRhythm::with_chronotype(Chronotype::NightOwl);

    group.bench_function("alertness", |b| {
        b.iter(|| c_rhythm.alertness(black_box(now)))
    });
    group.bench_function("mood_modulation", |b| {
        b.iter(|| c_rhythm.mood_modulation(black_box(now)))
    });
    group.bench_function("decay_rate_modifier", |b| {
        b.iter(|| c_rhythm.decay_rate_modifier(black_box(now)))
    });
    group.finish();
}

fn bench_flow(c: &mut Criterion) {
    use bhava::flow::FlowState;
    use bhava::mood::{Emotion, MoodVector};
    let mut group = c.benchmark_group("flow");

    let mut flow_mood = MoodVector::neutral();
    flow_mood.set(Emotion::Interest, 0.6);
    flow_mood.set(Emotion::Arousal, 0.3);
    flow_mood.set(Emotion::Dominance, 0.3);
    flow_mood.set(Emotion::Frustration, 0.1);

    group.bench_function("tick", |b| {
        b.iter_batched(
            FlowState::new,
            |mut f| f.tick(black_box(&flow_mood), 0.5, 0.5),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("check_conditions", |b| {
        let f = FlowState::new();
        b.iter(|| f.check_conditions(black_box(&flow_mood), 0.5, 0.5))
    });
    group.finish();
}

fn bench_eq(c: &mut Criterion) {
    use bhava::eq::{EqProfile, compose_eq_prompt};
    let mut group = c.benchmark_group("eq");

    let eq = EqProfile::with_scores(0.8, 0.6, 0.7, 0.9);
    group.bench_function("overall", |b| b.iter(|| black_box(&eq).overall()));
    group.bench_function("compose_prompt", |b| {
        b.iter(|| compose_eq_prompt(black_box(&eq)))
    });
    group.finish();
}

fn bench_display_rules(c: &mut Criterion) {
    use bhava::display_rules::{apply_display_rules, celebration_context, professional_context};
    use bhava::mood::{Emotion, EmotionalState};
    use bhava::regulation::RegulatedMood;
    let mut group = c.benchmark_group("display_rules");

    let prof = professional_context();
    let celeb = celebration_context();

    group.bench_function("apply_professional", |b| {
        b.iter_batched(
            || {
                let mut s = EmotionalState::new();
                s.stimulate(Emotion::Joy, 0.6);
                s.stimulate(Emotion::Frustration, 0.5);
                RegulatedMood::from_state(&s)
            },
            |mut r| apply_display_rules(&mut r, black_box(&prof)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("apply_celebration", |b| {
        b.iter_batched(
            || {
                let mut s = EmotionalState::new();
                s.stimulate(Emotion::Joy, 0.6);
                RegulatedMood::from_state(&s)
            },
            |mut r| apply_display_rules(&mut r, black_box(&celeb)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_microexpr(c: &mut Criterion) {
    use bhava::microexpr::detect_micro_expressions;
    use bhava::mood::{Emotion, EmotionalState};
    use bhava::regulation::{RegulatedMood, RegulationStrategy};
    let mut group = c.benchmark_group("microexpr");

    group.bench_function("detect", |b| {
        b.iter_batched(
            || {
                let mut s = EmotionalState::new();
                s.stimulate(Emotion::Frustration, 0.8);
                s.stimulate(Emotion::Arousal, 0.5);
                let mut r = RegulatedMood::from_state(&s);
                r.regulate(
                    RegulationStrategy::Suppress {
                        target: Emotion::Frustration,
                        strength: 0.9,
                    },
                    1.0,
                );
                r
            },
            |r| detect_micro_expressions(black_box(&r)),
            criterion::BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn bench_affective(c: &mut Criterion) {
    use bhava::affective::{compute_affective_metrics, snapshot_complexity, snapshot_granularity};
    use bhava::mood::{Emotion, MoodHistory, MoodSnapshot, MoodState, MoodVector};
    let mut group = c.benchmark_group("affective");

    let mut mood = MoodVector::neutral();
    mood.set(Emotion::Joy, 0.5);
    mood.set(Emotion::Arousal, 0.3);

    group.bench_function("snapshot_complexity", |b| {
        b.iter(|| snapshot_complexity(black_box(&mood)))
    });
    group.bench_function("snapshot_granularity", |b| {
        b.iter(|| snapshot_granularity(black_box(&mood)))
    });
    group.bench_function("compute_metrics_50", |b| {
        let mut h = MoodHistory::new(50);
        for i in 0..50 {
            let v = (i as f32 * 0.1).sin() * 0.5;
            let mut m = MoodVector::neutral();
            m.set(Emotion::Joy, v);
            m.set(Emotion::Arousal, v * 0.5);
            h.record(MoodSnapshot {
                mood: m,
                state: MoodState::Calm,
                deviation: v.abs(),
                timestamp: chrono::Utc::now(),
            });
        }
        b.iter(|| compute_affective_metrics(black_box(&h)))
    });
    group.finish();
}

fn bench_proximity(c: &mut Criterion) {
    use bhava::mood::{Emotion, MoodTrigger};
    use bhava::proximity::{Falloff, ProximityRule, ProximitySystem};
    let mut group = c.benchmark_group("proximity");

    let mut sys = ProximitySystem::new();
    for i in 0..20 {
        sys.add_rule(ProximityRule {
            location_tag: format!("loc_{}", i % 5),
            radius: 20.0,
            trigger: MoodTrigger::new("effect").respond(Emotion::Joy, 0.2),
            falloff: Falloff::Linear,
        });
    }
    group.bench_function("evaluate_20_rules", |b| {
        b.iter(|| sys.evaluate(black_box("loc_2"), 10.0))
    });
    group.finish();
}

fn bench_reasoning(c: &mut Criterion) {
    use bhava::reasoning::{reasoning_scores, select_reasoning_strategy};
    use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
    let mut group = c.benchmark_group("reasoning");

    let mut p = PersonalityProfile::new("test");
    p.set_trait(TraitKind::Precision, TraitLevel::Highest);
    p.set_trait(TraitKind::Skepticism, TraitLevel::High);

    group.bench_function("select_strategy", |b| {
        b.iter(|| select_reasoning_strategy(black_box(&p)))
    });
    group.bench_function("all_scores", |b| b.iter(|| reasoning_scores(black_box(&p))));
    group.finish();
}

fn bench_salience(c: &mut Criterion) {
    use bhava::appraisal::Appraisal;
    use bhava::salience::classify_salience;
    let mut group = c.benchmark_group("salience");

    let a = Appraisal::event("test", 0.8).with_praise(0.6);
    group.bench_function("classify", |b| {
        b.iter(|| classify_salience(black_box(&a), 0.5, 0.3))
    });
    group.finish();
}

fn bench_actr(c: &mut Criterion) {
    use bhava::actr::ActivationStore;
    let mut group = c.benchmark_group("actr");

    group.bench_function("rehearse_100", |b| {
        b.iter_batched(
            || ActivationStore::new(200),
            |mut store| {
                for i in 0..100 {
                    store.rehearse(format!("item_{}", i % 50), i as f64);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("retrieve_above", |b| {
        let mut store = ActivationStore::new(100);
        for i in 0..50 {
            store.rehearse(format!("item_{i}"), i as f64);
        }
        b.iter(|| store.retrieve_above(black_box(0.0), 100.0))
    });
    group.finish();
}

fn bench_preference(c: &mut Criterion) {
    use bhava::preference::PreferenceStore;
    let mut group = c.benchmark_group("preference");

    group.bench_function("record_100", |b| {
        b.iter_batched(
            || PreferenceStore::new(200),
            |mut store| {
                let now = chrono::Utc::now();
                for i in 0..100 {
                    store.record_outcome(format!("item_{}", i % 50), 0.5, now);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
    group.bench_function("top_preferences", |b| {
        let mut store = PreferenceStore::new(100);
        let now = chrono::Utc::now();
        for i in 0..50 {
            store.record_outcome(format!("item_{i}"), (i as f32 - 25.0) / 25.0, now);
        }
        b.iter(|| store.top_preferences(black_box(10)))
    });
    group.finish();
}

fn bench_belief(c: &mut Criterion) {
    use bhava::belief::{BeliefKind, BeliefSystem, SelfModel, WorldModel};

    let mut group = c.benchmark_group("belief");

    group.bench_function("reinforce_100", |b| {
        b.iter_batched(
            || BeliefSystem::new(64),
            |mut sys| {
                let now = chrono::Utc::now();
                for i in 0..100 {
                    sys.reinforce_or_create(
                        BeliefKind::SelfBelief,
                        format!("self:tag_{}", i % 30),
                        0.5,
                        "evidence",
                        now,
                    );
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("coherence_64", |b| {
        let mut sys = BeliefSystem::new(64);
        let now = chrono::Utc::now();
        for i in 0..64 {
            let valence = if i % 3 == 0 { -0.5 } else { 0.5 };
            sys.reinforce_or_create(
                BeliefKind::SelfBelief,
                format!("self:tag_{i}"),
                valence,
                "evidence",
                now,
            );
        }
        b.iter(|| sys.coherence())
    });

    group.bench_function("coherence_256", |b| {
        let mut sys = BeliefSystem::new(256);
        let now = chrono::Utc::now();
        let kinds = [
            BeliefKind::SelfBelief,
            BeliefKind::WorldBelief,
            BeliefKind::OtherBelief,
            BeliefKind::UniversalBelief,
        ];
        for i in 0..256 {
            let valence = if i % 3 == 0 { -0.5 } else { 0.5 };
            sys.reinforce_or_create(kinds[i % 4], format!("tag_{i}"), valence, "evidence", now);
        }
        b.iter(|| sys.coherence())
    });

    group.bench_function("beliefs_of_kind_iterate", |b| {
        let mut sys = BeliefSystem::new(64);
        let now = chrono::Utc::now();
        for i in 0..64 {
            let kind = if i % 2 == 0 {
                BeliefKind::SelfBelief
            } else {
                BeliefKind::WorldBelief
            };
            sys.reinforce_or_create(kind, format!("tag_{i}"), 0.5, "ev", now);
        }
        b.iter(|| {
            let count: usize = sys
                .beliefs_of_kind(black_box(BeliefKind::SelfBelief))
                .count();
            black_box(count)
        })
    });

    group.bench_function("self_model_update", |b| {
        let mut sys = BeliefSystem::new(64);
        let now = chrono::Utc::now();
        let tags = [
            "self:warm",
            "self:confident",
            "self:creative",
            "self:curious",
            "self:brave",
        ];
        for tag in &tags {
            for i in 0..10 {
                sys.reinforce_or_create(BeliefKind::SelfBelief, *tag, 0.7, &format!("ev_{i}"), now);
            }
        }
        b.iter_batched(
            SelfModel::new,
            |mut sm| sm.update_from_beliefs(black_box(&sys)),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("world_model_update", |b| {
        let mut sys = BeliefSystem::new(64);
        let now = chrono::Utc::now();
        for tag in &["world:safe", "world:meaningful", "world:trustworthy"] {
            for i in 0..10 {
                sys.reinforce_or_create(
                    BeliefKind::WorldBelief,
                    *tag,
                    0.6,
                    &format!("ev_{i}"),
                    now,
                );
            }
        }
        b.iter_batched(
            WorldModel::new,
            |mut wm| wm.update_from_beliefs(black_box(&sys)),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("cosmic_understanding", |b| {
        let mut sys = BeliefSystem::new(64);
        let now = chrono::Utc::now();
        sys.reinforce_or_create(BeliefKind::WorldBelief, "world:meaningful", 0.9, "ev", now);
        let mut wm = WorldModel::new();
        wm.update_from_beliefs(&sys);
        b.iter(|| bhava::belief::cosmic_understanding(black_box(0.8), black_box(&wm), 0.9))
    });

    group.bench_function("decay_64", |b| {
        b.iter_batched(
            || {
                let mut sys = BeliefSystem::new(64);
                let now = chrono::Utc::now();
                for i in 0..64 {
                    sys.reinforce_or_create(
                        BeliefKind::SelfBelief,
                        format!("tag_{i}"),
                        0.5,
                        "ev",
                        now,
                    );
                }
                sys
            },
            |mut sys| sys.decay(black_box(0.1)),
            criterion::BatchSize::SmallInput,
        )
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
    bench_appraisal,
    bench_ocean,
    bench_spirit,
    bench_relationship,
    bench_markdown,
    bench_ai,
    bench_monitor,
    bench_serde,
    bench_rhythm,
    bench_energy,
    bench_circadian,
    bench_flow,
    bench_eq,
    bench_display_rules,
    bench_microexpr,
    bench_affective,
    bench_proximity,
    bench_reasoning,
    bench_salience,
    bench_actr,
    bench_preference,
    bench_belief,
    bench_belief_emotion,
    bench_intuition,
    bench_aesthetic,
);
criterion_main!(benches);

fn bench_belief_emotion(c: &mut Criterion) {
    use bhava::appraisal::AppraisedEmotion;
    use bhava::belief::{
        BeliefKind, BeliefSystem, apply_emotion_to_beliefs, classify_emotion, shadow_beliefs,
    };

    let mut group = c.benchmark_group("belief_emotion");

    group.bench_function("classify_emotion", |b| {
        b.iter(|| classify_emotion(black_box(AppraisedEmotion::Pride)))
    });

    group.bench_function("apply_emotion_100", |b| {
        b.iter_batched(
            || BeliefSystem::new(64),
            |mut sys| {
                let now = chrono::Utc::now();
                let emotions = [
                    AppraisedEmotion::Joy,
                    AppraisedEmotion::Pride,
                    AppraisedEmotion::Fear,
                    AppraisedEmotion::Anger,
                    AppraisedEmotion::Gratitude,
                ];
                for i in 0..100 {
                    apply_emotion_to_beliefs(
                        &mut sys,
                        emotions[i % emotions.len()],
                        0.7,
                        i % 2 == 0,
                        Some("npc"),
                        (i % 3) as f32 * 0.3,
                        now,
                    );
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("shadow_beliefs_query", |b| {
        let mut sys = BeliefSystem::new(64);
        let now = chrono::Utc::now();
        for i in 0..30 {
            sys.reinforce_or_create_with_suppression(
                BeliefKind::SelfBelief,
                format!("tag_{i}"),
                0.5,
                "ev",
                (i as f32 / 30.0).min(1.0),
                now,
            );
        }
        b.iter(|| shadow_beliefs(black_box(&sys), 0.3))
    });

    group.bench_function("decay_with_shadow", |b| {
        b.iter_batched(
            || {
                let mut sys = BeliefSystem::new(64);
                let now = chrono::Utc::now();
                for i in 0..64 {
                    sys.reinforce_or_create_with_suppression(
                        BeliefKind::SelfBelief,
                        format!("tag_{i}"),
                        0.5,
                        "ev",
                        (i as f32 / 64.0).min(1.0),
                        now,
                    );
                }
                sys
            },
            |mut sys| sys.decay(black_box(0.1)),
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_intuition(c: &mut Criterion) {
    use bhava::intuition::*;
    use bhava::salience::SalienceScore;

    let mut group = c.benchmark_group("intuition");

    group.bench_function("synthesize_5_tags_3_sources", |b| {
        let profile = IntuitionProfile {
            sensitivity: 0.8,
            integration_depth: 0.8,
            trust_in_intuition: 0.8,
        };
        let activations = ActivationSignals {
            entries: (0..5).map(|i| (format!("tag_{i}"), 2.0)).collect(),
        };
        let salience = SalienceSignals {
            entries: (0..5)
                .map(|i| (format!("tag_{i}"), SalienceScore::new(0.7, 0.6)))
                .collect(),
        };
        let perception = PerceptionSignals {
            entries: (0..5).map(|i| (format!("tag_{i}"), 0.7)).collect(),
        };
        b.iter(|| {
            synthesize_intuition(
                black_box(&activations),
                black_box(&salience),
                &MicroExpressionSignals::default(),
                &AffectiveSignals::default(),
                black_box(&perception),
                &AestheticSignals::default(),
                &profile,
            )
        })
    });

    group.bench_function("synthesize_20_tags", |b| {
        let profile = IntuitionProfile {
            sensitivity: 0.8,
            integration_depth: 0.8,
            trust_in_intuition: 0.8,
        };
        let activations = ActivationSignals {
            entries: (0..20).map(|i| (format!("tag_{i}"), 1.5)).collect(),
        };
        let salience = SalienceSignals {
            entries: (0..10)
                .map(|i| (format!("tag_{i}"), SalienceScore::new(0.6, 0.5)))
                .collect(),
        };
        b.iter(|| {
            synthesize_intuition(
                black_box(&activations),
                black_box(&salience),
                &MicroExpressionSignals::default(),
                &AffectiveSignals::default(),
                &PerceptionSignals::default(),
                &AestheticSignals::default(),
                &profile,
            )
        })
    });

    group.bench_function("profile_from_personality", |b| {
        let profile = bhava::traits::PersonalityProfile::new("test");
        b.iter(|| IntuitionProfile::from_personality(black_box(&profile)))
    });

    group.bench_function("active_layer", |b| {
        b.iter(|| {
            active_layer(
                black_box(0.5),
                black_box(0.6),
                black_box(0.7),
                black_box(0.3),
            )
        })
    });

    group.finish();
}

fn bench_aesthetic(c: &mut Criterion) {
    use bhava::aesthetic::*;
    use bhava::belief::BeliefSystem;

    let mut group = c.benchmark_group("aesthetic");

    group.bench_function("record_exposure", |b| {
        let exposure = AestheticExposure {
            dimension: AestheticDimension::Beauty,
            tag: "music:classical".to_owned(),
            intensity: 0.8,
        };
        b.iter_batched(
            AestheticProfile::new,
            |mut profile| profile.record_exposure(black_box(&exposure), chrono::Utc::now()),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("record_exposure_50", |b| {
        b.iter_batched(
            || {
                let mut profile = AestheticProfile::new();
                let now = chrono::Utc::now();
                for i in 0..50 {
                    let dim = AestheticDimension::ALL[i % 5];
                    profile.record_exposure(
                        &AestheticExposure {
                            dimension: dim,
                            tag: format!("art:{i}"),
                            intensity: 0.7,
                        },
                        now,
                    );
                }
                profile
            },
            |mut profile| {
                profile.record_exposure(
                    &AestheticExposure {
                        dimension: AestheticDimension::Meaning,
                        tag: "test".to_owned(),
                        intensity: 0.8,
                    },
                    chrono::Utc::now(),
                );
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("crystallize_beliefs", |b| {
        let mut profile = AestheticProfile::new();
        let now = chrono::Utc::now();
        for _ in 0..20 {
            for &dim in AestheticDimension::ALL {
                profile.record_exposure(
                    &AestheticExposure {
                        dimension: dim,
                        tag: format!("test:{dim}"),
                        intensity: 0.8,
                    },
                    now,
                );
            }
        }
        b.iter_batched(
            || BeliefSystem::new(32),
            |mut bs| crystallize_beliefs(black_box(&profile), &mut bs, now),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("aesthetic_trait_pressure", |b| {
        let mut profile = AestheticProfile::new();
        let now = chrono::Utc::now();
        for _ in 0..20 {
            profile.record_exposure(
                &AestheticExposure {
                    dimension: AestheticDimension::Beauty,
                    tag: "art:test".to_owned(),
                    intensity: 0.8,
                },
                now,
            );
        }
        b.iter(|| aesthetic_trait_pressure(black_box(&profile)))
    });

    group.bench_function("aesthetic_mood_shift", |b| {
        let exposure = AestheticExposure {
            dimension: AestheticDimension::Sublimity,
            tag: "nature:mountains".to_owned(),
            intensity: 0.9,
        };
        b.iter(|| aesthetic_mood_shift(black_box(&exposure), black_box(0.7)))
    });

    group.finish();
}
