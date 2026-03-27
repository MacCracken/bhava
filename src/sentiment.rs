//! Sentiment analysis — classify text into emotional categories.
//!
//! Keyword-based for local/fast classification with negation handling and
//! intensity modifiers. For deep analysis, route through hoosh LLM via the
//! `ai` feature.

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

/// Per-sentence sentiment result for sentence-level analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentenceResult {
    /// The original sentence text.
    pub text: String,
    /// Sentiment for this sentence.
    pub sentiment: SentimentResult,
}

/// Result of sentence-level analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentResult {
    /// Aggregate sentiment across all sentences.
    pub aggregate: SentimentResult,
    /// Per-sentence breakdown.
    pub sentences: Vec<SentenceResult>,
}

// --- Negation and Intensity (v0.4) ---

/// Words that negate the following sentiment keyword.
const NEGATORS: &[&str] = &["not", "no", "never", "neither", "nor", "hardly", "barely"];

/// Intensity modifier and its multiplier.
struct IntensityModifier {
    word: &'static str,
    multiplier: f32,
}

const INTENSIFIERS: &[IntensityModifier] = &[
    IntensityModifier {
        word: "very",
        multiplier: 1.5,
    },
    IntensityModifier {
        word: "extremely",
        multiplier: 2.0,
    },
    IntensityModifier {
        word: "really",
        multiplier: 1.4,
    },
    IntensityModifier {
        word: "incredibly",
        multiplier: 1.8,
    },
    IntensityModifier {
        word: "absolutely",
        multiplier: 1.8,
    },
    IntensityModifier {
        word: "totally",
        multiplier: 1.5,
    },
    IntensityModifier {
        word: "somewhat",
        multiplier: 0.5,
    },
    IntensityModifier {
        word: "slightly",
        multiplier: 0.3,
    },
    IntensityModifier {
        word: "fairly",
        multiplier: 0.7,
    },
    IntensityModifier {
        word: "rather",
        multiplier: 0.8,
    },
];

fn is_negator(word: &str) -> bool {
    NEGATORS.contains(&word)
}

fn intensity_multiplier(word: &str) -> Option<f32> {
    INTENSIFIERS
        .iter()
        .find(|m| m.word == word)
        .map(|m| m.multiplier)
}

// --- Configurable Lexicons (v0.4) ---

/// Configuration for sentiment analysis with custom lexicons.
///
/// Custom words extend (not replace) the built-in lexicons.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SentimentConfig {
    /// Additional positive keywords.
    pub extra_positive: Vec<String>,
    /// Additional negative keywords.
    pub extra_negative: Vec<String>,
    /// Additional trust keywords.
    pub extra_trust: Vec<String>,
    /// Additional curiosity keywords.
    pub extra_curiosity: Vec<String>,
    /// Additional frustration keywords.
    pub extra_frustration: Vec<String>,
}

impl SentimentConfig {
    /// Create an empty config (uses only built-in lexicons).
    pub fn new() -> Self {
        Self::default()
    }

    fn is_positive(&self, word: &str) -> bool {
        SentimentLexicon::POSITIVE.contains(&word) || self.extra_positive.iter().any(|w| w == word)
    }

    fn is_negative(&self, word: &str) -> bool {
        SentimentLexicon::NEGATIVE.contains(&word) || self.extra_negative.iter().any(|w| w == word)
    }

    fn is_trust(&self, word: &str) -> bool {
        SentimentLexicon::TRUST.contains(&word) || self.extra_trust.iter().any(|w| w == word)
    }

    fn is_curiosity(&self, word: &str) -> bool {
        SentimentLexicon::CURIOSITY.contains(&word)
            || self.extra_curiosity.iter().any(|w| w == word)
    }

    fn is_frustration(&self, word: &str) -> bool {
        SentimentLexicon::FRUSTRATION.contains(&word)
            || self.extra_frustration.iter().any(|w| w == word)
    }
}

/// Keyword lists for basic sentiment detection.
///
/// Lexicons are sorted alphabetically (verified by test) for maintainability.
struct SentimentLexicon;

impl SentimentLexicon {
    const POSITIVE: &'static [&'static str] = &[
        "amazing",
        "awesome",
        "beautiful",
        "brilliant",
        "delighted",
        "enjoy",
        "excellent",
        "excited",
        "fantastic",
        "glad",
        "good",
        "great",
        "happy",
        "helpful",
        "impressive",
        "love",
        "nice",
        "outstanding",
        "perfect",
        "pleased",
        "superb",
        "thank",
        "thanks",
        "wonderful",
    ];

    const NEGATIVE: &'static [&'static str] = &[
        "angry",
        "annoyed",
        "awful",
        "bad",
        "broken",
        "bug",
        "confusing",
        "crash",
        "disappointed",
        "error",
        "fail",
        "frustrated",
        "hate",
        "horrible",
        "impossible",
        "painful",
        "problem",
        "slow",
        "stupid",
        "terrible",
        "ugly",
        "upset",
        "useless",
        "worst",
        "wrong",
    ];

    const TRUST: &'static [&'static str] = &[
        "confident",
        "depend",
        "faith",
        "honest",
        "loyal",
        "reliable",
        "safe",
        "secure",
        "sincere",
        "trust",
    ];

    const CURIOSITY: &'static [&'static str] = &[
        "curious",
        "discover",
        "explore",
        "how",
        "interesting",
        "investigate",
        "learn",
        "question",
        "why",
        "wonder",
    ];

    const FRUSTRATION: &'static [&'static str] = &[
        "annoyed",
        "broken",
        "confused",
        "frustrated",
        "hopeless",
        "impossible",
        "irritated",
        "pointless",
        "stuck",
        "useless",
    ];
}

/// Analyze sentiment of text using keyword matching.
///
/// This is a fast, local, zero-network analysis with negation handling and
/// intensity modifiers. For deeper analysis, use the `ai` feature.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn analyze(text: &str) -> SentimentResult {
    analyze_with_config(text, &SentimentConfig::default())
}

/// Analyze sentiment with a custom configuration.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn analyze_with_config(text: &str, config: &SentimentConfig) -> SentimentResult {
    let lower = text.to_lowercase();
    analyze_fragment(&lower, config)
}

/// Analyze a single sentence or text fragment (already lowercased).
fn analyze_fragment(lower: &str, config: &SentimentConfig) -> SentimentResult {
    let mut word_count = 0u32;
    let mut positive_score = 0.0f32;
    let mut negative_score = 0.0f32;
    let mut matched = Vec::new();
    let mut emotions: Vec<(Emotion, f32)> = Vec::new();

    let mut negated = false;
    let mut modifier = 1.0f32;

    for word in lower.split_whitespace() {
        word_count += 1;
        let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
        if clean.is_empty() {
            continue;
        }

        // Check for negation
        if is_negator(clean) {
            negated = true;
            continue;
        }

        // Check for intensity modifier
        if let Some(m) = intensity_multiplier(clean) {
            modifier = m;
            continue;
        }

        let sign = if negated { -1.0 } else { 1.0 };
        let weight = modifier * sign;

        if config.is_positive(clean) {
            if weight > 0.0 {
                positive_score += weight;
            } else {
                negative_score += weight.abs();
            }
            matched.push(clean.to_string());
        }
        if config.is_negative(clean) {
            if weight > 0.0 {
                negative_score += weight;
            } else {
                // negated negative = positive
                positive_score += weight.abs();
            }
            matched.push(clean.to_string());
        }

        // Emotion detection (scaled by modifier, flipped by negation)
        if config.is_trust(clean) {
            add_emotion(&mut emotions, Emotion::Trust, 0.3 * weight);
        }
        if config.is_curiosity(clean) {
            add_emotion(&mut emotions, Emotion::Interest, 0.3 * weight);
        }
        if config.is_frustration(clean) {
            add_emotion(&mut emotions, Emotion::Frustration, 0.3 * weight);
        }

        // Reset modifiers after consuming a content word
        negated = false;
        modifier = 1.0;
    }

    let word_count_f = word_count.max(1) as f32;

    // Compute valence
    let pos_ratio = positive_score / word_count_f;
    let neg_ratio = negative_score / word_count_f;
    let valence = (pos_ratio - neg_ratio).clamp(-1.0, 1.0);

    // Add joy based on valence
    if valence.abs() > 0.1 {
        add_emotion(&mut emotions, Emotion::Joy, valence);
    }

    // Confidence based on keyword density
    let total_matches = positive_score + negative_score;
    let confidence = if total_matches < f32::EPSILON {
        0.0
    } else {
        (total_matches / word_count_f).min(1.0) * 0.8 + 0.2
    };

    SentimentResult {
        valence,
        confidence,
        emotions,
        matched_keywords: matched,
    }
}

/// Analyze text at the sentence level.
///
/// Splits on sentence-ending punctuation (`.`, `!`, `?`) and analyzes each
/// sentence independently. Returns per-sentence results plus an aggregate.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn analyze_sentences(text: &str) -> DocumentResult {
    analyze_sentences_with_config(text, &SentimentConfig::default())
}

/// Analyze text at the sentence level with a custom configuration.
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn analyze_sentences_with_config(text: &str, config: &SentimentConfig) -> DocumentResult {
    let lower = text.to_lowercase();

    // Split on sentence boundaries
    let sentences: Vec<&str> = split_sentences(&lower);

    if sentences.is_empty() {
        return DocumentResult {
            aggregate: analyze_with_config(text, config),
            sentences: Vec::new(),
        };
    }

    let mut sentence_results = Vec::with_capacity(sentences.len());
    let mut total_valence = 0.0f32;
    let mut total_confidence = 0.0f32;
    let mut all_emotions: Vec<(Emotion, f32)> = Vec::new();
    let mut all_keywords = Vec::new();

    for sentence in &sentences {
        let trimmed = sentence.trim();
        if trimmed.is_empty() {
            continue;
        }
        let result = analyze_fragment(trimmed, config);
        total_valence += result.valence;
        total_confidence += result.confidence;
        for &(e, i) in &result.emotions {
            add_emotion(&mut all_emotions, e, i);
        }
        all_keywords.extend(result.matched_keywords.iter().cloned());
        sentence_results.push(SentenceResult {
            text: trimmed.to_string(),
            sentiment: result,
        });
    }

    let count = sentence_results.len().max(1) as f32;
    let aggregate = SentimentResult {
        valence: (total_valence / count).clamp(-1.0, 1.0),
        confidence: (total_confidence / count).clamp(0.0, 1.0),
        emotions: all_emotions,
        matched_keywords: all_keywords,
    };

    DocumentResult {
        aggregate,
        sentences: sentence_results,
    }
}

/// Split text into sentences on `.`, `!`, `?`.
fn split_sentences(text: &str) -> Vec<&str> {
    let mut sentences = Vec::new();
    let mut start = 0;
    for (i, c) in text.char_indices() {
        if c == '.' || c == '!' || c == '?' {
            let segment = text[start..i].trim();
            if !segment.is_empty() {
                sentences.push(segment);
            }
            start = i + c.len_utf8();
        }
    }
    // Trailing text without punctuation
    let tail = text[start..].trim();
    if !tail.is_empty() {
        sentences.push(tail);
    }
    sentences
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

    // --- Basic analysis (existing) ---

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

    #[test]
    fn test_confidence_positive() {
        let r = analyze("This is great and wonderful and amazing!");
        assert!(r.confidence > 0.0);
        assert!(r.confidence <= 1.0);
    }

    #[test]
    fn test_confidence_zero_for_neutral() {
        let r = analyze("The meeting is scheduled for noon.");
        assert!(r.confidence.abs() < f32::EPSILON);
    }

    #[test]
    fn test_multiple_trust_keywords_accumulate() {
        let r = analyze("I trust this safe, reliable, honest system.");
        let trust = r.emotions.iter().find(|(e, _)| *e == Emotion::Trust);
        assert!(trust.is_some());
        assert!(trust.unwrap().1 > 0.3);
    }

    #[test]
    fn test_punctuation_stripped() {
        let r = analyze("great! wonderful!! amazing!!!");
        assert!(r.is_positive());
        assert!(r.matched_keywords.len() >= 3);
    }

    #[test]
    fn test_single_word_positive() {
        let r = analyze("excellent");
        assert!(r.is_positive());
    }

    #[test]
    fn test_single_word_negative() {
        let r = analyze("terrible");
        assert!(r.is_negative());
    }

    #[test]
    fn test_mixed_emotions_detected() {
        let r = analyze("I'm curious but frustrated with this broken thing.");
        let has_interest = r.emotions.iter().any(|(e, _)| *e == Emotion::Interest);
        let has_frustration = r.emotions.iter().any(|(e, _)| *e == Emotion::Frustration);
        assert!(has_interest);
        assert!(has_frustration);
    }

    #[test]
    fn test_valence_clamped() {
        let r = analyze("good great excellent amazing wonderful fantastic love happy glad pleased");
        assert!(r.valence <= 1.0);
        assert!(r.valence >= -1.0);
    }

    #[test]
    fn test_is_neutral_boundary() {
        let r = analyze("The time is noon.");
        assert!(r.is_neutral());
        assert!(!r.is_positive());
        assert!(!r.is_negative());
    }

    #[test]
    fn test_dominant_emotion_frustration() {
        let r = analyze("frustrated annoyed stuck broken confused irritated");
        assert_eq!(r.dominant_emotion(), Some(Emotion::Frustration));
    }

    #[test]
    fn test_serde_roundtrip_with_emotions() {
        let r = analyze("I trust this curious interesting system, it's great!");
        let json = serde_json::to_string(&r).unwrap();
        let r2: SentimentResult = serde_json::from_str(&json).unwrap();
        assert_eq!(r2.emotions.len(), r.emotions.len());
        assert_eq!(r2.matched_keywords, r.matched_keywords);
        assert!((r2.confidence - r.confidence).abs() < 0.01);
    }

    #[test]
    fn test_whitespace_only() {
        let r = analyze("   ");
        assert!(r.is_neutral());
        assert_eq!(r.confidence, 0.0);
    }

    #[test]
    fn test_lexicons_sorted() {
        fn is_sorted(arr: &[&str], name: &str) {
            for w in arr.windows(2) {
                assert!(
                    w[0] <= w[1],
                    "{name} lexicon not sorted: {:?} > {:?}",
                    w[0],
                    w[1]
                );
            }
        }
        is_sorted(SentimentLexicon::POSITIVE, "POSITIVE");
        is_sorted(SentimentLexicon::NEGATIVE, "NEGATIVE");
        is_sorted(SentimentLexicon::TRUST, "TRUST");
        is_sorted(SentimentLexicon::CURIOSITY, "CURIOSITY");
        is_sorted(SentimentLexicon::FRUSTRATION, "FRUSTRATION");
    }

    // --- v0.4: Negation ---

    #[test]
    fn test_negation_not_good() {
        let r = analyze("This is not good.");
        assert!(r.is_negative() || r.is_neutral());
        // "good" negated should contribute negative
        assert!(r.valence <= 0.0);
    }

    #[test]
    fn test_negation_not_bad() {
        let r = analyze("This is not bad at all.");
        // "bad" negated should contribute positive
        assert!(r.valence >= 0.0);
    }

    #[test]
    fn test_negation_never_happy() {
        let r = analyze("I am never happy with this.");
        assert!(r.valence < 0.0);
    }

    #[test]
    fn test_negation_resets_after_word() {
        let r = analyze("not bad but great");
        // "not bad" → positive, "great" → positive
        assert!(r.is_positive());
    }

    #[test]
    fn test_negation_barely() {
        let r = analyze("This is barely good enough.");
        assert!(r.valence <= 0.0);
    }

    // --- v0.4: Intensity modifiers ---

    #[test]
    fn test_intensity_very_good() {
        // Use same word count for fair comparison
        let base = analyze("this is good stuff");
        let intensified = analyze("this is very good");
        assert!(intensified.valence > base.valence);
    }

    #[test]
    fn test_intensity_extremely_bad() {
        let base = analyze("this is bad stuff");
        let intensified = analyze("this is extremely bad");
        assert!(intensified.valence < base.valence);
    }

    #[test]
    fn test_intensity_slightly_good() {
        let base = analyze("this is good stuff");
        let dampened = analyze("this is slightly good");
        assert!(dampened.valence < base.valence);
        assert!(dampened.valence >= 0.0);
    }

    #[test]
    fn test_intensity_resets_after_word() {
        let r = analyze("very good and bad");
        // "very" only applies to "good", not "bad"
        assert!(r.matched_keywords.contains(&"good".to_string()));
        assert!(r.matched_keywords.contains(&"bad".to_string()));
    }

    #[test]
    fn test_negation_with_intensity() {
        let r = analyze("not very good");
        // "not" negates, "very" intensifies, applied to "good"
        assert!(r.valence < 0.0);
    }

    // --- v0.4: Configurable lexicons ---

    #[test]
    fn test_config_default_same_as_analyze() {
        let text = "This is great work!";
        let r1 = analyze(text);
        let r2 = analyze_with_config(text, &SentimentConfig::default());
        assert!((r1.valence - r2.valence).abs() < f32::EPSILON);
    }

    #[test]
    fn test_config_extra_positive() {
        let mut config = SentimentConfig::new();
        config.extra_positive.push("groovy".to_string());
        let r = analyze_with_config("This is groovy!", &config);
        assert!(r.is_positive());
    }

    #[test]
    fn test_config_extra_negative() {
        let mut config = SentimentConfig::new();
        config.extra_negative.push("janky".to_string());
        let r = analyze_with_config("This is janky.", &config);
        assert!(r.is_negative());
    }

    #[test]
    fn test_config_extra_trust() {
        let mut config = SentimentConfig::new();
        config.extra_trust.push("verified".to_string());
        let r = analyze_with_config("This is verified.", &config);
        let trust = r.emotions.iter().find(|(e, _)| *e == Emotion::Trust);
        assert!(trust.is_some());
    }

    #[test]
    fn test_config_extra_does_not_replace_builtins() {
        let mut config = SentimentConfig::new();
        config.extra_positive.push("groovy".to_string());
        // Built-in "great" should still work
        let r = analyze_with_config("great and groovy", &config);
        assert!(r.matched_keywords.contains(&"great".to_string()));
        assert!(r.matched_keywords.contains(&"groovy".to_string()));
    }

    #[test]
    fn test_config_serde() {
        let mut config = SentimentConfig::new();
        config.extra_positive.push("rad".to_string());
        let json = serde_json::to_string(&config).unwrap();
        let config2: SentimentConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config2.extra_positive, vec!["rad"]);
    }

    // --- v0.4: Sentence-level analysis ---

    #[test]
    fn test_analyze_sentences_single() {
        let r = analyze_sentences("This is great!");
        assert_eq!(r.sentences.len(), 1);
        assert!(r.aggregate.is_positive());
    }

    #[test]
    fn test_analyze_sentences_multiple() {
        let r = analyze_sentences("This is great! But that is terrible.");
        assert_eq!(r.sentences.len(), 2);
        assert!(r.sentences[0].sentiment.is_positive());
        assert!(r.sentences[1].sentiment.is_negative());
    }

    #[test]
    fn test_analyze_sentences_mixed_aggregate() {
        let r = analyze_sentences("Amazing work! Horrible result. Okay fine.");
        assert!(r.sentences.len() >= 2);
        // Aggregate should be moderated
        assert!(r.aggregate.valence.abs() < 1.0);
    }

    #[test]
    fn test_analyze_sentences_empty() {
        let r = analyze_sentences("");
        assert!(r.sentences.is_empty() || r.aggregate.is_neutral());
    }

    #[test]
    fn test_analyze_sentences_no_punctuation() {
        let r = analyze_sentences("This is great and wonderful");
        // No sentence terminators → treated as single sentence
        assert_eq!(r.sentences.len(), 1);
        assert!(r.aggregate.is_positive());
    }

    #[test]
    fn test_analyze_sentences_question() {
        let r = analyze_sentences("Is this good? I think it is great!");
        assert_eq!(r.sentences.len(), 2);
    }

    #[test]
    fn test_analyze_sentences_with_config() {
        let mut config = SentimentConfig::new();
        config.extra_positive.push("rad".to_string());
        let r = analyze_sentences_with_config("This is rad! That is great.", &config);
        assert_eq!(r.sentences.len(), 2);
        assert!(r.sentences[0].sentiment.is_positive());
        assert!(r.sentences[1].sentiment.is_positive());
    }

    #[test]
    fn test_document_result_serde() {
        let r = analyze_sentences("Great work! Bad result.");
        let json = serde_json::to_string(&r).unwrap();
        let r2: DocumentResult = serde_json::from_str(&json).unwrap();
        assert_eq!(r2.sentences.len(), r.sentences.len());
    }

    // --- split_sentences ---

    #[test]
    fn test_split_sentences_basic() {
        let s = split_sentences("hello. world!");
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_split_sentences_trailing() {
        let s = split_sentences("hello. world");
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_split_sentences_empty_segments() {
        let s = split_sentences("...");
        assert!(s.is_empty());
    }
}
