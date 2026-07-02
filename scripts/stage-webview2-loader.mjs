import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const tauriDir = join(repoRoot, "apps", "desktop", "src-tauri");
const stagedPath = join(tauriDir, "generated", "windows", "WebView2Loader.dll");

const profile = process.env.TAURI_ENV_DEBUG === "true" ? "debug" : "release";
const candidateRoots = [
  process.env.CARGO_TARGET_DIR,
  join(tauriDir, "target"),
  getWebView2SysRoot(),
].filter(Boolean).map((path) => resolve(path));

const arch = normalizeArch(process.env.TAURI_ENV_ARCH || process.arch);

function findLoaderIn(root) {
  const directCandidates = [
    join(root, profile, "WebView2Loader.dll"),
    join(root, "debug", "WebView2Loader.dll"),
    join(root, "release", "WebView2Loader.dll"),
    join(root, arch, "WebView2Loader.dll"),
    join(root, "x64", "WebView2Loader.dll"),
  ];

  for (const candidate of directCandidates) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }

  const buildOutput = findBuildOutputLoader(root);
  if (buildOutput) {
    return buildOutput;
  }

  return findByName(root, "WebView2Loader.dll", 6);
}

function findBuildOutputLoader(root) {
  const buildDirs = [
    join(root, profile, "build"),
    join(root, "debug", "build"),
    join(root, "release", "build"),
  ];

  for (const buildDir of buildDirs) {
    if (!existsSync(buildDir)) {
      continue;
    }

    for (const entry of readdirSync(buildDir, { withFileTypes: true })) {
      if (!entry.isDirectory() || !entry.name.includes("webview2-com-sys")) {
        continue;
      }

      const candidate = join(buildDir, entry.name, "out", arch, "WebView2Loader.dll");
      if (existsSync(candidate)) {
        return candidate;
      }
    }
  }

  return null;
}

function normalizeArch(rawArch) {
  switch (rawArch) {
    case "x64":
    case "x86_64":
    case "amd64":
      return "x64";
    case "ia32":
    case "x86":
    case "i686":
      return "x86";
    case "arm64":
    case "aarch64":
      return "arm64";
    default:
      return rawArch;
  }
}

function getWebView2SysRoot() {
  try {
    const metadata = execFileSync(
      "cargo",
      [
        "metadata",
        "--format-version",
        "1",
        "--manifest-path",
        join(tauriDir, "Cargo.toml"),
      ],
      {
        cwd: repoRoot,
        encoding: "utf8",
        stdio: ["ignore", "pipe", "inherit"],
      }
    );
    const parsed = JSON.parse(metadata);
    const pkg = parsed.packages.find((item) => item.name === "webview2-com-sys");
    return pkg ? dirname(pkg.manifest_path) : null;
  } catch {
    return null;
  }
}

function findByName(dir, fileName, maxDepth) {
  if (maxDepth < 0 || !existsSync(dir)) {
    return null;
  }

  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return null;
  }

  for (const entry of entries) {
    const fullPath = join(dir, entry.name);
    if (entry.isFile() && entry.name === fileName) {
      return fullPath;
    }
  }

  for (const entry of entries) {
    if (!entry.isDirectory()) {
      continue;
    }

    const fullPath = join(dir, entry.name);
    const found = findByName(fullPath, fileName, maxDepth - 1);
    if (found) {
      return found;
    }
  }

  return null;
}

const sourcePath = candidateRoots.map(findLoaderIn).find(Boolean);

if (!sourcePath) {
  throw new Error(
    `WebView2Loader.dll was not found under: ${candidateRoots.join(", ")}`
  );
}

const size = statSync(sourcePath).size;
if (size === 0) {
  throw new Error(`WebView2Loader.dll is empty: ${sourcePath}`);
}

mkdirSync(dirname(stagedPath), { recursive: true });
copyFileSync(sourcePath, stagedPath);

console.log(`Staged WebView2Loader.dll (${size} bytes)`);
