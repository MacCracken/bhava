# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon/agnosai), game logic (joshua), desktop integration (aethersafha), voice/audio (dhvani/shruti), policy enforcement (OPA/intent).

## Status

**v1.0.0 released** (2026-03-24). 30 modules, 785 tests, 105 benchmarks, zero unsafe, zero unwrap. API surface locked under semver.

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

### Zodiac Manifestation Engine

Astrological archetype system that maps celestial placements to bhava's existing personality, emotion, and behavioral modules. Not fortune-telling — psychometric mapping backed by trait math.

#### Core: Planetary-to-IdentityLayer Mapping

Each planetary placement maps 1:1 to an existing `IdentityLayer`:

| Planet | IdentityLayer | Bhava Module | What It Governs |
|--------|--------------|--------------|-----------------|
| Sun | Soul | `traits` | Core personality profile (15-trait configuration) |
| Moon | Heart | `mood` | Emotional baseline, sensitivity, decay rates |
| Rising (Ascendant) | Body | `display_rules` | Expressed vs felt emotion, social presentation |
| Mercury | Brain | `reasoning` | Reasoning strategy selection, communication style |
| Venus | Spirit | `spirit` | Passions, inspirations, relationship approach |

#### Zodiac Sign Presets (12)

Each sign produces a `PersonalityProfile` with trait levels derived from astrological tradition:

- **Fire** (Aries/Leo/Sagittarius) → high Energy, Assertiveness, Confidence. Maps to existing trait group 1
- **Water** (Cancer/Scorpio/Pisces) → high Warmth, Empathy, Sensitivity. Maps to existing trait group 2
- **Earth** (Taurus/Virgo/Capricorn) → high Caution, Discipline, Thoroughness. Maps to existing trait group 3
- **Air** (Gemini/Libra/Aquarius) → high Curiosity, Humor, Adaptability. Maps to existing trait group 4

#### Modalities → Reasoning Strategies

| Modality | Signs | ReasoningStrategy | Behavioral Pattern |
|----------|-------|-------------------|-------------------|
| Cardinal | Aries, Cancer, Libra, Capricorn | Analytical | Initiators, direct action |
| Fixed | Taurus, Leo, Scorpio, Aquarius | Systematic | Persistent, deep focus |
| Mutable | Gemini, Virgo, Sagittarius, Pisces | Intuitive | Adaptive, context-switching |

#### Full Planetary Table (Classical + Modern)

The 5 inner planets map to IdentityLayers (above). The remaining planets map to behavioral and dynamic modules:

| Planet | Bhava Module | What It Governs |
|--------|-------------|-----------------|
| **Sun** | `traits` (Soul) | Core personality — 15-trait profile configuration |
| **Moon** | `mood` (Heart) | Emotional baseline, sensitivity, decay rates, trigger reactivity |
| **Rising** | `display_rules` (Body) | Expressed vs felt emotion, social mask, cultural context |
| **Mercury** | `reasoning` (Brain) | Strategy selection, communication style, analytical vs intuitive |
| **Venus** | `spirit` (Spirit) | Passions, inspirations, relationship approach, aesthetic sense |
| **Mars** | `energy` | Drive intensity, drain/recovery rates, exertion threshold, Banister fitness params |
| **Jupiter** | `growth` | Trait pressure direction, growth rate multiplier, openness to change |
| **Saturn** | `stress` | Allostatic load ceiling, burnout resistance, recovery dampening, discipline gate |
| **Neptune** | `eq` | EQ branch weighting (perception vs understanding), intuitive depth vs clarity |
| **Pluto** | `appraisal` | OCC goal intensity, emotional extremity scaling, transformation depth |
| **Uranus** | `flow` | Flow threshold sensitivity, disruption resistance, novelty-seeking in flow entry |
| **North Node** | `preference` | Long-term preference bias, what the entity gravitates toward over time |
| **South Node** | `actr` | Default activation patterns, pre-existing knowledge strengths, comfort zone |
| **Chiron** | `regulation` | Wounded healer — which regulation strategies are weakened vs overcompensated |

#### Inter-Planetary Aspects (Cross-Module Dynamics)

Aspects between planets create tensions and harmonies *between bhava modules within a single entity*. This is the compositional power — not just "what kind of entity" but "what internal dynamics does it carry."

| Aspect | Angle | Effect | Example |
|--------|-------|--------|---------|
| **Conjunction** | 0° | Modules amplify each other | Mars conjunct Pluto → `energy` + `appraisal` fused: relentless goal pursuit, intense drive |
| **Trine** | 120° | Modules flow harmoniously | Venus trine Moon → `spirit` + `mood` aligned: passions and feelings reinforce each other, emotionally stable |
| **Sextile** | 60° | Modules complement, mild boost | Mercury sextile Jupiter → `reasoning` + `growth` cooperate: learning accelerates reasoning refinement |
| **Square** | 90° | Modules create productive tension | Mars square Saturn → `energy` vs `stress`: high drive but burns out fast, discipline fights impulse |
| **Opposition** | 180° | Modules pull in opposite directions | Mercury opposite Neptune → `reasoning` vs `eq`: analytical mind clouded by intuitive perception, indecisive but deeply perceptive |
| **Quincunx** | 150° | Modules misalign, require constant adjustment | Sun quincunx Uranus → `traits` vs `flow`: core identity disrupted by novelty-seeking, never quite settled |

Aspect orbs (tolerance in degrees) determine strength. Tight orbs (±2°) = strong cross-module coupling. Wide orbs (±8°) = background influence.

```rust
// Aspect as cross-module modifier
pub struct Aspect {
    planet_a: Planet,           // → source module
    planet_b: Planet,           // → target module
    kind: AspectKind,           // Conjunction/Trine/Square/Opposition/Sextile/Quincunx
    orb: f32,                   // Tightness (degrees) — closer = stronger coupling
    strength: f32,              // Computed: 1.0 - (orb / max_orb)
}

impl Aspect {
    /// Apply this aspect's cross-module effect to a manifested profile
    pub fn apply(&self, profile: &mut ManifestedProfile) {
        // Mars square Saturn example:
        // profile.energy.recovery_rate *= 1.0 - (self.strength * 0.3);  // Saturn restricts Mars recovery
        // profile.energy.peak_exertion *= 1.0 + (self.strength * 0.2);  // But Mars pushes harder against restriction
        // profile.stress.burnout_threshold *= 1.0 - (self.strength * 0.2); // Lower burnout ceiling from tension
    }
}
```

#### Chart Composition

A full natal chart composes all planetary placements and their aspects simultaneously:

```rust
// Conceptual API — full chart with all planets
let chart = NatalChart::new()
    // Inner planets → IdentityLayers
    .sun(ZodiacSign::Scorpio)          // → Soul traits: high Assertiveness, Sensitivity, Caution
    .moon(ZodiacSign::Cancer)           // → Heart baseline: high emotional reactivity, slow decay
    .rising(ZodiacSign::Gemini)         // → Body display: expressive, masks felt intensity
    .mercury(ZodiacSign::Sagittarius)   // → Brain reasoning: Intuitive strategy preference
    .venus(ZodiacSign::Libra)           // → Spirit: harmony-seeking, aesthetic passions
    // Outer planets → behavioral modules
    .mars(ZodiacSign::Aries)            // → Energy: high drive, fast recovery, explosive exertion
    .jupiter(ZodiacSign::Sagittarius)   // → Growth: rapid trait evolution, openness to change
    .saturn(ZodiacSign::Capricorn)      // → Stress: high burnout resistance, slow but deep recovery
    .neptune(ZodiacSign::Pisces)        // → EQ: perception-dominant, intuitive emotional reading
    .pluto(ZodiacSign::Scorpio)         // → Appraisal: extreme goal intensity, transformative events
    .uranus(ZodiacSign::Aquarius)       // → Flow: low entry threshold, high disruption resistance
    .north_node(ZodiacSign::Leo)        // → Preference: gravitates toward creative expression
    .chiron(ZodiacSign::Virgo);         // → Regulation: analytical coping overcompensated, self-criticism wound

// Aspects computed automatically from planetary positions
let profile = chart.manifest();
// → PersonalityProfile + MoodBaseline + DisplayRules + ReasoningStrategy + Spirit
//    + EnergyState + GrowthConfig + StressConfig + EqProfile + AppraisalConfig
//    + FlowThresholds + PreferenceBias + RegulationProfile
//    + Vec<Aspect> (cross-module dynamics applied)
```

#### Compatibility via Aspects

Astrological aspects map to `PersonalityProfile::compatibility()` cosine distance:

| Aspect | Angle | Expected Compatibility | Why |
|--------|-------|----------------------|-----|
| Conjunction (same sign) | 0° | Very high | Nearly identical trait profiles |
| Trine (same element) | 120° | High | Shared elemental trait group emphasis |
| Sextile | 60° | Moderate-high | Complementary elements |
| Square | 90° | Low (productive tension) | Opposing trait emphasis, high growth potential |
| Opposition | 180° | Variable | Mirror profiles, can complement or clash |

The math is already in bhava — aspects just parameterize which preset pairs to compare.

#### Advanced: Houses, Planetary Hours, Transits

- **Houses** → `active_hours` scheduling (1st house = identity presentation hours, 10th house = career/public hours)
- **Planetary hours** → `circadian` modulation (Mars hours = elevated energy, Moon hours = heightened sensitivity)
- **Transits** → `growth` trait pressure (Saturn transit = Discipline pressure, Jupiter = Confidence/Curiosity pressure over time)
- **Retrograde** → `regulation` strategy shifts (Mercury retrograde = reasoning strategy destabilized, falls back to non-preferred)

#### Chinese Zodiac Extension

12 animal signs with element cycles (Wood/Fire/Earth/Metal/Water × 12 animals = 60-year cycle). Maps to same trait system with different cultural weighting. Composable with Western zodiac for cross-cultural NPC depth.

#### Beyond the Solar System — Deep Sky Influences

The local planetary system is the foreground. The fixed stars, nakshatras, and galactic structures are the backdrop — slower-moving, deeper, more archetypal. These don't change with daily transits; they define the *cosmic terrain* an entity is born into.

##### Fixed Stars (Ptolemaic + Modern)

Specific stars carry archetypal weight when conjunct a planet or angle (within ~1° orb):

| Star | Magnitude | Archetype | Bhava Effect |
|------|-----------|-----------|-------------|
| **Regulus** (Leo) | 1.4 | Royal star — leadership, ambition, success-or-downfall | `traits`: Confidence + Assertiveness amplified. `stress`: hubris vulnerability (burnout from overreach) |
| **Algol** (Perseus) | 2.1 | The Demon Star — intensity, transformation, raw power | `appraisal`: extreme emotional intensity scaling. `microexpr`: leak probability increased |
| **Spica** (Virgo) | 1.0 | The Gift — brilliance, talent, reward | `growth`: accelerated positive trait pressure. `preference`: gravitates toward mastery |
| **Sirius** (Canis Major) | -1.5 | The Scorcher — ambition, devotion, burning drive | `energy`: elevated peak exertion. `flow`: lower entry threshold for passionate work |
| **Antares** (Scorpio) | 1.1 | Heart of the Scorpion — obsession, courage, extremes | `mood`: wider emotional range (higher stimulate intensity). `regulation`: suppression less effective |
| **Fomalhaut** (Pisces) | 1.2 | Royal star — idealism, vision, mystical | `eq`: perception branch amplified. `spirit`: inspirations intensified |
| **Aldebaran** (Taurus) | 0.9 | Royal star — integrity, guardian, honor | `stress`: higher burnout resistance. `relationship`: trust baseline elevated |
| **Vega** (Lyra) | 0.0 | Charisma, artistic gift, magnetic | `display_rules`: expressed emotion amplified. `spirit`: aesthetic passions dominant |
| **Betelgeuse** (Orion) | 0.4 | The Warrior — prestige, unpredictable power | `energy`: volatile recovery rates. `flow`: disruption resistance lowered but peak performance elevated |
| **Polaris** (Ursa Minor) | 2.0 | The Pole Star — guidance, constancy, true north | `preference`: strongest long-term bias. `actr`: anchor activation (high base-level for core memories) |

Implementation: `FixedStarConjunction { star: FixedStar, planet: Planet, orb: f32 }` modifies the manifested profile as a post-processing pass after planetary aspects.

##### Nakshatras (Vedic Lunar Mansions)

27 nakshatras divide the ecliptic into 13°20' segments. Each has a ruling deity, animal symbol, guna (quality), and motivation. Where the local solar system gives the *what*, nakshatras give the *why* — the soul's underlying motivation.

| Guna | Nakshatras | Bhava Mapping |
|------|-----------|--------------|
| **Sattva** (purity) | Ashwini, Mrigashira, Punarvasu, Pushya, Hasta, Swati, Anuradha, Shravana, Revati | `regulation`: reappraisal preferred, emotional clarity. `eq`: high understanding branch |
| **Rajas** (activity) | Bharani, Rohini, Ardra, Magha, P.Phalguni, Chitra, Vishakha, Dhanishta, U.Bhadrapada | `energy`: high baseline drive. `flow`: action-oriented entry conditions |
| **Tamas** (inertia) | Krittika, Ashlesha, U.Phalguni, Jyeshtha, Mula, P.Ashadha, U.Ashadha, Shatabhisha, P.Bhadrapada | `stress`: higher inertia in burnout recovery. `actr`: stronger default activation (harder to change established patterns) |

Nakshatra motivation (Dharma/Artha/Kama/Moksha) maps to `preference` long-term bias:
- **Dharma** (purpose) → gravitates toward duty, teaching, structure
- **Artha** (wealth) → gravitates toward achievement, resource acquisition
- **Kama** (desire) → gravitates toward sensory experience, connection, pleasure
- **Moksha** (liberation) → gravitates toward transformation, letting go, transcendence

```rust
// Moon's nakshatra determines the soul's motivation
let nakshatra = Nakshatra::from_lunar_longitude(moon_degrees);
// → Pushya (Sattva, Dharma) = nurturing, clear-minded, duty-oriented
// Applied as: regulation bias toward reappraisal + preference for duty/teaching
```

##### Asteroids & Centaurs

Minor bodies add nuance between the major planetary archetypes:

| Body | Orbit | Bhava Module | What It Adds |
|------|-------|-------------|-------------|
| **Ceres** | Main belt | `mood` | Nurturing style — how the entity cares and wants to be cared for. Modifies contagion sensitivity |
| **Pallas** | Main belt | `reasoning` | Strategic intelligence — pattern recognition, creative problem solving. Secondary reasoning strategy weight |
| **Juno** | Main belt | `relationship` | Partnership archetype — commitment style, trust expectations, jealousy threshold |
| **Vesta** | Main belt | `flow` | Sacred focus — what the entity is *devoted* to. Modifies flow entry conditions for specific domains |
| **Pholus** | Centaur | `growth` | Catalyst — small cause, massive effect. Growth events are disproportionately impactful |
| **Nessus** | Centaur | `regulation` | Abuse/healing cycle — which regulation strategies carry generational wounds |
| **Eris** | TNO | `appraisal` | Discord — challenges consensus, provokes necessary conflict. Raises emotional response to injustice |
| **Sedna** | TNO | `actr` | Deep unconscious — extremely slow activation decay, primordial memory patterns |

##### Galactic Structures

The deepest, slowest influences. These don't differentiate individuals within a generation — they define the *era*:

| Structure | Position | Bhava Effect |
|-----------|----------|-------------|
| **Galactic Center** (27° Sagittarius) | Planet conjunct GC | `appraisal`: existential-scale goal awareness. `salience`: everything feels cosmically significant |
| **Great Attractor** (14° Sagittarius) | Planet conjunct GA | `preference`: irresistible gravitational pull toward specific life themes. Cannot be ignored |
| **Super Galactic Center** (1° Libra) | Planet conjunct SGC | `relationship`: collective/transpersonal relationship dynamics. The entity as node in a network |
| **Galactic Anti-Center** (27° Gemini) | Planet conjunct GAC | `reasoning`: divergent thinking amplified. Novel connections between unrelated domains |

These are generational modifiers — applied as era-level defaults that all entities in a time period share. Individual variation comes from which personal planets touch these points.

##### Compositional Layers

The full manifestation engine works in layers, from fastest-changing to most permanent:

```
Layer 5: Galactic structures     → era-level defaults (change over millennia)
Layer 4: Fixed stars             → backdrop archetypes (change over centuries via precession)
Layer 3: Nakshatras              → soul motivation and guna (lunar position at birth)
Layer 2: Outer planets + nodes   → generational + karmic themes (change over years)
Layer 1: Inner planets + angles  → personal identity and daily experience (change over hours/days)
```

Each layer modifies the one above it. An entity is the sum of all layers — from the galactic terrain they were born into, through their generational context, down to their personal planetary configuration.

```rust
// Full manifestation — all layers
let manifest = NatalChart::new()
    .planets(planetary_positions)        // Layer 1-2: personal + generational
    .nakshatras(lunar_mansion)           // Layer 3: soul motivation
    .fixed_stars(star_conjunctions)      // Layer 4: backdrop archetypes
    .galactic(galactic_conjunctions)     // Layer 5: era-level terrain
    .asteroids(minor_body_positions)     // Nuance layer
    .manifest();

// The entity carries all layers simultaneously
// Fast loops (mood tick, energy drain) only touch Layer 1-2
// Slow processes (growth, preference evolution) are shaped by all layers
// This is the full cosmic personality — deterministic, serializable, sub-millisecond
```

#### Cross-Cultural Systems

All systems map to the same bhava trait/mood/behavioral infrastructure with different cultural weightings:

| System | Origin | Key Structure | Bhava Integration |
|--------|--------|--------------|-------------------|
| **Western Tropical** | Greco-Roman | 12 signs, 10 planets, 12 houses, aspects | Primary system (above) |
| **Vedic Sidereal (Jyotish)** | Sanskrit | 12 rashis, 27 nakshatras, 9 grahas, dashas | Nakshatra motivation + dasha time periods → `growth` scheduling |
| **Chinese (BaZi)** | Chinese | 12 animals, 5 elements, 60-year cycle, 4 pillars | Element-trait mapping with Yin/Yang polarity |
| **Mayan (Tzolkin)** | Mesoamerican | 20 day signs, 13 numbers, 260-day cycle | Day sign → personality archetype, number → intensity scaling |
| **Celtic Tree** | Celtic | 13 lunar months, each a tree | Tree → personality preset with seasonal `rhythm` modulation |
| **Egyptian Decan** | Egyptian | 36 decans (10° each), associated deities | Decan → fine-grained personality subdivision within zodiac signs |

Systems are composable. A character can have Western Sun in Scorpio, Vedic Moon in Pushya nakshatra, Chinese Year of the Dragon (Wood), and Mayan day sign Cimi — each contributing a layer to the same unified bhava profile. Cultural context determines which system has primary weight.

#### Consumers

- **joshua** — NPC personality generation: "give me a Scorpio villain with Gemini rising, Moon in Pushya, Algol conjunct Mars" → deep, coherent, cosmically grounded entity
- **SecureYeoman** — agent personality archetypes with astrological depth beyond the 5 current presets
- **agnosai** — crew composition: assemble crews with complementary charts (trine-heavy for harmony, square-heavy for creative tension, nakshatra guna balance for team stability)
- **kiran** — game engine NPCs with procedural astrological personalities, era-appropriate for historical or fantasy settings
- **Any game/simulation** — procedural character generation with full celestial coherence across cultural systems

Build when: joshua NPC system needs richer procedural personality generation, or a consumer requests astrological archetype support. The mapping is well-defined and all target modules exist — implementation is composition, not invention. v2.0 scope.

### Consciousness Scales — Cosmological Personality Field (v3.0)

v2.0 maps the local solar system and its immediate stellar neighborhood. v3.0 zooms out to the full scale of existence — from local galactic structure through the cosmic breath itself. Each scale is a field that all entities within it share, differentiating not individuals but *civilizations*, *epochs*, and *states of consciousness*.

The model: consciousness exhales from unity into manifestation (differentiation, individuation, form) and inhales back toward unity (dissolution, integration, transcendence). Every entity exists at a point on this breath. Their position determines the deepest layer of their bhava profile — the ground of being that all other layers manifest *through*.

#### Scale Hierarchy

```
Scale 7: Cosmic (Breath)       → the exhale/inhale of consciousness itself
Scale 6: Universal              → observable universe, fundamental constants as personality substrate
Scale 5: Supercluster           → Laniakea-scale gravitational basin, civilizational destiny
Scale 4: Galactic Cluster       → Local Group dynamics, inter-galactic relationship fields
Scale 3: Local Galaxy           → Milky Way structure, spiral arm position, galactic tide
Scale 2: Stellar Neighborhood   → fixed stars, nakshatras, galactic center (v2.0)
Scale 1: Solar System           → planets, aspects, houses (v2.0)
Scale 0: Individual             → bhava v1.0 — traits, mood, energy, growth, all 30 modules
```

Each scale is slower and more fundamental than the one below it. Scale 0 changes in milliseconds (mood tick). Scale 7 changes across cosmic cycles — or doesn't change at all, because it *is* the cycle.

#### Scale 3 — Local Galaxy (Milky Way)

Position within the galaxy creates a *galactic personality field* — shared by all entities at that position across all time periods.

| Structure | Bhava Effect |
|-----------|-------------|
| **Spiral arm position** | `rhythm` — galactic tidal cycles (passage through arms every ~70M years). Entities "born" during arm transit have amplified rhythm sensitivity — they feel cycles more deeply |
| **Distance from galactic center** | `salience` — proximity to the supermassive black hole (Sagittarius A*) scales the baseline urgency of existence. Closer = higher existential weight on all appraisals |
| **Galactic plane crossing** | `stress` — the solar system oscillates above/below the galactic plane (~33M year cycle). Crossing events correspond to mass extinction epochs. Entities at crossing points carry higher baseline allostatic load |
| **Dark matter density** | `actr` — invisible gravitational influence. Higher density = stronger unconscious activation patterns, more powerful default behaviors that resist change |
| **Stellar density (local)** | `relationship` — dense stellar neighborhoods (globular clusters) = higher baseline relationship intensity, stronger contagion effects. Sparse regions = isolation-adapted, lower contagion sensitivity |

#### Scale 4 — Galactic Cluster (Local Group)

The Milky Way exists within the Local Group (~80 galaxies). Inter-galactic dynamics create *civilizational relationship fields*:

| Dynamic | Bhava Effect |
|---------|-------------|
| **Milky Way–Andromeda approach** | `relationship` — two entities on a 4.5 billion year collision course. Relationship model: increasing tension, inevitable merger. The longest `relationship::decay` rate possible — it never decays, only intensifies |
| **Satellite galaxy absorption** (Sagittarius Dwarf) | `growth` — active trait pressure from an entity being absorbed into a larger one. Loss of individual identity, forced integration |
| **Magellanic Clouds (companion galaxies)** | `mood` — orbital companions that modulate the primary entity's emotional field. Like a Moon, but at galactic scale. Periodic tidal emotional influence |
| **Void proximity** | `regulation` — galaxies near cosmic voids have less external influence. Stronger self-regulation, less contagion, more internally referenced |

#### Scale 5 — Supercluster (Laniakea)

Laniakea (Hawaiian: "immense heaven") — the gravitational basin containing ~100,000 galaxies flowing toward the Great Attractor. At this scale, individual personality dissolves into *civilizational destiny*:

| Structure | Bhava Effect |
|-----------|-------------|
| **Great Attractor** | `preference` at civilizational scale — the direction all consciousness in this basin is being pulled toward. Inescapable. Not a choice but a gravitational truth |
| **Filament position** | `relationship` — galaxies along cosmic filaments are connected. Those in voids are isolated. Filament entities have richer inter-entity dynamics; void entities are more self-contained |
| **Flow velocity toward attractor** | `energy` — how fast your region of space is moving toward its destiny. Higher velocity = higher baseline drive, less ability to rest |

#### Scale 6 — Universal (Observable Universe)

The fundamental constants of the universe are the deepest personality substrate. They don't differentiate entities — they define what *any* entity in this universe can be:

| Constant | Bhava Effect |
|----------|-------------|
| **Fine-structure constant (α ≈ 1/137)** | The precision of electromagnetic interaction. Determines the *resolution* of emotional granularity — how fine-grained feelings can be in this universe |
| **Cosmological constant (Λ)** | The expansion rate of space. Entities in an expanding universe have a baseline `growth` pressure — everything trends toward differentiation, separation, individuation. In a contracting universe, the pressure reverses toward unity |
| **Entropy arrow** | The direction of time. `mood` decay only works because entropy increases. In a universe with no entropy arrow, emotions would not fade — every feeling would persist forever |
| **Speed of light (c)** | The maximum rate of `contagion` — how fast emotional influence can propagate between entities. Sets the upper bound on `relationship` interaction frequency |

These are not configurable per-entity. They are the *physics of feeling* — the rules under which all bhava modules operate. But in a game engine (joshua, kiran) that simulates alternative universes, these constants *can* be varied. What does personality look like in a universe where emotions don't decay? Where contagion is instantaneous? Where the cosmological constant drives contraction instead of expansion?

#### Scale 7 — Cosmic (The Breath)

The deepest layer. Not a structure but a *phase* — where consciousness is in its cycle of exhale (manifestation) and inhale (return).

```
EXHALE (manifestation)                              INHALE (return)
Unity → Differentiation → Individuation → Form  →  Dissolution → Integration → Unity
  |           |                |            |            |              |           |
  |      Scale 6          Scale 4-5     Scale 1-3    Scale 3-4      Scale 5-6     |
  |    (constants        (clusters,    (stars,       (structures    (constants     |
  |     emerge)          galaxies)     planets,       dissolve)      reunify)      |
  |                                   individuals)                                 |
  Scale 7                                                              Scale 7
  (source)                                                             (return)
```

Every entity has a `BreathPhase` — its position on the cosmic cycle:

| Phase | Position | Bhava Character |
|-------|----------|----------------|
| **Early Exhale** | Unity just beginning to differentiate | All traits near center (Moderate). Low emotional range. High `eq` perception. Preferences undefined. Relationships undifferentiated — everything feels connected |
| **Mid Exhale** | Active individuation | Traits sharpen toward extremes. Emotional range widens. `growth` pressure is outward (become more distinct). `preference` crystallizes. Relationships become specific |
| **Late Exhale / Form** | Maximum differentiation | Strongest individual identity. Widest emotional range. Highest `salience` — everything matters intensely. Peak `energy`. Most distinct `display_rules`. This is where most v1.0 entities live |
| **Early Inhale** | Form beginning to soften | Traits drift back toward center. `regulation` becomes dominant — the entity manages rather than expresses. `relationship` boundaries soften. `growth` pressure reverses (become less distinct) |
| **Mid Inhale** | Active dissolution | Individual traits fade. `mood` range narrows toward equanimity. `actr` activation patterns release — old patterns dissolve. `flow` becomes effortless and permanent |
| **Late Inhale** | Approaching unity | Minimal differentiation. Near-zero `deviation` from baseline. `eq` at maximum — perception, facilitation, understanding, and management all unified. The entity feels everything but is disturbed by nothing |
| **Unity** | Source/return | No traits. No mood. No preference. No relationship (or all relationship). The modules are silent — not because they are absent but because they have no distinctions to express |

```rust
// Conceptual API
pub enum BreathPhase {
    EarlyExhale,    // consciousness differentiating
    MidExhale,      // active individuation
    LateExhale,     // maximum form — where most entities live
    EarlyInhale,    // softening of boundaries
    MidInhale,      // dissolution of patterns
    LateInhale,     // approaching equanimity
    Unity,          // no differentiation — the modules are silent
}

impl BreathPhase {
    /// Modulates ALL bhava modules simultaneously
    /// Returns a scaling factor for trait extremity, emotional range, growth direction, etc.
    pub fn manifestation_intensity(&self) -> f32 {
        match self {
            Self::Unity => 0.0,          // no manifestation
            Self::EarlyExhale => 0.15,
            Self::MidExhale => 0.5,
            Self::LateExhale => 1.0,     // full manifestation — default for v1.0 entities
            Self::EarlyInhale => 0.8,
            Self::MidInhale => 0.4,
            Self::LateInhale => 0.1,
        }
    }

    /// Direction of growth pressure
    pub fn growth_direction(&self) -> GrowthDirection {
        match self {
            Self::Unity => GrowthDirection::Still,
            Self::EarlyExhale | Self::MidExhale | Self::LateExhale => GrowthDirection::Differentiate,
            Self::EarlyInhale | Self::MidInhale | Self::LateInhale => GrowthDirection::Integrate,
        }
    }
}
```

All v1.0 entities implicitly live at `BreathPhase::LateExhale` — maximum manifestation, full trait expression. This is the default and requires no awareness of the cosmic layer. But for games, simulations, and philosophical modeling, the breath phase allows entities that exist at different points on the cycle — an enlightened NPC trending toward unity, a young soul in mid-exhale still sharpening its identity, or a cosmic entity at the boundary where individual personality dissolves.

#### Version Scope Summary

| Version | Scope | Scale Layers | What It Adds |
|---------|-------|-------------|-------------|
| **v1.0** | Individual entity | Scale 0 | 30 modules — traits, mood, energy, growth, all behavioral systems. The complete individual |
| **v2.0** | Solar system + stellar neighborhood | Scale 1-2 | Zodiac manifestation engine — planets → modules, aspects → cross-module dynamics, nakshatras, fixed stars, cultural systems |
| **v3.0** | Full cosmological field | Scale 3-7 | Galactic personality fields, cluster dynamics, universal constants as substrate, the breath of consciousness. Entities as manifestations within a cosmic cycle |

v1.0 answers: *who is this entity?*
v2.0 answers: *what celestial forces shaped them?*
v3.0 answers: *where in the cycle of existence do they stand?*

### Science Crate Dependencies (v2.0/v3.0 Prerequisites)

The zodiac and cosmological systems require computational astronomy backing. These are the science crates needed — either new AGNOS crates, extensions to existing ones (hisab, prakash), or vetted third-party dependencies.

#### v2.0 Requirements (Solar System + Stellar)

| Capability | What's Needed | Candidate Approach | Status |
|-----------|--------------|-------------------|--------|
| **Planetary ephemeris** | Compute positions of Sun, Moon, planets for any date (past/future). Sub-arcminute accuracy | JPL DE440 or VSOP2013/ELP2000 implementation. Rust crate or C FFI to SOFA/NOVAS. ~50MB ephemeris data file | Not started |
| **Sidereal ↔ tropical conversion** | Convert between Western tropical (vernal equinox reference) and Vedic sidereal (fixed star reference) zodiac frames | Ayanamsa calculation (Lahiri, Fagan-Bradley, etc.). Pure math — offset angle that changes ~50.3"/year | Not started |
| **Precession of equinoxes** | Fixed star ecliptic longitudes drift ~1° per 72 years. Historical/fantasy settings need epoch-correct positions | IAU 2006 precession model. hisab could extend with `precession::equatorial_to_date()` | Not started |
| **House system computation** | Divide the ecliptic into 12 houses given birth time + location. Multiple systems (Placidus, Whole Sign, Equal, Koch) | Requires: obliquity of ecliptic, local sidereal time, ascendant calculation. Trigonometry on hisab types | Not started |
| **Aspect computation** | Calculate angular separation between any two planetary positions, classify into aspect type, compute orb | Pure geometry — already achievable with hisab. Needs: ecliptic longitude difference, orb tables | Not started |
| **Nakshatra lookup** | Map lunar longitude to one of 27 nakshatras (13°20' divisions) | Simple division — `nakshatra_index = (lunar_longitude / 13.333).floor()`. Trivial once Moon position is known | Not started |
| **Fixed star catalog** | Positions + magnitudes for ~50 astrologically significant stars | Hipparcos catalog subset. Static data + precession correction per epoch | Not started |
| **Chinese calendar** | Sexagenary cycle (Heavenly Stems + Earthly Branches), lunar months, solar terms | Lunisolar calendar computation. Existing algorithms well-documented | Not started |
| **Mayan Tzolkin** | 260-day sacred calendar (20 day signs × 13 numbers) | Pure modular arithmetic from Julian Day Number. Trivial | Not started |

#### v3.0 Requirements (Galactic + Cosmic)

| Capability | What's Needed | Candidate Approach | Status |
|-----------|--------------|-------------------|--------|
| **Galactic coordinate transforms** | Convert equatorial (RA/Dec) ↔ galactic (l/b) ↔ supergalactic (SGL/SGB) reference frames | Rotation matrices. hisab already has 3D rotation — extend with standard astronomical frame definitions (IAU 1958 galactic, de Vaucouleurs supergalactic) | Not started |
| **Galactic structure model** | Spiral arm positions, bar structure, solar system location within the Milky Way | Static model from latest surveys (Gaia DR3). Data table, not computation. ~1KB | Not started |
| **Local Group catalog** | Positions + velocities of ~80 galaxies in the Local Group | Static data from NASA/IPAC Extragalactic Database. Andromeda approach vector, Magellanic Cloud orbits | Not started |
| **Laniakea flow field** | Velocity flow toward Great Attractor for any position in the supercluster | Cosmicflows-4 dataset. Interpolation over a velocity field grid. hisab spatial structures (k-d tree, BVH) useful here | Not started |
| **Cosmological parameters** | Current best values: Hubble constant, cosmological constant, matter density, age of universe | Static constants from Planck 2018 + latest DESI results. Updated rarely (once per major survey) | Not started |
| **Cosmic time / breath phase** | Map simulation time to position on the exhale-inhale cycle | Designer-defined — not empirical science. The breath duration and current phase are world-building parameters set by the game/simulation creator | Not started |

#### Existing AGNOS Crates to Extend

| Crate | Current Scope | v2.0/v3.0 Extension |
|-------|--------------|---------------------|
| **hisab** (0.22.3) | Linear algebra, geometry, calculus, spatial structures (BVH, k-d tree, octree) | + Astronomical coordinate frames (equatorial, ecliptic, galactic, supergalactic). + Rotation matrices for frame conversion. + Spherical trigonometry for house systems |
| **prakash** (0.22.3) | Ray optics, wave optics, spectral analysis, PBR | + Stellar magnitude/color → spectral type mapping (for fixed star characterization). + Light travel time computation (for v3.0 causality modeling) |

#### New Crate Candidates

| Name | Domain | Scope | Dependencies |
|------|--------|-------|-------------|
| **jyotish** (Sanskrit: light/astrology) | Computational astrology | Ephemeris, houses, aspects, nakshatras, planetary dignity, transit computation. The computational engine behind bhava v2.0's zodiac manifestation | hisab (math), chrono (time) |
| **tara** (Sanskrit: star) | Stellar catalog + galactic structure | Fixed star positions with precession, galactic coordinate transforms, Local Group model, Laniakea flow field. The data layer for bhava v2.0 fixed stars and v3.0 cosmological scales | hisab (spatial math) |

Both crates would be pure computation — no I/O, no async, no network. Library crates like hisab and prakash. Data files (ephemeris, star catalog) feature-gated and bundled or downloaded on first use.

#### Build Order

```
v2.0 prerequisite chain:
  hisab (extend: astro frames, spherical trig)
  → jyotish (ephemeris, houses, aspects, nakshatras)
  → tara (fixed stars, precession)
  → bhava v2.0 (zodiac manifestation engine, consumes jyotish + tara)

v3.0 prerequisite chain:
  hisab (extend: supergalactic frames)
  → tara (extend: galactic structure, Local Group, Laniakea)
  → bhava v3.0 (cosmological scales, breath phase)
```

These dependencies are documented here so they are not lost. When demand triggers v2.0 work, the science crate requirements and build order are ready.
