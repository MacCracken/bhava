//! Active hours — time-of-day personality activation scheduling.
//!
//! Controls when an entity is "active" and at what intensity. Models
//! work schedules, sleep cycles, and availability patterns. The activation
//! level can modulate personality intensity, response likelihood, and
//! behavioral thresholds.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A time window with an activation level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWindow {
    /// Start hour (0–23, inclusive).
    pub start_hour: u8,
    /// End hour (0–23, exclusive). If end < start, wraps past midnight.
    pub end_hour: u8,
    /// Activation level during this window: 0.0 (dormant) to 1.0 (fully active).
    pub activation: f32,
}

impl ActiveWindow {
    /// Create a new window (hours clamped to 0–23, activation to 0.0–1.0).
    #[must_use]
    pub fn new(start_hour: u8, end_hour: u8, activation: f32) -> Self {
        Self {
            start_hour: start_hour.min(23),
            end_hour: end_hour.min(23),
            activation: activation.clamp(0.0, 1.0),
        }
    }

    /// Whether a given hour falls within this window.
    #[must_use]
    #[inline]
    pub fn contains_hour(&self, hour: u8) -> bool {
        if self.start_hour <= self.end_hour {
            // Normal range: e.g., 9–17
            hour >= self.start_hour && hour < self.end_hour
        } else {
            // Wraps midnight: e.g., 22–6
            hour >= self.start_hour || hour < self.end_hour
        }
    }
}

/// Schedule of active windows with timezone offset.
///
/// When evaluating activation, the UTC time is adjusted by
/// `timezone_offset_secs` to get the entity's local time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveHoursSchedule {
    /// Time windows with activation levels.
    pub windows: Vec<ActiveWindow>,
    /// Timezone offset from UTC in seconds (e.g., -18000 for EST).
    pub timezone_offset_secs: i32,
    /// Default activation when no window matches. Default: 0.0.
    pub default_activation: f32,
}

impl Default for ActiveHoursSchedule {
    fn default() -> Self {
        Self {
            windows: Vec::new(),
            timezone_offset_secs: 0,
            default_activation: 0.0,
        }
    }
}

impl ActiveHoursSchedule {
    /// Create an empty schedule (always at default activation).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a time window.
    pub fn add_window(&mut self, window: ActiveWindow) {
        self.windows.push(window);
    }

    /// Get the local hour for a UTC time, adjusted by timezone offset.
    #[must_use]
    fn local_hour(&self, now: DateTime<Utc>) -> u8 {
        let utc_secs = now.timestamp();
        let local_secs = utc_secs + self.timezone_offset_secs as i64;
        // Convert to hour of day (0-23)
        let hour = ((local_secs % 86400 + 86400) % 86400) / 3600;
        hour as u8
    }

    /// Get the activation level at the given time.
    ///
    /// If multiple windows match, returns the highest activation.
    /// If no window matches, returns `default_activation`.
    #[must_use]
    pub fn activation_at(&self, now: DateTime<Utc>) -> f32 {
        let hour = self.local_hour(now);
        self.windows
            .iter()
            .filter(|w| w.contains_hour(hour))
            .map(|w| w.activation)
            .fold(self.default_activation, f32::max)
    }

    /// Whether the entity is considered active (activation > 0.5).
    #[must_use]
    pub fn is_active(&self, now: DateTime<Utc>) -> bool {
        self.activation_at(now) > 0.5
    }

    /// Whether the entity is dormant (activation < 0.1).
    #[must_use]
    pub fn is_dormant(&self, now: DateTime<Utc>) -> bool {
        self.activation_at(now) < 0.1
    }

    /// Number of configured windows.
    #[must_use]
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }
}

impl fmt::Display for ActiveHoursSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.windows.is_empty() {
            return f.write_str("no schedule (always default)");
        }
        for (i, w) in self.windows.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(
                f,
                "{:02}:00–{:02}:00 ({:.0}%)",
                w.start_hour,
                w.end_hour,
                w.activation * 100.0
            )?;
        }
        Ok(())
    }
}

// ─── Factory Functions ──────────────────────────────────────────────────────

/// Standard 9-to-5 workday schedule. Fully active during work hours,
/// dormant outside.
#[must_use]
pub fn default_schedule() -> ActiveHoursSchedule {
    ActiveHoursSchedule {
        windows: vec![ActiveWindow::new(9, 17, 1.0)],
        timezone_offset_secs: 0,
        default_activation: 0.0,
    }
}

/// Night owl schedule — active from 14:00 to 02:00.
#[must_use]
pub fn night_owl_schedule() -> ActiveHoursSchedule {
    ActiveHoursSchedule {
        windows: vec![ActiveWindow::new(14, 2, 1.0)],
        timezone_offset_secs: 0,
        default_activation: 0.0,
    }
}

/// Early bird schedule — active from 05:00 to 14:00.
#[must_use]
pub fn early_bird_schedule() -> ActiveHoursSchedule {
    ActiveHoursSchedule {
        windows: vec![ActiveWindow::new(5, 14, 1.0)],
        timezone_offset_secs: 0,
        default_activation: 0.0,
    }
}

/// Always-on schedule — fully active 24/7.
#[must_use]
pub fn always_on() -> ActiveHoursSchedule {
    ActiveHoursSchedule {
        windows: Vec::new(),
        timezone_offset_secs: 0,
        default_activation: 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn utc_at_hour(hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 6, 15, hour, 30, 0).unwrap()
    }

    // ── ActiveWindow ──

    #[test]
    fn test_window_normal_range() {
        let w = ActiveWindow::new(9, 17, 1.0);
        assert!(w.contains_hour(9));
        assert!(w.contains_hour(12));
        assert!(w.contains_hour(16));
        assert!(!w.contains_hour(17));
        assert!(!w.contains_hour(8));
    }

    #[test]
    fn test_window_wraps_midnight() {
        let w = ActiveWindow::new(22, 6, 1.0);
        assert!(w.contains_hour(22));
        assert!(w.contains_hour(23));
        assert!(w.contains_hour(0));
        assert!(w.contains_hour(5));
        assert!(!w.contains_hour(6));
        assert!(!w.contains_hour(12));
    }

    #[test]
    fn test_window_clamps() {
        let w = ActiveWindow::new(25, 30, 2.0);
        assert_eq!(w.start_hour, 23);
        assert_eq!(w.end_hour, 23);
        assert!((w.activation - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_window_same_start_end() {
        // Zero-width window: start == end → never active
        let w = ActiveWindow::new(12, 12, 1.0);
        for h in 0..24 {
            assert!(
                !w.contains_hour(h),
                "hour {h} should not match zero-width window"
            );
        }
    }

    // ── Schedule ──

    #[test]
    fn test_default_schedule() {
        let s = default_schedule();
        assert!(s.is_active(utc_at_hour(12)));
        assert!(!s.is_active(utc_at_hour(3)));
    }

    #[test]
    fn test_night_owl() {
        let s = night_owl_schedule();
        assert!(s.is_active(utc_at_hour(20)));
        assert!(s.is_active(utc_at_hour(0)));
        assert!(!s.is_active(utc_at_hour(8)));
    }

    #[test]
    fn test_early_bird() {
        let s = early_bird_schedule();
        assert!(s.is_active(utc_at_hour(6)));
        assert!(!s.is_active(utc_at_hour(20)));
    }

    #[test]
    fn test_always_on() {
        let s = always_on();
        for hour in 0..24 {
            assert!(
                s.is_active(utc_at_hour(hour)),
                "should be active at hour {hour}"
            );
        }
    }

    #[test]
    fn test_empty_schedule_dormant() {
        let s = ActiveHoursSchedule::new();
        assert!(s.is_dormant(utc_at_hour(12)));
    }

    #[test]
    fn test_timezone_offset() {
        let mut s = default_schedule(); // 9-17 local
        s.timezone_offset_secs = -5 * 3600; // EST = UTC-5
        // At UTC 14:00, local time is 9:00 → should be active
        assert!(s.is_active(utc_at_hour(14)));
        // At UTC 12:00, local time is 7:00 → should be dormant
        assert!(!s.is_active(utc_at_hour(12)));
    }

    #[test]
    fn test_multiple_windows_max() {
        let mut s = ActiveHoursSchedule::new();
        s.add_window(ActiveWindow::new(9, 17, 0.5));
        s.add_window(ActiveWindow::new(12, 14, 1.0));
        // At 13:00, both match — should return max (1.0)
        let a = s.activation_at(utc_at_hour(13));
        assert!((a - 1.0).abs() < f32::EPSILON);
        // At 10:00, only first matches — should return 0.5
        let a2 = s.activation_at(utc_at_hour(10));
        assert!((a2 - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_display() {
        let s = default_schedule();
        let text = s.to_string();
        assert!(text.contains("09:00"), "display: {text}");
        assert!(text.contains("17:00"), "display: {text}");
    }

    #[test]
    fn test_display_empty() {
        let s = ActiveHoursSchedule::new();
        let text = s.to_string();
        assert!(text.contains("no schedule"));
    }

    #[test]
    fn test_window_count() {
        let mut s = ActiveHoursSchedule::new();
        assert_eq!(s.window_count(), 0);
        s.add_window(ActiveWindow::new(9, 17, 1.0));
        assert_eq!(s.window_count(), 1);
    }

    #[test]
    fn test_serde_schedule() {
        let s = default_schedule();
        let json = serde_json::to_string(&s).unwrap();
        let s2: ActiveHoursSchedule = serde_json::from_str(&json).unwrap();
        assert_eq!(s2.window_count(), s.window_count());
    }

    #[test]
    fn test_serde_window() {
        let w = ActiveWindow::new(9, 17, 0.8);
        let json = serde_json::to_string(&w).unwrap();
        let w2: ActiveWindow = serde_json::from_str(&json).unwrap();
        assert_eq!(w2.start_hour, w.start_hour);
    }
}
