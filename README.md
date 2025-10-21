# Template MCP Server

A template repository for creating Model Context Protocol (MCP) servers using the PulseEngine MCP framework in Rust.

## 🚀 Getting Started

### Option 1: Automated Setup (Recommended) ⚡

The fastest way to customize this template for your project:

```bash
# 1. Use this template on GitHub (click "Use this template" button)
# 2. Clone your new repository
git clone https://github.com/yourusername/your-mcp-server.git
cd your-mcp-server

# 3. Run the initialization script
./init.sh

# 4. Follow the prompts to configure your server
# The script will automatically:
#   - Replace all template placeholders
#   - Rename directories
#   - Update package.json, Cargo.toml, and all config files
#   - Validate all changes

# 5. Build and test
cargo build --release
./target/release/your-mcp-server
```

The initialization script takes ~2 minutes and handles all the tedious find-and-replace work automatically.

### Option 2: Manual Setup 🔧

If you prefer to set up manually:

<details>
<summary>Click to expand manual setup instructions</summary>

#### 1. Clone the template

```bash
git clone https://github.com/yourusername/your-mcp-server.git
cd your-mcp-server
```

#### 2. Replace placeholders (use checklist)

**Required replacements across all files:**

- [ ] `@yourusername/template-mcp-server` → your scoped package name (e.g., `@mycompany/myapp-mcp-server`)
- [ ] `template-mcp-server` → your package name (e.g., `myapp-mcp-server`)
- [ ] `@yourusername` → your npm scope (e.g., `@mycompany`)
- [ ] `yourusername` in GitHub URLs → your GitHub username
- [ ] `Your Name <your.email@example.com>` → your author info
- [ ] `Template MCP Server` → your server name

**Files to update:**

- All `package.json` files (npm/, platform-packages/\*/)
- All `.js` files (npm/_.js, platform-packages/_/index.js, scripts/\*.js)
- Both `Cargo.toml` files (root and template-mcp-server/)
- `.github/workflows/npm-publish.yml`
- `README.md`, `PUBLISHING.md`, `CONTRIBUTING.md`

#### 3. Rename directory

```bash
mv template-mcp-server your-package-name
# Update Cargo.toml workspace members to match
```

#### 4. Validate your changes

```bash
./scripts/validate.sh
```

#### 5. Build and test

```bash
cargo build
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/debug/your-mcp-server
```

</details>

### Quick Start After Setup

Once initialized, build and test your server:

```bash
# Build
cargo build --release

# Test tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/release/your-mcp-server

# Test resources
echo '{"jsonrpc":"2.0","id":2,"method":"resources/list","params":{}}' | ./target/release/your-mcp-server
```

## 🛠 What's Included

This template provides:

- **Complete MCP server setup** using PulseEngine MCP framework
- **Automatic tool & resource discovery** with `#[mcp_tools]` and `#[mcp_resource]` macros
- **Example tools** demonstrating different parameter types:
  - Simple status check (no parameters)
  - Echo with optional parameters
  - Numeric calculations
  - Structured data creation
  - List processing
  - Error handling examples
- **Example resources** for read-only data access:
  - Server status information (`template://server-status`)
  - Server configuration (`template://server-config`)
  - Parameterized data lookup (`template://example-data/{id}`)
- **URI template support** for parameterized resources
- **STDIO transport** for integration with MCP clients
- **Proper logging configuration** for debugging

## 📁 Project Structure

```
template-mcp-server/
├── Cargo.toml                    # Workspace configuration
├── template-mcp-server/
│   ├── Cargo.toml                # Package configuration
│   ├── src/
│   │   ├── main.rs               # Server entry point
│   │   └── lib.rs                # Server implementation & tools
├── README.md                     # This file
├── LICENSE                       # MIT License
└── .github/                      # GitHub templates
    ├── ISSUE_TEMPLATE/
    ├── PULL_REQUEST_TEMPLATE.md
    └── dependabot.yml
```

## 📦 Installation

### From npm (Recommended)

Install globally to use with any MCP client:

```bash
npm install -g @yourusername/template-mcp-server
```

Or use directly with npx:

```bash
npx @yourusername/template-mcp-server
```

### From Source

1. **Prerequisites**

   - Rust 1.75.0 or later
   - Git
   - Node.js 16+ (for npm distribution)

2. **Clone and Build**

   ```bash
   git clone https://github.com/yourusername/template-mcp-server.git
   cd template-mcp-server
   cargo build --release
   ```

3. **Run the Server**

   ```bash
   ./target/release/template-mcp-server
   ```

### Platform-Specific Binaries

Pre-built binaries are available for:

- macOS (x64, arm64)
- Linux (x64, arm64)
- Windows (x64)

Download from [GitHub Releases](https://github.com/yourusername/template-mcp-server/releases)

## 🔧 Development

### Pre-commit Hooks Setup

This template includes pre-commit hooks for code quality:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install the git hooks
pre-commit install

# Run manually on all files
pre-commit run --all-files
```

The hooks include:

- **Rust formatting** (cargo fmt)
- **Rust linting** (cargo clippy)
- **Secret detection** (detect-secrets)
- **File checks** (trailing whitespace, EOF, YAML/JSON/TOML validation)
- **Markdown linting**
- **JavaScript/JSON formatting** (prettier)

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing with MCP Inspector

```bash
# Install MCP Inspector
npm install -g @modelcontextprotocol/inspector

# Test your server
npx @modelcontextprotocol/inspector ./target/debug/template-mcp-server
```

### Testing with Direct JSON-RPC

```bash
# List available tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/debug/template-mcp-server

# Call a tool
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_status","arguments":{}}}' | ./target/debug/template-mcp-server

# List available resources
echo '{"jsonrpc":"2.0","id":3,"method":"resources/list","params":{}}' | ./target/debug/template-mcp-server

# Read a resource
echo '{"jsonrpc":"2.0","id":4,"method":"resources/read","params":{"uri":"template://server-status"}}' | ./target/debug/template-mcp-server
```

## 🔍 Tools vs Resources

This template demonstrates both **MCP Tools** and **MCP Resources**:

### Tools (Operations)

Tools are functions that **perform operations** or **modify state**. They:

- Take parameters as input
- Can have side effects (create, update, delete)
- Return results from their execution
- Are called via `tools/call` method

**Examples in template:**

- `get_status()` - Checks server status
- `echo(message, prefix)` - Transforms input
- `add_numbers(a, b)` - Performs calculations
- `create_data(...)` - Creates new data

### Resources (Read-Only Data)

Resources provide **read-only access to data**. They:

- Use URI templates for identification
- Cannot modify state (read-only)
- Are accessed via `resources/read` method
- Perfect for configuration, status, or reference data

**Examples in template:**

- `template://server-status` - Current server status
- `template://server-config` - Server configuration
- `template://example-data/{id}` - Data lookup by ID

### When to Use Each

| Use Tools For        | Use Resources For      |
| -------------------- | ---------------------- |
| Operations & actions | Read-only data access  |
| Data modification    | Configuration settings |
| Calculations         | Status information     |
| API calls            | Reference data         |
| File operations      | Cached data            |
| Dynamic processing   | Static information     |

## 📝 Customizing Your Server

### 1. Update Package Information

Edit `template-mcp-server/Cargo.toml`:

```toml
[package]
name = "your-mcp-server"
description = "Your server description"
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/yourusername/your-mcp-server"
```

### 2. Implement Your Tools

In `src/lib.rs`, modify the `#[mcp_tools]` impl block:

```rust
#[mcp_tools]
impl YourMcpServer {
    /// Your custom tool
    pub async fn your_tool(&self, param: String) -> anyhow::Result<String> {
        // Your implementation here
        Ok(format!("Result: {}", param))
    }
}
```

### 3. Add Server State

Add fields to your server struct:

```rust
#[mcp_server(name = "Your Server")]
#[derive(Clone)]
pub struct YourMcpServer {
    data: Arc<RwLock<HashMap<String, String>>>,
    config: YourConfig,
}
```

### 4. Update Server Configuration

Modify the `#[mcp_server]` attributes:

```rust
#[mcp_server(
    name = "Your Amazing MCP Server",
    version = "1.0.0",
    description = "Does amazing things",
    auth = "file"  // or "memory", "disabled"
)]
```

## 🔌 Integration with MCP Clients

### Claude Desktop

Using npm installation:

```json
{
  "servers": {
    "your-server": {
      "command": "npx",
      "args": ["@yourusername/template-mcp-server"]
    }
  }
}
```

Using local binary:

```json
{
  "servers": {
    "your-server": {
      "command": "/path/to/your-mcp-server",
      "args": []
    }
  }
}
```

### Continue.dev

Using npm installation:

```json
{
  "mcpServers": {
    "your-server": {
      "command": "npx",
      "args": ["@yourusername/template-mcp-server"]
    }
  }
}
```

Using local binary:

```json
{
  "mcpServers": {
    "your-server": {
      "command": "/path/to/your-mcp-server"
    }
  }
}
```

## 📚 Framework Features

This template uses the PulseEngine MCP framework which provides:

- **Automatic tool discovery** - Public methods become MCP tools
- **Type-safe parameter handling** - Automatic JSON deserialization
- **Error handling** - Proper MCP error responses
- **Authentication** - Optional auth with multiple backends
- **Transport support** - STDIO, HTTP, WebSocket
- **Monitoring** - Built-in metrics and tracing
- **Validation** - Request/response validation

## 🔐 Authentication

The template includes authentication support:

- `auth = "disabled"` - No authentication (development)
- `auth = "memory"` - In-memory auth (testing)
- `auth = "file"` - File-based auth (production)

For production use, configure file-based auth:

```rust
#[mcp_server(auth = "file")]
```

## 📊 Monitoring & Debugging

The server includes comprehensive logging. Set log levels:

```bash
RUST_LOG=debug ./target/debug/template-mcp-server
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This template is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🔍 Troubleshooting

### Common Issues After Setup

**"Found unreplaced placeholders" error**

- Run `./scripts/validate.sh` to see which placeholders remain
- Use `./init.sh` to automatically fix all placeholders
- Or manually search and replace the listed placeholders

**Build fails with missing dependencies**

- Make sure you renamed the `template-mcp-server` directory to match your package name
- Update `Cargo.toml` workspace members to reference the new directory name
- Run `cargo clean && cargo build`

**NPM publish fails**

- Verify all `package.json` files have matching versions
- Check that platform package names match the main package's `optionalDependencies`
- Ensure you have the correct NPM_TOKEN secret configured

**Pre-commit hooks fail**

- Install required tools: `pip install pre-commit detect-secrets`
- Install Rust components: `rustup component add rustfmt clippy`
- Run `pre-commit install` to set up the hooks
- If detect-secrets fails, update `.secrets.baseline`: `detect-secrets scan --baseline .secrets.baseline`

**CI/CD workflow issues**

- Ensure all GitHub secrets are configured (NPM_TOKEN)
- Check that workflow references use your package name, not `template-mcp-server`
- Verify binary names in workflows match your package name

### Validation

Run the validation script anytime to check for configuration issues:

```bash
./scripts/validate.sh
```

This will check for:

- Unreplaced template placeholders
- Incorrect package names
- Missing author information
- Broken repository URLs

## 🆘 Support

- [PulseEngine MCP Documentation](https://docs.rs/pulseengine-mcp-protocol)
- [MCP Specification](https://modelcontextprotocol.io/specification/2025-06-18)
- [GitHub Issues](https://github.com/yourusername/your-mcp-server/issues)

## 🏷 Template Usage

When using this template:

1. **Click "Use this template"** on GitHub
2. **Create your repository** with a descriptive name
3. **Clone and customize** as described above
4. **Delete this section** from your README
5. **Update all placeholder information** with your project details

Happy building! 🎉
