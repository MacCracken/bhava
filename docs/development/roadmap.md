# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon/agnosai), game logic (joshua), desktop integration (aethersafha), voice/audio (dhvani/shruti), policy enforcement (OPA/intent).

## Status

**v2.0.0 in development**. 44 modules, 1331 tests, 168 benchmarks, zero unsafe, zero unwrap. Completed versions in [CHANGELOG.md](../../CHANGELOG.md).

## Research

### Unified Consciousness Model Paper
- **Outline**: `agnosticos/docs/development/paper-unified-consciousness-model.md`
- **Thesis**: bhava demonstrates that "as above, so below; as within, so without" is a provable mathematical property of multi-scale modular systems — the fixed point at zero (Unity) emerges from the arithmetic, not as an axiom
- **Dependency**: requires v3.0 (cosmic scales) for complete mathematical specification
- **Target**: Nature Computational Science / PNAS after formal verification

## v2.0.0 Scope

### Infrastructure
- ~~**Release benchmark artifacts**~~ ✓ — release workflow runs `bench-history.sh`, attaches CSV + MD as release assets

### Type Safety & Abstractions
- ~~**Normalized number types**~~ ✓ — `Normalized01(f32)` and `Balanced11(f32)` in `types` module. Adopted in EnergyState, StressState, SalienceScore, EqProfile, PreferenceEntry. MoodVector fields deferred (360 touch points, already guarded by set/get API).
- ~~**Generic capacity-bounded store**~~ ✓ — `evict_min()` helper function (not full trait — only 2 stores, insufficient justification for trait). Adopted in ActivationStore and PreferenceStore.
- ~~**Threshold classifier**~~ ✓ — `ThresholdClassifier<L>` in `types` module. Adopted in EnergyLevel, SalienceLevel, EqLevel. StressLevel skipped (dynamic thresholds).
- ~~**Enum display macro**~~ ✓ — `impl_display!` macro. 22 enum Display impls migrated.
- ~~**Decay/recovery curve abstraction**~~ ✓ — `DecayCurve` trait + `ExponentialDecay` + `LogisticCurve` in `curves` module. Available for new modules; existing one-liner decay formulas left as-is.
- ~~**MoodVector iterator**~~ ✓ — `iter()`, `dot()`, `magnitude()` added. `dominant_emotion()` refactored.
- **State machine base trait** — deferred (only 1 implementor; CLAUDE.md rule of three)

### Zodiac Manifestation Engine
~~**Core engine**~~ ✓ — 12 signs, 4 elements, 3 modalities, 14 planets, 6 aspect types. `NatalChart` builder with `manifest()` producing full `ManifestedProfile`. 12 planetary modifier functions, aspect detection + cross-module effects. Sub-microsecond full chart manifestation (~923 ns). Pluto (appraisal) and Chiron (regulation) deferred — target modules lack config structs.

See detailed design below for future extensions (houses, transits, Chinese zodiac, deep sky).

## Future Features (post-v2.0)

### Neuroscience Bridge Expansion (Biochemistry Layer)

The v1.8 neuroscience bridge consumes mastishk. A future expansion could add rasayan (biochemistry) for the biochemical pathways that produce/regulate neurotransmitters — deeper fidelity on the same bridge pattern.

#### Bridge Functions (`neuroscience` feature → mastishk + rasayan)

| Neurochemical | Bhava Module | Effect |
|--------------|-------------|--------|
| **Serotonin level** | `mood` | Baseline mood floor — low serotonin → reduced joy, elevated irritability |
| **Dopamine level** | `preference`, `reward` | Reward sensitivity, preference reinforcement strength, motivation drive |
| **Cortisol level** | `stress` | Allostatic load amplifier — high cortisol → accelerated stress accumulation |
| **Norepinephrine** | `mood` (arousal), `salience` | Arousal modulation, salience sensitivity — fight-or-flight readiness |
| **GABA/Glutamate ratio** | `mood` (anxiety), `regulation` | Anxiolytic balance — low GABA → elevated anxiety, impaired regulation |
| **Oxytocin** | `relationship`, `trust` | Social bonding strength, trust baseline, contagion sensitivity |
| **Endorphins** | `energy`, `stress` | Pain dampening, stress recovery boost, energy recovery modifier |
| **Melatonin** | `circadian` | Sleep pressure, circadian phase marker — feeds existing Borbély two-process model |
| **Acetylcholine** | `actr`, `flow` | Memory consolidation (activation strength), attention/flow entry threshold |
| **BDNF** | `growth` | Neuroplasticity → trait pressure rate multiplier — high BDNF = faster personality adaptation |

#### Sleep Neuroscience → Circadian

- Sleep stage cycling (NREM1-3, REM) → energy recovery rate modulation
- Sleep debt accumulation → stress baseline elevation, cognitive performance (flow threshold)
- Adenosine buildup → sleep pressure (extends existing circadian Process S)

#### API Surface

```rust
// Same bridge pattern as physiology/microbiology — pure functions, f32 primitives in
pub fn mood_from_serotonin(level: f32) -> MoodVector;
pub fn stress_from_cortisol(level: f32) -> f32;
pub fn reward_sensitivity(dopamine: f32) -> f32;
pub fn arousal_from_norepinephrine(level: f32) -> f32;
pub fn anxiety_from_gaba_ratio(gaba: f32, glutamate: f32) -> f32;
pub fn trust_from_oxytocin(level: f32) -> f32;
pub fn sleep_pressure(adenosine: f32, melatonin: f32) -> f32;
pub fn plasticity_rate(bdnf: f32) -> f32;
```

No new emotional systems. The brain pressing on the modules we already have — same as the body (sharira), the immune system (jivanu), and the physical world (environment).

Build when: mastishk (neuroscience) and rasayan (biochemistry) scaffolded and hardened. Bridge consumes both — mastishk for neural circuits/sleep/neurotransmitter dynamics, rasayan for the biochemical pathways that produce/regulate those neurotransmitters. Bridge is pure mapping functions, not architecture.

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

#### Cross-Cultural Systems (sankhya crate — separate)

Cultural zodiac/calendar systems live in the **sankhya** crate (ancient mathematical systems), not in bhava. Sankhya provides the mathematical computations (Mayan Long Count/Tzolkin, Chinese BaZi, Vedic nakshatras/dashas, Egyptian decans, Celtic tree calendar); bhava consumes the outputs as personality overlay layers. Sankhya needs hardening and depends on the science stack (jyotish, tara, hisab) maturing first.

| System | Origin | Sankhya Module | Bhava Integration |
|--------|--------|---------------|-------------------|
| **Western Tropical** | Greco-Roman | (jyotish — not sankhya) | Primary system (above) |
| **Vedic Sidereal (Jyotish)** | Sanskrit | `vedic` (Katapayadi, Meru Prastara) | Nakshatra motivation + dasha time periods → `growth` scheduling |
| **Chinese (BaZi)** | Chinese | `chinese` (CRT, rod numerals) | Element-trait mapping with Yin/Yang polarity |
| **Mayan (Tzolkin)** | Mesoamerican | `mayan` (Long Count, Tzolkin, Haab) | Day sign → personality archetype, number → intensity scaling |
| **Celtic Tree** | Celtic | (future) | Tree → personality preset with seasonal `rhythm` modulation |
| **Egyptian Decan** | Egyptian | `egyptian` (stellar decans) | Decan → fine-grained personality subdivision within zodiac signs |

Systems are composable. A character can have Western Sun in Scorpio, Vedic Moon in Pushya nakshatra, Chinese Year of the Dragon (Wood), and Mayan day sign Cimi — each contributing a layer to the same unified bhava profile. Cultural context determines which system has primary weight.

Build order: sankhya hardening (blocked on varna) → bhava cultural overlay bridge (post-v2.0).

### Multilingual Emotion & Sentiment — post-v2.0 (varna bridge)

Bhava's sentiment, mood, and archetype modules are currently English-centric. A varna bridge would internalize multilingual emotion vocabulary and cultural expression patterns — feelings that don't translate directly but map to specific MoodVector regions.

**Prerequisite**: varna (multilingual language engine)

#### Capabilities

- **Untranslatable emotion mapping**: culture-specific emotion concepts → MoodVector coordinates
  - Portuguese *saudade* → bittersweet longing (joy↓, trust↑, arousal↓)
  - Japanese *mono no aware* → transient beauty awareness (joy↑, arousal↓, interest↑)
  - German *Schadenfreude* → pleasure at misfortune (joy↑, trust↓, dominance↑)
  - Danish *hygge* → cozy contentment (joy↑, trust↑, arousal↓)
  - Fivarnano *gigil* → overwhelming cuteness urge (arousal↑, joy↑)
- **Multilingual sentiment analysis**: sentiment module operates across languages, not just English keywords
- **Cultural display rules**: varna script/language detection → display_rules cultural context selection
- **Archetype localization**: personality archetypes expressed in culturally appropriate terms

Sits alongside sankhya cultural overlay — both are "culture pressing on personality" at the same layer. varna provides the linguistic dimension, sankhya provides the calendrical/mathematical dimension.

Build when: varna is stable and bhava has at least one non-English consumer.

### Divine Archetype Overlay — post-v2.0 (avatara crate)

Theological and mythological archetype system — mapping divine/celestial beings across traditions to bhava personality configurations. Not religion simulation — psychometric archetype mapping backed by trait math, same as the zodiac engine but from the theological dimension.

**Prerequisite**: avatara crate (Sanskrit: descent of the divine — the manifestation of archetypes into personality)

#### Tradition → Bhava Mappings

| Tradition | Entities | Bhava Integration |
|-----------|----------|-------------------|
| **Kabbalistic Tree of Life** | 10 Sephiroth (Kether→Malkuth) | Each Sephira → distinct PersonalityProfile. Paths between → growth trajectories. Kether = Unity (`BreathPhase::Unity`), Malkuth = full manifestation (`BreathPhase::LateExhale`) |
| **Angelic Orders** | 9 orders (Seraphim→Angels) | Map to manifestation intensity levels. Seraphim = closest to source (early exhale), Angels = most individuated (late exhale) |
| **Archangels** | Michael (Sun/courage), Gabriel (Moon/communication), Raphael (Mercury/healing), Uriel (Earth/wisdom) | Planetary associations → same module mappings as zodiac engine |
| **Hindu Trimurti** | Brahma (creation), Vishnu (preservation), Shiva (transformation) | → `BreathPhase` directly: Brahma = exhale, Vishnu = form, Shiva = inhale |
| **Hindu Devas** | Indra (energy), Saraswati (knowledge), Lakshmi (abundance), Hanuman (devotion) | → trait profiles with specific `spirit` passions |
| **Greek Olympians** | Athena (reasoning), Aphrodite (spirit/relationship), Ares (energy), Hermes (curiosity) | → module-specific amplification, same pattern as planetary aspects |
| **Norse Aesir** | Odin (wisdom/sacrifice), Thor (strength), Loki (chaos/creativity), Freya (love/war) | → trait profiles with `growth` pressure direction |
| **Egyptian Ennead** | Ra (sun/energy), Thoth (knowledge/actr), Ma'at (balance/regulation), Anubis (transition) | → module associations with afterlife/transformation themes |
| **Buddhist** | Avalokiteshvara (compassion/empathy), Manjushri (wisdom/reasoning), Tara (protection) | → `eq` emphasis, `regulation` toward equanimity |

#### Compositional System

Like zodiac signs, archetypes are composable. An entity can carry multiple archetypal influences:

```rust
// Conceptual API
let archetype = DivineBirth::new()
    .sephira(Sephiroth::Tiphareth)          // Beauty/harmony — Sun center
    .archangel(Archangel::Michael)           // Courage, protection
    .hindu_aspect(Trimurti::Vishnu)          // Preservation, stability
    .greek_patron(Olympian::Athena)          // Strategic wisdom
    .manifest();
// → PersonalityProfile + BreathPhase + Spirit + growth direction
```

Cross-cultural composability: a character with Kabbalistic Tiphareth + Hindu Vishnu + Greek Athena gets reinforcing archetypes (all Sun/preservation/wisdom aligned). Conflicting archetypes (Shiva + Vishnu) create productive internal tension — same mechanics as planetary squares.

avatara consumes jyotish (planetary correspondences), sankhya (sacred number systems), and feeds into bhava alongside the zodiac engine and cultural overlay. Same post-v2.0 layer: the divine pressing on personality.

Build when: zodiac engine (v2.0) is stable and a consumer requests theological archetype depth. The mapping infrastructure from v2.0 (aspects, cross-module dynamics) is reusable — avatara adds the archetype data, not new mechanics.

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
| **v1.3** | Math hardening | Scale 0 + validated math | Bodh psychology + Sangha sociology bridges — backing existing systems with peer-reviewed formulas |
| **v1.4** | Body + immune | Scale 0 + body state | Sharira physiology + Jivanu microbiology bridges — the body presses on emotion |
| **v1.6** | Earth-local environment | Scale 0 + physical world | Environmental reactivity — temperature, light, noise, weather, air quality as behavioral modifiers on existing modules. No new emotional systems |
| **v1.7** | Atomic time awareness | Scale 0 + physical time | tanmatra atomic time bridge — simulation time vs wall-clock distinction, time-scale-aware circadian/rhythm, proper temporal grounding |
| **v1.8** | Neuroscience + biochemistry | Scale 0 + brain chemistry | mastishk + rasayan bridge — serotonin→mood, dopamine→preference, cortisol→stress, melatonin→circadian, BDNF→growth plasticity |
| **v2.0** | Solar system + stellar neighborhood | Scale 1-2 | Zodiac manifestation engine — planets → modules, aspects → cross-module dynamics, nakshatras, fixed stars. Cultural systems deferred to sankhya overlay (post-v2.0) |
| **v3.0** | Full cosmological field | Scale 3-7 | Galactic personality fields, cluster dynamics, universal constants as substrate, the breath of consciousness. Entities as manifestations within a cosmic cycle |

v1.0 answers: *who is this entity?*
v1.3 answers: *are the math foundations validated?*
v1.4 answers: *how does the body press on emotion?*
v1.6 answers: *how does the physical world press on them?*
v1.7 answers: *what time is it, physically?*
v1.8 answers: *what is their brain chemistry doing to them?*
v2.0 answers: *what celestial forces shaped them?*
v3.0 answers: *where in the cycle of existence do they stand?*

### Science Crate Dependencies (v1.7/v2.0/v3.0 Prerequisites)

The environmental, zodiac, and cosmological systems build on existing AGNOS science crates. No new physics — bhava consumes simulation output from the ecosystem.

#### v2.0 Requirements (Solar System + Stellar)

| Capability | Crate | Status |
|-----------|-------|--------|
| **Planetary ephemeris** (VSOP87D, <1" accuracy) | jyotish | Done |
| **Lunar position** (Meeus Ch. 47, ~10" accuracy) | jyotish | Done |
| **Sidereal ↔ tropical** (Lahiri ayanamsa) | jyotish | Done |
| **Precession** (IAU 2006) | jyotish | Done |
| **House systems** (Placidus, Equal, Whole Sign, Porphyry) | jyotish | Done |
| **Aspect computation** (conjunction through quincunx, configurable orbs) | jyotish | Done |
| **Nakshatra lookup** (27 lunar mansions) | jyotish | Done |
| **Fixed star catalog** (58 navigational stars, proper motion) | tara | Done |
| **Chinese calendar** (sexagenary cycle, BaZi) | sankhya | Needs hardening |
| **Mayan Tzolkin** (260-day sacred calendar) | sankhya | Needs hardening |

#### v3.0 Requirements (Galactic + Cosmic)

| Capability | Crate | Status |
|-----------|-------|--------|
| **Galactic coordinate transforms** (equatorial ↔ galactic ↔ supergalactic) | tara / hisab | Not started |
| **Galactic structure model** (spiral arms, bar, solar system position) | brahmanda | Cosmic web done, needs galactic geometry |
| **Local Group catalog** (~80 galaxies, Andromeda approach) | brahmanda | Not started |
| **Laniakea flow field** (Cosmicflows-4, Great Attractor) | brahmanda | Not started |
| **Cosmological parameters** (Planck 2018, Hubble, Λ) | hisab-mimamsa | Done (cosmology module) |
| **Scale bridge Scales 4-5** (stellar → personality, galactic → personality) | hisab-mimamsa | Stubs — needs tara + brahmanda wiring |
| **Cosmic time / breath phase** | hisab-mimamsa | Done (Scale 7, fixed_point) |

#### Upstream Crate Status

| Crate | Version | Role | Status |
|-------|---------|------|--------|
| **jyotish** | ~1.0 | Ephemeris, houses, aspects, nakshatras, transits | Built, pre-1.0 repairs tracked |
| **tara** | 1.0.0 | Stellar astrophysics, classification, evolution, spectral | Released |
| **brahmanda** | 0.1.0 | Galactic structure, cosmic web, halos, power spectrum | Built, pre-publish repairs tracked |
| **hisab-mimamsa** | 1.0.0 | GR, QFT, cosmology, unified field, scale bridge | Released on crates.io |
| **tanmatra** | 1.0.0 | Atomic/subatomic physics (v1.5: frequency standards + atomic time) | v1.5 planned |
| **sankhya** | 0.1.0 | Ancient math systems (Mayan, Vedic, Chinese, etc.) | Needs hardening |

#### Build Order

```
v1.6 — done:
  environment module (no new deps, plain f32 values from consumer)

v1.7 prerequisite chain:
  tanmatra v1.5 (frequency standards, atomic time scales)
    → bhava v1.7 (TimeContext for circadian/rhythm/growth)

v2.0 prerequisite chain:
  jyotish v1.0 (ephemeris, houses, aspects, nakshatras)
  + tara v1.0 (fixed stars, precession)
    → bhava v2.0 (zodiac manifestation engine)

v2.0+ cultural + linguistic overlay:
  varna (stable) → sankhya (hardened) → bhava cultural bridge (post-v2.0)
  varna (stable) → bhava multilingual emotion/sentiment (post-v2.0)

v3.0 prerequisite chain:
  brahmanda (galactic structure, Laniakea)
  + hisab-mimamsa (scale bridge Scales 4-5 wired)
  + tara (extend: galactic coords, Local Group)
    → bhava v3.0 (cosmological scales, breath phase)
```
