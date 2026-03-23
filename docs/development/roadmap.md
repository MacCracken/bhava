# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon), game logic (joshua), desktop integration (aethersafha).

## Completed (0.22.3)

- 15-trait personality system with 4 groups (Social, Cognitive, Behavioral, Professional)
- Trait compatibility (cosine similarity), blending, mutation, markdown serialization
- PAD-extended mood vectors with decay, triggers, history, named states, baseline derivation
- 7 compound trait effects, mood tone guides, mood-aware prompt composition
- "In Our Image" archetype hierarchy with validation, templates, crew composition
- Sentiment analysis with negation, intensity modifiers, configurable lexicons, sentence-level
- Spirit module (passions, inspirations, pains)
- Relationship graph (affinity, trust, interaction tracking, decay)
- AI integration (system prompt composition, sentiment feedback, agent metadata)
- 5 personality presets matching SecureYeoman configurations
- `#[must_use]` on 37 pure functions, `# Errors` doc sections
- 386 tests, 63 benchmarks across 12 groups

## Next

- Ecosystem integration: wire bhava into agnosai crew runners and hoosh inference pipelines
- Persistence adapters: SQLite/Postgres storage for mood history, relationships, spirit
- Mood-driven model routing: map emotional state to temperature/model tier selection
- Relationship-aware crew composition: affinity/trust influence task assignment
- Live sentiment monitoring: continuous feedback during streaming responses
