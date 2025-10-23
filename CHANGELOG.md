# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-10-23

### Added

- **Bulk Operations for AI Efficiency**: New tools for high-efficiency job searching
  - `search_jobs_with_details`: Combines search and detail fetching in a single call
    - Search for jobs and automatically get full details for top N results
    - Configurable number of details to fetch (default: 5, max: 20)
    - Reduces tool calls from N+1 to 1 for typical AI workflows

  - `batch_search_jobs`: Perform multiple searches simultaneously
    - Compare different employers, locations, or job types in one call
    - Up to 10 parallel searches supported
    - Each search can fetch details for top N results (default: 3, max: 10)
    - Perfect for systematic job comparison workflows

- **Field Filtering Infrastructure**: Prepared for future response optimization
  - `FieldFilter` struct with `include_fields` and `exclude_fields`
  - Reduces token usage for AI by returning only relevant fields
  - Infrastructure ready, full implementation in future release

### Changed

- Server now reports 5 tools (was 3)
- Improved error handling for bulk operations (continues on individual failures)

### Performance

- **Typical AI Workflow Improvement**:
  - Before: 1 search + 5 detail calls = 6 tool invocations
  - After: 1 `search_jobs_with_details` call = 1 tool invocation
  - **83% reduction in tool calls**

## [0.2.0] - 2025-10-23

### Changed

- **Updated to jobsuche 0.3.0**: Upgraded underlying API library with significant structural changes
  - Improved error handling and API stability
  - Streamlined data structures for better performance
  - Note: Some fields from jobsuche 0.2.0 are no longer available in the API

### Added

- **New API v0.3.0 Fields**:
  - `fulltime`: Boolean indicator for full-time employment
  - `entry_period`: Formatted entry date range
  - `publication_period`: Formatted publication date range
  - `is_minor_employment`: Minor employment indicator (Geringfügige Beschäftigung)
  - `is_temp_agency`: Temporary employment agency indicator (Zeitarbeit)
  - `is_private_agency`: Private employment agency indicator
  - `career_changer_suitable`: Suitability for career changers (Quereinsteiger)
  - `cipher_number`: Cipher number for anonymous postings

- **Comprehensive Job Details**: Extended `GetJobDetailsResult` with additional fields:
  - `salary`: Salary/compensation information
  - `contract_duration`: Contract duration
  - `job_type`: Type of position (arbeitsstelle, ausbildung, praktikum)
  - `first_published`: First publication date
  - `only_for_disabled`: Whether job is only for severely disabled persons
  - `partner_url`: Partner URL

**Note**: The following fields are no longer available due to API changes in jobsuche 0.3.0:
  - `employer_profile_url` (removed from API)
  - `takeover_opportunity` (removed from API)
  - `open_positions` (removed from API)
  - `company_size` (removed from API)
  - `skills_required` (removed from API)
  - `leadership_skills` (removed from API)
  - `employer_description` (removed from API)
  - `branch` (removed from API)
  - `published_date` (removed from API)
  - `mobility` (removed from API)
  - `suitable_for_refugees` (removed from API)
  - `contract_type` (removed from API)

These fields remain in the response structure for backward compatibility but will always return `None`.

- **Enhanced Search Filters**: Added new search parameters:
  - `employer`: Search by employer name (e.g., "BARMER", "Siemens")
  - `branch`: Search by industry/branch (e.g., "IT", "Gesundheitswesen")

### Changed

- Now utilizing 27 out of 36 available API fields (previously 10)
- Improved search functionality by combining job_title, employer, and branch parameters
- Enhanced documentation with complete field reference and API limitations

### Fixed

- Fixed `external_url` mapping in job details (no longer hardcoded to None)
- Added proper documentation for API limitations and workarounds

## [0.1.1] - 2025-10-22

### Changed

- Updated release workflow configuration
- Updated package keywords to meet crates.io requirements

## [0.1.0] - 2025-10-21

### Added

- Initial release
- Basic job search functionality via German Federal Employment Agency API
- Search filters for location, job title, employment type, contract type
- Job details retrieval
- Server status endpoint
- Pagination support
- Zero-configuration setup with sensible defaults
