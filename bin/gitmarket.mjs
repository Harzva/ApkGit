#!/usr/bin/env node

import { createWriteStream, existsSync, mkdirSync, rmSync, chmodSync } from "node:fs";
import { homedir, platform, arch } from "node:os";
import { basename, join } from "node:path";
import { spawnSync } from "node:child_process";
import https from "node:https";

const repo = process.env.GITMARKET_REPO || "Harzva/GitReleaseMarket";
const requestedVersion = process.env.GITMARKET_VERSION || "latest";
const cacheRoot =
  process.env.GITMARKET_CACHE_DIR ||
  join(homedir(), ".cache", "gitmarket", "npx-release");

const targets = {
  "darwin:arm64": {
    asset: "gitmarket-macos-aarch64.tar.gz",
    binary: "gitmarket",
    extract: "tar",
  },
  "darwin:x64": {
    asset: "gitmarket-macos-x86_64.tar.gz",
    binary: "gitmarket",
    extract: "tar",
  },
  "linux:x64": {
    asset: "gitmarket-linux-x86_64.tar.gz",
    binary: "gitmarket",
    extract: "tar",
  },
  "win32:x64": {
    asset: "gitmarket-windows-x86_64.zip",
    binary: "gitmarket.exe",
    extract: "zip",
  },
};

main().catch((error) => {
  console.error(`gitmarket launcher failed: ${error.message}`);
  process.exit(1);
});

async function main() {
  const target = targets[`${platform()}:${arch()}`];
  if (!target) {
    throw new Error(`unsupported platform ${platform()} ${arch()}`);
  }

  const release = await getRelease();
  const asset = release.assets.find((item) => item.name === target.asset);
  if (!asset) {
    throw new Error(`release ${release.tag_name} does not contain ${target.asset}`);
  }

  const installDir = join(cacheRoot, release.tag_name, target.asset.replace(/\.(tar\.gz|zip)$/, ""));
  const binaryPath = join(installDir, "package", target.binary);
  if (!existsSync(binaryPath)) {
    mkdirSync(installDir, { recursive: true });
    const archivePath = join(installDir, basename(target.asset));
    await download(asset.browser_download_url, archivePath);
    extractArchive(target.extract, archivePath, installDir);
    if (platform() !== "win32") {
      chmodSync(binaryPath, 0o755);
    }
  }

  const child = spawnSync(binaryPath, process.argv.slice(2), { stdio: "inherit" });
  if (child.error) {
    throw child.error;
  }
  if (child.signal) {
    process.kill(process.pid, child.signal);
  }
  process.exit(child.status ?? 0);
}

async function getRelease() {
  const releasePath =
    requestedVersion === "latest"
      ? `/repos/${repo}/releases/latest`
      : `/repos/${repo}/releases/tags/${requestedVersion}`;
  return JSON.parse(await requestText(`https://api.github.com${releasePath}`));
}

function requestText(url, redirects = 0) {
  return new Promise((resolve, reject) => {
    const headers = {
      "User-Agent": "gitmarket-npx-launcher",
      Accept: "application/vnd.github+json",
    };
    if (process.env.GITHUB_TOKEN) {
      headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
    }

    https
      .get(url, { headers }, (response) => {
        if ([301, 302, 303, 307, 308].includes(response.statusCode ?? 0)) {
          if (!response.headers.location || redirects > 5) {
            reject(new Error(`redirect failed for ${url}`));
            return;
          }
          resolve(requestText(new URL(response.headers.location, url).toString(), redirects + 1));
          return;
        }

        if ((response.statusCode ?? 500) >= 400) {
          reject(new Error(`HTTP ${response.statusCode} for ${url}`));
          response.resume();
          return;
        }

        let body = "";
        response.setEncoding("utf8");
        response.on("data", (chunk) => {
          body += chunk;
        });
        response.on("end", () => resolve(body));
      })
      .on("error", reject);
  });
}

function download(url, destination, redirects = 0) {
  return new Promise((resolve, reject) => {
    https
      .get(url, { headers: { "User-Agent": "gitmarket-npx-launcher" } }, (response) => {
        if ([301, 302, 303, 307, 308].includes(response.statusCode ?? 0)) {
          if (!response.headers.location || redirects > 5) {
            reject(new Error(`redirect failed for ${url}`));
            return;
          }
          resolve(download(new URL(response.headers.location, url).toString(), destination, redirects + 1));
          return;
        }

        if ((response.statusCode ?? 500) >= 400) {
          reject(new Error(`HTTP ${response.statusCode} while downloading ${url}`));
          response.resume();
          return;
        }

        const file = createWriteStream(destination);
        response.pipe(file);
        file.on("finish", () => {
          file.close(resolve);
        });
        file.on("error", (error) => {
          rmSync(destination, { force: true });
          reject(error);
        });
      })
      .on("error", reject);
  });
}

function extractArchive(kind, archivePath, installDir) {
  const command =
    kind === "zip" && platform() === "win32"
      ? {
          bin: "powershell",
          args: [
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            `Expand-Archive -LiteralPath '${archivePath.replaceAll("'", "''")}' -DestinationPath '${installDir.replaceAll("'", "''")}' -Force`,
          ],
        }
      : kind === "zip"
        ? { bin: "unzip", args: ["-q", "-o", archivePath, "-d", installDir] }
        : { bin: "tar", args: ["-xzf", archivePath, "-C", installDir] };

  const result = spawnSync(command.bin, command.args, { stdio: "inherit" });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new Error(`${command.bin} exited with ${result.status}`);
  }
}
