#!/usr/bin/env node

const path = require("path");

// Export the binary path for this platform
module.exports = path.join(__dirname, "jobsuche-mcp-server.exe");
