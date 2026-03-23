//! AI integration — daimon/hoosh client for bhava.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaimonConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
}

impl Default for DaimonConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8090".into(),
            api_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooshConfig {
    pub endpoint: String,
}

impl Default for HooshConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8088".into(),
        }
    }
}

/// HTTP client for daimon agent registration and communication.
pub struct DaimonClient {
    config: DaimonConfig,
    client: reqwest::Client,
}

impl DaimonClient {
    /// Create a new client with the given config.
    pub fn new(config: DaimonConfig) -> crate::error::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(crate::error::BhavaError::Network)?;
        Ok(Self { config, client })
    }

    pub async fn register_agent(&self) -> crate::error::Result<String> {
        let body = serde_json::json!({
            "name": "bhava",
            "capabilities": ["personality", "emotion", "mood", "sentiment", "archetype"],
        });
        let resp = self
            .client
            .post(format!("{}/v1/agents/register", self.config.endpoint))
            .json(&body)
            .send()
            .await?;
        let data: serde_json::Value = resp.json().await?;
        Ok(data["agent_id"].as_str().unwrap_or("unknown").to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let c = DaimonConfig::default();
        assert_eq!(c.endpoint, "http://localhost:8090");
    }

    #[test]
    fn test_hoosh_default() {
        let c = HooshConfig::default();
        assert_eq!(c.endpoint, "http://localhost:8088");
    }

    #[test]
    fn test_daimon_config_custom() {
        let c = DaimonConfig {
            endpoint: "https://custom:9090".into(),
            api_key: Some("secret".into()),
        };
        assert_eq!(c.endpoint, "https://custom:9090");
        assert_eq!(c.api_key.as_deref(), Some("secret"));
    }

    #[test]
    fn test_daimon_config_serde() {
        let c = DaimonConfig::default();
        let json = serde_json::to_string(&c).unwrap();
        let c2: DaimonConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(c2.endpoint, c.endpoint);
        assert_eq!(c2.api_key, None);
    }

    #[test]
    fn test_hoosh_config_serde() {
        let c = HooshConfig::default();
        let json = serde_json::to_string(&c).unwrap();
        let c2: HooshConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(c2.endpoint, c.endpoint);
    }

    #[test]
    fn test_daimon_client_new() {
        let client = DaimonClient::new(DaimonConfig::default()).unwrap();
        assert_eq!(client.config.endpoint, "http://localhost:8090");
    }

    #[test]
    fn test_daimon_config_no_api_key() {
        let c = DaimonConfig::default();
        assert!(c.api_key.is_none());
    }
}
