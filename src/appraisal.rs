//! OCC Appraisal — goal-aware emotion generation.
//!
//! Implements the Ortony, Clore & Collins (OCC) model for deriving emotions
//! from cognitive appraisals of events. Instead of hardcoded mood triggers,
//! the agent evaluates events against its goals and standards to produce
//! contextually appropriate emotional responses.
//!
//! The caller provides the appraisal (what happened, how desirable, who caused it);
//! bhava computes the resulting emotions.

use serde::{Deserialize, Serialize};

use crate::mood::{Emotion, MoodVector};

/// An appraisal of an event — the agent's cognitive evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appraisal {
    /// What happened (for logging/tracking).
    pub event: String,
    /// How desirable is this for the agent's goals? (-1.0 = terrible, 1.0 = excellent).
    pub desirability: f32,
    /// How praiseworthy is the action? (-1.0 = deplorable, 1.0 = admirable).
    pub praiseworthiness: f32,
    /// How likely is this to happen (for prospect emotions)? (0.0 = impossible, 1.0 = certain).
    pub likelihood: f32,
    /// Who caused this event (if known).
    pub causal_agent: Option<String>,
    /// Is this about self or another?
    pub is_self: bool,
}

impl Appraisal {
    /// Create a simple appraisal for a confirmed event.
    #[must_use]
    pub fn event(description: impl Into<String>, desirability: f32) -> Self {
        Self {
            event: description.into(),
            desirability: desirability.clamp(-1.0, 1.0),
            praiseworthiness: 0.0,
            likelihood: 1.0,
            causal_agent: None,
            is_self: false,
        }
    }

    /// Set praiseworthiness.
    #[must_use]
    pub fn with_praise(mut self, praiseworthiness: f32) -> Self {
        self.praiseworthiness = praiseworthiness.clamp(-1.0, 1.0);
        self
    }

    /// Set likelihood (for prospect/uncertain events).
    #[must_use]
    pub fn with_likelihood(mut self, likelihood: f32) -> Self {
        self.likelihood = likelihood.clamp(0.0, 1.0);
        self
    }

    /// Set causal agent.
    #[must_use]
    pub fn caused_by(mut self, agent: impl Into<String>) -> Self {
        self.causal_agent = Some(agent.into());
        self
    }

    /// Mark as self-caused.
    #[must_use]
    pub fn by_self(mut self) -> Self {
        self.is_self = true;
        self
    }
}

/// Named emotion derived from OCC appraisal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AppraisedEmotion {
    // Well-being emotions (desirability of events)
    Joy,
    Distress,
    // Prospect emotions (desirability + likelihood)
    Hope,
    Fear,
    Relief,
    Disappointment,
    // Attribution emotions (praiseworthiness of actions)
    Pride,
    Shame,
    Admiration,
    Reproach,
    // Compound
    Gratitude,
    Anger,
}

impl std::fmt::Display for AppraisedEmotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Joy => "joy",
            Self::Distress => "distress",
            Self::Hope => "hope",
            Self::Fear => "fear",
            Self::Relief => "relief",
            Self::Disappointment => "disappointment",
            Self::Pride => "pride",
            Self::Shame => "shame",
            Self::Admiration => "admiration",
            Self::Reproach => "reproach",
            Self::Gratitude => "gratitude",
            Self::Anger => "anger",
        };
        f.write_str(s)
    }
}

/// Result of an OCC appraisal — which emotions are generated and how they map to mood.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppraisalResult {
    /// Named emotions produced by this appraisal.
    pub emotions: Vec<(AppraisedEmotion, f32)>,
    /// Mood delta to apply to the agent's emotional state.
    pub mood_delta: MoodVector,
}

/// Appraise an event and derive emotions + mood delta.
///
/// This is the core OCC function. The caller describes what happened via an
/// `Appraisal`; this function returns the appropriate emotional response.
///
/// Optionally takes the relationship affinity toward the causal agent to
/// modulate attribution emotions (gratitude/anger scale with affinity).
#[must_use]
pub fn appraise(appraisal: &Appraisal, affinity_to_cause: Option<f32>) -> AppraisalResult {
    let d = appraisal.desirability;
    let p = appraisal.praiseworthiness;
    let l = appraisal.likelihood;
    let affinity = affinity_to_cause.unwrap_or(0.0);

    let mut emotions: Vec<(AppraisedEmotion, f32)> = Vec::new();
    let mut delta = MoodVector::neutral();

    // --- Well-being emotions (confirmed events) ---
    if l > 0.7 {
        if d > 0.1 {
            emotions.push((AppraisedEmotion::Joy, d));
            delta.joy += d * 0.6;
            delta.arousal += d * 0.2;
        } else if d < -0.1 {
            emotions.push((AppraisedEmotion::Distress, d.abs()));
            delta.joy += d * 0.6; // negative
            delta.arousal += d.abs() * 0.2;
            delta.frustration += d.abs() * 0.3;
        }
    }

    // --- Prospect emotions (uncertain events) ---
    if l < 0.7 && l > 0.0 {
        if d > 0.1 {
            emotions.push((AppraisedEmotion::Hope, d * l));
            delta.interest += d * l * 0.4;
            delta.arousal += l * 0.2;
        } else if d < -0.1 {
            emotions.push((AppraisedEmotion::Fear, d.abs() * l));
            delta.trust -= d.abs() * l * 0.3;
            delta.arousal += l * 0.3;
            delta.dominance -= d.abs() * l * 0.2;
        }
    }

    // --- Confirmed prospect outcomes ---
    if l >= 1.0 {
        // If this was a previously feared event that didn't happen → relief
        // If this was a previously hoped event that didn't happen → disappointment
        // (caller signals this via desirability sign with likelihood=1.0)
    }

    // --- Attribution emotions (praiseworthiness) ---
    if p.abs() > 0.1 {
        if appraisal.is_self {
            if p > 0.0 {
                emotions.push((AppraisedEmotion::Pride, p));
                delta.dominance += p * 0.4;
                delta.joy += p * 0.2;
            } else {
                emotions.push((AppraisedEmotion::Shame, p.abs()));
                delta.dominance -= p.abs() * 0.4;
                delta.joy -= p.abs() * 0.2;
            }
        } else if p > 0.0 {
            emotions.push((AppraisedEmotion::Admiration, p));
            delta.trust += p * 0.3;
        } else {
            emotions.push((AppraisedEmotion::Reproach, p.abs()));
            delta.trust -= p.abs() * 0.3;
            delta.frustration += p.abs() * 0.2;
        }
    }

    // --- Compound emotions (desirability + attribution + relationship) ---
    if d > 0.1 && !appraisal.is_self && appraisal.causal_agent.is_some() {
        let gratitude = d * (1.0 + affinity).clamp(0.0, 2.0) * 0.5;
        emotions.push((AppraisedEmotion::Gratitude, gratitude));
        delta.trust += gratitude * 0.3;
    }
    if d < -0.1 && !appraisal.is_self && appraisal.causal_agent.is_some() {
        let anger = d.abs() * (1.0 - affinity).clamp(0.0, 2.0) * 0.5;
        emotions.push((AppraisedEmotion::Anger, anger));
        delta.frustration += anger * 0.4;
        delta.dominance += anger * 0.2;
    }

    // Clamp all deltas
    for &e in Emotion::ALL {
        delta.set(e, delta.get(e).clamp(-1.0, 1.0));
    }

    AppraisalResult {
        emotions,
        mood_delta: delta,
    }
}

/// Apply an appraisal result to an emotional state.
pub fn apply_appraisal(state: &mut crate::mood::EmotionalState, result: &AppraisalResult) {
    for &emotion in Emotion::ALL {
        let val = result.mood_delta.get(emotion);
        if val.abs() > f32::EPSILON {
            state.stimulate(emotion, val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_event() {
        let a = Appraisal::event("got promoted", 0.8);
        let r = appraise(&a, None);
        assert!(r.emotions.iter().any(|(e, _)| *e == AppraisedEmotion::Joy));
        assert!(r.mood_delta.joy > 0.0);
    }

    #[test]
    fn test_negative_event() {
        let a = Appraisal::event("project failed", -0.7);
        let r = appraise(&a, None);
        assert!(
            r.emotions
                .iter()
                .any(|(e, _)| *e == AppraisedEmotion::Distress)
        );
        assert!(r.mood_delta.joy < 0.0);
        assert!(r.mood_delta.frustration > 0.0);
    }

    #[test]
    fn test_hope() {
        let a = Appraisal::event("might get bonus", 0.6).with_likelihood(0.4);
        let r = appraise(&a, None);
        assert!(r.emotions.iter().any(|(e, _)| *e == AppraisedEmotion::Hope));
        assert!(r.mood_delta.interest > 0.0);
    }

    #[test]
    fn test_fear() {
        let a = Appraisal::event("might get fired", -0.8).with_likelihood(0.5);
        let r = appraise(&a, None);
        assert!(r.emotions.iter().any(|(e, _)| *e == AppraisedEmotion::Fear));
        assert!(r.mood_delta.trust < 0.0);
    }

    #[test]
    fn test_pride() {
        let a = Appraisal::event("wrote great code", 0.5)
            .with_praise(0.8)
            .by_self();
        let r = appraise(&a, None);
        assert!(
            r.emotions
                .iter()
                .any(|(e, _)| *e == AppraisedEmotion::Pride)
        );
        assert!(r.mood_delta.dominance > 0.0);
    }

    #[test]
    fn test_shame() {
        let a = Appraisal::event("broke production", -0.5)
            .with_praise(-0.9)
            .by_self();
        let r = appraise(&a, None);
        assert!(
            r.emotions
                .iter()
                .any(|(e, _)| *e == AppraisedEmotion::Shame)
        );
        assert!(r.mood_delta.dominance < 0.0);
    }

    #[test]
    fn test_gratitude_with_high_affinity() {
        let a = Appraisal::event("teammate helped", 0.7).caused_by("alice");
        let r = appraise(&a, Some(0.8));
        assert!(
            r.emotions
                .iter()
                .any(|(e, _)| *e == AppraisedEmotion::Gratitude)
        );
        assert!(r.mood_delta.trust > 0.0);
    }

    #[test]
    fn test_anger_with_low_affinity() {
        let a = Appraisal::event("rival sabotaged", -0.8).caused_by("rival");
        let r = appraise(&a, Some(-0.5));
        assert!(
            r.emotions
                .iter()
                .any(|(e, _)| *e == AppraisedEmotion::Anger)
        );
        assert!(r.mood_delta.frustration > 0.0);
    }

    #[test]
    fn test_admiration() {
        let a = Appraisal::event("mentor's speech", 0.3).with_praise(0.9);
        let r = appraise(&a, None);
        assert!(
            r.emotions
                .iter()
                .any(|(e, _)| *e == AppraisedEmotion::Admiration)
        );
    }

    #[test]
    fn test_neutral_event() {
        let a = Appraisal::event("nothing happened", 0.0);
        let r = appraise(&a, None);
        assert!(r.emotions.is_empty());
    }

    #[test]
    fn test_apply_appraisal() {
        let mut state = crate::mood::EmotionalState::new();
        let a = Appraisal::event("great news", 0.8);
        let r = appraise(&a, None);
        apply_appraisal(&mut state, &r);
        assert!(state.mood.joy > 0.0);
    }

    #[test]
    fn test_builder_pattern() {
        let a = Appraisal::event("test", 0.5)
            .with_praise(0.3)
            .with_likelihood(0.6)
            .caused_by("bob")
            .by_self();
        assert_eq!(a.desirability, 0.5);
        assert!(a.is_self);
        assert_eq!(a.causal_agent.as_deref(), Some("bob"));
    }

    #[test]
    fn test_serde_roundtrip() {
        let a = Appraisal::event("test", 0.5).with_praise(0.3);
        let json = serde_json::to_string(&a).unwrap();
        let a2: Appraisal = serde_json::from_str(&json).unwrap();
        assert!((a2.desirability - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_appraised_emotion_display() {
        assert_eq!(AppraisedEmotion::Gratitude.to_string(), "gratitude");
        assert_eq!(AppraisedEmotion::Anger.to_string(), "anger");
    }

    #[test]
    fn test_mood_delta_clamped() {
        let a = Appraisal::event("extreme", 1.0)
            .with_praise(1.0)
            .caused_by("x");
        let r = appraise(&a, Some(1.0));
        for &e in Emotion::ALL {
            assert!(((-1.0)..=1.0).contains(&r.mood_delta.get(e)));
        }
    }
}
