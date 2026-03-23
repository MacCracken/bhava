//! Personality trait spectrums — behavioral dimensions with graduated levels.
//!
//! Each trait is a spectrum from one extreme to another (e.g. humor: deadpan → comedic).
//! Traits map to behavioral instructions that guide LLM system prompts.
//! Derived from SecureYeoman's soul/trait-descriptions system.

mod descriptions;
mod kind;
mod ocean;
mod profile;

pub use descriptions::{trait_behavior, trait_level_name};
pub use kind::{TraitGroup, TraitKind, TraitLevel, TraitValue};
pub use ocean::{OceanScores, personality_entropy, personality_extremity, profile_from_ocean};
pub use profile::PersonalityProfile;

#[cfg(test)]
mod tests {
    use super::profile::{parse_trait_kind, parse_trait_level};
    use super::*;

    #[test]
    fn test_trait_kind_all() {
        assert_eq!(TraitKind::ALL.len(), 15);
        assert_eq!(TraitKind::ALL.len(), TraitKind::COUNT);
    }

    #[test]
    fn test_trait_kind_index() {
        // Verify index matches ALL order
        for (i, &kind) in TraitKind::ALL.iter().enumerate() {
            assert_eq!(kind.index(), i, "{kind} has wrong index");
        }
    }

    #[test]
    fn test_trait_kind_index_unique() {
        let mut seen = [false; TraitKind::COUNT];
        for &kind in TraitKind::ALL {
            assert!(!seen[kind.index()], "{kind} has duplicate index");
            seen[kind.index()] = true;
        }
    }

    #[test]
    fn test_trait_kind_display() {
        assert_eq!(TraitKind::Formality.to_string(), "formality");
        assert_eq!(TraitKind::RiskTolerance.to_string(), "risk_tolerance");
        assert_eq!(TraitKind::Curiosity.to_string(), "curiosity");
    }

    #[test]
    fn test_trait_level_numeric() {
        assert_eq!(TraitLevel::Lowest.numeric(), -2);
        assert_eq!(TraitLevel::Balanced.numeric(), 0);
        assert_eq!(TraitLevel::Highest.numeric(), 2);
    }

    #[test]
    fn test_trait_level_normalized() {
        assert!((TraitLevel::Lowest.normalized() - (-1.0)).abs() < f32::EPSILON);
        assert!((TraitLevel::Balanced.normalized()).abs() < f32::EPSILON);
        assert!((TraitLevel::Highest.normalized() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_trait_level_from_numeric() {
        assert_eq!(TraitLevel::from_numeric(0).unwrap(), TraitLevel::Balanced);
        assert_eq!(TraitLevel::from_numeric(-2).unwrap(), TraitLevel::Lowest);
        assert!(TraitLevel::from_numeric(5).is_err());
    }

    #[test]
    fn test_trait_level_ordering() {
        assert!(TraitLevel::Lowest < TraitLevel::Low);
        assert!(TraitLevel::Low < TraitLevel::Balanced);
        assert!(TraitLevel::Balanced < TraitLevel::High);
        assert!(TraitLevel::High < TraitLevel::Highest);
    }

    #[test]
    fn test_trait_level_name() {
        assert_eq!(
            trait_level_name(TraitKind::Humor, TraitLevel::Lowest),
            "deadpan"
        );
        assert_eq!(
            trait_level_name(TraitKind::Humor, TraitLevel::Highest),
            "comedic"
        );
        assert_eq!(
            trait_level_name(TraitKind::Warmth, TraitLevel::High),
            "friendly"
        );
        assert_eq!(
            trait_level_name(TraitKind::Confidence, TraitLevel::Highest),
            "authoritative"
        );
    }

    #[test]
    fn test_trait_behavior_balanced_returns_none() {
        for &kind in TraitKind::ALL {
            assert!(trait_behavior(kind, TraitLevel::Balanced).is_none());
        }
    }

    #[test]
    fn test_trait_behavior_non_balanced() {
        let b = trait_behavior(TraitKind::Humor, TraitLevel::Highest).unwrap();
        assert!(b.contains("funny"));
    }

    #[test]
    fn test_personality_profile_new() {
        let p = PersonalityProfile::new("test");
        assert_eq!(p.name, "test");
        assert_eq!(p.trait_count(), 15);
        assert!(p.active_traits().is_empty());
    }

    #[test]
    fn test_personality_set_get() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Humor), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Warmth), TraitLevel::Balanced);
    }

    #[test]
    fn test_active_traits() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::High);
        p.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        let active = p.active_traits();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_behavioral_instructions() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Directness, TraitLevel::Highest);
        let instructions = p.behavioral_instructions();
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn test_compose_prompt_empty() {
        let p = PersonalityProfile::new("neutral");
        assert!(p.compose_prompt().is_empty());
    }

    #[test]
    fn test_compose_prompt_with_traits() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let prompt = p.compose_prompt();
        assert!(prompt.contains("## Personality"));
        assert!(prompt.contains("funny"));
    }

    #[test]
    fn test_distance_same() {
        let p = PersonalityProfile::new("a");
        assert!((p.distance(&p)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_distance_different() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        b.set_trait(TraitKind::Humor, TraitLevel::Highest);
        assert!(a.distance(&b) > 0.0);
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Warmth, TraitLevel::High);
        let json = serde_json::to_string(&p).unwrap();
        let p2: PersonalityProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(p2.get_trait(TraitKind::Warmth), TraitLevel::High);
    }

    #[test]
    fn test_trait_kind_default_level() {
        for &kind in TraitKind::ALL {
            assert_eq!(kind.default_level(), TraitLevel::Balanced);
        }
    }

    #[test]
    fn test_trait_kind_levels() {
        for &kind in TraitKind::ALL {
            let levels = kind.levels();
            assert_eq!(levels.len(), 5);
            assert_eq!(levels[0], TraitLevel::Lowest);
            assert_eq!(levels[4], TraitLevel::Highest);
        }
    }

    #[test]
    fn test_trait_kind_display_all() {
        let names: Vec<String> = TraitKind::ALL.iter().map(|k| k.to_string()).collect();
        assert!(names.contains(&"formality".to_string()));
        assert!(names.contains(&"humor".to_string()));
        assert!(names.contains(&"verbosity".to_string()));
        assert!(names.contains(&"directness".to_string()));
        assert!(names.contains(&"warmth".to_string()));
        assert!(names.contains(&"empathy".to_string()));
        assert!(names.contains(&"patience".to_string()));
        assert!(names.contains(&"confidence".to_string()));
        assert!(names.contains(&"creativity".to_string()));
        assert!(names.contains(&"risk_tolerance".to_string()));
        assert!(names.contains(&"curiosity".to_string()));
    }

    #[test]
    fn test_trait_level_display() {
        assert_eq!(TraitLevel::Lowest.to_string(), "lowest");
        assert_eq!(TraitLevel::Low.to_string(), "low");
        assert_eq!(TraitLevel::Balanced.to_string(), "balanced");
        assert_eq!(TraitLevel::High.to_string(), "high");
        assert_eq!(TraitLevel::Highest.to_string(), "highest");
    }

    #[test]
    fn test_trait_level_numeric_all() {
        assert_eq!(TraitLevel::Low.numeric(), -1);
        assert_eq!(TraitLevel::High.numeric(), 1);
    }

    #[test]
    fn test_trait_level_normalized_all() {
        assert!((TraitLevel::Low.normalized() - (-0.5)).abs() < f32::EPSILON);
        assert!((TraitLevel::High.normalized() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_trait_level_from_numeric_all_valid() {
        assert_eq!(TraitLevel::from_numeric(-1).unwrap(), TraitLevel::Low);
        assert_eq!(TraitLevel::from_numeric(1).unwrap(), TraitLevel::High);
        assert_eq!(TraitLevel::from_numeric(2).unwrap(), TraitLevel::Highest);
    }

    #[test]
    fn test_trait_level_from_numeric_invalid() {
        assert!(TraitLevel::from_numeric(3).is_err());
        assert!(TraitLevel::from_numeric(-3).is_err());
        assert!(TraitLevel::from_numeric(100).is_err());
    }

    #[test]
    fn test_trait_level_name_all_kinds() {
        // Every trait kind should have a name for every level
        for &kind in TraitKind::ALL {
            for &level in kind.levels() {
                let name = trait_level_name(kind, level);
                assert!(!name.is_empty(), "{kind}/{level} has empty name");
            }
        }
    }

    #[test]
    fn test_trait_level_name_balanced_always_balanced() {
        for &kind in TraitKind::ALL {
            assert_eq!(trait_level_name(kind, TraitLevel::Balanced), "balanced");
        }
    }

    #[test]
    fn test_trait_behavior_all_non_balanced_return_some() {
        let non_balanced = [
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::High,
            TraitLevel::Highest,
        ];
        for &kind in TraitKind::ALL {
            for &level in &non_balanced {
                assert!(
                    trait_behavior(kind, level).is_some(),
                    "{kind}/{level} returned None"
                );
            }
        }
    }

    #[test]
    fn test_trait_behavior_text_nonempty() {
        let non_balanced = [
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::High,
            TraitLevel::Highest,
        ];
        for &kind in TraitKind::ALL {
            for &level in &non_balanced {
                let text = trait_behavior(kind, level).unwrap();
                assert!(text.len() > 10, "{kind}/{level} behavior too short");
            }
        }
    }

    #[test]
    fn test_trait_value_struct() {
        let tv = TraitValue {
            trait_name: TraitKind::Humor,
            level: TraitLevel::High,
        };
        assert_eq!(tv.trait_name, TraitKind::Humor);
        assert_eq!(tv.level, TraitLevel::High);
    }

    #[test]
    fn test_trait_value_serde() {
        let tv = TraitValue {
            trait_name: TraitKind::Warmth,
            level: TraitLevel::Highest,
        };
        let json = serde_json::to_string(&tv).unwrap();
        let tv2: TraitValue = serde_json::from_str(&json).unwrap();
        assert_eq!(tv2.trait_name, TraitKind::Warmth);
        assert_eq!(tv2.level, TraitLevel::Highest);
    }

    #[test]
    fn test_personality_profile_description() {
        let mut p = PersonalityProfile::new("test");
        assert!(p.description.is_none());
        p.description = Some("A test personality".into());
        assert_eq!(p.description.as_deref(), Some("A test personality"));
    }

    #[test]
    fn test_active_traits_returns_correct_values() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Warmth, TraitLevel::Low);
        let active = p.active_traits();
        assert_eq!(active.len(), 2);
        assert!(
            active
                .iter()
                .any(|t| t.trait_name == TraitKind::Humor && t.level == TraitLevel::Highest)
        );
        assert!(
            active
                .iter()
                .any(|t| t.trait_name == TraitKind::Warmth && t.level == TraitLevel::Low)
        );
    }

    #[test]
    fn test_compose_prompt_bullet_count() {
        let mut p = PersonalityProfile::new("test");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Warmth, TraitLevel::High);
        p.set_trait(TraitKind::Directness, TraitLevel::Lowest);
        let prompt = p.compose_prompt();
        let bullet_count = prompt.lines().filter(|l| l.starts_with("- ")).count();
        assert_eq!(bullet_count, 3);
    }

    #[test]
    fn test_distance_max_extremes() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        for &kind in TraitKind::ALL {
            a.set_trait(kind, TraitLevel::Lowest);
            b.set_trait(kind, TraitLevel::Highest);
        }
        let d = a.distance(&b);
        // max distance: sqrt(15 * (1.0 - (-1.0))^2) = sqrt(15 * 4) = sqrt(60)
        let expected = (60.0f32).sqrt();
        assert!((d - expected).abs() < 0.01);
    }

    #[test]
    fn test_serde_roundtrip_with_description() {
        let mut p = PersonalityProfile::new("full");
        p.description = Some("detailed".into());
        p.set_trait(TraitKind::Curiosity, TraitLevel::Highest);
        let json = serde_json::to_string(&p).unwrap();
        let p2: PersonalityProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(p2.description.as_deref(), Some("detailed"));
        assert_eq!(p2.get_trait(TraitKind::Curiosity), TraitLevel::Highest);
    }

    #[test]
    fn test_trait_kind_serde() {
        let json = serde_json::to_string(&TraitKind::RiskTolerance).unwrap();
        let kind: TraitKind = serde_json::from_str(&json).unwrap();
        assert_eq!(kind, TraitKind::RiskTolerance);
    }

    #[test]
    fn test_trait_level_serde() {
        for &level in &[
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::Balanced,
            TraitLevel::High,
            TraitLevel::Highest,
        ] {
            let json = serde_json::to_string(&level).unwrap();
            let restored: TraitLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, level);
        }
    }

    // --- v0.2: TraitGroup ---

    #[test]
    fn test_trait_group_all() {
        assert_eq!(TraitGroup::ALL.len(), 4);
    }

    #[test]
    fn test_trait_group_covers_all_traits() {
        let mut covered = std::collections::HashSet::new();
        for &group in TraitGroup::ALL {
            for &kind in group.traits() {
                covered.insert(kind);
            }
        }
        for &kind in TraitKind::ALL {
            assert!(covered.contains(&kind), "{kind} not in any group");
        }
    }

    #[test]
    fn test_trait_group_no_overlap() {
        let mut seen = std::collections::HashSet::new();
        for &group in TraitGroup::ALL {
            for &kind in group.traits() {
                assert!(seen.insert(kind), "{kind} in multiple groups");
            }
        }
    }

    #[test]
    fn test_trait_kind_group_roundtrip() {
        for &kind in TraitKind::ALL {
            let group = kind.group();
            assert!(
                group.traits().contains(&kind),
                "{kind} claims group {group} but group doesn't contain it"
            );
        }
    }

    #[test]
    fn test_trait_group_display() {
        assert_eq!(TraitGroup::Social.to_string(), "social");
        assert_eq!(TraitGroup::Cognitive.to_string(), "cognitive");
        assert_eq!(TraitGroup::Behavioral.to_string(), "behavioral");
    }

    #[test]
    fn test_trait_group_serde() {
        for &group in TraitGroup::ALL {
            let json = serde_json::to_string(&group).unwrap();
            let restored: TraitGroup = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, group);
        }
    }

    #[test]
    fn test_set_group() {
        let mut p = PersonalityProfile::new("test");
        p.set_group(TraitGroup::Social, TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Warmth), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Empathy), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Humor), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Patience), TraitLevel::Highest);
        // Other groups unchanged
        assert_eq!(p.get_trait(TraitKind::Curiosity), TraitLevel::Balanced);
    }

    #[test]
    fn test_group_average_balanced() {
        let p = PersonalityProfile::new("test");
        for &group in TraitGroup::ALL {
            assert!(p.group_average(group).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_group_average_extreme() {
        let mut p = PersonalityProfile::new("test");
        p.set_group(TraitGroup::Social, TraitLevel::Highest);
        assert!((p.group_average(TraitGroup::Social) - 1.0).abs() < f32::EPSILON);
    }

    // --- v0.2: Compatibility ---

    #[test]
    fn test_compatibility_identical() {
        let p = PersonalityProfile::new("a");
        assert!((p.compatibility(&p) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_compatibility_opposite() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        for &kind in TraitKind::ALL {
            a.set_trait(kind, TraitLevel::Lowest);
            b.set_trait(kind, TraitLevel::Highest);
        }
        assert!(a.compatibility(&b) < 0.01);
    }

    #[test]
    fn test_compatibility_symmetric() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Humor, TraitLevel::High);
        b.set_trait(TraitKind::Humor, TraitLevel::Low);
        assert!((a.compatibility(&b) - b.compatibility(&a)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_compatibility_range() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Warmth, TraitLevel::High);
        b.set_trait(TraitKind::Warmth, TraitLevel::Low);
        let c = a.compatibility(&b);
        assert!((0.0..=1.0).contains(&c));
    }

    #[test]
    fn test_cosine_same_direction_different_magnitude() {
        // Cosine similarity: profiles pointing the same direction should be highly compatible
        // even if one is more extreme than the other
        let mut mild = PersonalityProfile::new("mild");
        let mut extreme = PersonalityProfile::new("extreme");
        // Same pattern (warm+creative+curious), different intensities
        mild.set_trait(TraitKind::Warmth, TraitLevel::High);
        mild.set_trait(TraitKind::Creativity, TraitLevel::High);
        mild.set_trait(TraitKind::Curiosity, TraitLevel::High);
        extreme.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        extreme.set_trait(TraitKind::Creativity, TraitLevel::Highest);
        extreme.set_trait(TraitKind::Curiosity, TraitLevel::Highest);
        // Should be very compatible (same direction)
        assert!(mild.compatibility(&extreme) > 0.9);
    }

    #[test]
    fn test_cosine_orthogonal_traits() {
        // Two profiles with non-overlapping active traits should be ~0.5 (orthogonal)
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        b.set_trait(TraitKind::Precision, TraitLevel::Highest);
        let c = a.compatibility(&b);
        // Orthogonal vectors → cosine=0 → mapped to 0.5
        assert!((c - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_group_compatibility_identical() {
        let p = PersonalityProfile::new("a");
        for &group in TraitGroup::ALL {
            assert!(
                (p.group_compatibility(&p, group) - 1.0).abs() < f32::EPSILON,
                "{group} compatibility with self should be 1.0"
            );
        }
    }

    #[test]
    fn test_group_compatibility_partial_match() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        // Same social traits, different cognitive
        a.set_group(TraitGroup::Social, TraitLevel::High);
        b.set_group(TraitGroup::Social, TraitLevel::High);
        a.set_group(TraitGroup::Cognitive, TraitLevel::Highest);
        b.set_group(TraitGroup::Cognitive, TraitLevel::Lowest);

        assert!((a.group_compatibility(&b, TraitGroup::Social) - 1.0).abs() < f32::EPSILON);
        assert!(a.group_compatibility(&b, TraitGroup::Cognitive) < 0.1);
    }

    // --- v0.2: Blending ---

    #[test]
    fn test_blend_zero() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let b = PersonalityProfile::new("b");
        let blended = a.blend(&b, 0.0);
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_blend_one() {
        let a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        b.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let blended = a.blend(&b, 1.0);
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_blend_midpoint() {
        let mut a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest); // -1.0
        b.set_trait(TraitKind::Humor, TraitLevel::Highest); // 1.0
        let blended = a.blend(&b, 0.5);
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Balanced); // 0.0
    }

    #[test]
    fn test_blend_name() {
        let a = PersonalityProfile::new("alpha");
        let b = PersonalityProfile::new("beta");
        let blended = a.blend(&b, 0.5);
        assert_eq!(blended.name, "alpha+beta");
    }

    #[test]
    fn test_blend_clamps_t() {
        let a = PersonalityProfile::new("a");
        let mut b = PersonalityProfile::new("b");
        b.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let blended = a.blend(&b, 5.0); // should clamp to 1.0
        assert_eq!(blended.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    // --- v0.2: from_normalized ---

    #[test]
    fn test_from_normalized() {
        assert_eq!(TraitLevel::from_normalized(-1.0), TraitLevel::Lowest);
        assert_eq!(TraitLevel::from_normalized(-0.5), TraitLevel::Low);
        assert_eq!(TraitLevel::from_normalized(0.0), TraitLevel::Balanced);
        assert_eq!(TraitLevel::from_normalized(0.5), TraitLevel::High);
        assert_eq!(TraitLevel::from_normalized(1.0), TraitLevel::Highest);
    }

    #[test]
    fn test_from_normalized_snaps() {
        assert_eq!(TraitLevel::from_normalized(0.3), TraitLevel::High); // rounds to 1
        assert_eq!(TraitLevel::from_normalized(-0.3), TraitLevel::Low); // rounds to -1
        assert_eq!(TraitLevel::from_normalized(0.1), TraitLevel::Balanced); // rounds to 0
    }

    #[test]
    fn test_from_normalized_clamps() {
        assert_eq!(TraitLevel::from_normalized(5.0), TraitLevel::Highest);
        assert_eq!(TraitLevel::from_normalized(-5.0), TraitLevel::Lowest);
    }

    // --- v0.2: Mutation ---

    #[test]
    fn test_mutate_toward_no_change_when_equal() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::High);
        let target = a.clone();
        let changed = a.mutate_toward(&target, 0.5);
        assert_eq!(changed, 0);
    }

    #[test]
    fn test_mutate_toward_gradual() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Highest);

        // At low rate, should move one step
        let changed = a.mutate_toward(&target, 0.1);
        assert!(changed > 0);
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::Low);
    }

    #[test]
    fn test_mutate_toward_full_rate() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Highest);

        let _changed = a.mutate_toward(&target, 1.0);
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_mutate_toward_multiple_traits() {
        let mut a = PersonalityProfile::new("a");
        let mut target = PersonalityProfile::new("target");
        a.set_group(TraitGroup::Social, TraitLevel::Lowest);
        target.set_group(TraitGroup::Social, TraitLevel::Highest);

        let changed = a.mutate_toward(&target, 0.1);
        assert_eq!(changed, 4); // all 4 social traits should move
    }

    #[test]
    fn test_mutate_toward_converges() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Highest);

        // Repeated mutation at low rate should eventually reach target
        for _ in 0..10 {
            a.mutate_toward(&target, 0.3);
        }
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::Highest);
    }

    #[test]
    fn test_mutate_toward_downward() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Lowest);

        a.mutate_toward(&target, 0.1);
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::High);
    }

    #[test]
    fn test_mutate_toward_zero_rate_is_noop() {
        let mut a = PersonalityProfile::new("a");
        a.set_trait(TraitKind::Humor, TraitLevel::Lowest);
        let mut target = PersonalityProfile::new("target");
        target.set_trait(TraitKind::Humor, TraitLevel::Highest);

        let changed = a.mutate_toward(&target, 0.0);
        assert_eq!(changed, 0);
        assert_eq!(a.get_trait(TraitKind::Humor), TraitLevel::Lowest);
    }

    // --- New traits (SY parity) ---

    #[test]
    fn test_new_trait_level_names() {
        assert_eq!(
            trait_level_name(TraitKind::Skepticism, TraitLevel::Lowest),
            "gullible"
        );
        assert_eq!(
            trait_level_name(TraitKind::Skepticism, TraitLevel::Highest),
            "contrarian"
        );
        assert_eq!(
            trait_level_name(TraitKind::Autonomy, TraitLevel::Highest),
            "autonomous"
        );
        assert_eq!(
            trait_level_name(TraitKind::Pedagogy, TraitLevel::Highest),
            "socratic"
        );
        assert_eq!(
            trait_level_name(TraitKind::Precision, TraitLevel::Highest),
            "meticulous"
        );
    }

    #[test]
    fn test_new_trait_behaviors() {
        assert!(
            trait_behavior(TraitKind::Skepticism, TraitLevel::Highest)
                .unwrap()
                .contains("devil")
        );
        assert!(
            trait_behavior(TraitKind::Autonomy, TraitLevel::Highest)
                .unwrap()
                .contains("independently")
        );
        assert!(
            trait_behavior(TraitKind::Pedagogy, TraitLevel::Highest)
                .unwrap()
                .contains("discovery")
        );
        assert!(
            trait_behavior(TraitKind::Precision, TraitLevel::Highest)
                .unwrap()
                .contains("meticulous")
        );
    }

    #[test]
    fn test_new_traits_in_groups() {
        assert_eq!(TraitKind::Skepticism.group(), TraitGroup::Cognitive);
        assert_eq!(TraitKind::Autonomy.group(), TraitGroup::Professional);
        assert_eq!(TraitKind::Pedagogy.group(), TraitGroup::Professional);
        assert_eq!(TraitKind::Precision.group(), TraitGroup::Professional);
    }

    #[test]
    fn test_professional_group() {
        let traits = TraitGroup::Professional.traits();
        assert_eq!(traits.len(), 3);
        assert!(traits.contains(&TraitKind::Autonomy));
        assert!(traits.contains(&TraitKind::Pedagogy));
        assert!(traits.contains(&TraitKind::Precision));
    }

    #[test]
    fn test_professional_group_display() {
        assert_eq!(TraitGroup::Professional.to_string(), "professional");
    }

    #[test]
    fn test_set_professional_group() {
        let mut p = PersonalityProfile::new("test");
        p.set_group(TraitGroup::Professional, TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Autonomy), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Pedagogy), TraitLevel::Highest);
        assert_eq!(p.get_trait(TraitKind::Precision), TraitLevel::Highest);
    }

    #[test]
    fn test_new_trait_display() {
        assert_eq!(TraitKind::Skepticism.to_string(), "skepticism");
        assert_eq!(TraitKind::Autonomy.to_string(), "autonomy");
        assert_eq!(TraitKind::Pedagogy.to_string(), "pedagogy");
        assert_eq!(TraitKind::Precision.to_string(), "precision");
    }

    #[test]
    fn test_new_trait_serde() {
        for &kind in &[
            TraitKind::Skepticism,
            TraitKind::Autonomy,
            TraitKind::Pedagogy,
            TraitKind::Precision,
        ] {
            let json = serde_json::to_string(&kind).unwrap();
            let restored: TraitKind = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, kind);
        }
    }

    #[test]
    fn test_professional_group_serde() {
        let json = serde_json::to_string(&TraitGroup::Professional).unwrap();
        let restored: TraitGroup = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, TraitGroup::Professional);
    }

    // --- Markdown serialization ---

    #[test]
    fn test_to_markdown() {
        let mut p = PersonalityProfile::new("TestBot");
        p.description = Some("A test personality".into());
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let md = p.to_markdown();
        assert!(md.contains("# TestBot"));
        assert!(md.contains("A test personality"));
        assert!(md.contains("| humor | highest | comedic |"));
    }

    #[test]
    fn test_from_markdown_roundtrip() {
        let mut p = PersonalityProfile::new("RoundTrip");
        p.description = Some("Full roundtrip test".into());
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Warmth, TraitLevel::Low);
        p.set_trait(TraitKind::Precision, TraitLevel::High);

        let md = p.to_markdown();
        let restored = PersonalityProfile::from_markdown(&md).unwrap();
        assert_eq!(restored.name, "RoundTrip");
        assert_eq!(restored.description.as_deref(), Some("Full roundtrip test"));
        assert_eq!(restored.get_trait(TraitKind::Humor), TraitLevel::Highest);
        assert_eq!(restored.get_trait(TraitKind::Warmth), TraitLevel::Low);
        assert_eq!(restored.get_trait(TraitKind::Precision), TraitLevel::High);
        // Unset traits should be Balanced
        assert_eq!(
            restored.get_trait(TraitKind::Curiosity),
            TraitLevel::Balanced
        );
    }

    #[test]
    fn test_from_markdown_missing_name() {
        assert!(PersonalityProfile::from_markdown("no header").is_none());
        assert!(PersonalityProfile::from_markdown("# ").is_none());
    }

    #[test]
    fn test_from_markdown_no_description() {
        let md = "# Simple\n\n## Traits\n\n| Trait | Level | Name |\n|-------|-------|------|\n| humor | high | witty |\n";
        let p = PersonalityProfile::from_markdown(md).unwrap();
        assert_eq!(p.name, "Simple");
        assert!(p.description.is_none());
        assert_eq!(p.get_trait(TraitKind::Humor), TraitLevel::High);
    }

    #[test]
    fn test_markdown_all_presets_roundtrip() {
        // Every trait for every preset should survive roundtrip
        for &kind in TraitKind::ALL {
            for &level in &[
                TraitLevel::Lowest,
                TraitLevel::Low,
                TraitLevel::Balanced,
                TraitLevel::High,
                TraitLevel::Highest,
            ] {
                let mut p = PersonalityProfile::new("test");
                p.set_trait(kind, level);
                let md = p.to_markdown();
                let restored = PersonalityProfile::from_markdown(&md).unwrap();
                assert_eq!(
                    restored.get_trait(kind),
                    level,
                    "{kind}/{level} failed roundtrip"
                );
            }
        }
    }

    #[test]
    fn test_parse_trait_kind_all() {
        for &kind in TraitKind::ALL {
            let s = kind.to_string();
            assert_eq!(
                parse_trait_kind(&s),
                Some(kind),
                "parse_trait_kind failed for {s}"
            );
        }
    }

    #[test]
    fn test_parse_trait_level_all() {
        for &level in &[
            TraitLevel::Lowest,
            TraitLevel::Low,
            TraitLevel::Balanced,
            TraitLevel::High,
            TraitLevel::Highest,
        ] {
            let s = level.to_string();
            assert_eq!(
                parse_trait_level(&s),
                Some(level),
                "parse_trait_level failed for {s}"
            );
        }
    }

    #[test]
    fn test_parse_unknown() {
        assert!(parse_trait_kind("nonexistent").is_none());
        assert!(parse_trait_level("ultra").is_none());
    }

    // --- OCEAN Conversion ---

    #[test]
    fn test_to_ocean_balanced() {
        let p = PersonalityProfile::new("neutral");
        let o = p.to_ocean();
        // All balanced → near zero across all dimensions
        assert!(o.openness.abs() < 0.1);
        assert!(o.conscientiousness.abs() < 0.1);
        assert!(o.extraversion.abs() < 0.1);
        assert!(o.agreeableness.abs() < 0.1);
        assert!(o.neuroticism.abs() < 0.1);
    }

    #[test]
    fn test_to_ocean_warm_creative() {
        let mut p = PersonalityProfile::new("warm");
        p.set_trait(TraitKind::Warmth, TraitLevel::Highest);
        p.set_trait(TraitKind::Creativity, TraitLevel::Highest);
        p.set_trait(TraitKind::Curiosity, TraitLevel::Highest);
        let o = p.to_ocean();
        assert!(o.openness > 0.3, "creative+curious should be high openness");
        assert!(o.extraversion > 0.1, "warm should boost extraversion");
    }

    #[test]
    fn test_to_ocean_skeptical_impatient() {
        let mut p = PersonalityProfile::new("neurotic");
        p.set_trait(TraitKind::Skepticism, TraitLevel::Highest);
        p.set_trait(TraitKind::Patience, TraitLevel::Lowest);
        p.set_trait(TraitKind::Confidence, TraitLevel::Lowest);
        let o = p.to_ocean();
        assert!(
            o.neuroticism > 0.0,
            "skeptical+impatient should be neurotic"
        );
    }

    #[test]
    fn test_profile_from_ocean_roundtrip_direction() {
        // High openness should produce high creativity/curiosity
        let ocean = OceanScores {
            openness: 0.8,
            conscientiousness: 0.0,
            extraversion: 0.0,
            agreeableness: 0.0,
            neuroticism: 0.0,
        };
        let p = profile_from_ocean("test", &ocean);
        assert!(p.get_trait(TraitKind::Creativity) >= TraitLevel::High);
        assert!(p.get_trait(TraitKind::Curiosity) >= TraitLevel::High);
    }

    #[test]
    fn test_ocean_serde() {
        let ocean = OceanScores {
            openness: 0.5,
            conscientiousness: -0.3,
            extraversion: 0.7,
            agreeableness: 0.2,
            neuroticism: -0.4,
        };
        let json = serde_json::to_string(&ocean).unwrap();
        let o2: OceanScores = serde_json::from_str(&json).unwrap();
        assert!((o2.openness - 0.5).abs() < f32::EPSILON);
    }

    // --- Personality Entropy / Extremity ---

    #[test]
    fn test_entropy_all_balanced() {
        let p = PersonalityProfile::new("neutral");
        let e = personality_entropy(&p);
        assert!(e < 0.01, "all balanced should be zero entropy");
    }

    #[test]
    fn test_entropy_mixed() {
        let mut p = PersonalityProfile::new("mixed");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        p.set_trait(TraitKind::Warmth, TraitLevel::Lowest);
        p.set_trait(TraitKind::Precision, TraitLevel::High);
        let e = personality_entropy(&p);
        assert!(e > 0.1, "mixed profile should have positive entropy");
    }

    #[test]
    fn test_extremity_all_balanced() {
        let p = PersonalityProfile::new("neutral");
        assert!(personality_extremity(&p) < f32::EPSILON);
    }

    #[test]
    fn test_extremity_all_highest() {
        let mut p = PersonalityProfile::new("extreme");
        for &kind in TraitKind::ALL {
            p.set_trait(kind, TraitLevel::Highest);
        }
        assert!((personality_extremity(&p) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_extremity_range() {
        let mut p = PersonalityProfile::new("mixed");
        p.set_trait(TraitKind::Humor, TraitLevel::Highest);
        let e = personality_extremity(&p);
        assert!((0.0..=1.0).contains(&e));
    }
}
