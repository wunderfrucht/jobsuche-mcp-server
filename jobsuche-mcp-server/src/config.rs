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
        let api_url = env::var("JOBSUCHE_API_URL")
            .unwrap_or_else(|_| {
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
