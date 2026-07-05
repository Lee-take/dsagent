#!/usr/bin/env node

import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";

const desktopPackagePath = path.join("apps", "desktop", "package.json");

if (!existsSync(desktopPackagePath)) {
  console.error(
    "Desktop workspace is missing from this source checkout. Restore apps/desktop/package.json before running the DS Agent desktop dev app.",
  );
  process.exit(1);
}

const cargoTargetDir =
  process.env.CARGO_TARGET_DIR ??
  path.join(os.tmpdir(), "deepseek_agent_os_tauri_dev_target");

const env = {
  ...process.env,
  CARGO_TARGET_DIR: cargoTargetDir,
};

if (process.argv.includes("--print-env")) {
  console.log(JSON.stringify({ CARGO_TARGET_DIR: cargoTargetDir }));
  process.exit(0);
}

const result = spawnSync(
  "npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop tauri dev",
  {
    env,
    shell: true,
    stdio: "inherit",
  },
);

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);
