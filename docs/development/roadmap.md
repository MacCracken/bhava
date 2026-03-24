# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon/agnosai), game logic (joshua), desktop integration (aethersafha), voice/audio (dhvani/shruti), policy enforcement (OPA/intent).

## Engineering Backlog

No open items.

## Future Features (demand-gated)

These abstractions are not needed today but may become worthwhile if the crate grows or consumers request them. Gate on concrete demand — do not build speculatively.

### Normalized Number Types
Wrapper types `Normalized01(f32)` (0.0..=1.0) and `Balanced11(f32)` (-1.0..=1.0) to replace 100+ `.clamp()` calls with compile-time range safety. Build when: a consumer reports an out-of-range bug, or a new module would benefit from enforced ranges at the type level.

### Generic Capacity-Bounded Store
Trait `Evictable` + `CapacityBoundedStore<T>` to unify eviction logic in `ActivationStore`, `PreferenceStore`, `MoodHistory`, `EmotionalMemoryBank`. Build when: a 5th bounded store is needed, or eviction bugs surface from inconsistent implementations.

### Threshold Classifier
Generic `ThresholdClassifier<T>` to replace ad-hoc `level()` methods in `EnergyLevel`, `StressLevel`, `EqLevel`, `SalienceLevel`. Build when: a consumer needs runtime-configurable thresholds, or a new level enum is added.

### Enum Display Macro
Derive macro or `impl_display!` to eliminate 16+ manual `Display` match blocks. Build when: enum count exceeds 20, or a derive macro is already in the build for another reason.

### Decay/Recovery Curve Abstraction
`CurveModel` trait with `ExponentialDecay`, `LogisticCurve` implementations to unify decay math across `EmotionalState`, `EnergyState`, `actr`, `circadian`. Build when: a consumer needs pluggable decay curves (e.g., Gompertz for aging NPCs).

### MoodVector Iterator
`MoodVector::iter() -> impl Iterator<Item = (Emotion, f32)>` plus `magnitude()` and `fold` helpers. Build when: a new module needs mood aggregation and the `for e in Emotion::ALL` pattern appears a 7th+ time.

### State Machine Base Trait
Generic `StateMachine { type State; type Input; fn tick(); fn state(); }` for `FlowState`, `CircadianRhythm`, and future phase-based systems. Build when: a 3rd state machine module is added.
