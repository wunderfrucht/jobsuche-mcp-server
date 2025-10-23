# Jobsuche MCP Server

An AI-friendly job search integration server using the Model Context Protocol (MCP).
This server provides tools for searching German job listings via the Federal Employment
Agency (Bundesagentur für Arbeit) API.

## Features

- **AI-Friendly Interface**: Simple, semantic parameters for job searching
- **Official API Integration**: Uses the `jobsuche` crate for reliable API access
- **Rich Filtering**: Search by location, job title, employment type, contract type, and more
- **Comprehensive Details**: Get full job information including descriptions and requirements
- **Pagination Support**: Handle large result sets efficiently
- **Zero Configuration**: Works out of the box with sensible defaults

## Installation

### From npm (Recommended)

```bash
npm install -g @wunderfrucht/jobsuche-mcp-server
```

### From Source

```bash
git clone https://github.com/wunderfrucht/jobsuche-mcp-server.git
cd jobsuche-mcp-server
cargo build --release
```

## Configuration

The server uses environment variables for configuration (all optional):

- `JOBSUCHE_API_URL`: API base URL (default: official Bundesagentur für Arbeit API)
- `JOBSUCHE_API_KEY`: Custom API key (default: public API key)
- `JOBSUCHE_DEFAULT_PAGE_SIZE`: Default results per page (default: 25)
- `JOBSUCHE_MAX_PAGE_SIZE`: Maximum results per page (default: 100)

## Usage with MCP Clients

### Claude Desktop

Add to your Claude Desktop configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "jobsuche": {
      "command": "npx",
      "args": ["@wunderfrucht/jobsuche-mcp-server"]
    }
  }
}
```

Or using local binary:

```json
{
  "mcpServers": {
    "jobsuche": {
      "command": "/path/to/jobsuche-mcp-server"
    }
  }
}
```

### Continue.dev

Add to your Continue configuration:

```json
{
  "mcpServers": {
    "jobsuche": {
      "command": "npx",
      "args": ["@wunderfrucht/jobsuche-mcp-server"]
    }
  }
}
```

## Available Tools

### 1. `search_jobs`

Search for jobs in Germany using various filters.

**Parameters:**

- `job_title` (optional): Job title or keywords (e.g., "Software Engineer", "Data Scientist")
- `location` (optional): Location name (e.g., "Berlin", "München", "Deutschland")
- `radius_km` (optional): Search radius in kilometers from the location
- `employment_type` (optional): Employment type filter
  - Options: `"fulltime"`, `"parttime"`, `"mini_job"`, `"home_office"`, `"shift"`
- `contract_type` (optional): Contract type filter
  - Options: `"permanent"`, `"temporary"`
- `published_since_days` (optional): Days since publication (0-100, default: 30)
- `page_size` (optional): Number of results per page (1-100)
- `page` (optional): Page number for pagination (starting from 1)
- `employer` (optional): Employer name to search for (e.g., "BARMER", "Siemens")
- `branch` (optional): Industry/branch to search in (e.g., "IT", "Gesundheitswesen")

**Examples:**

```json
{
  "job_title": "Software Engineer",
  "location": "Berlin",
  "employment_type": ["fulltime"]
}
```

```json
{
  "location": "München",
  "published_since_days": 7,
  "radius_km": 50
}
```

```json
{
  "job_title": "Data Scientist",
  "location": "Deutschland",
  "employment_type": ["fulltime", "parttime"],
  "page_size": 50
}
```

```json
{
  "employer": "BARMER",
  "location": "Wuppertal",
  "employment_type": ["parttime"]
}
```

### 2. `get_job_details`

Get detailed information about a specific job posting.

**Parameters:**

- `reference_number` (required): Job reference number from search results

**Example:**

```json
{
  "reference_number": "10001-1234567890-S"
}
```

### 3. `get_server_status`

Get server status and connection information.

**Example:**

```json
{}
```

## Response Examples

### Search Jobs Response

```json
{
  "total_results": 1523,
  "current_page": 1,
  "page_size": 25,
  "jobs_count": 25,
  "jobs": [
    {
      "reference_number": "10001-1234567890-S",
      "title": "Software Engineer (m/w/d)",
      "employer": "Example GmbH",
      "location": "Berlin (10115)",
      "published_date": "2025-10-15",
      "external_url": null
    }
  ],
  "search_duration_ms": 342
}
```

### Job Details Response

```json
{
  "reference_number": "10001-1234567890-S",
  "title": "Software Engineer (m/w/d)",
  "description": "We are looking for an experienced software engineer...",
  "employer": "Example GmbH",
  "location": "Berlin",
  "employment_type": "Vollzeit",
  "contract_type": "unbefristet",
  "start_date": "2025-11-01",
  "application_deadline": null,
  "contact_info": null,
  "external_url": null,
  "employer_profile_url": null,
  "partner_url": "https://example.com/partner",
  "salary": "50.000 - 70.000 EUR",
  "contract_duration": "12 Monate",
  "takeover_opportunity": null,
  "job_type": "arbeitsstelle",
  "open_positions": null,
  "company_size": null,
  "employer_description": null,
  "branch": null,
  "published_date": null,
  "first_published": "2025-10-10",
  "only_for_disabled": false,
  "fulltime": true,
  "entry_period": "ab 2025-11-01",
  "publication_period": "2025-10-01 - 2025-11-30",
  "is_minor_employment": false,
  "is_temp_agency": false,
  "is_private_agency": false,
  "career_changer_suitable": true,
  "cipher_number": null,
  "raw_data": { ... }
}
```

**Available Fields:**

- **Basic Information:**
  - `reference_number`: Unique job reference
  - `title`: Job title
  - `description`: Full job description
  - `employer`: Company name
  - `location`: Job location

- **Employment Details:**
  - `employment_type`: Type of employment (Vollzeit, Teilzeit, derived from fulltime flag)
  - `fulltime`: Boolean indicator for full-time employment (new in v0.2.0)
  - `contract_duration`: Duration of contract (if temporary)
  - `start_date`: Expected start date (formatted from entry_period)
  - `entry_period`: Entry date range (new in v0.2.0)
  - `publication_period`: Publication date range (new in v0.2.0)

- **Employment Types (new in v0.2.0):**
  - `is_minor_employment`: Geringfügige Beschäftigung/Minijob
  - `is_temp_agency`: Temporary employment agency (Zeitarbeit)
  - `is_private_agency`: Private employment agency
  - `career_changer_suitable`: Suitable for career changers (Quereinsteiger)

- **Compensation:**
  - `salary`: Salary information (if available)

- **Application Information:**
  - `external_url`: External application URL (may be available in search results)
  - `partner_url`: Partner/alliance URL
  - `cipher_number`: Cipher number for anonymous postings (new in v0.2.0)
  - `application_deadline`: Application deadline (not available in API)
  - `contact_info`: Contact information (not available in API)

- **Additional Information:**
  - `job_type`: Type of position (arbeitsstelle, ausbildung, praktikum)
  - `first_published`: First publication date
  - `only_for_disabled`: Only for severely disabled persons
  - `raw_data`: Complete raw API response

- **Fields No Longer Available (API v0.3.0):**
  - `employer_profile_url`: Removed from API
  - `takeover_opportunity`: Removed from API
  - `open_positions`: Removed from API
  - `company_size`: Removed from API
  - `employer_description`: Removed from API
  - `branch`: Removed from API
  - `published_date`: Removed from API
  - `contract_type`: Removed from API

  *These fields remain in the response structure for backward compatibility but will always return `null`.*

## Development

### Prerequisites

- Rust 1.75.0 or later
- Node.js 16+ (for npm distribution)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Testing with MCP Inspector

```bash
npm install -g @modelcontextprotocol/inspector
npx @modelcontextprotocol/inspector ./target/debug/jobsuche-mcp-server
```

### Testing with Direct JSON-RPC

```bash
# List available tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/debug/jobsuche-mcp-server

# Call search_jobs
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search_jobs","arguments":{"location":"Berlin","page_size":5}}}' | ./target/debug/jobsuche-mcp-server
```

## API Information

This server uses the official Bundesagentur für Arbeit (German Federal Employment Agency) Jobsuche API:

- **Base URL**: `https://rest.arbeitsagentur.de/jobboerse/jobsuche-service`
- **Documentation**: Available via the `jobsuche` Rust crate
- **Rate Limiting**: Subject to API provider limits

### Known API Limitations

- **Contact Information**: The API does not provide direct contact details (email, phone) or application deadlines
- **External URLs**: May only be available in search results, not in detailed job information
- **Employer Search**: Combined with job title in search query (no dedicated filter)
- **Branch Search**: Combined with job title in search query (no dedicated filter)
- **API v0.3.0 Changes**: Several fields from previous API versions are no longer available (see "Fields No Longer Available" section)
- Results are sorted oldest-to-newest (no custom sorting available)
- Maximum 100 results per page
- Job details may return 404 if jobs expire quickly

### Workarounds for Missing Data

- **For Application URLs**: Use the `external_url` field from search results, or check the `partner_url` in job details
- **For Employer-Specific Search**: Use the `employer` parameter which combines with `job_title` in the search
- **For Removed Fields**: Check the `raw_data` field which contains the complete API response - some data may still be available in undocumented fields

## Troubleshooting

### Server won't start

- Check that the API URL is accessible
- Verify environment variables are set correctly
- Ensure you have internet connectivity

### No results found

- Try broader search criteria
- Check spelling of location names
- Increase `published_since_days` parameter
- Try removing employment type filters

### Connection errors

- Verify internet connectivity
- Check if the Bundesagentur für Arbeit API is accessible
- Try the default API URL without custom configuration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with [PulseEngine MCP Framework](https://github.com/wunderfrucht/pulseengine-mcp)
- Uses the [jobsuche](https://crates.io/crates/jobsuche) Rust crate
- Powered by the [Bundesagentur für Arbeit API](https://www.arbeitsagentur.de)

## Support

- [GitHub Issues](https://github.com/wunderfrucht/jobsuche-mcp-server/issues)
- [MCP Specification](https://modelcontextprotocol.io)
- [Jobsuche Crate Documentation](https://docs.rs/jobsuche)
