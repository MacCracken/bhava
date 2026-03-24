//! Live sentiment monitoring — continuous feedback during text streams.
//!
//! Provides `SentimentMonitor` for tracking sentiment across a stream of text
//! chunks (e.g., tokens from an LLM response). Accumulates text, analyzes at
//! sentence boundaries, and feeds results back into an emotional state.

use crate::mood::EmotionalState;
use crate::sentiment::{self, SentimentConfig, SentimentResult};
use serde::{Deserialize, Serialize};

/// Live sentiment monitor for streaming text.
///
/// Buffers incoming text chunks, analyzes sentiment at sentence boundaries,
/// and optionally feeds results back into an emotional state.
///
/// # Example
///
/// ```no_run
/// use bhava::monitor::SentimentMonitor;
/// use bhava::mood::EmotionalState;
///
/// let mut state = EmotionalState::new();
/// let mut monitor = SentimentMonitor::new(0.5);
///
/// // Feed streaming tokens
/// monitor.feed("This is ");
/// monitor.feed("wonderful ");
/// monitor.feed("work! ");
/// monitor.feed("But this part ");
/// monitor.feed("is broken.");
///
/// // Flush remaining text and apply to mood
/// let results = monitor.flush();
/// for result in &results {
///     monitor.apply_to_mood(&mut state, result);
/// }
/// assert!(state.mood.joy != 0.0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentMonitor {
    /// Accumulated text buffer.
    buffer: String,
    /// Scale factor for mood feedback (0.0 = no effect, 1.0 = full).
    scale: f32,
    /// All sentence results collected so far.
    results: Vec<SentimentResult>,
    /// Optional custom sentiment config.
    config: SentimentConfig,
}

impl SentimentMonitor {
    /// Create a new monitor with the given feedback scale.
    #[must_use]
    pub fn new(scale: f32) -> Self {
        Self {
            buffer: String::new(),
            scale: scale.clamp(0.0, 1.0),
            results: Vec::new(),
            config: SentimentConfig::default(),
        }
    }

    /// Create a monitor with a custom sentiment config.
    #[must_use]
    pub fn with_config(scale: f32, config: SentimentConfig) -> Self {
        Self {
            buffer: String::new(),
            scale: scale.clamp(0.0, 1.0),
            results: Vec::new(),
            config,
        }
    }

    /// Feed a text chunk (e.g., a token from a streaming response).
    ///
    /// Analyzes and emits results whenever a sentence boundary (`.`, `!`, `?`) is detected.
    /// Returns any sentence results produced by this chunk.
    pub fn feed(&mut self, chunk: &str) -> Vec<SentimentResult> {
        self.buffer.push_str(chunk);
        self.drain_sentences()
    }

    /// Feed a chunk and immediately apply any results to an emotional state.
    ///
    /// Convenience method that combines `feed()` + `apply_to_mood()`.
    pub fn feed_and_apply(&mut self, chunk: &str, state: &mut EmotionalState) {
        let results = self.feed(chunk);
        for result in &results {
            self.apply_to_mood(state, result);
        }
    }

    /// Flush remaining buffered text (call at end of stream).
    ///
    /// Analyzes any remaining text that didn't end with sentence punctuation.
    pub fn flush(&mut self) -> Vec<SentimentResult> {
        let trimmed_is_empty = self.buffer.trim().is_empty();
        if trimmed_is_empty {
            self.buffer.clear();
            return Vec::new();
        }
        let result = sentiment::analyze_with_config(self.buffer.trim(), &self.config);
        self.buffer.clear();
        self.results.push(result.clone());
        vec![result]
    }

    /// Apply a sentiment result to an emotional state using this monitor's scale.
    pub fn apply_to_mood(&self, state: &mut EmotionalState, result: &SentimentResult) {
        for &(emotion, intensity) in &result.emotions {
            state.stimulate(emotion, intensity * self.scale);
        }
    }

    /// All results collected so far (including from `feed` and `flush`).
    #[must_use]
    pub fn results(&self) -> &[SentimentResult] {
        &self.results
    }

    /// Number of sentences analyzed so far.
    #[must_use]
    pub fn sentence_count(&self) -> usize {
        self.results.len()
    }

    /// Average valence across all analyzed sentences.
    #[must_use]
    pub fn average_valence(&self) -> f32 {
        if self.results.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.results.iter().map(|r| r.valence).sum();
        sum / self.results.len() as f32
    }

    /// Overall sentiment summary.
    #[must_use]
    pub fn summary(&self) -> MonitorSummary {
        let count = self.results.len();
        let positive = self.results.iter().filter(|r| r.is_positive()).count();
        let negative = self.results.iter().filter(|r| r.is_negative()).count();
        let neutral = count - positive - negative;
        MonitorSummary {
            sentence_count: count,
            positive_count: positive,
            negative_count: negative,
            neutral_count: neutral,
            average_valence: self.average_valence(),
        }
    }

    /// Reset the monitor, clearing buffer and results.
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.results.clear();
    }

    /// Drain completed sentences from the buffer.
    fn drain_sentences(&mut self) -> Vec<SentimentResult> {
        let mut results = Vec::new();
        loop {
            let boundary = self.buffer.find(['.', '!', '?']);
            match boundary {
                Some(pos) => {
                    let sentence: String = self.buffer.drain(..=pos).collect();
                    let trimmed = sentence.trim();
                    if !trimmed.is_empty() {
                        let result = sentiment::analyze_with_config(trimmed, &self.config);
                        self.results.push(result.clone());
                        results.push(result);
                    }
                }
                None => break,
            }
        }
        results
    }
}

/// Summary of monitored sentiment across a stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorSummary {
    pub sentence_count: usize,
    pub positive_count: usize,
    pub negative_count: usize,
    pub neutral_count: usize,
    pub average_valence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_new() {
        let m = SentimentMonitor::new(0.5);
        assert_eq!(m.sentence_count(), 0);
        assert!(m.results().is_empty());
    }

    #[test]
    fn test_feed_single_sentence() {
        let mut m = SentimentMonitor::new(1.0);
        let results = m.feed("This is great!");
        assert_eq!(results.len(), 1);
        assert!(results[0].is_positive());
        assert_eq!(m.sentence_count(), 1);
    }

    #[test]
    fn test_feed_partial_then_complete() {
        let mut m = SentimentMonitor::new(1.0);
        assert!(m.feed("This is ").is_empty());
        assert!(m.feed("wonderful ").is_empty());
        let results = m.feed("work!");
        assert_eq!(results.len(), 1);
        assert!(results[0].is_positive());
    }

    #[test]
    fn test_feed_multiple_sentences() {
        let mut m = SentimentMonitor::new(1.0);
        let results = m.feed("Great work! Terrible result.");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_flush_remaining() {
        let mut m = SentimentMonitor::new(1.0);
        m.feed("This is great but no period");
        let results = m.flush();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_flush_empty() {
        let mut m = SentimentMonitor::new(1.0);
        assert!(m.flush().is_empty());
    }

    #[test]
    fn test_feed_and_apply() {
        let mut m = SentimentMonitor::new(1.0);
        let mut state = EmotionalState::new();
        m.feed_and_apply("This is wonderful!", &mut state);
        assert!(state.mood.joy > 0.0);
    }

    #[test]
    fn test_apply_to_mood_scaled() {
        let m = SentimentMonitor::new(0.0); // zero scale = no effect
        let mut state = EmotionalState::new();
        let result = sentiment::analyze("Amazing wonderful fantastic!");
        m.apply_to_mood(&mut state, &result);
        assert!(state.deviation() < f32::EPSILON);
    }

    #[test]
    fn test_average_valence() {
        let mut m = SentimentMonitor::new(1.0);
        m.feed("Great! Terrible.");
        let avg = m.average_valence();
        // One positive, one negative — average should be near zero
        assert!(avg.abs() < 0.5);
    }

    #[test]
    fn test_average_valence_empty() {
        let m = SentimentMonitor::new(1.0);
        assert!(m.average_valence().abs() < f32::EPSILON);
    }

    #[test]
    fn test_summary() {
        let mut m = SentimentMonitor::new(1.0);
        m.feed("Amazing! Terrible! Whatever.");
        let s = m.summary();
        assert_eq!(s.sentence_count, 3);
        assert!(s.positive_count >= 1);
        assert!(s.negative_count >= 1);
    }

    #[test]
    fn test_reset() {
        let mut m = SentimentMonitor::new(1.0);
        m.feed("Great work!");
        assert_eq!(m.sentence_count(), 1);
        m.reset();
        assert_eq!(m.sentence_count(), 0);
        assert!(m.results().is_empty());
    }

    #[test]
    fn test_with_config() {
        let mut config = SentimentConfig::default();
        config.extra_positive.push("rad".to_string());
        let mut m = SentimentMonitor::with_config(1.0, config);
        let results = m.feed("This is rad!");
        assert_eq!(results.len(), 1);
        assert!(results[0].is_positive());
    }

    #[test]
    fn test_summary_serde() {
        let mut m = SentimentMonitor::new(1.0);
        m.feed("Great!");
        let s = m.summary();
        let json = serde_json::to_string(&s).unwrap();
        let s2: MonitorSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(s2.sentence_count, s.sentence_count);
    }

    #[test]
    fn test_streaming_simulation() {
        let mut m = SentimentMonitor::new(0.5);
        let mut state = EmotionalState::new();

        // Simulate token-by-token streaming
        let tokens = [
            "I ",
            "love ",
            "this ",
            "project! ",
            "But ",
            "the ",
            "bugs ",
            "are ",
            "terrible.",
        ];
        for token in &tokens {
            m.feed_and_apply(token, &mut state);
        }
        let remaining = m.flush();
        for r in &remaining {
            m.apply_to_mood(&mut state, r);
        }

        assert_eq!(m.sentence_count(), 2);
        // Mixed sentiment — mood should reflect both
        assert!(state.deviation() > 0.0);
    }
}
