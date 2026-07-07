#!/usr/bin/env node

import { existsSync, readFileSync, statSync } from "node:fs";
import path from "node:path";

const builtinOfficePluginDir = "apps/desktop/public/plugins/builtin-office-control";
const builtinOfficeManifestPath = path.join(builtinOfficePluginDir, "plugin.json");
const requiredOfficeSkillFiles = [
  "skills/office-control.md",
  "skills/word-control.md",
  "skills/excel-control.md",
  "skills/powerpoint-control.md",
];

const failures = [];
const checks = [];

checkBuiltinOfficeControlPlugin();

if (failures.length > 0) {
  console.error(
    JSON.stringify(
      {
        ok: false,
        failures,
        checks,
      },
      null,
      2,
    ),
  );
  process.exit(1);
}

console.log(
  JSON.stringify(
    {
      ok: true,
      checks,
    },
    null,
    2,
  ),
);

function checkBuiltinOfficeControlPlugin() {
  const manifest = readJsonIfPresent(builtinOfficeManifestPath, "builtin Office plugin manifest");
  if (!manifest) {
    return;
  }

  checkManifestField(manifest, "id", "ds-agent.builtin.office-control");
  checkManifestField(manifest, "kind", "builtin_skill_pack");
  checkManifestField(manifest, "version", "0.1.0");
  checkManifestField(manifest, "builtin", true);
  checkManifestField(manifest, "entry", "skills/office-control.md");

  const skillPaths = Array.isArray(manifest.skills)
    ? manifest.skills.map((skill) => skill?.path).filter(Boolean)
    : [];
  for (const skillFile of requiredOfficeSkillFiles) {
    if (!skillPaths.includes(skillFile)) {
      failures.push(`builtin Office manifest must list ${skillFile}`);
    } else {
      checks.push(`builtin Office manifest lists ${skillFile}`);
    }
    checkTextFile(path.join(builtinOfficePluginDir, skillFile), `${skillFile} exists`);
  }

  for (const appId of ["word", "excel", "powerpoint"]) {
    if (!manifest.office_apps?.some?.((app) => app?.id === appId)) {
      failures.push(`builtin Office manifest must declare office app ${appId}`);
    } else {
      checks.push(`builtin Office manifest declares ${appId}`);
    }
  }

  for (const actionType of ["office_create", "office_update", "office_open"]) {
    if (!manifest.actions?.some?.((action) => action?.action_type === actionType)) {
      failures.push(`builtin Office manifest must declare action ${actionType}`);
    } else {
      checks.push(`builtin Office manifest declares action ${actionType}`);
    }
  }

  for (const capability of [
    "file_read",
    "file_write",
    "computer_screenshot",
    "computer_control",
  ]) {
    if (!manifest.capabilities?.includes?.(capability)) {
      failures.push(`builtin Office manifest must include capability ${capability}`);
    } else {
      checks.push(`builtin Office manifest includes ${capability}`);
    }
  }

  checkSkillSnippet(
    "skills/office-control.md",
    "Prefer deterministic file creation or file editing before desktop UI control.",
  );
  checkSkillSnippet(
    "skills/office-control.md",
    "Use computer_screenshot before any screen-dependent computer_control action.",
  );
  checkSkillSnippet(
    "skills/office-control.md",
    "Use `office_open` to open a workspace `.docx`, `.xlsx`, or `.pptx` file",
  );
  checkSkillSnippet(
    "skills/office-control.md",
    "Use `office_update` to update an existing workspace Office file",
  );
  checkSkillSnippet(
    "skills/office-control.md",
    "Right rail output should show only compact user-facing steps and their state.",
  );
  checkSkillSnippet("skills/word-control.md", "If Microsoft Word is unavailable");
  checkSkillSnippet("skills/excel-control.md", "Keep workbook calculations formula driven");
  checkSkillSnippet("skills/powerpoint-control.md", "Render or preview every final slide");
}

function checkSkillSnippet(relativePath, expected) {
  const filePath = path.join(builtinOfficePluginDir, relativePath);
  if (!existsSync(filePath)) {
    failures.push(`${relativePath} is required before checking snippet ${expected}`);
    return;
  }

  const content = readFileSync(filePath, "utf8");
  if (!content.includes(expected)) {
    failures.push(`${relativePath} must include ${expected}`);
    return;
  }

  checks.push(`${relativePath} includes ${expected}`);
}

function checkManifestField(manifest, field, expected) {
  const actual = manifest[field];
  if (actual !== expected) {
    failures.push(`builtin Office manifest ${field} must be ${JSON.stringify(expected)}`);
    return;
  }

  checks.push(`builtin Office manifest ${field}`);
}

function checkTextFile(filePath, label) {
  if (!existsSync(filePath)) {
    failures.push(`${filePath} is required`);
    return null;
  }

  if (statSync(filePath).size === 0) {
    failures.push(`${filePath} must not be empty`);
    return null;
  }

  checks.push(label);
  return readFileSync(filePath, "utf8");
}

function readJsonIfPresent(filePath, label) {
  if (!checkTextFile(filePath, label)) {
    return null;
  }

  try {
    return JSON.parse(readFileSync(filePath, "utf8"));
  } catch (error) {
    failures.push(`${filePath} must be valid JSON: ${error.message}`);
    return null;
  }
}
