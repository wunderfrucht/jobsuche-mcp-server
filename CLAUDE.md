# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Run a single test
cargo test test_name

# Test via direct JSON-RPC
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/debug/jobsuche-mcp-server
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search_jobs","arguments":{"location":"Berlin","page_size":5}}}' | ./target/debug/jobsuche-mcp-server

# Test with MCP Inspector (requires: npm install -g @modelcontextprotocol/inspector)
npx @modelcontextprotocol/inspector ./target/debug/jobsuche-mcp-server
```

## Architecture

This is a **Rust MCP (Model Context Protocol) server** that wraps the German Federal Employment Agency (Bundesagentur für Arbeit) job search API.

### Structure

- **`jobsuche-mcp-server/src/lib.rs`** — Core server implementation. Contains all tool parameter structs, result structs, and the `JobsucheMcpServer` struct with the `#[mcp_server]` and `#[mcp_tools]` macros from PulseEngine.
- **`jobsuche-mcp-server/src/main.rs`** — Entrypoint: creates the server and runs it via STDIO transport.
- **`jobsuche-mcp-server/src/config.rs`** — `JobsucheConfig` loaded from env vars; validates page sizes and API URL.
- **`Cargo-dev.toml`** — Workspace Cargo.toml for development with local framework paths (rename to `Cargo.toml` when working locally with PulseEngine framework).

### Key Dependencies

- **`pulseengine-mcp-server`** (v0.13.0) — MCP server framework; `#[mcp_server]` macro generates STDIO transport boilerplate
- **`pulseengine-mcp-macros`** — `#[mcp_tools]` macro auto-registers tool methods on the server struct
- **`jobsuche`** (v0.3.0, async feature) — Rust crate for the Bundesagentur für Arbeit API
- **`schemars`** — Derives `JsonSchema` on param structs (required by `#[mcp_tools]` for tool discovery)

### Tool Pattern

Each MCP tool is an `async fn` on `JobsucheMcpServer`. Tools take a typed params struct (implementing `JsonSchema + Deserialize`) and return `anyhow::Result<SomeResult>`. The `#[mcp_tools]` macro exposes these automatically.

Available tools: `search_jobs`, `get_job_details`, `search_jobs_with_details`, `batch_search_jobs`, `get_server_status`.

### npm Distribution

The `npm/` directory and `platform-packages/` handle cross-platform binary distribution via npm. The binary is compiled per platform and bundled as optional dependencies. `scripts/prepare-dist.js` handles packaging.

### Rate Limiting

`search_jobs_with_details` inserts 100ms delays between detail fetches; `batch_search_jobs` inserts 200ms delays between searches and 100ms between details. This is intentional to respect the BA API.

## Commit Style

Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.
