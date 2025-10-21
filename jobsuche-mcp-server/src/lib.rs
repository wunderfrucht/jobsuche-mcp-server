//! Jobsuche MCP Server Library
//!
//! An AI-friendly job search integration server using the Model Context Protocol (MCP).
//! This server provides tools for searching German job listings via the Federal Employment
//! Agency (Bundesagentur für Arbeit) API without requiring knowledge of API internals.
//!
//! ## Features
//!
//! - **AI-Friendly Interface**: Simple, semantic parameters for job searching
//! - **Official API Integration**: Uses the jobsuche crate for reliable API access
//! - **Rich Filtering**: Search by location, job title, employment type, salary, etc.
//! - **Comprehensive Details**: Get full job information including descriptions and requirements
//! - **Pagination Support**: Handle large result sets efficiently

use jobsuche::{Arbeitszeit, Credentials, Jobsuche, JobSearchResponse, SearchOptions, JobDetails};
use pulseengine_mcp_macros::{mcp_server, mcp_tools};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, instrument};

pub mod config;
use config::JobsucheConfig;

/// Server status information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobsucheServerStatus {
    pub server_name: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub api_url: String,
    pub api_connection_status: String,
    pub tools_count: usize,
}

/// Parameters for searching jobs
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchJobsParams {
    /// Job title or keywords (e.g., "Software Engineer", "Data Scientist")
    pub job_title: Option<String>,

    /// Location name (e.g., "Berlin", "München", "Deutschland")
    pub location: Option<String>,

    /// Search radius in kilometers from the location (default: 25)
    pub radius_km: Option<u64>,

    /// Employment type filter
    /// Options: "fulltime" (Vollzeit), "parttime" (Teilzeit), "mini_job", "home_office"
    pub employment_type: Option<Vec<String>>,

    /// Contract type filter
    /// Options: "permanent" (unbefristet), "temporary" (befristet)
    pub contract_type: Option<Vec<String>>,

    /// Days since publication (0-100, default: 30)
    /// Example: 7 for jobs posted in the last week
    pub published_since_days: Option<u64>,

    /// Number of results per page (1-100, default from config)
    pub page_size: Option<u64>,

    /// Page number for pagination (starting from 1)
    pub page: Option<u64>,
}

/// Result from job search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchJobsResult {
    /// Total number of results found
    pub total_results: Option<u64>,

    /// Current page number
    pub current_page: Option<u64>,

    /// Page size used
    pub page_size: Option<u64>,

    /// Number of jobs in this response
    pub jobs_count: usize,

    /// Job listings
    pub jobs: Vec<JobSummary>,

    /// Search performance info
    pub search_duration_ms: u64,
}

/// Summary information for a job listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSummary {
    /// Reference number (use this to get job details)
    pub reference_number: String,

    /// Job title
    pub title: String,

    /// Employer name
    pub employer: String,

    /// Location information
    pub location: String,

    /// Publication date (YYYY-MM-DD format)
    pub published_date: Option<String>,

    /// External URL if available
    pub external_url: Option<String>,
}

/// Parameters for getting job details
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GetJobDetailsParams {
    /// Job reference number (refnr from search results)
    pub reference_number: String,
}

/// Detailed job information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetJobDetailsResult {
    /// Reference number
    pub reference_number: String,

    /// Job title
    pub title: Option<String>,

    /// Job description
    pub description: Option<String>,

    /// Employer name
    pub employer: Option<String>,

    /// Location information
    pub location: Option<String>,

    /// Employment type
    pub employment_type: Option<String>,

    /// Contract type
    pub contract_type: Option<String>,

    /// Start date
    pub start_date: Option<String>,

    /// Application deadline
    pub application_deadline: Option<String>,

    /// Contact information
    pub contact_info: Option<String>,

    /// External application URL
    pub external_url: Option<String>,

    /// Raw JSON for additional fields
    pub raw_data: serde_json::Value,
}

/// Jobsuche MCP Server
///
/// Main server implementation providing AI-friendly tools for German job search.
#[mcp_server(
    name = "Jobsuche MCP Server",
    version = "0.1.0",
    description = "AI-friendly job search integration using the German Federal Employment Agency API",
    auth = "disabled"
)]
#[derive(Clone)]
pub struct JobsucheMcpServer {
    /// Server start time
    start_time: Instant,

    /// Jobsuche API client
    client: Arc<Jobsuche>,

    /// Configuration
    config: Arc<JobsucheConfig>,
}

impl Default for JobsucheMcpServer {
    fn default() -> Self {
        panic!("JobsucheMcpServer cannot be created with default(). Use JobsucheMcpServer::new() instead.")
    }
}

impl JobsucheMcpServer {
    /// Create a new Jobsuche MCP Server
    #[instrument]
    pub fn new() -> anyhow::Result<Self> {
        info!("Initializing Jobsuche MCP Server");

        let config = Arc::new(JobsucheConfig::load()?);
        config.validate()?;

        info!("Configuration loaded: API URL = {}", config.api_url);

        let credentials = if let Some(ref api_key) = config.api_key {
            info!("Using custom API key");
            Credentials::ApiKey(api_key.clone())
        } else {
            info!("Using default API credentials");
            Credentials::default()
        };

        let client = Jobsuche::new(&config.api_url, credentials)?;

        info!("Jobsuche MCP Server initialized successfully");

        Ok(Self {
            start_time: Instant::now(),
            client: Arc::new(client),
            config,
        })
    }

    /// Get server uptime in seconds
    fn get_uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Convert employment type string to Arbeitszeit enum
    fn parse_employment_type(emp_type: &str) -> Option<Arbeitszeit> {
        match emp_type.to_lowercase().as_str() {
            "fulltime" | "full" | "vollzeit" | "vz" => Some(Arbeitszeit::Vollzeit),
            "parttime" | "part" | "teilzeit" | "tz" => Some(Arbeitszeit::Teilzeit),
            "mini" | "minijob" | "mini_job" => Some(Arbeitszeit::Minijob),
            "home" | "homeoffice" | "home_office" | "ho" => Some(Arbeitszeit::HeimTelearbeit),
            "shift" | "schicht" | "snw" => Some(Arbeitszeit::SchichtNachtarbeitWochenende),
            _ => None,
        }
    }
}

/// MCP tools implementation
#[mcp_tools]
impl JobsucheMcpServer {
    /// Search for jobs in Germany using the Federal Employment Agency database
    ///
    /// This tool allows searching for jobs with various filters including location,
    /// job title, employment type, and more. Results include job summaries with
    /// reference numbers that can be used to get detailed information.
    ///
    /// # Examples
    /// - Search for software jobs in Berlin: `{"job_title": "Software Engineer", "location": "Berlin"}`
    /// - Recent jobs in München: `{"location": "München", "published_since_days": 7}`
    /// - Full-time jobs nationwide: `{"employment_type": ["fulltime"]}`
    #[instrument(skip(self))]
    pub async fn search_jobs(
        &self,
        params: SearchJobsParams,
    ) -> anyhow::Result<SearchJobsResult> {
        info!("Searching jobs with params: {:?}", params);
        let start = Instant::now();

        let mut search_opts = SearchOptions::builder();

        // Job title
        if let Some(ref title) = params.job_title {
            search_opts.was(title);
        }

        // Location
        if let Some(ref location) = params.location {
            search_opts.wo(location);
        }

        // Radius
        if let Some(radius) = params.radius_km {
            search_opts.umkreis(radius);
        }

        // Employment type
        if let Some(ref emp_types) = params.employment_type {
            let arbeitszeit: Vec<Arbeitszeit> = emp_types
                .iter()
                .filter_map(|t| Self::parse_employment_type(t))
                .collect();

            if !arbeitszeit.is_empty() {
                search_opts.arbeitszeit(arbeitszeit);
            }
        }

        // Published since
        if let Some(days) = params.published_since_days {
            search_opts.veroeffentlichtseit(days);
        }

        // Pagination
        let page_size = params
            .page_size
            .unwrap_or(self.config.default_page_size)
            .min(self.config.max_page_size);

        search_opts.size(page_size);

        if let Some(page) = params.page {
            search_opts.page(page);
        }

        let options = search_opts.build();
        let response: JobSearchResponse = self.client.search().list(options)?;

        let jobs: Vec<JobSummary> = response
            .stellenangebote
            .iter()
            .map(|job| {
                let location = format!(
                    "{}{}",
                    job.arbeitsort.ort.as_deref().unwrap_or(""),
                    job.arbeitsort
                        .plz
                        .as_ref()
                        .map(|plz| format!(" ({})", plz))
                        .unwrap_or_default()
                );

                JobSummary {
                    reference_number: job.refnr.clone(),
                    title: job.titel.clone().unwrap_or_else(|| job.beruf.clone()),
                    employer: job.arbeitgeber.clone(),
                    location,
                    published_date: job.aktuelle_veroeffentlichungsdatum.clone(),
                    external_url: job.externe_url.clone(),
                }
            })
            .collect();

        let duration = start.elapsed();
        info!(
            "Search completed: {} jobs found in {:?}",
            jobs.len(),
            duration
        );

        Ok(SearchJobsResult {
            total_results: response.max_ergebnisse,
            current_page: response.page,
            page_size: response.size,
            jobs_count: jobs.len(),
            jobs,
            search_duration_ms: duration.as_millis() as u64,
        })
    }

    /// Get detailed information about a specific job posting
    ///
    /// Retrieves comprehensive information about a job including the full description,
    /// requirements, application instructions, and contact details.
    ///
    /// # Examples
    /// - Get job details: `{"reference_number": "10001-1234567890-S"}`
    #[instrument(skip(self))]
    pub async fn get_job_details(
        &self,
        params: GetJobDetailsParams,
    ) -> anyhow::Result<GetJobDetailsResult> {
        info!("Getting job details for: {}", params.reference_number);

        let details: JobDetails = self.client.job_details(&params.reference_number)?;

        // Serialize to JSON for raw_data field
        let raw_data = serde_json::to_value(&details)?;

        let location_str = details
            .arbeitsorte
            .first()
            .and_then(|loc| loc.ort.clone())
            .or_else(|| {
                details
                    .arbeitgeber_adresse
                    .as_ref()
                    .map(|addr| addr.ort.clone())
            });

        let result = GetJobDetailsResult {
            reference_number: params.reference_number.clone(),
            title: details.titel,
            description: details.stellenbeschreibung,
            employer: details.arbeitgeber,
            location: location_str,
            employment_type: details.arbeitszeitmodelle.first().cloned(),
            contract_type: details.befristung,
            start_date: details.eintrittsdatum,
            application_deadline: None, // Not available in API
            contact_info: None,          // Not available in current API version
            external_url: None,          // Not available in current API version
            raw_data,
        };

        info!("Job details retrieved successfully");
        Ok(result)
    }

    /// Get server status and connection information
    ///
    /// Returns information about the server status, uptime, API configuration,
    /// and available tools.
    #[instrument(skip(self))]
    pub async fn get_server_status(&self) -> anyhow::Result<JobsucheServerStatus> {
        info!("Getting server status");

        // Test API connectivity by making a minimal search
        let connection_status = match self
            .client
            .search()
            .list(SearchOptions::builder().size(1).build())
        {
            Ok(_) => "Connected".to_string(),
            Err(e) => format!("Connection Error: {}", e),
        };

        Ok(JobsucheServerStatus {
            server_name: "Jobsuche MCP Server".to_string(),
            version: "0.1.0".to_string(),
            uptime_seconds: self.get_uptime_seconds(),
            api_url: self.config.api_url.clone(),
            api_connection_status: connection_status,
            tools_count: 3, // search_jobs, get_job_details, get_server_status
        })
    }
}
