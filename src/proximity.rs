//! Spatial proximity triggers — location-based mood effects.
//!
//! Maps spatial proximity to emotional responses. When an entity enters
//! the radius of a location with an associated mood trigger, the trigger
//! fires with intensity modulated by distance and falloff function.
//!
//! Useful for game NPCs: a graveyard might lower joy, a tavern might raise
//! trust and arousal, a battlefield might spike frustration and arousal.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::mood::MoodTrigger;

/// Distance-to-intensity falloff function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Falloff {
    /// Full intensity within radius, zero outside. Binary on/off.
    Step,
    /// Linear ramp from full intensity at center to zero at radius edge.
    Linear,
    /// Exponential decay — intensity drops rapidly with distance.
    Exponential,
}

impl Falloff {
    /// Compute the intensity multiplier for a given distance and radius.
    ///
    /// Returns 0.0–1.0 where 1.0 = at center, 0.0 = at or beyond radius.
    #[must_use]
    #[inline]
    pub fn intensity(&self, distance: f32, radius: f32) -> f32 {
        if radius <= 0.0 || distance >= radius {
            return 0.0;
        }
        if distance <= 0.0 {
            return 1.0;
        }
        match self {
            Self::Step => 1.0,
            Self::Linear => 1.0 - distance / radius,
            Self::Exponential => {
                // e^(-3 * d/r) gives ~95% decay at the edge
                (-3.0 * distance / radius).exp()
            }
        }
    }
}

impl fmt::Display for Falloff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Step => "step",
            Self::Linear => "linear",
            Self::Exponential => "exponential",
        };
        f.write_str(s)
    }
}

/// A rule mapping a location to a mood trigger with distance falloff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityRule {
    /// Tag identifying the location (e.g., "graveyard", "tavern").
    pub location_tag: String,
    /// Effective radius of the trigger zone.
    pub radius: f32,
    /// The mood trigger to fire when within radius.
    pub trigger: MoodTrigger,
    /// How intensity decays with distance.
    pub falloff: Falloff,
}

/// A trigger evaluation result — a trigger and its distance-scaled intensity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityHit {
    /// The location tag that matched.
    pub location_tag: String,
    /// The mood trigger to apply.
    pub trigger: MoodTrigger,
    /// Intensity multiplier from distance falloff (0.0–1.0).
    pub intensity: f32,
}

/// System for evaluating proximity-based mood triggers.
///
/// Holds a set of rules and evaluates them against entity positions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProximitySystem {
    rules: Vec<ProximityRule>,
}

impl ProximitySystem {
    /// Create an empty proximity system.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a proximity rule.
    pub fn add_rule(&mut self, rule: ProximityRule) {
        self.rules.push(rule);
    }

    /// Number of registered rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Evaluate all rules for a given location and distance.
    ///
    /// Returns all rules matching the location tag where the entity is
    /// within radius, with intensity scaled by the falloff function.
    #[must_use]
    pub fn evaluate(&self, location_tag: &str, distance: f32) -> Vec<ProximityHit> {
        self.rules
            .iter()
            .filter(|r| r.location_tag == location_tag)
            .filter_map(|r| {
                let intensity = r.falloff.intensity(distance, r.radius);
                if intensity < f32::EPSILON {
                    return None;
                }
                Some(ProximityHit {
                    location_tag: r.location_tag.clone(),
                    trigger: r.trigger.clone(),
                    intensity,
                })
            })
            .collect()
    }

    /// Evaluate all rules against multiple nearby locations.
    ///
    /// Each entry in `locations` is `(tag, distance)`. Returns combined hits.
    #[must_use]
    pub fn evaluate_many(&self, locations: &[(&str, f32)]) -> Vec<ProximityHit> {
        locations
            .iter()
            .flat_map(|&(tag, dist)| self.evaluate(tag, dist))
            .collect()
    }

    /// Remove all rules for a given location tag.
    ///
    /// Returns the number of rules removed.
    pub fn remove_location(&mut self, location_tag: &str) -> usize {
        let before = self.rules.len();
        self.rules.retain(|r| r.location_tag != location_tag);
        before - self.rules.len()
    }
}

/// Create a proximity rule with builder-style convenience.
#[must_use]
pub fn rule(location_tag: impl Into<String>, radius: f32, trigger: MoodTrigger) -> ProximityRule {
    ProximityRule {
        location_tag: location_tag.into(),
        radius,
        trigger,
        falloff: Falloff::Linear,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mood::{Emotion, MoodTrigger};

    fn tavern_trigger() -> MoodTrigger {
        MoodTrigger::new("tavern_warmth")
            .respond(Emotion::Joy, 0.2)
            .respond(Emotion::Trust, 0.15)
    }

    fn graveyard_trigger() -> MoodTrigger {
        MoodTrigger::new("graveyard_dread")
            .respond(Emotion::Joy, -0.3)
            .respond(Emotion::Arousal, 0.1)
    }

    // ── Falloff ──

    #[test]
    fn test_step_inside() {
        assert!((Falloff::Step.intensity(5.0, 10.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_step_outside() {
        assert!(Falloff::Step.intensity(15.0, 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_linear_center() {
        assert!((Falloff::Linear.intensity(0.0, 10.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_linear_half() {
        assert!((Falloff::Linear.intensity(5.0, 10.0) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_linear_edge() {
        assert!(Falloff::Linear.intensity(10.0, 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_exponential_center() {
        assert!((Falloff::Exponential.intensity(0.0, 10.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_exponential_decays() {
        let mid = Falloff::Exponential.intensity(5.0, 10.0);
        assert!(mid > 0.0 && mid < 1.0, "mid-distance: {mid}");
    }

    #[test]
    fn test_exponential_edge_near_zero() {
        let edge = Falloff::Exponential.intensity(10.0, 10.0);
        assert!(edge.abs() < f32::EPSILON);
    }

    #[test]
    fn test_falloff_negative_distance() {
        // Negative distance treated as "at center" → full intensity
        assert!((Falloff::Linear.intensity(-5.0, 10.0) - 1.0).abs() < f32::EPSILON);
        assert!((Falloff::Step.intensity(-1.0, 10.0) - 1.0).abs() < f32::EPSILON);
        assert!((Falloff::Exponential.intensity(-1.0, 10.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_falloff_negative_radius() {
        // Negative radius → 0.0 (invalid zone)
        assert!(Falloff::Linear.intensity(0.0, -10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_serde_proximity_hit() {
        let hit = ProximityHit {
            location_tag: "tavern".into(),
            trigger: tavern_trigger(),
            intensity: 0.75,
        };
        let json = serde_json::to_string(&hit).unwrap();
        let hit2: ProximityHit = serde_json::from_str(&json).unwrap();
        assert_eq!(hit2.location_tag, "tavern");
        assert!((hit2.intensity - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn test_falloff_zero_radius() {
        assert!(Falloff::Linear.intensity(0.0, 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_falloff_display() {
        assert_eq!(Falloff::Step.to_string(), "step");
        assert_eq!(Falloff::Linear.to_string(), "linear");
        assert_eq!(Falloff::Exponential.to_string(), "exponential");
    }

    // ── ProximitySystem ──

    #[test]
    fn test_empty_system() {
        let sys = ProximitySystem::new();
        assert_eq!(sys.rule_count(), 0);
        let hits = sys.evaluate("tavern", 0.0);
        assert!(hits.is_empty());
    }

    #[test]
    fn test_hit_within_radius() {
        let mut sys = ProximitySystem::new();
        sys.add_rule(ProximityRule {
            location_tag: "tavern".into(),
            radius: 20.0,
            trigger: tavern_trigger(),
            falloff: Falloff::Linear,
        });
        let hits = sys.evaluate("tavern", 5.0);
        assert_eq!(hits.len(), 1);
        assert!((hits[0].intensity - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn test_miss_outside_radius() {
        let mut sys = ProximitySystem::new();
        sys.add_rule(ProximityRule {
            location_tag: "tavern".into(),
            radius: 20.0,
            trigger: tavern_trigger(),
            falloff: Falloff::Linear,
        });
        let hits = sys.evaluate("tavern", 25.0);
        assert!(hits.is_empty());
    }

    #[test]
    fn test_wrong_location_no_hit() {
        let mut sys = ProximitySystem::new();
        sys.add_rule(ProximityRule {
            location_tag: "tavern".into(),
            radius: 20.0,
            trigger: tavern_trigger(),
            falloff: Falloff::Linear,
        });
        let hits = sys.evaluate("graveyard", 5.0);
        assert!(hits.is_empty());
    }

    #[test]
    fn test_evaluate_many() {
        let mut sys = ProximitySystem::new();
        sys.add_rule(ProximityRule {
            location_tag: "tavern".into(),
            radius: 20.0,
            trigger: tavern_trigger(),
            falloff: Falloff::Linear,
        });
        sys.add_rule(ProximityRule {
            location_tag: "graveyard".into(),
            radius: 30.0,
            trigger: graveyard_trigger(),
            falloff: Falloff::Step,
        });
        let hits = sys.evaluate_many(&[("tavern", 10.0), ("graveyard", 15.0)]);
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn test_remove_location() {
        let mut sys = ProximitySystem::new();
        sys.add_rule(rule("tavern", 20.0, tavern_trigger()));
        sys.add_rule(rule("graveyard", 30.0, graveyard_trigger()));
        assert_eq!(sys.rule_count(), 2);
        let removed = sys.remove_location("tavern");
        assert_eq!(removed, 1);
        assert_eq!(sys.rule_count(), 1);
    }

    #[test]
    fn test_rule_builder() {
        let r = rule("market", 15.0, tavern_trigger());
        assert_eq!(r.location_tag, "market");
        assert!((r.radius - 15.0).abs() < f32::EPSILON);
        assert!(matches!(r.falloff, Falloff::Linear));
    }

    #[test]
    fn test_serde_system() {
        let mut sys = ProximitySystem::new();
        sys.add_rule(rule("tavern", 20.0, tavern_trigger()));
        let json = serde_json::to_string(&sys).unwrap();
        let sys2: ProximitySystem = serde_json::from_str(&json).unwrap();
        assert_eq!(sys2.rule_count(), 1);
    }

    #[test]
    fn test_serde_falloff() {
        let f = Falloff::Exponential;
        let json = serde_json::to_string(&f).unwrap();
        let f2: Falloff = serde_json::from_str(&json).unwrap();
        assert_eq!(f2, f);
    }
}
