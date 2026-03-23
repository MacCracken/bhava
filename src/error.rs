//! Error types for bhava.

/// All errors that bhava can produce.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BhavaError {
    #[error("unknown trait: {name}")]
    UnknownTrait { name: String },

    #[error("unknown trait level '{level}' for trait '{trait_name}'")]
    UnknownTraitLevel { trait_name: String, level: String },

    #[error("mood dimension out of range: {dimension} = {value} (must be -1.0..=1.0)")]
    MoodOutOfRange { dimension: String, value: f32 },

    #[error("unknown archetype layer: {name}")]
    UnknownLayer { name: String },

    #[error("unknown preset: {id}")]
    UnknownPreset { id: String },

    #[error("invalid personality config: {reason}")]
    InvalidConfig { reason: String },

    #[error("decay rate must be positive: {rate}")]
    InvalidDecayRate { rate: f32 },

    #[cfg(feature = "ai")]
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, BhavaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_trait() {
        let e = BhavaError::UnknownTrait {
            name: "charisma".into(),
        };
        assert!(e.to_string().contains("charisma"));
    }

    #[test]
    fn test_unknown_level() {
        let e = BhavaError::UnknownTraitLevel {
            trait_name: "humor".into(),
            level: "manic".into(),
        };
        assert!(e.to_string().contains("humor"));
        assert!(e.to_string().contains("manic"));
    }

    #[test]
    fn test_mood_out_of_range() {
        let e = BhavaError::MoodOutOfRange {
            dimension: "joy".into(),
            value: 1.5,
        };
        assert!(e.to_string().contains("joy"));
        assert!(e.to_string().contains("1.5"));
    }

    #[test]
    fn test_invalid_config() {
        let e = BhavaError::InvalidConfig {
            reason: "empty name".into(),
        };
        assert!(e.to_string().contains("empty name"));
    }

    #[test]
    fn test_invalid_decay_rate() {
        let e = BhavaError::InvalidDecayRate { rate: -0.5 };
        assert!(e.to_string().contains("-0.5"));
    }

    #[test]
    fn test_result_alias() {
        let ok: Result<i32> = Ok(42);
        assert!(ok.is_ok());
    }

    #[test]
    fn test_unknown_layer() {
        let e = BhavaError::UnknownLayer {
            name: "aura".into(),
        };
        assert!(e.to_string().contains("aura"));
    }

    #[test]
    fn test_unknown_preset() {
        let e = BhavaError::UnknownPreset { id: "ghost".into() };
        assert!(e.to_string().contains("ghost"));
    }

    #[test]
    fn test_mood_out_of_range_format() {
        let e = BhavaError::MoodOutOfRange {
            dimension: "arousal".into(),
            value: 2.5,
        };
        let msg = e.to_string();
        assert!(msg.contains("arousal"));
        assert!(msg.contains("2.5"));
        assert!(msg.contains("-1.0..=1.0"));
    }

    #[test]
    fn test_result_alias_err() {
        let err: Result<i32> = Err(BhavaError::InvalidConfig {
            reason: "bad".into(),
        });
        assert!(err.is_err());
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<BhavaError>();
        assert_sync::<BhavaError>();
    }
}
