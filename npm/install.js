#!/usr/bin/env node

const os = require("os");
const path = require("path");
const fs = require("fs");
const https = require("https");
const crypto = require("crypto");
const { execFileSync } = require("child_process");

// Check if platform packages are available first
const { getPlatformPackageName } = require("./index.js");

try {
  const platformPackage = getPlatformPackageName();
  try {
    require.resolve(platformPackage);
    console.log(`✅ Platform package ${platformPackage} is already available`);
    process.exit(0);
  } catch (err) {
    console.log(
      `⚠️ Platform package ${platformPackage} not found, falling back to GitHub release download`,
    );
  }
} catch (err) {
  console.log(
    "⚠️ Platform not supported for platform packages, using GitHub release download",
  );
}

// Fallback to GitHub releases download
function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  let platform;
  let archSuffix;

  if (type === "Windows_NT") {
    platform = "pc-windows-msvc";
  } else if (type === "Linux") {
    platform = "unknown-linux-gnu";
  } else if (type === "Darwin") {
    platform = "apple-darwin";
  } else {
    throw new Error(`Unsupported platform: ${type}`);
  }

  if (arch === "x64") {
    archSuffix = "x86_64";
  } else if (arch === "arm64") {
    archSuffix = "aarch64";
  } else {
    throw new Error(
      `Unsupported architecture: ${arch}. Supported architectures: x64 (Intel), arm64 (Apple Silicon). Please install from source: cargo install --git https://github.com/wunderfrucht/jobsuche-mcp-server.git jobsuche-mcp-server`,
    );
  }

  return `${archSuffix}-${platform}`;
}

function getBinaryName() {
  return os.type() === "Windows_NT"
    ? "jobsuche-mcp-server.exe"
    : "jobsuche-mcp-server";
}

function getArchiveName(version, platform) {
  const ext = os.type() === "Windows_NT" ? "zip" : "tar.gz";
  return `jobsuche-mcp-server-v${version}-${platform}.${ext}`;
}

function getDownloadUrl(version, archiveName) {
  return `https://github.com/wunderfrucht/jobsuche-mcp-server/releases/download/v${version}/${archiveName}`;
}

function getChecksumUrl(version, archiveName) {
  return `https://github.com/wunderfrucht/jobsuche-mcp-server/releases/download/v${version}/${archiveName}.sha256`;
}

function downloadFile(url, destination, redirectCount) {
  if ((redirectCount || 0) > 5) {
    return Promise.reject(new Error("Too many redirects"));
  }
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(destination);

    https
      .get(url, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          const location = response.headers.location;
          if (!location) {
            reject(new Error("Redirect without location header"));
            return;
          }
          file.destroy();
          fs.unlink(destination, () => {});
          return downloadFile(location, destination, (redirectCount || 0) + 1)
            .then(resolve)
            .catch(reject);
        }

        if (response.statusCode !== 200) {
          reject(
            new Error(`HTTP ${response.statusCode}: ${response.statusMessage}`),
          );
          return;
        }

        response.pipe(file);

        file.on("finish", () => {
          file.close();
          resolve();
        });

        file.on("error", (err) => {
          fs.unlink(destination, () => {});
          reject(err);
        });
      })
      .on("error", (err) => {
        reject(err);
      });
  });
}

function downloadText(url, redirectCount) {
  if ((redirectCount || 0) > 5) {
    return Promise.reject(new Error("Too many redirects"));
  }
  const MAX_SIZE = 2048;
  return new Promise((resolve, reject) => {
    https
      .get(url, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          const location = response.headers.location;
          if (!location) {
            reject(new Error("Redirect without location header"));
            return;
          }
          return downloadText(location, (redirectCount || 0) + 1)
            .then(resolve)
            .catch(reject);
        }

        if (response.statusCode !== 200) {
          reject(
            new Error(`HTTP ${response.statusCode}: ${response.statusMessage}`),
          );
          return;
        }

        let data = "";
        let size = 0;
        response.on("data", (chunk) => {
          size += chunk.length;
          if (size > MAX_SIZE) {
            response.destroy();
            reject(new Error("Checksum file too large (>2KB)"));
            return;
          }
          data += chunk;
        });
        response.on("end", () => resolve(data));
      })
      .on("error", reject);
  });
}

function computeSHA256(filePath) {
  const content = fs.readFileSync(filePath);
  return crypto.createHash("sha256").update(content).digest("hex");
}

async function verifyChecksum(archivePath, checksumUrl) {
  console.log("🔒 Verifying checksum...");
  try {
    const checksumContent = await downloadText(checksumUrl);
    // Format: "<sha256>  <filename>" or "<sha256> <filename>" or just "<sha256>"
    const expectedHash = checksumContent.trim().split(/\s+/)[0].toLowerCase();
    if (!expectedHash || !/^[0-9a-f]{64}$/.test(expectedHash)) {
      throw new Error("Invalid checksum format in checksum file");
    }
    const actualHash = computeSHA256(archivePath);
    if (actualHash !== expectedHash) {
      throw new Error(
        `Checksum mismatch!\n  Expected: ${expectedHash}\n  Got:      ${actualHash}\n\nAbort: refusing to extract potentially tampered archive.`,
      );
    }
    console.log("✅ Checksum verified");
  } catch (err) {
    if (err.message.startsWith("Checksum mismatch")) {
      // Always abort on mismatch - this is a hard failure
      throw err;
    }
    // Warn but continue if the checksum file is simply not present (e.g. older release)
    console.warn(`⚠️  Could not verify checksum: ${err.message}`);
    console.warn("    Continuing without verification — upgrade to a newer release for checksum support.");
  }
}

async function installBinary() {
  const version = require("./package.json").version;
  const platform = getPlatform();
  const binaryName = getBinaryName();
  const archiveName = getArchiveName(version, platform);
  const downloadUrl = getDownloadUrl(version, archiveName);
  const checksumUrl = getChecksumUrl(version, archiveName);

  console.log(`Platform detected: ${os.type()} ${os.arch()}`);
  console.log(`Target: ${platform}`);
  console.log(`Binary: ${binaryName}`);
  console.log(`Download URL: ${downloadUrl}`);

  const binDir = path.join(__dirname, "bin");
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const archivePath = path.join(binDir, archiveName);

  console.log("📥 Downloading binary...");
  try {
    await downloadFile(downloadUrl, archivePath);
    console.log("✅ Download completed");
  } catch (err) {
    throw new Error(`Download failed: ${err.message}`);
  }

  // Verify integrity before extracting
  await verifyChecksum(archivePath, checksumUrl);

  console.log("📦 Extracting binary...");
  try {
    if (os.type() === "Windows_NT") {
      // Use execFileSync with array args to prevent shell injection
      execFileSync(
        "powershell.exe",
        [
          "-NoProfile",
          "-NonInteractive",
          "-Command",
          // Single-quote doubling is PowerShell's escaping mechanism
          `Expand-Archive -LiteralPath '${archivePath.replace(/'/g, "''")}' -DestinationPath '${binDir.replace(/'/g, "''")}' -Force`,
        ],
        { stdio: "inherit" },
      );
    } else {
      // Use execFileSync with array args to prevent shell injection
      execFileSync("tar", ["-xzf", archivePath], {
        cwd: binDir,
        stdio: "inherit",
      });
    }

    fs.unlinkSync(archivePath);

    if (os.type() !== "Windows_NT") {
      const binaryPath = path.join(binDir, binaryName);
      if (fs.existsSync(binaryPath)) {
        fs.chmodSync(binaryPath, 0o755);
      }
    }

    console.log("✅ Binary installed successfully");
  } catch (err) {
    throw new Error(`Extraction failed: ${err.message}`);
  }
}

installBinary().catch((err) => {
  console.error("❌ Failed to install jobsuche-mcp-server binary:", err.message);
  console.error("\n📋 Installation failed. You can:");
  console.error("1. Install Rust and build from source:");
  console.error(
    "   git clone https://github.com/wunderfrucht/jobsuche-mcp-server.git",
  );
  console.error("   cd jobsuche-mcp-server");
  console.error("   cargo install --path jobsuche-mcp-server");
  console.error("");
  console.error("2. Download binary manually from:");
  console.error(
    "   https://github.com/wunderfrucht/jobsuche-mcp-server/releases",
  );
  process.exit(1);
});
