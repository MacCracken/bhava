use serde::{Deserialize, Serialize};

use super::core::MoodVector;

// --- Plutchik Compound Emotions ---

/// Named compound emotion from Plutchik's wheel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CompoundEmotion {
    Love,           // Joy + Trust
    Optimism,       // Joy + Anticipation (mapped via Interest)
    Submission,     // Trust + Fear (mapped via low Dominance)
    Awe,            // Fear + Surprise (mapped via Arousal)
    Remorse,        // Sadness + Disgust (mapped via negative Joy + Frustration)
    Contempt,       // Anger + Disgust (mapped via Frustration)
    Aggressiveness, // Anger + Anticipation (mapped via Frustration + Interest)
}

impl std::fmt::Display for CompoundEmotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Love => "love",
            Self::Optimism => "optimism",
            Self::Submission => "submission",
            Self::Awe => "awe",
            Self::Remorse => "remorse",
            Self::Contempt => "contempt",
            Self::Aggressiveness => "aggressiveness",
        };
        f.write_str(s)
    }
}

/// Detect compound emotions present in a mood vector.
///
/// Returns all compound emotions whose constituent dimensions exceed the threshold.
#[must_use]
pub fn detect_compound_emotions(mood: &MoodVector, threshold: f32) -> Vec<(CompoundEmotion, f32)> {
    let t = threshold;
    let mut results = Vec::new();

    // Love: Joy + Trust
    if mood.joy > t && mood.trust > t {
        results.push((CompoundEmotion::Love, (mood.joy + mood.trust) / 2.0));
    }
    // Optimism: Joy + Interest (anticipation proxy)
    if mood.joy > t && mood.interest > t {
        results.push((CompoundEmotion::Optimism, (mood.joy + mood.interest) / 2.0));
    }
    // Submission: Trust + low Dominance (fear proxy)
    if mood.trust > t && mood.dominance < -t {
        results.push((
            CompoundEmotion::Submission,
            (mood.trust - mood.dominance) / 2.0,
        ));
    }
    // Awe: low Dominance + high Arousal (fear + surprise)
    if mood.dominance < -t && mood.arousal > t {
        results.push((CompoundEmotion::Awe, (-mood.dominance + mood.arousal) / 2.0));
    }
    // Remorse: negative Joy + Frustration
    if mood.joy < -t && mood.frustration > t {
        results.push((
            CompoundEmotion::Remorse,
            (-mood.joy + mood.frustration) / 2.0,
        ));
    }
    // Contempt: Frustration + negative Trust
    if mood.frustration > t && mood.trust < -t {
        results.push((
            CompoundEmotion::Contempt,
            (mood.frustration - mood.trust) / 2.0,
        ));
    }
    // Aggressiveness: Frustration + Interest (anger + anticipation)
    if mood.frustration > t && mood.interest > t {
        results.push((
            CompoundEmotion::Aggressiveness,
            (mood.frustration + mood.interest) / 2.0,
        ));
    }

    results
}
