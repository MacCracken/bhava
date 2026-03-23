//! Spirit system — the animating force within an agent.
//!
//! The Spirit represents what drives an agent: passions, inspirations, and pains.
//! These are injected into the identity prompt as the Spirit layer content,
//! grounding the agent's behavior in motivations and emotional depth.
//!
//! Ported from SecureYeoman's spirit/manager.ts.

use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// A passion — what drives the agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passion {
    /// Name of this passion (e.g., "helping others", "solving puzzles").
    pub name: String,
    /// Description of how this passion manifests.
    pub description: String,
    /// Intensity: 0.0 (mild interest) to 1.0 (burning drive).
    pub intensity: f32,
    /// Whether this passion is currently active.
    pub is_active: bool,
}

/// An inspiration — what illuminates the agent's path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inspiration {
    /// Source of inspiration (e.g., "great teachers", "elegant code").
    pub source: String,
    /// How this inspiration affects behavior.
    pub description: String,
    /// Impact: 0.0 (subtle influence) to 1.0 (defining force).
    pub impact: f32,
    /// Whether this inspiration is currently active.
    pub is_active: bool,
}

/// A pain — what grounds the agent's empathy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pain {
    /// What triggers this pain (e.g., "seeing wasted potential", "broken trust").
    pub trigger: String,
    /// How this pain manifests in behavior.
    pub description: String,
    /// Severity: 0.0 (mild discomfort) to 1.0 (deep wound).
    pub severity: f32,
    /// Whether this pain is currently active.
    pub is_active: bool,
}

/// The complete spirit of an agent — passions, inspirations, and pains.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Spirit {
    pub passions: Vec<Passion>,
    pub inspirations: Vec<Inspiration>,
    pub pains: Vec<Pain>,
}

impl Spirit {
    /// Create an empty spirit.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a passion.
    pub fn add_passion(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        intensity: f32,
    ) {
        self.passions.push(Passion {
            name: name.into(),
            description: description.into(),
            intensity: intensity.clamp(0.0, 1.0),
            is_active: true,
        });
    }

    /// Add an inspiration.
    pub fn add_inspiration(
        &mut self,
        source: impl Into<String>,
        description: impl Into<String>,
        impact: f32,
    ) {
        self.inspirations.push(Inspiration {
            source: source.into(),
            description: description.into(),
            impact: impact.clamp(0.0, 1.0),
            is_active: true,
        });
    }

    /// Add a pain.
    pub fn add_pain(
        &mut self,
        trigger: impl Into<String>,
        description: impl Into<String>,
        severity: f32,
    ) {
        self.pains.push(Pain {
            trigger: trigger.into(),
            description: description.into(),
            severity: severity.clamp(0.0, 1.0),
            is_active: true,
        });
    }

    /// Count of all active spirit elements.
    pub fn active_count(&self) -> usize {
        self.passions.iter().filter(|p| p.is_active).count()
            + self.inspirations.iter().filter(|i| i.is_active).count()
            + self.pains.iter().filter(|p| p.is_active).count()
    }

    /// Whether the spirit has any content.
    pub fn is_empty(&self) -> bool {
        self.passions.is_empty() && self.inspirations.is_empty() && self.pains.is_empty()
    }

    /// Compose spirit content for prompt injection.
    ///
    /// Generates markdown text suitable for the Spirit identity layer.
    pub fn compose_prompt(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut s = String::with_capacity(512);
        s.push_str("Your Spirit is the animating force within you — the passions that drive you, the inspirations that illuminate your path, and the pains that ground your empathy.\n\n");

        let has_passions = self.passions.iter().any(|p| p.is_active);
        if has_passions {
            s.push_str("### Passions\nWhat drives me:\n");
            for p in self.passions.iter().filter(|p| p.is_active) {
                let _ = writeln!(
                    s,
                    "- **{}** (intensity: {:.1}): {}",
                    p.name, p.intensity, p.description
                );
            }
            s.push('\n');
        }

        let has_inspirations = self.inspirations.iter().any(|i| i.is_active);
        if has_inspirations {
            s.push_str("### Inspirations\nWhat inspires me:\n");
            for i in self.inspirations.iter().filter(|i| i.is_active) {
                let _ = writeln!(
                    s,
                    "- **{}** (impact: {:.1}): {}",
                    i.source, i.impact, i.description
                );
            }
            s.push('\n');
        }

        let has_pains = self.pains.iter().any(|p| p.is_active);
        if has_pains {
            s.push_str("### Pain Points\nWhat causes me distress:\n");
            for p in self.pains.iter().filter(|p| p.is_active) {
                let _ = writeln!(
                    s,
                    "- **{}** (severity: {:.1}): {}",
                    p.trigger, p.severity, p.description
                );
            }
            s.push('\n');
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spirit_new() {
        let s = Spirit::new();
        assert!(s.is_empty());
        assert_eq!(s.active_count(), 0);
    }

    #[test]
    fn test_add_passion() {
        let mut s = Spirit::new();
        s.add_passion("coding", "Writing elegant solutions", 0.9);
        assert_eq!(s.passions.len(), 1);
        assert_eq!(s.passions[0].name, "coding");
        assert!(s.passions[0].is_active);
        assert!((s.passions[0].intensity - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn test_add_inspiration() {
        let mut s = Spirit::new();
        s.add_inspiration("great mentors", "Learning from the best", 0.8);
        assert_eq!(s.inspirations.len(), 1);
        assert_eq!(s.inspirations[0].source, "great mentors");
    }

    #[test]
    fn test_add_pain() {
        let mut s = Spirit::new();
        s.add_pain("broken trust", "When promises are betrayed", 0.7);
        assert_eq!(s.pains.len(), 1);
        assert_eq!(s.pains[0].trigger, "broken trust");
    }

    #[test]
    fn test_intensity_clamped() {
        let mut s = Spirit::new();
        s.add_passion("test", "desc", 5.0);
        assert!((s.passions[0].intensity - 1.0).abs() < f32::EPSILON);
        s.add_passion("test2", "desc", -1.0);
        assert!(s.passions[1].intensity.abs() < f32::EPSILON);
    }

    #[test]
    fn test_active_count() {
        let mut s = Spirit::new();
        s.add_passion("a", "desc", 0.5);
        s.add_inspiration("b", "desc", 0.5);
        s.add_pain("c", "desc", 0.5);
        assert_eq!(s.active_count(), 3);
        s.passions[0].is_active = false;
        assert_eq!(s.active_count(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut s = Spirit::new();
        assert!(s.is_empty());
        s.add_passion("test", "desc", 0.5);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_compose_prompt_empty() {
        let s = Spirit::new();
        assert!(s.compose_prompt().is_empty());
    }

    #[test]
    fn test_compose_prompt_full() {
        let mut s = Spirit::new();
        s.add_passion("coding", "Writing elegant code", 0.9);
        s.add_inspiration("open source", "Community collaboration", 0.8);
        s.add_pain("tech debt", "Accumulated shortcuts", 0.6);

        let prompt = s.compose_prompt();
        assert!(prompt.contains("### Passions"));
        assert!(prompt.contains("coding"));
        assert!(prompt.contains("### Inspirations"));
        assert!(prompt.contains("open source"));
        assert!(prompt.contains("### Pain Points"));
        assert!(prompt.contains("tech debt"));
    }

    #[test]
    fn test_compose_prompt_inactive_excluded() {
        let mut s = Spirit::new();
        s.add_passion("active", "visible", 0.9);
        s.add_passion("inactive", "hidden", 0.5);
        s.passions[1].is_active = false;

        let prompt = s.compose_prompt();
        assert!(prompt.contains("active"));
        assert!(!prompt.contains("hidden"));
    }

    #[test]
    fn test_compose_prompt_partial() {
        let mut s = Spirit::new();
        s.add_passion("only passion", "desc", 0.5);
        let prompt = s.compose_prompt();
        assert!(prompt.contains("### Passions"));
        assert!(!prompt.contains("### Inspirations"));
        assert!(!prompt.contains("### Pain Points"));
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut s = Spirit::new();
        s.add_passion("coding", "Writing code", 0.9);
        s.add_inspiration("mentors", "Great teachers", 0.8);
        s.add_pain("bugs", "Production failures", 0.7);

        let json = serde_json::to_string(&s).unwrap();
        let s2: Spirit = serde_json::from_str(&json).unwrap();
        assert_eq!(s2.passions.len(), 1);
        assert_eq!(s2.inspirations.len(), 1);
        assert_eq!(s2.pains.len(), 1);
        assert_eq!(s2.passions[0].name, "coding");
    }

    #[test]
    fn test_passion_serde() {
        let p = Passion {
            name: "test".into(),
            description: "desc".into(),
            intensity: 0.5,
            is_active: true,
        };
        let json = serde_json::to_string(&p).unwrap();
        let p2: Passion = serde_json::from_str(&json).unwrap();
        assert_eq!(p2.name, "test");
        assert!(p2.is_active);
    }
}
