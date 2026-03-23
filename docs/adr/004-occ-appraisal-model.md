# ADR-004: OCC Appraisal Model for Goal-Aware Emotions

**Status:** Accepted
**Date:** 2026-03-23

## Context

The original emotion system used hardcoded `MoodTrigger` presets (praised, criticized, surprised, threatened) with fixed emotion deltas. This produces emotions disconnected from context — the agent doesn't know *why* it feels something.

The OCC model (Ortony, Clore & Collins, 1988) derives emotions from cognitive *appraisals* of events:
- **Desirability**: is this good or bad for my goals?
- **Praiseworthiness**: does this action align with my standards?
- **Likelihood**: how certain is this outcome?
- **Attribution**: who caused this?

## Decision

Add an `appraisal` module with:
- `Appraisal` struct (builder pattern) for describing events
- `appraise()` function that maps appraisals to 12 named emotions
- `apply_appraisal()` to feed results into `EmotionalState`

The caller provides the appraisal context; bhava computes the appropriate emotional response. This keeps bhava as a computation library without requiring it to understand game semantics.

## Consequences

- **Contextual emotions**: Joy from a praised event is different from joy from a personal achievement (different attribution emotions)
- **12 named emotions**: Joy, Distress, Hope, Fear, Relief, Disappointment, Pride, Shame, Admiration, Reproach, Gratitude, Anger
- **Relationship-aware**: Gratitude/Anger scale with affinity toward the causal agent
- **Coexists with triggers**: Old `MoodTrigger` system still works for simple stimuli; appraisals are for richer events
- **18ns per appraisal**: Pure computation, zero allocation for simple events
