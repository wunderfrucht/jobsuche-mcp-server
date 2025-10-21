//! Jobsuche MCP Server - AI-friendly job search via MCP
//!
//! This server provides tools for searching German job listings without
//! requiring knowledge of the Bundesagentur für Arbeit API internals.

use jobsuche_mcp_server::JobsucheMcpServer;
use pulseengine_mcp_server::McpServerBuilder;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure logging for STDIO transport
    JobsucheMcpServer::configure_stdio_logging();

    info!("Starting Jobsuche MCP Server...");

    // Create the Jobsuche MCP server instance
    let jobsuche_server = match JobsucheMcpServer::new() {
        Ok(server) => {
            info!("Jobsuche MCP Server created successfully");
            server
        }
        Err(e) => {
            error!("Failed to create Jobsuche MCP Server: {}", e);
            eprintln!("Failed to start Jobsuche MCP Server: {}", e);
            eprintln!("\nPlease check:");
            eprintln!(
                "  - JOBSUCHE_API_URL environment variable (optional, uses default if not set)"
            );
            eprintln!(
                "  - JOBSUCHE_API_KEY environment variable (optional, uses default if not set)"
            );
            eprintln!("\nFor help, see the README.md file.");
            std::process::exit(1);
        }
    };

    info!("Starting MCP server with STDIO transport...");

    // Start the server using the macro-generated infrastructure
    let mut server = jobsuche_server.serve_stdio().await?;

    info!("Jobsuche MCP Server is running and ready to serve requests");

    server.run().await?;

    Ok(())
}
