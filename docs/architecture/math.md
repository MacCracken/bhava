# Mathematical Reference

All algorithms and formulas used in bhava.

## Personality

### Cosine Similarity (compatibility)

Measures pattern similarity between two personality profiles, independent of magnitude.

```
cos(A, B) = (A · B) / (|A| × |B|)
mapped to 0–1: similarity = (cos + 1) / 2
```

Where A and B are 15-dimensional vectors of normalized trait values (-1.0 to 1.0).

- 1.0 = identical behavioral pattern
- 0.5 = orthogonal (unrelated)
- 0.0 = opposite pattern

**Why cosine over Euclidean:** Two agents can be "mildly warm" and "very warm" — cosine sees them as similar (same direction), Euclidean sees them as different (different magnitude).

### Euclidean Distance

Used for `distance()` — raw geometric distance in trait space.

```
d(A, B) = sqrt(Σ (a_i - b_i)²)
```

Max distance: sqrt(15 × 4) = sqrt(60) ≈ 7.75 (all Lowest vs all Highest).

### Profile Blending

Linear interpolation in normalized trait space with level snapping.

```
blended_i = a_i + (b_i - a_i) × t
level = from_normalized(blended_i)
```

Where `t` ∈ [0, 1] controls the mix.

### Trait Level Normalization

```
Lowest  = -1.0 (numeric: -2)
Low     = -0.5 (numeric: -1)
Balanced =  0.0 (numeric:  0)
High    =  0.5 (numeric:  1)
Highest =  1.0 (numeric:  2)

normalized = numeric / 2.0
from_normalized: round(v × 2), clamp to [-2, 2]
```

### OCEAN Projection

Weighted linear projection from 15 traits to 5 OCEAN dimensions:

```
Openness        = 0.35×Creativity + 0.35×Curiosity + 0.15×RiskTolerance - 0.15×Precision
Conscientiousness = 0.40×Precision + 0.25×Formality - 0.20×RiskTolerance + 0.15×Autonomy
Extraversion    = 0.30×Warmth + 0.20×Humor + 0.15×Verbosity + 0.20×Confidence + 0.15×Directness
Agreeableness   = 0.30×Empathy + 0.25×Warmth + 0.20×Patience - 0.15×Skepticism - 0.10×Directness
Neuroticism     = -0.30×Patience - 0.30×Confidence + 0.20×Skepticism - 0.20×Empathy
```

### Personality Entropy (Shannon)

```
H = -Σ p_i × ln(p_i) / ln(5)
```

Where p_i is the fraction of traits at each level. Normalized to 0–1.

### Personality Extremity

```
extremity = mean(|trait_i.normalized()|)
```

Range: 0.0 (all Balanced) to 1.0 (all at extremes).

## Mood / Emotion

### PAD Model

6-dimensional extension of Mehrabian's Pleasure-Arousal-Dominance:

```
Joy:         [-1.0, 1.0]  pleasure/sadness
Arousal:     [-1.0, 1.0]  activation/calm
Dominance:   [-1.0, 1.0]  control/submission
Trust:       [-1.0, 1.0]  connection/isolation
Interest:    [-1.0, 1.0]  curiosity/apathy
Frustration: [-1.0, 1.0]  blocked/satisfied
```

### Mood Intensity (Euclidean magnitude)

```
intensity = sqrt(joy² + arousal² + dominance² + trust² + interest² + frustration²)
```

### Exponential Decay

```
factor = 1 - 2^(-elapsed / half_life)
mood = mood.blend(baseline, factor)
```

Emotions with active causes skip decay.

### Second-Order Damping

Discrete-time step of a damped harmonic oscillator:

```
acceleration = -2ζω × velocity - ω² × position
velocity += acceleration × dt
position += velocity × dt
```

- ζ < 1: underdamped (oscillatory — neurotic agents)
- ζ = 1: critically damped (fastest smooth return)
- ζ > 1: overdamped (sluggish — stoic agents)

### Trait-to-Mood Baseline

```
baseline_valence = mean(trait_modifier_valence_i) + compound_effects_valence
baseline_arousal = mean(trait_modifier_arousal_i) + compound_effects_arousal
```

Each of the 15 traits contributes a (valence, arousal) modifier based on its level. 7 compound effects add emergent modifiers when specific trait combinations are present.

### Adaptive Baseline (Hedonic Treadmill)

```
adapted = adapted.blend(recent_mood, adaptation_rate)
adapted = adapted.blend(core_baseline, recovery_rate)
```

Called periodically. `adaptation_rate` << `recovery_rate` ensures transient events barely shift the baseline but sustained patterns do.

### Emotional Contagion

```
delta[i] = sender_mood[i] × expressiveness × susceptibility × |affinity|
sign = if affinity >= 0 then 1 else -1
```

Rivals invert the emotional signal.

### Action Tendency Scoring

5 competing impulses computed from mood dimensions:

```
approach  = max(0, joy×0.5 + trust×0.3 + arousal×0.2)
avoid     = max(0, -trust×0.4 - dominance×0.3 + arousal×0.2)
confront  = max(0, frustration×0.4 + dominance×0.3 + arousal×0.3)
withdraw  = max(0, -joy×0.4 - arousal×0.3 - dominance×0.2)
protect   = max(0, trust×0.3 + dominance×0.4 - frustration×0.2)
```

Highest score wins. Below 0.1 = Neutral.

### Emotion Amplifier

```
amplifier = 1.0 + personality_modifier
clamped to [0.5, 2.0]
```

Applied to incoming stimuli before they affect the mood vector. Personality modifiers vary by emotion type and stimulus valence.

## Sentiment

### Valence Computation

```
valence = (positive_score - negative_score) / word_count
clamped to [-1.0, 1.0]
```

Positive/negative scores accumulate per-word weights:
- Base weight: 1.0 per matched keyword
- Negation: flips sign (weight × -1)
- Intensity modifier: scales weight (e.g., "very" × 1.5, "slightly" × 0.3)

### Confidence

```
confidence = if no_matches then 0.0
             else min(total_matches / word_count, 1.0) × 0.8 + 0.2
```

## Appraisal (OCC)

### Well-being Emotions

```
if desirability > 0.1 and likelihood > 0.7: Joy (intensity = desirability)
if desirability < -0.1 and likelihood > 0.7: Distress (intensity = |desirability|)
```

### Prospect Emotions

```
if desirability > 0.1 and likelihood < 0.7: Hope (intensity = desirability × likelihood)
if desirability < -0.1 and likelihood < 0.7: Fear (intensity = |desirability| × likelihood)
```

### Attribution Emotions

```
if praiseworthy > 0.1 and self-caused: Pride
if praiseworthy < -0.1 and self-caused: Shame
if praiseworthy > 0.1 and other-caused: Admiration
if praiseworthy < -0.1 and other-caused: Reproach
```

### Compound Emotions

```
if desirable and other-caused: Gratitude (scaled by affinity)
if undesirable and other-caused: Anger (scaled by inverse affinity)
```

## Relationships

### Decay

```
affinity = affinity + (0.0 - affinity) × decay_rate
trust = trust + (0.5 - trust) × decay_rate
```

Affinity decays toward 0 (neutral). Trust decays toward 0.5 (uncertain).

### Reciprocity

```
reciprocity = 1.0 - (|affinity_AB - affinity_BA| + |trust_AB - trust_BA|) / 4.0
```

Range: 0.0 (completely asymmetric) to 1.0 (perfectly mutual).

## Energy / Fatigue (Banister)

### Impulse-Response Model

```
fitness(n+1) = fitness(n) × e^(-1/τ₁) + k₁ × exertion
fatigue(n+1) = fatigue(n) × e^(-1/τ₂) + k₂ × exertion
```

Where τ₁ = 60 (slow fitness decay), τ₂ = 15 (fast fatigue decay), k₁ = 0.01, k₂ = 0.03.

### Cognitive Performance (Sigmoid)

```
performance = 1 / (1 + e^(-4 × (fitness - fatigue)))
```

Range: 0.0–1.0. Above 0.5 = net-positive adaptation. Below 0.5 = overreached.

### Exertion from Mood

```
exertion = mood.intensity() / sqrt(N)
```

Where N = number of emotion dimensions (6). Clamped to [0.0, 1.0].

## Circadian Rhythm (Borbely)

### Dual-Cosine Alertness

```
local_hour = (utc_hour + offset + chronotype_shift) mod 24
primary = cos(2π(h - 10) / 24)         // peaks at hour 10
secondary = -cos(2π(h - 14) / 12)      // dips at hour 14
alertness = clamp(0.5 + A₁×primary + A₂×secondary, 0, 1)
```

A₁ = 0.3 (primary amplitude), A₂ = 0.1 (secondary amplitude).

### Chronotype Phase Shifts

```
EarlyBird: -2h, MorningLeaning: -1h, Neutral: 0, EveningLeaning: +1h, NightOwl: +2h
```

## Flow State Detection

### Condition Thresholds

```
interest >= 0.4
frustration <= 0.3
0.1 <= arousal <= 0.7
dominance >= 0.1
energy >= 0.3
alertness >= 0.3
```

### State Machine

```
Inactive → Building (all conditions met, accumulator += build_rate)
Building → Active (accumulator >= entry_threshold)
Building → Inactive (any condition breaks, accumulator resets)
Active → Disrupted (any condition breaks, instant)
Disrupted → Inactive (one tick refractory)
```

### Performance Bonus

```
bonus = 1.1 + min(flow_duration / 60, 1.0) × 0.2
```

Range: 1.1 (entering flow) to 1.3 (deep flow after 60 ticks).

## Emotional Intelligence (EQ)

### Weighted Overall Score

```
overall = 0.15×perception + 0.20×facilitation + 0.30×understanding + 0.35×management
```

Hierarchical weights: higher branches weighted more (depend on lower ones).

## ACT-R Activation

### Base-Level Activation

```
B = ln(n) - d × ln(L)
```

n = rehearsal count, L = age since first presentation (min 1.0), d = 0.5 (decay).

### Recency Bonus

```
bonus = e^(-λ × (now - last_seen))
λ = ln(2) / half_life
```

### Hebbian Link Strengthening

```
s_new = s_old + δ × (1.0 - s_old)
```

Asymptotically approaches 1.0. Spreading activation dampened by 0.1 factor.

## Salience Classification

### Urgency × Importance

```
urgency = |desirability| × likelihood × (1 + mood_deviation)
importance = max(|desirability|, |praiseworthiness|) × (1 + memory_intensity)
magnitude = sqrt(urgency × importance)
```

Geometric mean ensures both dimensions must contribute. Levels: Background (<0.2), Notable (<0.45), Significant (<0.75), Critical.

## Preference Learning

### Exponential Moving Average

```
α = 1 / (1 + exposure_count)
valence = valence × (1 - α) + biased_outcome × α
```

Learning rate decreases with exposure: first experience α=1.0, tenth α≈0.09.
