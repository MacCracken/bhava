use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_trait_behavior(c: &mut Criterion) {
    use bhava::traits::{TraitKind, TraitLevel, trait_behavior};
    c.bench_function("trait_behavior_lookup", |b| {
        b.iter(|| trait_behavior(black_box(TraitKind::Humor), black_box(TraitLevel::Highest)))
    });
}

fn bench_personality_prompt(c: &mut Criterion) {
    use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
    let mut p = PersonalityProfile::new("test");
    p.set_trait(TraitKind::Humor, TraitLevel::Highest);
    p.set_trait(TraitKind::Warmth, TraitLevel::High);
    p.set_trait(TraitKind::Directness, TraitLevel::Highest);
    c.bench_function("compose_prompt", |b| {
        b.iter(|| black_box(&p).compose_prompt())
    });
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
        b.iter(|| sentiment::analyze(black_box(
            "The meeting is scheduled for three o'clock in the main conference room on the second floor."
        )))
    });
    group.finish();
}

fn bench_archetype(c: &mut Criterion) {
    c.bench_function("compose_preamble", |b| {
        b.iter(bhava::archetype::compose_preamble)
    });
}

criterion_group!(
    benches,
    bench_trait_behavior,
    bench_personality_prompt,
    bench_mood_operations,
    bench_sentiment,
    bench_archetype,
);
criterion_main!(benches);
