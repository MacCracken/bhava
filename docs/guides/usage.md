# Usage Guide

## Philosophy

Bhava is a **computation library** — it provides the math, models, and data structures for personality and emotional simulation. It does not perform I/O, manage threads, or make network calls in its core modules.

**The consumer controls the lifecycle.** Bhava computes; your application decides when to call it, where to store results, and how to act on behavioral signals.

### Design Principles

1. **Pure computation** — no side effects, no I/O in core modules
2. **Feature-gated** — pay only for what you use
3. **Serde everywhere** — persist anything, transmit anything
4. **Pluggable storage** — implement `BhavaStore` for your backend
5. **Deterministic** — same inputs always produce same outputs (fixed-order iteration)

## Common Patterns

### Creating an Agent with Personality

```rust
use bhava::traits::{PersonalityProfile, TraitKind, TraitLevel};
use bhava::mood::EmotionalState;
use bhava::spirit::Spirit;
use bhava::archetype::{IdentityContent, IdentityLayer};

// 1. Define personality
let mut profile = PersonalityProfile::new("Friday");
profile.set_trait(TraitKind::Formality, TraitLevel::High);
profile.set_trait(TraitKind::Precision, TraitLevel::High);

// 2. Create emotional state with personality-derived baseline
let baseline = bhava::mood::derive_mood_baseline(&profile);
let mut state = EmotionalState::with_baseline(baseline);

// 3. Define identity
let mut identity = IdentityContent::default();
identity.set(IdentityLayer::Soul, "You are Friday, a capable assistant.");

// 4. Define spirit
let mut spirit = Spirit::new();
spirit.add_passion("efficiency", "Getting things done right", 0.9);
```

### The Emotion Loop

```rust
// Stimulus → Appraisal → Emotion → Decay → Classify → Act

// 1. Something happens
let appraisal = bhava::appraisal::Appraisal::event("user praised work", 0.7)
    .with_praise(0.5)
    .caused_by("user");

// 2. Appraise the event
let result = bhava::appraisal::appraise(&appraisal, Some(0.6));

// 3. Apply to emotional state
bhava::appraisal::apply_appraisal(&mut state, &result);

// 4. Time passes — decay toward baseline
let now = chrono::Utc::now();
state.apply_decay(now);

// 5. Classify current mood
let mood_state = state.classify();

// 6. Derive action tendency
let tendency = bhava::mood::action_tendency(&state.mood);
```

### Building System Prompts

```rust
use bhava::ai::compose_system_prompt;

let prompt = compose_system_prompt(
    &profile,
    &identity,
    Some(&state),                    // inject current mood
    Some(&spirit.compose_prompt()),  // inject spirit
);
// → Single string ready for InferenceRequest.system
```

### Sentiment Feedback Loop

```rust
use bhava::ai::apply_sentiment_feedback;

let ai_response = "I'm happy to help with that!";
let result = apply_sentiment_feedback(ai_response, &mut state, 0.5);
// state.mood is now updated based on the AI's own emotional tone
```

### Live Streaming Monitoring

```rust
use bhava::monitor::SentimentMonitor;

let mut monitor = SentimentMonitor::new(0.3);
for token in stream {
    monitor.feed_and_apply(&token, &mut state);
}
let remaining = monitor.flush();
let summary = monitor.summary();
```

### Multi-Agent Scenarios

```rust
use bhava::mood::{compute_contagion, contagion_from_personality};

// Emotional contagion between agents
let sender_params = contagion_from_personality(&sender_profile);
let receiver_params = contagion_from_personality(&receiver_profile);
let affinity = relationship_graph.get("sender", "receiver")
    .map(|r| r.affinity).unwrap_or(0.0);
let delta = compute_contagion(&sender_state.mood, &sender_params, &receiver_params, affinity);

// Apply contagion to receiver
for &e in bhava::mood::Emotion::ALL {
    receiver_state.stimulate(e, delta.get(e));
}
```

### Persistence

```rust
use bhava::storage::SqliteStore;
use bhava::store::BhavaStore;

let store = SqliteStore::open("agent_data.db")?;
store.save_profile("friday", &profile)?;
store.save_emotional_state("friday", &state)?;

// Later...
let restored = store.load_profile("friday");
```

### OCEAN Interop

```rust
use bhava::traits::{OceanScores, profile_from_ocean};

// From Big Five scores
let ocean = OceanScores {
    openness: 0.7,
    conscientiousness: 0.8,
    extraversion: 0.3,
    agreeableness: 0.6,
    neuroticism: -0.4,
};
let profile = profile_from_ocean("Agent", &ocean);

// To Big Five scores
let scores = profile.to_ocean();
```

### Energy / Circadian / Flow Loop

```rust
use bhava::energy::{EnergyState, exertion_from_mood};
use bhava::circadian::{CircadianRhythm, Chronotype};
use bhava::flow::FlowState;

let mut energy = EnergyState::new();
let circadian = CircadianRhythm::with_chronotype(Chronotype::NightOwl);
let mut flow = FlowState::new();

// Game loop tick
let now = chrono::Utc::now();
let alertness = circadian.alertness(now);
let exertion = exertion_from_mood(&state.mood) * flow.energy_drain_modifier();
energy.tick(exertion);
energy.apply_recovery_modifier(circadian.energy_recovery_modifier(now));
flow.tick(&state.mood, energy.energy, alertness);

// Use flow bonus for performance
let perf = energy.performance() * flow.performance_bonus();
```

### Emotional Intelligence

```rust
use bhava::eq::{eq_from_personality, compose_eq_prompt};

let eq = eq_from_personality(&profile);
let prompt_fragment = compose_eq_prompt(&eq);

// Use EQ bonuses in other systems
let regulation_eff = stress.regulation_effectiveness()
    * energy.regulation_effectiveness()
    * eq.management_bonus();
```

### Cultural Display Rules

```rust
use bhava::display_rules::{apply_display_rules, professional_context};
use bhava::regulation::RegulatedMood;

let mut regulated = RegulatedMood::from_state(&state);
apply_display_rules(&mut regulated, &professional_context());
// regulated.expressed is now culturally filtered
// regulated.felt is unchanged
```

### Preference Learning

```rust
use bhava::preference::{PreferenceStore, bias_from_personality};

let bias = bias_from_personality(&profile);
let mut prefs = PreferenceStore::with_bias(100, bias);

// Record interaction outcomes
prefs.record_outcome("agent_alice", 0.8, chrono::Utc::now());
prefs.record_outcome("agent_bob", -0.3, chrono::Utc::now());

// Query preferences
let top = prefs.top_preferences(5);
let avoid = prefs.bottom_preferences(3);
```
