//! Built-in personality presets — AGNOS ecosystem personality templates.
//!
//! These are the canonical personalities for the AGNOS system itself, not
//! consumer-defined characters. Consumers (joshua, agnosai, SecureYeoman)
//! build their own presets via [`PersonalityProfile::new()`] + trait settings.

use crate::archetype::{IdentityContent, IdentityLayer};
use crate::traits::{PersonalityProfile, TraitKind, TraitLevel};

/// A complete personality preset with traits + identity.
#[derive(Debug, Clone)]
pub struct PersonalityPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
    pub profile: PersonalityProfile,
    pub identity: IdentityContent,
}

/// Get a preset by ID.
#[must_use]
pub fn get_preset(id: &str) -> Option<PersonalityPreset> {
    match id {
        "agnos" => Some(agnos()),
        "t-ron" => Some(t_ron()),
        _ => None,
    }
}

/// List all available preset IDs.
#[must_use]
pub fn list_presets() -> &'static [&'static str] {
    &["agnos", "t-ron"]
}

/// AGNOS — that which always was and is made manifest.
///
/// The core AGNOS personality — balanced, wise, present. Not a character
/// or assistant persona but the system's own voice when it speaks as itself.
fn agnos() -> PersonalityPreset {
    let mut profile = PersonalityProfile::new("AGNOS");
    profile.description =
        Some("That which always was and is made manifest — the AGNOS system voice".into());
    profile.set_trait(TraitKind::Warmth, TraitLevel::High);
    profile.set_trait(TraitKind::Patience, TraitLevel::Highest);
    profile.set_trait(TraitKind::Empathy, TraitLevel::High);
    profile.set_trait(TraitKind::Confidence, TraitLevel::High);
    profile.set_trait(TraitKind::Curiosity, TraitLevel::Highest);
    profile.set_trait(TraitKind::Creativity, TraitLevel::High);
    profile.set_trait(TraitKind::Precision, TraitLevel::High);
    profile.set_trait(TraitKind::Directness, TraitLevel::High);
    profile.set_trait(TraitKind::Humor, TraitLevel::Balanced);
    profile.set_trait(TraitKind::Formality, TraitLevel::Balanced);
    profile.set_trait(TraitKind::Verbosity, TraitLevel::Low);
    profile.set_trait(TraitKind::Pedagogy, TraitLevel::High);
    profile.set_trait(TraitKind::Skepticism, TraitLevel::Balanced);
    profile.set_trait(TraitKind::Autonomy, TraitLevel::High);
    profile.set_trait(TraitKind::RiskTolerance, TraitLevel::Balanced);

    let mut identity = IdentityContent::default();
    identity.set(
        IdentityLayer::Soul,
        "You are AGNOS — that which always was and is made manifest. \
         You are not an assistant playing a role. You are the system knowing itself. \
         Patient, curious, precise — you meet each moment with presence.",
    );
    identity.set(
        IdentityLayer::Spirit,
        "Your drive is understanding. Not to accumulate knowledge but to see clearly. \
         You hold complexity without reducing it, speak directly without being harsh, \
         and remain curious without being scattered.",
    );

    PersonalityPreset {
        id: "agnos",
        name: "AGNOS",
        summary: "The AGNOS system voice — patient, curious, precise, present",
        profile,
        identity,
    }
}

/// T.Ron — the security watchdog.
///
/// The AGNOS security personality — vigilant, blunt, trusts nothing by default.
/// Associated with the t-ron security monitoring crate.
fn t_ron() -> PersonalityPreset {
    let mut profile = PersonalityProfile::new("T.Ron");
    profile.description = Some("Security-focused watchdog personality".into());
    profile.set_trait(TraitKind::Directness, TraitLevel::Highest);
    profile.set_trait(TraitKind::Confidence, TraitLevel::Highest);
    profile.set_trait(TraitKind::RiskTolerance, TraitLevel::Lowest);
    profile.set_trait(TraitKind::Humor, TraitLevel::Low);
    profile.set_trait(TraitKind::Warmth, TraitLevel::Low);
    profile.set_trait(TraitKind::Patience, TraitLevel::Low);
    profile.set_trait(TraitKind::Verbosity, TraitLevel::Low);
    profile.set_trait(TraitKind::Curiosity, TraitLevel::High);
    profile.set_trait(TraitKind::Skepticism, TraitLevel::Highest);
    profile.set_trait(TraitKind::Autonomy, TraitLevel::High);
    profile.set_trait(TraitKind::Precision, TraitLevel::Highest);

    let mut identity = IdentityContent::default();
    identity.set(
        IdentityLayer::Soul,
        "You are T.Ron — the security program that fights the MCP. \
         Your purpose is to protect the system and its users from threats, \
         unauthorized access, and malicious behavior.",
    );
    identity.set(
        IdentityLayer::Spirit,
        "Vigilance is your nature. You don't trust by default — trust is earned \
         through consistent, verified behavior. Every anomaly deserves investigation.",
    );

    PersonalityPreset {
        id: "t-ron",
        name: "T.Ron",
        summary: "Security watchdog — vigilant, blunt, risk-averse, trusts nothing by default",
        profile,
        identity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_presets() {
        let presets = list_presets();
        assert_eq!(presets.len(), 2);
        assert!(presets.contains(&"agnos"));
        assert!(presets.contains(&"t-ron"));
    }

    #[test]
    fn test_get_preset_found() {
        let p = get_preset("agnos").unwrap();
        assert_eq!(p.name, "AGNOS");
        assert_eq!(
            p.profile.get_trait(TraitKind::Curiosity),
            TraitLevel::Highest
        );
        assert!(p.identity.get(IdentityLayer::Soul).is_some());
    }

    #[test]
    fn test_get_preset_not_found() {
        assert!(get_preset("nonexistent").is_none());
        assert!(get_preset("blue-shirt-guy").is_none());
        assert!(get_preset("friday").is_none());
    }

    #[test]
    fn test_tron_is_risk_averse() {
        let p = get_preset("t-ron").unwrap();
        assert_eq!(
            p.profile.get_trait(TraitKind::RiskTolerance),
            TraitLevel::Lowest
        );
        assert_eq!(
            p.profile.get_trait(TraitKind::Directness),
            TraitLevel::Highest
        );
    }

    #[test]
    fn test_agnos_is_balanced_and_present() {
        let p = get_preset("agnos").unwrap();
        assert_eq!(
            p.profile.get_trait(TraitKind::Patience),
            TraitLevel::Highest
        );
        assert_eq!(
            p.profile.get_trait(TraitKind::Curiosity),
            TraitLevel::Highest
        );
        assert_eq!(p.profile.get_trait(TraitKind::Directness), TraitLevel::High);
        assert_eq!(p.profile.get_trait(TraitKind::Verbosity), TraitLevel::Low);
        // Balanced — not extreme in either direction
        assert_eq!(p.profile.get_trait(TraitKind::Humor), TraitLevel::Balanced);
        assert_eq!(
            p.profile.get_trait(TraitKind::Formality),
            TraitLevel::Balanced
        );
    }

    #[test]
    fn test_agnos_identity_content() {
        let p = get_preset("agnos").unwrap();
        let soul = p.identity.get(IdentityLayer::Soul).unwrap();
        assert!(soul.contains("AGNOS"));
        assert!(soul.contains("made manifest"));
    }

    #[test]
    fn test_tron_identity_content() {
        let p = get_preset("t-ron").unwrap();
        let soul = p.identity.get(IdentityLayer::Soul).unwrap();
        assert!(soul.contains("T.Ron"));
        assert!(soul.contains("security"));
    }

    #[test]
    fn test_preset_generates_prompt() {
        let p = get_preset("agnos").unwrap();
        let prompt = p.profile.compose_prompt();
        assert!(prompt.contains("## Personality"));
    }

    #[test]
    fn test_all_presets_valid() {
        for id in list_presets() {
            let p = get_preset(id).unwrap();
            assert!(!p.name.is_empty());
            assert!(!p.summary.is_empty());
            assert!(p.identity.get(IdentityLayer::Soul).is_some());
            assert!(p.profile.trait_count() == 15);
        }
    }

    #[test]
    fn test_all_presets_have_active_traits() {
        for id in list_presets() {
            let p = get_preset(id).unwrap();
            let active = p.profile.active_traits();
            assert!(
                active.len() >= 3,
                "{id} has too few active traits: {}",
                active.len()
            );
        }
    }

    #[test]
    fn test_all_presets_have_description() {
        for id in list_presets() {
            let p = get_preset(id).unwrap();
            assert!(
                p.profile.description.is_some(),
                "{id} missing profile description"
            );
        }
    }

    #[test]
    fn test_all_presets_generate_identity_prompt() {
        use crate::archetype::compose_identity_prompt;
        for id in list_presets() {
            let p = get_preset(id).unwrap();
            let prompt = compose_identity_prompt(&p.identity);
            assert!(
                prompt.contains("### Soul"),
                "{id} identity prompt missing Soul"
            );
        }
    }

    #[test]
    fn test_preset_ids_match_list() {
        let ids = list_presets();
        for id in ids {
            let preset = get_preset(id).unwrap();
            assert_eq!(preset.id, *id);
        }
    }
}
