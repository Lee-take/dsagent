#!/usr/bin/env node

import { spawn, spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { mkdir, readFile, readdir, rm, writeFile } from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import net from "node:net";

const isWindows = process.platform === "win32";
const rawArgs = process.argv.slice(2).filter((arg) => arg !== "--");
const allowedArgs = new Set(["--agent-chat", "--help", "--self-test", "--workflow"]);
validateArgs(rawArgs, allowedArgs, "test:windows-installed-ui");

if (rawArgs.includes("--help")) {
  console.log(
    [
      "Usage: pnpm test:windows-installed-ui [-- <flags>]",
      "",
      "Flags:",
      "  --agent-chat Exercise the installed Tauri agent chat command bridge.",
      "  --self-test Run deterministic helper checks without launching DS Agent.",
      "  --workflow  Exercise the installed Tauri workflow and report exports.",
    ].join("\n"),
  );
  process.exit(0);
}

const args = new Set(rawArgs);
const selfTestMode = args.has("--self-test");
const executablePath = selfTestMode ? null : resolveExecutablePath();
const includeWorkflowSmoke =
  args.has("--workflow") ||
  process.env.DEEPSEEK_AGENT_OS_INSTALLED_UI_WORKFLOW_SMOKE === "1";
const includeAgentChatSmoke =
  args.has("--agent-chat") ||
  process.env.DEEPSEEK_AGENT_OS_INSTALLED_AGENT_CHAT_SMOKE === "1";
const expectModelTelemetry = Boolean(process.env.DEEPSEEK_API_KEY?.trim());
const timeoutMs = readPositiveInteger(
  process.env.DEEPSEEK_AGENT_OS_INSTALLED_UI_TIMEOUT_MS ?? "20000",
  "DEEPSEEK_AGENT_OS_INSTALLED_UI_TIMEOUT_MS",
);
const workflowTimeoutMs = readPositiveInteger(
  process.env.DEEPSEEK_AGENT_OS_INSTALLED_WORKFLOW_TIMEOUT_MS ?? "120000",
  "DEEPSEEK_AGENT_OS_INSTALLED_WORKFLOW_TIMEOUT_MS",
);
const screenshotDir =
  process.env.DEEPSEEK_AGENT_OS_UI_SMOKE_SCREENSHOT_DIR ??
  path.join(os.tmpdir(), "deepseek-agent-os-ui-smoke");
const workflowRootDir =
  process.env.DEEPSEEK_AGENT_OS_INSTALLED_WORKFLOW_DIR ??
  path.join(os.tmpdir(), "deepseek-agent-os-installed-workflow-smoke");

if (!selfTestMode && !isWindows) {
  console.error("test:windows-installed-ui only runs on Windows.");
  process.exit(1);
}

if (!selfTestMode && (!executablePath || !existsSync(executablePath))) {
  console.error(
    [
      "DS Agent executable was not found.",
      "Set DEEPSEEK_AGENT_OS_INSTALLED_EXE or install the app first.",
      `Default checked: ${defaultInstalledExecutablePath()}`,
    ].join("\n"),
  );
  process.exit(1);
}

if (!selfTestMode && typeof WebSocket !== "function") {
  console.error("This script requires a Node.js runtime with global WebSocket.");
  process.exit(1);
}

let child;
let cdp;

async function main() {
  try {
    const port = await findFreePort();
    const env = {
      ...process.env,
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: webView2ArgsForPort(port),
    };
    child = spawn(executablePath, [], {
      env,
      stdio: ["ignore", "ignore", "pipe"],
      windowsHide: false,
    });

    child.stderr?.on("data", () => undefined);

    const target = await waitForWebViewTarget(port, timeoutMs);
    cdp = await CdpClient.connect(target.webSocketDebuggerUrl);
    await cdp.send("Runtime.enable");
    await cdp.send("Page.enable");
    await cdp.send("Page.bringToFront").catch(() => undefined);
    await waitForReadyState(cdp, timeoutMs);
    const bodyText = await waitForMeaningfulBodyText(cdp, timeoutMs);

    const title = await evaluate(cdp, "document.title");
    const url = await evaluate(cdp, "location.href");
    const hasTauriInternals = await evaluate(
      cdp,
      "Boolean(window.__TAURI_INTERNALS__ || window.__TAURI__)",
    );
    const screenshotPath = await captureScreenshot(cdp, screenshotDir);
    const agentChat = includeAgentChatSmoke
      ? await runInstalledAgentChatSmoke(cdp)
      : null;
    const workflow = includeWorkflowSmoke
      ? await runInstalledWorkflowSmoke(cdp)
      : null;

    const checks = {
      title: title === "DS Agent",
      url: typeof url === "string" && url.startsWith("http://tauri.localhost"),
      not_blank: meaningfulBodyText(bodyText),
      tauri_bridge_present: hasTauriInternals === true,
      no_framework_overlay: !hasFrameworkOverlay(bodyText),
    };
    const failedChecks = Object.entries(checks)
      .filter(([, passed]) => !passed)
      .map(([name]) => name);

    if (failedChecks.length > 0) {
      throw new Error(`Installed UI smoke failed checks: ${failedChecks.join(", ")}`);
    }

    console.log(
      JSON.stringify(
        {
          ok: true,
          executable: describeLocalExecutable(executablePath),
          title,
          url,
          body_chars: String(bodyText).length,
          checks,
          screenshot: screenshotPath,
          agent_chat: agentChat ?? "skipped",
          workflow: workflow ?? "skipped",
        },
        null,
        2,
      ),
    );
  } catch (error) {
    console.error(
      JSON.stringify(
        {
          ok: false,
          executable: executablePath
            ? describeLocalExecutable(executablePath)
            : "[not found]",
          error: String(error?.message ?? error),
        },
        null,
        2,
      ),
    );
    process.exitCode = 1;
  } finally {
    if (cdp) {
      await cdp.send("Browser.close").catch(() => undefined);
      cdp.close();
    }
    terminateProcessTree(child);
  }
}

function resolveExecutablePath() {
  return (
    process.env.DEEPSEEK_AGENT_OS_INSTALLED_EXE?.trim() ||
    defaultInstalledExecutablePath()
  );
}

function defaultInstalledExecutablePath() {
  return path.join(
    process.env.LOCALAPPDATA ?? path.join(os.homedir(), "AppData", "Local"),
    "DS Agent",
    "ds-agent.exe",
  );
}

function webView2ArgsForPort(port) {
  const existing = process.env.WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS?.trim();
  return [
    existing,
    `--remote-debugging-port=${port}`,
    "--remote-allow-origins=*",
  ]
    .filter(Boolean)
    .join(" ");
}

async function findFreePort() {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.unref();
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      const port = typeof address === "object" && address ? address.port : null;
      server.close(() => {
        if (port) {
          resolve(port);
        } else {
          reject(new Error("Could not allocate a local debugging port."));
        }
      });
    });
  });
}

async function waitForWebViewTarget(port, timeout) {
  const deadline = Date.now() + timeout;
  let lastError = "";

  while (Date.now() < deadline) {
    for (const host of ["127.0.0.1", "localhost"]) {
      try {
        const response = await fetch(`http://${host}:${port}/json/list`);
        if (response.ok) {
          const targets = await response.json();
          const page = targets.find(
            (target) =>
              target.type === "page" &&
              typeof target.webSocketDebuggerUrl === "string" &&
              (String(target.url).startsWith("http://tauri.localhost") ||
                String(target.title).includes("DS Agent")),
          );
          if (page) {
            return page;
          }
        }
      } catch (error) {
        lastError = `${host}: ${String(error?.message ?? error)}`;
      }
    }
    await delay(250);
  }

  throw new Error(
    [
      "Timed out waiting for WebView2 remote debugging target.",
      "Close any already-running DS Agent window and retry if this persists.",
      lastError,
    ].join(" "),
  );
}

async function waitForReadyState(client, timeout) {
  const deadline = Date.now() + timeout;
  while (Date.now() < deadline) {
    const readyState = await evaluate(client, "document.readyState").catch(
      () => "",
    );
    if (readyState === "interactive" || readyState === "complete") {
      return;
    }
    await delay(150);
  }
  throw new Error("Timed out waiting for installed UI document readiness.");
}

async function waitForMeaningfulBodyText(client, timeout) {
  const deadline = Date.now() + timeout;
  let lastText = "";
  while (Date.now() < deadline) {
    lastText = String(
      (await evaluate(
        client,
        "document.body ? document.body.innerText.slice(0, 6000) : ''",
      ).catch(() => "")) ?? "",
    );
    if (meaningfulBodyText(lastText)) {
      return lastText;
    }
    await delay(150);
  }
  throw new Error(
    `Timed out waiting for installed UI meaningful body text; last body chars: ${lastText.length}.`,
  );
}

async function evaluate(client, expression) {
  const result = await client.send("Runtime.evaluate", {
    expression,
    returnByValue: true,
    awaitPromise: true,
  });

  if (result.exceptionDetails) {
    const detail =
      result.exceptionDetails.exception?.description ??
      result.exceptionDetails.text ??
      "unknown error";
    throw new Error(`CDP evaluation failed for: ${expression}. ${detail}`);
  }
  return result.result?.value;
}

async function invokeTauri(client, command, params = {}, timeout = workflowTimeoutMs) {
  const expression = `
    (async () => {
      const command = ${JSON.stringify(command)};
      const params = ${JSON.stringify(params)};
      const invoke =
        window.__TAURI_INTERNALS__?.invoke ??
        window.__TAURI__?.core?.invoke ??
        window.__TAURI__?.invoke;
      if (typeof invoke !== "function") {
        throw new Error("Tauri invoke bridge is not available.");
      }
      try {
        return await invoke(command, params);
      } catch (error) {
        throw new Error(String(error?.message ?? error));
      }
    })()
  `;

  return withTimeout(
    evaluate(client, expression),
    timeout,
    `Timed out waiting for Tauri command ${command}.`,
  );
}

async function runInstalledAgentChatSmoke(client) {
  if (!process.env.DEEPSEEK_API_KEY?.trim()) {
    throw new Error("DEEPSEEK_API_KEY is required for --agent-chat smoke.");
  }

  const telemetryBefore = await listDeepSeekTelemetry(client);
  const response = await invokeTauri(
    client,
    "run_agent_chat",
    {
      prompt: "请用一句中文回答：DS Agent 的桌面对话桥已经连到 DeepSeek 了吗？",
      largeModelProvider: "deepseek",
      modelRoute: "flash",
      thinkingLevel: "fast",
      accessMode: "ask_on_risk",
      networkSearchSourceModel: null,
      apiKeyOverride: null,
    },
    workflowTimeoutMs,
  );
  const content = String(response?.content ?? "").trim();
  if (!content) {
    throw new Error("Installed agent chat smoke expected non-empty assistant content.");
  }

  const telemetryAfter = await listDeepSeekTelemetry(client);
  const newEntries = newTelemetryEntries(telemetryBefore, telemetryAfter);
  const latest = newEntries[0];
  if (!latest) {
    throw new Error("Installed agent chat smoke expected a new DeepSeek telemetry event.");
  }

  return {
    ok: true,
    content_chars: content.length,
    response_model: response?.model ?? null,
    protocol_version: response?.protocol_version ?? null,
    proposed_actions: Array.isArray(response?.proposed_actions)
      ? response.proposed_actions.length
      : null,
    missing_prerequisites: Array.isArray(response?.missing_prerequisites)
      ? response.missing_prerequisites.length
      : null,
    model_telemetry: {
      new_entries: newEntries.length,
      latest_model: latest.model ?? null,
      latest_cache_status: latest.cache_status ?? null,
      latest_total_tokens: latest.total_tokens ?? null,
    },
  };
}

async function runInstalledWorkflowSmoke(client) {
  const startedAt = new Date();
  const runRoot = path.join(
    workflowRootDir,
    startedAt.toISOString().replaceAll(":", "-").replaceAll(".", "-"),
  );
  const workspaceDir = path.join(runRoot, "workspace");
  const evidenceDir = path.join(workspaceDir, "evidence");
  const exportDir = path.join(workspaceDir, "exports");
  await mkdir(workspaceDir, { recursive: true });
  await mkdir(evidenceDir, { recursive: true });
  await mkdir(exportDir, { recursive: true });

  const directoryState = await invokeTauri(client, "get_local_directory_state", {});
  const settingsBackup = await backupSettingsFile(directoryState?.settings_file);
  const appDataEventsBackup = await backupAppDataEventsFile(
    directoryState?.settings_file,
  );
  const approvals = [];
  let workflowResult = null;
  let restoreVerified = false;
  let appDataEventsRestored = false;

  try {
    const savedDirectoryState = await invokeTauri(
      client,
      "save_local_directory_settings",
      {
        workspaceDir,
        workspaceName: "Installed Workflow Smoke",
        evidenceDir,
        exportDir,
      },
    );
    if (savedDirectoryState?.needs_setup) {
      throw new Error("Temporary installed workflow directories were not accepted.");
    }

    const seed = await invokeWithApproval(
      client,
      "seed_operations_briefing_evidence_templates",
      { accessMode: "full_access" },
      "file_write",
      approvals,
    );
    assertInvocationSucceeded(seed.value, "template seed");
    const evidenceFolderPath = seed.value.evidence_ref ?? evidenceDir;
    const telemetryBefore = expectModelTelemetry
      ? await listDeepSeekTelemetry(client)
      : [];

    const run = await invokeWithApproval(
      client,
      "run_operations_briefing",
      {
        accessMode: "full_access",
        evidenceFolderPath,
        largeModelProvider: "deepseek",
        modelRoute: "auto",
        thinkingLevel: "fast",
      },
      "file_read",
      approvals,
    );
    if (run.value?.status !== "draft_ready") {
      throw new Error(
        `Operations Briefing run did not reach draft_ready: ${run.value?.status ?? "unknown"}`,
      );
    }
    const modelTelemetry = await summarizeModelTelemetry(client, telemetryBefore);

    const markdown = await exportOperationsBriefingReport(
      client,
      "export_operations_briefing_report",
      run.value.id,
      approvals,
    );
    const html = await exportOperationsBriefingReport(
      client,
      "export_operations_briefing_html_report",
      run.value.id,
      approvals,
    );
    const pdf = await exportOperationsBriefingReport(
      client,
      "export_operations_briefing_pdf_report",
      run.value.id,
      approvals,
    );
    const exportedFiles = await readdir(exportDir);
    const exportedRefs = [markdown.value, html.value, pdf.value]
      .map((invocation) => invocation.evidence_ref)
      .filter(Boolean);

    for (const filePath of exportedRefs) {
      if (!existsSync(filePath)) {
        throw new Error(`Expected exported report was not found: ${filePath}`);
      }
    }

    workflowResult = {
      ok: true,
      mode: "workflow",
      run_id: run.value.id,
      run_status: run.value.status,
      evidence_ref: describeLocalPath(evidenceFolderPath),
      export_dir: describeLocalPath(exportDir),
      exported_files: exportedFiles,
      approvals_resolved: approvals.length,
      model_telemetry: modelTelemetry,
      settings_file_restored: false,
      app_data_events: "pending_restore",
      app_data_events_restored: false,
    };
  } finally {
    await closeInstalledAppForAppDataRestore();
    restoreVerified = await restoreSettingsFile(settingsBackup);
    appDataEventsRestored = await restoreAppDataEventsFile(appDataEventsBackup);
  }

  assertWorkflowRestoresVerified({
    settingsRestored: restoreVerified,
    appDataEventsRestored,
  });

  return {
    ...workflowResult,
    settings_file_restored: restoreVerified,
    app_data_events: appDataEventsRestored ? "restored" : "restore_failed",
    app_data_events_restored: appDataEventsRestored,
  };
}

async function invokeWithApproval(client, command, params, capability, approvals) {
  const firstValue = await invokeTauri(client, command, params);
  if (firstValue?.status !== "pending_approval") {
    return { value: firstValue, retried: false };
  }

  const requestId = await approveNewestPendingCapability(client, capability);
  approvals.push(requestId);
  const retryValue = await invokeTauri(client, command, params);
  return { value: retryValue, retried: true };
}

async function approveNewestPendingCapability(client, capability) {
  const records = await invokeTauri(
    client,
    "list_pending_capability_access_records",
    {},
  );
  const matches = records
    .filter((record) => record?.request?.capability === capability)
    .sort((left, right) =>
      String(right.request.created_at).localeCompare(String(left.request.created_at)),
    );
  const requestId = matches[0]?.request?.id;

  if (!requestId) {
    throw new Error(`No pending ${capability} approval was found.`);
  }

  await invokeTauri(client, "resolve_capability_access_request", {
    requestId,
    approved: true,
    note: "Installed workflow smoke approval",
  });
  return requestId;
}

async function exportOperationsBriefingReport(client, command, runId, approvals) {
  const result = await invokeWithApproval(
    client,
    command,
    {
      accessMode: "full_access",
      runId,
    },
    "drive_write",
    approvals,
  );
  assertInvocationSucceeded(result.value, command);
  return result;
}

async function listDeepSeekTelemetry(client) {
  const telemetry = await invokeTauri(client, "list_deepseek_chat_telemetry", {});
  return Array.isArray(telemetry) ? telemetry : [];
}

async function summarizeModelTelemetry(client, telemetryBefore) {
  if (!expectModelTelemetry) {
    return {
      expected: false,
      observed: false,
      reason: "DEEPSEEK_API_KEY not configured",
    };
  }

  const beforeIds = new Set(telemetryBefore.map((entry) => entry?.id));
  const telemetryAfter = await listDeepSeekTelemetry(client);
  const newEntries = newTelemetryEntries(telemetryBefore, telemetryAfter);
  const latest = newEntries[0];

  if (!latest) {
    throw new Error(
      "Installed workflow smoke expected a DeepSeek telemetry event, but none was recorded.",
    );
  }

  return {
    expected: true,
    observed: true,
    new_entries: newEntries.length,
    latest_model: latest.model ?? null,
    latest_cache_status: latest.cache_status ?? null,
    latest_total_tokens: latest.total_tokens ?? null,
  };
}

function newTelemetryEntries(telemetryBefore, telemetryAfter) {
  const beforeIds = new Set((telemetryBefore ?? []).map((entry) => entry?.id));
  return (telemetryAfter ?? []).filter((entry) => !beforeIds.has(entry?.id));
}

function assertInvocationSucceeded(invocation, label) {
  if (invocation?.status !== "succeeded") {
    throw new Error(
      `${label} invocation did not succeed: ${invocation?.status ?? "unknown"}`,
    );
  }
}

async function closeInstalledAppForAppDataRestore() {
  if (cdp) {
    await cdp.send("Browser.close").catch(() => undefined);
    cdp.close();
    cdp = null;
  }
  terminateProcessTree(child);
  child = undefined;
  await delay(500);
}

async function backupSettingsFile(settingsFile) {
  return backupLocalFile(settingsFile);
}

async function restoreSettingsFile(backup) {
  return restoreLocalFile(backup);
}

async function backupAppDataEventsFile(settingsFile) {
  if (!settingsFile) {
    return null;
  }

  return backupLocalFile(path.join(path.dirname(settingsFile), "kernel-events.sqlite3"));
}

async function restoreAppDataEventsFile(backup) {
  return restoreLocalFile(backup);
}

async function backupLocalFile(filePath) {
  if (!filePath) {
    return null;
  }

  try {
    return {
      filePath,
      existed: true,
      content: await readFile(filePath),
    };
  } catch (error) {
    if (error?.code === "ENOENT") {
      return {
        filePath,
        existed: false,
        content: null,
      };
    }
    throw error;
  }
}

async function restoreLocalFile(backup) {
  if (!backup?.filePath) {
    return true;
  }

  if (backup.existed) {
    await mkdir(path.dirname(backup.filePath), { recursive: true });
    await writeFile(backup.filePath, backup.content);
    const restoredContent = await readFile(backup.filePath);
    return restoredContent.equals(backup.content);
  }

  await rm(backup.filePath, { force: true });
  return !existsSync(backup.filePath);
}

function assertWorkflowRestoresVerified({
  settingsRestored,
  appDataEventsRestored,
}) {
  const failedRestores = [];
  if (settingsRestored !== true) {
    failedRestores.push("settings file");
  }
  if (appDataEventsRestored !== true) {
    failedRestores.push("app-data event store");
  }

  if (failedRestores.length > 0) {
    throw new Error(
      `Installed workflow smoke could not verify restored ${failedRestores.join(
        " and ",
      )}.`,
    );
  }
}

async function captureScreenshot(client, directory) {
  await mkdir(directory, { recursive: true });
  const response = await client.send("Page.captureScreenshot", {
    format: "png",
    captureBeyondViewport: false,
  });
  const filePath = path.join(
    directory,
    `ds-agent-installed-ui-${new Date()
      .toISOString()
      .replaceAll(":", "-")
      .replaceAll(".", "-")}.png`,
  );
  await writeFile(filePath, Buffer.from(response.data, "base64"));
  return filePath;
}

function meaningfulBodyText(value) {
  const text = String(value ?? "");
  if (text.length > 200) {
    return ["DS Agent", "工作台", "Workbench", "Operations", "记忆", "审批"].some(
      (token) => text.includes(token),
    );
  }

  const compactChatFirstTokens = [
    "新对话",
    "运行步骤",
    "理解任务",
    "调用 DeepSeek",
    "生成与导出",
    "New chat",
    "Run steps",
    "Understand task",
    "Call DeepSeek",
    "Generate and export",
  ];
  const matchedCompactTokens = compactChatFirstTokens.filter((token) =>
    text.includes(token),
  ).length;
  return text.length > 80 && matchedCompactTokens >= 3;
}

function hasFrameworkOverlay(value) {
  const text = String(value ?? "");
  return [
    "Vite",
    "React Refresh",
    "Unhandled Runtime Error",
    "Build Error",
  ].some((token) => text.includes(token));
}

function describeLocalExecutable(value) {
  return path.isAbsolute(value)
    ? `[local executable]/${path.basename(value)}`
    : value.replace(/\\/g, "/");
}

function describeLocalPath(value) {
  const normalized = path.resolve(String(value));
  const tempRoot = path.resolve(os.tmpdir());
  if (normalized.toLowerCase().startsWith(tempRoot.toLowerCase())) {
    return `[temp]/${path.relative(tempRoot, normalized).replace(/\\/g, "/")}`;
  }

  return path.isAbsolute(value)
    ? `[local path]/${path.basename(value)}`
    : String(value).replace(/\\/g, "/");
}

function readPositiveInteger(value, name) {
  const parsed = Number(value);
  if (!Number.isInteger(parsed) || parsed <= 0) {
    throw new Error(`${name} must be a positive integer.`);
  }
  return parsed;
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function withTimeout(promise, timeout, message) {
  let timeoutId;
  const timeoutPromise = new Promise((_, reject) => {
    timeoutId = setTimeout(() => reject(new Error(message)), timeout);
  });

  return Promise.race([promise, timeoutPromise]).finally(() => {
    clearTimeout(timeoutId);
  });
}

function terminateProcessTree(process) {
  if (!process || process.exitCode !== null || !process.pid) {
    return;
  }

  if (isWindows) {
    spawnSync("taskkill", ["/PID", String(process.pid), "/T", "/F"], {
      stdio: "ignore",
    });
    return;
  }

  process.kill();
}

function validateArgs(values, allowed, commandName) {
  const unknown = values.filter((arg) => !allowed.has(arg));
  if (unknown.length === 0) {
    return;
  }

  console.error(
    JSON.stringify(
      {
        ok: false,
        command: commandName,
        error: `Unknown argument(s): ${unknown.join(", ")}`,
        allowed: Array.from(allowed).sort(),
      },
      null,
      2,
    ),
  );
  process.exit(1);
}

class CdpClient {
  constructor(socket) {
    this.socket = socket;
    this.nextId = 1;
    this.pending = new Map();

    socket.addEventListener("message", (event) => {
      const message = JSON.parse(event.data);
      if (!message.id || !this.pending.has(message.id)) {
        return;
      }
      const { resolve, reject } = this.pending.get(message.id);
      this.pending.delete(message.id);
      if (message.error) {
        reject(new Error(message.error.message));
      } else {
        resolve(message.result ?? {});
      }
    });
  }

  static connect(url) {
    return new Promise((resolve, reject) => {
      const socket = new WebSocket(url);
      socket.addEventListener("open", () => resolve(new CdpClient(socket)), {
        once: true,
      });
      socket.addEventListener(
        "error",
        () => reject(new Error("Could not connect to WebView2 CDP target.")),
        { once: true },
      );
    });
  }

  send(method, params = {}) {
    const id = this.nextId++;
    this.socket.send(JSON.stringify({ id, method, params }));
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
    });
  }

  close() {
    this.socket.close();
  }
}

async function runSelfTest() {
  if (meaningfulBodyText("中\nEN\n设置")) {
    throw new Error("Self-test expected short settings-only text to stay blank.");
  }
  if (!meaningfulBodyText("DS Agent 工作台 Operations 记忆 审批 ".repeat(16))) {
    throw new Error("Self-test expected legacy workbench text to stay meaningful.");
  }

  const attempts = [
    "",
    "中\nEN\n新对话\n对话\n1\n未命名对话\n7月5日 11:25\n设置\n发送\n运行步骤\n6\n1\n理解任务\n已完成\n2\n读取证据\n已完成\n3\n选择记忆\n已完成\n4\n调用 DeepSeek\n已完成\n5\n校验结果\n已完成\n6\n生成与导出\n已完成",
  ];
  const fakeClient = {
    async send(method) {
      if (method !== "Runtime.evaluate") {
        throw new Error(`Unexpected method in self-test: ${method}`);
      }
      return {
        result: {
          value: attempts.shift() ?? "",
        },
      };
    },
  };
  const bodyText = await waitForMeaningfulBodyText(fakeClient, 1_000);
  if (!meaningfulBodyText(bodyText)) {
    throw new Error("Self-test expected meaningful body text.");
  }
  const restoreTestDir = await mkdir(
    path.join(os.tmpdir(), "deepseek-agent-os-ui-smoke-self-test"),
    { recursive: true },
  ).then(() => path.join(os.tmpdir(), "deepseek-agent-os-ui-smoke-self-test"));
  const missingSettingsFile = path.join(restoreTestDir, "missing-local-directories.json");
  await rm(missingSettingsFile, { force: true });
  const missingBackup = await backupSettingsFile(missingSettingsFile);
  await writeFile(missingSettingsFile, '{"workspace_dir":"temporary"}', "utf8");
  if ((await restoreSettingsFile(missingBackup)) !== true) {
    throw new Error("Self-test expected missing settings restore to report true.");
  }
  if (existsSync(missingSettingsFile)) {
    throw new Error("Self-test expected missing settings file to be removed.");
  }

  const existingSettingsFile = path.join(restoreTestDir, "existing-local-directories.json");
  await writeFile(existingSettingsFile, '{"workspace_dir":"original"}', "utf8");
  const existingBackup = await backupSettingsFile(existingSettingsFile);
  await writeFile(existingSettingsFile, '{"workspace_dir":"temporary"}', "utf8");
  if ((await restoreSettingsFile(existingBackup)) !== true) {
    throw new Error("Self-test expected existing settings restore to report true.");
  }
  const restoredContent = await readFile(existingSettingsFile, "utf8");
  if (restoredContent !== '{"workspace_dir":"original"}') {
    throw new Error("Self-test expected existing settings content to be restored.");
  }
  const missingEventsFile = path.join(restoreTestDir, "missing-kernel-events.sqlite3");
  await rm(missingEventsFile, { force: true });
  const missingEventsBackup = await backupLocalFile(missingEventsFile);
  await writeFile(missingEventsFile, Buffer.from([1, 2, 3]));
  if ((await restoreLocalFile(missingEventsBackup)) !== true) {
    throw new Error("Self-test expected missing event store restore to report true.");
  }
  if (existsSync(missingEventsFile)) {
    throw new Error("Self-test expected missing event store file to be removed.");
  }

  const existingEventsFile = path.join(restoreTestDir, "existing-kernel-events.sqlite3");
  const originalEventsContent = Buffer.from([4, 5, 6, 7]);
  await writeFile(existingEventsFile, originalEventsContent);
  const existingEventsBackup = await backupLocalFile(existingEventsFile);
  await writeFile(existingEventsFile, Buffer.from([8, 9]));
  if ((await restoreLocalFile(existingEventsBackup)) !== true) {
    throw new Error("Self-test expected existing event store restore to report true.");
  }
  const restoredEventsContent = await readFile(existingEventsFile);
  if (!restoredEventsContent.equals(originalEventsContent)) {
    throw new Error("Self-test expected existing event store content to be restored.");
  }
  assertWorkflowRestoresVerified({
    settingsRestored: true,
    appDataEventsRestored: true,
  });
  assertSelfTestThrows(
    () =>
      assertWorkflowRestoresVerified({
        settingsRestored: false,
        appDataEventsRestored: true,
      }),
    "settings file",
  );
  assertSelfTestThrows(
    () =>
      assertWorkflowRestoresVerified({
        settingsRestored: true,
        appDataEventsRestored: false,
      }),
    "app-data event store",
  );
  console.log("windows-installed-ui-smoke self-test ok");
}

function assertSelfTestThrows(action, expectedMessage) {
  try {
    action();
  } catch (error) {
    if (String(error?.message ?? error).includes(expectedMessage)) {
      return;
    }
    throw error;
  }

  throw new Error(`Self-test expected error containing: ${expectedMessage}`);
}

if (args.has("--self-test")) {
  await runSelfTest();
} else {
  await main();
}
