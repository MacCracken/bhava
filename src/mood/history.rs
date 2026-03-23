use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use super::core::{MoodState, MoodVector};

// --- Mood History (v0.3) ---

/// A point-in-time snapshot of mood state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodSnapshot {
    pub mood: MoodVector,
    pub state: MoodState,
    pub deviation: f32,
    pub timestamp: DateTime<Utc>,
}

/// Ring buffer of mood snapshots for trend analysis.
///
/// Uses `VecDeque` for O(1) insertion and eviction at capacity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodHistory {
    snapshots: VecDeque<MoodSnapshot>,
    capacity: usize,
}

impl MoodHistory {
    /// Create a history buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            snapshots: VecDeque::with_capacity(cap.min(1024)),
            capacity: cap,
        }
    }

    /// Record a snapshot. Drops the oldest if at capacity.
    pub fn record(&mut self, snapshot: MoodSnapshot) {
        if self.snapshots.len() >= self.capacity {
            self.snapshots.pop_front();
        }
        self.snapshots.push_back(snapshot);
    }

    /// All snapshots as a slice pair (VecDeque may be non-contiguous).
    /// Use `iter()` for iteration.
    pub fn snapshots(&self) -> (&[MoodSnapshot], &[MoodSnapshot]) {
        self.snapshots.as_slices()
    }

    /// Iterator over snapshots, oldest first.
    pub fn iter(&self) -> impl Iterator<Item = &MoodSnapshot> {
        self.snapshots.iter()
    }

    /// Number of recorded snapshots.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    /// Whether the history is empty.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }

    /// Average deviation across all snapshots.
    pub fn average_deviation(&self) -> f32 {
        if self.snapshots.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.snapshots.iter().map(|s| s.deviation).sum();
        sum / self.snapshots.len() as f32
    }

    /// Most recent mood state, if any.
    pub fn latest_state(&self) -> Option<MoodState> {
        self.snapshots.back().map(|s| s.state)
    }

    /// Count occurrences of each mood state in history.
    pub fn state_distribution(&self) -> Vec<(MoodState, usize)> {
        use std::collections::HashMap;
        let mut counts: HashMap<MoodState, usize> = HashMap::new();
        for snap in &self.snapshots {
            *counts.entry(snap.state).or_insert(0) += 1;
        }
        let mut dist: Vec<_> = counts.into_iter().collect();
        dist.sort_by(|a, b| b.1.cmp(&a.1));
        dist
    }

    /// Trend: is deviation increasing or decreasing?
    /// Returns positive for escalating, negative for calming, near-zero for stable.
    pub fn deviation_trend(&self) -> f32 {
        if self.snapshots.len() < 2 {
            return 0.0;
        }
        let half = self.snapshots.len() / 2;
        let first_half: f32 = self.snapshots.iter().take(half).map(|s| s.deviation).sum();
        let second_half: f32 = self.snapshots.iter().skip(half).map(|s| s.deviation).sum();
        let first_avg = first_half / half as f32;
        let second_avg = second_half / (self.snapshots.len() - half) as f32;
        second_avg - first_avg
    }

    /// Emotional volatility — standard deviation of deviation across history.
    ///
    /// High volatility means the agent's emotional state swings wildly.
    /// Low volatility means stable, predictable emotional behavior.
    #[must_use]
    pub fn volatility(&self) -> f32 {
        if self.snapshots.len() < 2 {
            return 0.0;
        }
        let mean = self.average_deviation();
        let variance: f32 = self
            .snapshots
            .iter()
            .map(|s| (s.deviation - mean).powi(2))
            .sum::<f32>()
            / (self.snapshots.len() - 1) as f32;
        variance.sqrt()
    }

    /// Sentiment momentum — linear regression slope of deviation over time.
    ///
    /// Positive = escalating emotional intensity. Negative = calming down.
    /// More precise than `deviation_trend()` which only compares halves.
    #[must_use]
    pub fn momentum(&self) -> f32 {
        let n = self.snapshots.len();
        if n < 2 {
            return 0.0;
        }
        let nf = n as f32;
        let mut sum_x = 0.0f32;
        let mut sum_y = 0.0f32;
        let mut sum_xy = 0.0f32;
        let mut sum_xx = 0.0f32;
        for (i, s) in self.snapshots.iter().enumerate() {
            let x = i as f32;
            let y = s.deviation;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }
        let denom = nf * sum_xx - sum_x * sum_x;
        if denom.abs() < f32::EPSILON {
            return 0.0;
        }
        (nf * sum_xy - sum_x * sum_y) / denom
    }
}
