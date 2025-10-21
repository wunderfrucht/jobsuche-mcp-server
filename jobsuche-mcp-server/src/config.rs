//! Configuration module for Jobsuche MCP Server

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

/// Configuration for the Jobsuche MCP Server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobsucheConfig {
    /// The Jobsuche API base URL
    pub api_url: String,

    /// Optional API key (the default public key is used if not specified)
    pub api_key: Option<String>,

    /// Default page size for search results
    #[serde(default = "default_page_size")]
    pub default_page_size: u64,

    /// Maximum page size allowed
    #[serde(default = "default_max_page_size")]
    pub max_page_size: u64,
}

fn default_page_size() -> u64 {
    25
}

fn default_max_page_size() -> u64 {
    100
}

impl Default for JobsucheConfig {
    fn default() -> Self {
        Self {
            api_url: "https://rest.arbeitsagentur.de/jobboerse/jobsuche-service".to_string(),
            api_key: None,
            default_page_size: default_page_size(),
            max_page_size: default_max_page_size(),
        }
    }
}

impl JobsucheConfig {
    /// Load configuration from environment variables
    ///
    /// Environment variables:
    /// - `JOBSUCHE_API_URL`: API base URL (optional, defaults to official API)
    /// - `JOBSUCHE_API_KEY`: API key (optional, uses default if not specified)
    /// - `JOBSUCHE_DEFAULT_PAGE_SIZE`: Default page size (optional, defaults to 25)
    /// - `JOBSUCHE_MAX_PAGE_SIZE`: Maximum page size (optional, defaults to 100)
    pub fn load() -> Result<Self> {
        let api_url = env::var("JOBSUCHE_API_URL").unwrap_or_else(|_| {
            "https://rest.arbeitsagentur.de/jobboerse/jobsuche-service".to_string()
        });

        let api_key = env::var("JOBSUCHE_API_KEY").ok();

        let default_page_size = env::var("JOBSUCHE_DEFAULT_PAGE_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_page_size());

        let max_page_size = env::var("JOBSUCHE_MAX_PAGE_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_max_page_size());

        // Validate configuration
        if default_page_size == 0 {
            anyhow::bail!("Default page size must be greater than 0");
        }

        if max_page_size == 0 {
            anyhow::bail!("Max page size must be greater than 0");
        }

        if default_page_size > max_page_size {
            anyhow::bail!(
                "Default page size ({}) cannot exceed max page size ({})",
                default_page_size,
                max_page_size
            );
        }

        // The API limits page size to 100
        if max_page_size > 100 {
            anyhow::bail!("Max page size cannot exceed 100 (API limitation)");
        }

        Ok(Self {
            api_url,
            api_key,
            default_page_size,
            max_page_size,
        })
    }

    /// Validate that the configuration is correct
    pub fn validate(&self) -> Result<()> {
        if self.api_url.is_empty() {
            anyhow::bail!("API URL cannot be empty");
        }

        if !self.api_url.starts_with("http://") && !self.api_url.starts_with("https://") {
            anyhow::bail!("API URL must start with http:// or https://");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = JobsucheConfig::default();
        assert_eq!(
            config.api_url,
            "https://rest.arbeitsagentur.de/jobboerse/jobsuche-service"
        );
        assert_eq!(config.api_key, None);
        assert_eq!(config.default_page_size, 25);
        assert_eq!(config.max_page_size, 100);
    }

    #[test]
    fn test_default_page_size() {
        assert_eq!(default_page_size(), 25);
    }

    #[test]
    fn test_default_max_page_size() {
        assert_eq!(default_max_page_size(), 100);
    }

    #[test]
    fn test_load_with_defaults() {
        // Clear env vars
        env::remove_var("JOBSUCHE_API_URL");
        env::remove_var("JOBSUCHE_API_KEY");
        env::remove_var("JOBSUCHE_DEFAULT_PAGE_SIZE");
        env::remove_var("JOBSUCHE_MAX_PAGE_SIZE");

        let config = JobsucheConfig::load().unwrap();
        assert_eq!(
            config.api_url,
            "https://rest.arbeitsagentur.de/jobboerse/jobsuche-service"
        );
        assert_eq!(config.api_key, None);
        assert_eq!(config.default_page_size, 25);
        assert_eq!(config.max_page_size, 100);
    }

    #[test]
    fn test_load_with_custom_api_url() {
        env::set_var("JOBSUCHE_API_URL", "https://custom.api.example.com");
        let config = JobsucheConfig::load().unwrap();
        assert_eq!(config.api_url, "https://custom.api.example.com");
        env::remove_var("JOBSUCHE_API_URL");
    }

    #[test]
    fn test_load_with_api_key() {
        env::set_var("JOBSUCHE_API_KEY", "test-key-123");
        let config = JobsucheConfig::load().unwrap();
        assert_eq!(config.api_key, Some("test-key-123".to_string()));
        env::remove_var("JOBSUCHE_API_KEY");
    }

    #[test]
    fn test_load_with_custom_page_sizes() {
        env::set_var("JOBSUCHE_DEFAULT_PAGE_SIZE", "50");
        env::set_var("JOBSUCHE_MAX_PAGE_SIZE", "75");
        let config = JobsucheConfig::load().unwrap();
        assert_eq!(config.default_page_size, 50);
        assert_eq!(config.max_page_size, 75);
        env::remove_var("JOBSUCHE_DEFAULT_PAGE_SIZE");
        env::remove_var("JOBSUCHE_MAX_PAGE_SIZE");
    }

    #[test]
    fn test_load_with_invalid_page_size() {
        env::set_var("JOBSUCHE_DEFAULT_PAGE_SIZE", "not-a-number");
        let config = JobsucheConfig::load().unwrap();
        assert_eq!(config.default_page_size, 25); // Falls back to default
        env::remove_var("JOBSUCHE_DEFAULT_PAGE_SIZE");
    }

    #[test]
    fn test_load_with_zero_default_page_size() {
        env::set_var("JOBSUCHE_DEFAULT_PAGE_SIZE", "0");
        let result = JobsucheConfig::load();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Default page size must be greater than 0"));
        env::remove_var("JOBSUCHE_DEFAULT_PAGE_SIZE");
    }

    #[test]
    fn test_load_with_zero_max_page_size() {
        // Clear env vars first
        env::remove_var("JOBSUCHE_MAX_PAGE_SIZE");
        env::remove_var("JOBSUCHE_DEFAULT_PAGE_SIZE");

        env::set_var("JOBSUCHE_MAX_PAGE_SIZE", "0");
        let result = JobsucheConfig::load();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Max page size") || err_msg.contains("greater than 0"));
        env::remove_var("JOBSUCHE_MAX_PAGE_SIZE");
    }

    #[test]
    fn test_load_with_default_exceeding_max() {
        // Clear any existing values first
        env::remove_var("JOBSUCHE_DEFAULT_PAGE_SIZE");
        env::remove_var("JOBSUCHE_MAX_PAGE_SIZE");

        env::set_var("JOBSUCHE_DEFAULT_PAGE_SIZE", "200");
        env::set_var("JOBSUCHE_MAX_PAGE_SIZE", "100");
        let result = JobsucheConfig::load();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Default page size") || err_msg.contains("cannot exceed"));
        env::remove_var("JOBSUCHE_DEFAULT_PAGE_SIZE");
        env::remove_var("JOBSUCHE_MAX_PAGE_SIZE");
    }

    #[test]
    fn test_load_with_max_exceeding_api_limit() {
        env::set_var("JOBSUCHE_MAX_PAGE_SIZE", "150");
        let result = JobsucheConfig::load();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Max page size cannot exceed 100"));
        env::remove_var("JOBSUCHE_MAX_PAGE_SIZE");
    }

    #[test]
    fn test_validate_valid_config() {
        let config = JobsucheConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_url() {
        let mut config = JobsucheConfig::default();
        config.api_url = "".to_string();
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("API URL cannot be empty"));
    }

    #[test]
    fn test_validate_invalid_url_scheme() {
        let mut config = JobsucheConfig::default();
        config.api_url = "ftp://example.com".to_string();
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("API URL must start with http:// or https://"));
    }

    #[test]
    fn test_validate_http_url() {
        let mut config = JobsucheConfig::default();
        config.api_url = "http://example.com".to_string();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_https_url() {
        let mut config = JobsucheConfig::default();
        config.api_url = "https://example.com".to_string();
        assert!(config.validate().is_ok());
    }
}
