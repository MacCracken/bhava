//! Basic sentiment analysis — classify text into emotional categories.
//!
//! Keyword-based for local/fast classification. For deep analysis,
//! route through hoosh LLM via the `ai` feature.

use serde::{Deserialize, Serialize};

use crate::mood::Emotion;

/// Sentiment classification result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentimentResult {
    /// Overall valence: -1.0 (very negative) to 1.0 (very positive).
    pub valence: f32,
    /// Confidence in the classification: 0.0 to 1.0.
    pub confidence: f32,
    /// Detected emotions with intensities.
    pub emotions: Vec<(Emotion, f32)>,
    /// Matched keywords that contributed to the classification.
    pub matched_keywords: Vec<String>,
}

impl SentimentResult {
    /// Is the overall sentiment positive?
    pub fn is_positive(&self) -> bool {
        self.valence > 0.1
    }

    /// Is the overall sentiment negative?
    pub fn is_negative(&self) -> bool {
        self.valence < -0.1
    }

    /// Is the overall sentiment neutral?
    pub fn is_neutral(&self) -> bool {
        self.valence.abs() <= 0.1
    }

    /// Dominant detected emotion (highest intensity).
    pub fn dominant_emotion(&self) -> Option<Emotion> {
        self.emotions
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(e, _)| *e)
    }
}

/// Keyword lists for basic sentiment detection.
struct SentimentLexicon;

impl SentimentLexicon {
    const POSITIVE: &'static [&'static str] = &[
        "good",
        "great",
        "excellent",
        "amazing",
        "wonderful",
        "fantastic",
        "love",
        "happy",
        "glad",
        "pleased",
        "thank",
        "thanks",
        "awesome",
        "perfect",
        "beautiful",
        "brilliant",
        "enjoy",
        "helpful",
        "nice",
        "excited",
        "impressive",
        "outstanding",
        "superb",
        "delighted",
    ];

    const NEGATIVE: &'static [&'static str] = &[
        "bad",
        "terrible",
        "awful",
        "horrible",
        "hate",
        "angry",
        "upset",
        "frustrated",
        "annoyed",
        "disappointed",
        "wrong",
        "broken",
        "fail",
        "error",
        "bug",
        "crash",
        "slow",
        "ugly",
        "confusing",
        "worst",
        "useless",
        "stupid",
        "impossible",
        "painful",
        "problem",
    ];

    const TRUST: &'static [&'static str] = &[
        "trust",
        "reliable",
        "honest",
        "safe",
        "secure",
        "confident",
        "depend",
        "faith",
        "loyal",
        "sincere",
    ];

    const CURIOSITY: &'static [&'static str] = &[
        "curious",
        "wonder",
        "interesting",
        "how",
        "why",
        "explore",
        "learn",
        "discover",
        "question",
        "investigate",
    ];

    const FRUSTRATION: &'static [&'static str] = &[
        "frustrated",
        "annoyed",
        "irritated",
        "stuck",
        "confused",
        "broken",
        "doesn't work",
        "can't",
        "impossible",
        "give up",
    ];
}

/// Analyze sentiment of text using keyword matching.
///
/// This is a fast, local, zero-network analysis. For deeper analysis,
/// use the `ai` feature to route through hoosh LLM.
pub fn analyze(text: &str) -> SentimentResult {
    let lower = text.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    let word_count = words.len().max(1) as f32;

    let mut positive_count = 0u32;
    let mut negative_count = 0u32;
    let mut matched = Vec::new();
    let mut emotions: Vec<(Emotion, f32)> = Vec::new();

    // Check each word against lexicons
    for word in &words {
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
        if clean.is_empty() {
            continue;
        }

        if SentimentLexicon::POSITIVE.contains(&clean) {
            positive_count += 1;
            matched.push(clean.to_string());
        }
        if SentimentLexicon::NEGATIVE.contains(&clean) {
            negative_count += 1;
            matched.push(clean.to_string());
        }
        if SentimentLexicon::TRUST.contains(&clean) {
            add_emotion(&mut emotions, Emotion::Trust, 0.3);
        }
        if SentimentLexicon::CURIOSITY.contains(&clean) {
            add_emotion(&mut emotions, Emotion::Interest, 0.3);
        }
        if SentimentLexicon::FRUSTRATION.contains(&clean) {
            add_emotion(&mut emotions, Emotion::Frustration, 0.3);
        }
    }

    // Compute valence
    let pos_score = positive_count as f32 / word_count;
    let neg_score = negative_count as f32 / word_count;
    let valence = (pos_score - neg_score).clamp(-1.0, 1.0);

    // Add joy/arousal based on valence
    if valence > 0.1 {
        add_emotion(&mut emotions, Emotion::Joy, valence);
    } else if valence < -0.1 {
        add_emotion(&mut emotions, Emotion::Joy, valence); // negative joy
    }

    // Confidence based on keyword density
    let total_matches = positive_count + negative_count;
    let confidence = if total_matches == 0 {
        0.0
    } else {
        (total_matches as f32 / word_count).min(1.0) * 0.8 + 0.2
    };

    SentimentResult {
        valence,
        confidence,
        emotions,
        matched_keywords: matched,
    }
}

fn add_emotion(emotions: &mut Vec<(Emotion, f32)>, emotion: Emotion, intensity: f32) {
    if let Some(existing) = emotions.iter_mut().find(|(e, _)| *e == emotion) {
        existing.1 = (existing.1 + intensity).clamp(-1.0, 1.0);
    } else {
        emotions.push((emotion, intensity.clamp(-1.0, 1.0)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_text() {
        let r = analyze("This is great and wonderful work!");
        assert!(r.is_positive());
        assert!(r.valence > 0.0);
        assert!(!r.matched_keywords.is_empty());
    }

    #[test]
    fn test_negative_text() {
        let r = analyze("This is terrible and broken, I hate it.");
        assert!(r.is_negative());
        assert!(r.valence < 0.0);
    }

    #[test]
    fn test_neutral_text() {
        let r = analyze("The meeting is at three o'clock in the conference room.");
        assert!(r.is_neutral());
        assert!(r.matched_keywords.is_empty());
    }

    #[test]
    fn test_mixed_sentiment() {
        let r = analyze("The design is beautiful but the performance is terrible.");
        // Should have both positive and negative matches
        assert!(r.matched_keywords.len() >= 2);
    }

    #[test]
    fn test_trust_detection() {
        let r = analyze("I trust this system, it feels safe and reliable.");
        let trust = r.emotions.iter().find(|(e, _)| *e == Emotion::Trust);
        assert!(trust.is_some());
        assert!(trust.unwrap().1 > 0.0);
    }

    #[test]
    fn test_curiosity_detection() {
        let r = analyze("I wonder how this works, it's very interesting.");
        let interest = r.emotions.iter().find(|(e, _)| *e == Emotion::Interest);
        assert!(interest.is_some());
    }

    #[test]
    fn test_frustration_detection() {
        let r = analyze("I'm frustrated and stuck, this is broken.");
        let frust = r.emotions.iter().find(|(e, _)| *e == Emotion::Frustration);
        assert!(frust.is_some());
    }

    #[test]
    fn test_empty_text() {
        let r = analyze("");
        assert!(r.is_neutral());
        assert_eq!(r.confidence, 0.0);
    }

    #[test]
    fn test_dominant_emotion() {
        let r = analyze("I absolutely love this, it's amazing and wonderful!");
        assert_eq!(r.dominant_emotion(), Some(Emotion::Joy));
    }

    #[test]
    fn test_dominant_emotion_none() {
        let r = analyze("The time is noon.");
        assert!(r.dominant_emotion().is_none());
    }

    #[test]
    fn test_case_insensitive() {
        let r = analyze("GREAT AMAZING WONDERFUL");
        assert!(r.is_positive());
    }

    #[test]
    fn test_serde_roundtrip() {
        let r = analyze("This is a great test.");
        let json = serde_json::to_string(&r).unwrap();
        let r2: SentimentResult = serde_json::from_str(&json).unwrap();
        assert!((r2.valence - r.valence).abs() < 0.01);
    }
}
