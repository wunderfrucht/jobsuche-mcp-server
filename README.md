# Jobsuche MCP Server

An AI-friendly job search integration server using the Model Context Protocol (MCP). This server provides tools for searching German job listings via the Federal Employment Agency (Bundesagentur für Arbeit) API.

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
  "raw_data": { ... }
}
```

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

- Results are sorted oldest-to-newest (no custom sorting available)
- Maximum 100 results per page
- Job details may return 404 if jobs expire quickly
- Employer search is case-sensitive and exact-match only

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
