#!/usr/bin/env node

const https = require("https");
const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

const GITHUB_RELEASES_URL =
  "https://api.github.com/repos/wunderfrucht/jobsuche-mcp-server/releases/latest";

async function downloadBinary() {
  // Only download in production installs, not during development
  if (
    process.env.NODE_ENV === "development" ||
    fs.existsSync(path.join(__dirname, "..", "Cargo.toml"))
  ) {
    console.log("Development environment detected, skipping binary download");
    return;
  }

  const platform = process.platform;
  const arch = process.arch;

  // Map to expected binary names in GitHub releases
  const binaryMap = {
    "darwin-x64": "jobsuche-mcp-server-darwin-x64",
    "darwin-arm64": "jobsuche-mcp-server-darwin-arm64",
    "linux-x64": "jobsuche-mcp-server-linux-x64",
    "linux-arm64": "jobsuche-mcp-server-linux-arm64",
    "win32-x64": "jobsuche-mcp-server-windows-x64.exe",
  };

  const binaryName = binaryMap[`${platform}-${arch}`];
  if (!binaryName) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    console.error("You may need to build from source");
    process.exit(1);
  }

  console.log(`Downloading binary for ${platform}-${arch}...`);

  try {
    // Get latest release info
    const releaseInfo = await fetchJson(GITHUB_RELEASES_URL);

    // Find the asset URL for our platform
    const asset = releaseInfo.assets.find((a) => a.name === binaryName);
    if (!asset) {
      throw new Error(`No binary found for ${platform}-${arch}`);
    }

    // Find the matching checksum asset
    const checksumAsset = releaseInfo.assets.find(
      (a) => a.name === `${binaryName}.sha256`,
    );

    const distDir = path.join(__dirname, "..", "dist");
    if (!fs.existsSync(distDir)) {
      fs.mkdirSync(distDir, { recursive: true });
    }

    const localBinaryName =
      platform === "win32" ? "jobsuche-mcp-server.exe" : "jobsuche-mcp-server";
    const binaryPath = path.join(distDir, localBinaryName);

    await downloadFile(asset.browser_download_url, binaryPath);

    // Verify checksum if available
    if (checksumAsset) {
      await verifyChecksum(binaryPath, checksumAsset.browser_download_url);
    } else {
      console.warn("⚠️  No checksum file found for this release — skipping integrity check.");
    }

    fs.chmodSync(binaryPath, 0o755);
    console.log("Binary downloaded successfully");
  } catch (error) {
    console.error("Failed to download binary:", error.message);
    console.error(
      'You may need to build from source using "cargo build --release"',
    );
    // Don't exit with error - allow npm install to complete
  }
}

function fetchJson(url, redirectCount) {
  if ((redirectCount || 0) > 5) {
    return Promise.reject(new Error("Too many redirects"));
  }
  const MAX_SIZE = 512 * 1024; // 512 KB is more than enough for a release JSON
  return new Promise((resolve, reject) => {
    https
      .get(url, { headers: { "User-Agent": "jobsuche-mcp-server" } }, (res) => {
        if (res.statusCode === 302 || res.statusCode === 301) {
          const location = res.headers.location;
          if (!location) {
            reject(new Error("Redirect without location header"));
            return;
          }
          return fetchJson(location, (redirectCount || 0) + 1)
            .then(resolve)
            .catch(reject);
        }

        if (res.statusCode !== 200) {
          reject(new Error(`HTTP ${res.statusCode}: ${res.statusMessage}`));
          return;
        }

        let data = "";
        let size = 0;
        res.on("data", (chunk) => {
          size += chunk.length;
          if (size > MAX_SIZE) {
            res.destroy();
            reject(new Error("Response too large"));
            return;
          }
          data += chunk;
        });
        res.on("end", () => {
          try {
            resolve(JSON.parse(data));
          } catch (e) {
            reject(e);
          }
        });
      })
      .on("error", reject);
  });
}

function downloadFile(url, dest, redirectCount) {
  if ((redirectCount || 0) > 5) {
    return Promise.reject(new Error("Too many redirects"));
  }
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https
      .get(
        url,
        { headers: { "User-Agent": "jobsuche-mcp-server" } },
        (response) => {
          if (response.statusCode === 302 || response.statusCode === 301) {
            const location = response.headers.location;
            if (!location) {
              reject(new Error("Redirect without location header"));
              return;
            }
            file.destroy();
            fs.unlink(dest, () => {});
            downloadFile(location, dest, (redirectCount || 0) + 1)
              .then(resolve)
              .catch(reject);
            return;
          }

          if (response.statusCode !== 200) {
            reject(new Error(`HTTP ${response.statusCode}: ${response.statusMessage}`));
            return;
          }

          response.pipe(file);
          file.on("finish", () => {
            file.close(resolve);
          });
        },
      )
      .on("error", (err) => {
        fs.unlink(dest, () => {});
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
      .get(url, { headers: { "User-Agent": "jobsuche-mcp-server" } }, (res) => {
        if (res.statusCode === 302 || res.statusCode === 301) {
          const location = res.headers.location;
          if (!location) {
            reject(new Error("Redirect without location header"));
            return;
          }
          return downloadText(location, (redirectCount || 0) + 1)
            .then(resolve)
            .catch(reject);
        }

        if (res.statusCode !== 200) {
          reject(new Error(`HTTP ${res.statusCode}: ${res.statusMessage}`));
          return;
        }

        let data = "";
        let size = 0;
        res.on("data", (chunk) => {
          size += chunk.length;
          if (size > MAX_SIZE) {
            res.destroy();
            reject(new Error("Checksum file too large (>2KB)"));
            return;
          }
          data += chunk;
        });
        res.on("end", () => resolve(data));
      })
      .on("error", reject);
  });
}

async function verifyChecksum(binaryPath, checksumUrl) {
  console.log("🔒 Verifying checksum...");
  const checksumContent = await downloadText(checksumUrl);
  const expectedHash = checksumContent.trim().split(/\s+/)[0].toLowerCase();
  if (!expectedHash || !/^[0-9a-f]{64}$/.test(expectedHash)) {
    throw new Error("Invalid checksum format");
  }
  const content = fs.readFileSync(binaryPath);
  const actualHash = crypto.createHash("sha256").update(content).digest("hex");
  if (actualHash !== expectedHash) {
    // Remove the downloaded file to avoid leaving a potentially tampered binary
    fs.unlinkSync(binaryPath);
    throw new Error(
      `Checksum mismatch!\n  Expected: ${expectedHash}\n  Got:      ${actualHash}`,
    );
  }
  console.log("✅ Checksum verified");
}

// Run if this is a postinstall script
if (require.main === module) {
  downloadBinary().catch(console.error);
}
