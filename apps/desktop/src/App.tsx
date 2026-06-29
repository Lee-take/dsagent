import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import {
  ArchiveRestore,
  Brain,
  Check,
  Clipboard,
  ClipboardList,
  Cloud,
  Database,
  FileText,
  FolderOpen,
  Globe2,
  Languages,
  Link2,
  Mail,
  MonitorCog,
  MousePointerClick,
  Network,
  PackageOpen,
  Pencil,
  Plus,
  Search,
  ShieldCheck,
  TerminalSquare,
  X,
} from "lucide-react";
import { useEffect, useState } from "react";
import type { ChangeEvent, FormEvent } from "react";
import { translations } from "./i18n";
import type {
  AccessMode,
  CapabilityAccessRecord,
  CapabilityDescriptor,
  CapabilityFamily,
  CapabilityInvocation,
  CapabilityKind,
  ComputerControlUnlockStatus,
  DeepSeekChatCacheState,
  DeepSeekChatTelemetry,
  DeepSeekPricingState,
  ComputerUseBackendStatus,
  DeepSeekCredentialStatus,
  FoundationState,
  LargeModelProvider,
  Language,
  LocalDirectoryState,
  MemoryCandidateMergePreview,
  MemoryCandidateReplacePreview,
  MemoryCandidateRecord,
  MemoryLifecycle,
  MemoryRecord,
  MemoryRecordDeletion,
  MemoryRecordUpdate,
  MemoryScope,
  MemorySensitivity,
  MemoryType,
  ModelRoute,
  ModelDrivenToolStrategy,
  NetworkSearchSourceModel,
  NetworkSearchRouteStatus,
  OperationsBriefingRun,
  PermissionAuditEntry,
  TaskRecord,
  TerminalReadCommand,
  ThemeStyle,
  ThinkingLevel,
  WorkPackage,
  WorkPackageImportPreview,
  WorkPackageImportSummary,
} from "./types";

const fallbackState: FoundationState = {
  app_name: "DeepSeek Agent OS",
  large_model_provider: "deepseek",
  model_route: "auto",
  thinking_level: "auto",
  access_mode: "ask_on_risk",
  workspace_scope: "workspace",
  network_search_source_model: null,
  tool_backends: {
    network_search: "source_backed_model",
    email: "architecture_only",
    drive: "local_folder_export_package",
    computer_screenshot: "local_windows_screen_capture",
    computer_control: "local_windows_input_control",
  },
};

const fallbackDeepSeekCredentialStatus: DeepSeekCredentialStatus = {
  base_url: "https://api.deepseek.com",
  chat_completions_url: "https://api.deepseek.com/chat/completions",
  api_key_env_var: "DEEPSEEK_API_KEY",
  api_key_configured: false,
  chat_completion_ready: false,
  flash_model: "deepseek-v4-flash",
  pro_model: "deepseek-v4-pro",
  readiness_note:
    "set DEEPSEEK_API_KEY in the local process environment to enable Chat Completions requests",
};

const fallbackDeepSeekChatCacheState: DeepSeekChatCacheState = {
  entries: 0,
};

const fallbackNetworkSearchRouteStatus: NetworkSearchRouteStatus = {
  backend: "source_backed_model",
  execution_mode: "permission_audit_only",
  evidence_policy: "pending_user_confirmation",
  network_requests_enabled: false,
  deepseek_orchestration_ready: false,
  requires_user_confirmation: true,
  note:
    "The selected large model does not provide source-backed NetworkSearch; choose a free NetworkSearch source model before running search.",
};

const fallbackComputerUseBackendStatus: ComputerUseBackendStatus = {
  screenshot_backend: "local_windows_screen_capture",
  screenshot_available: true,
  screenshot_note:
    "screen pixels are routed through the local Windows screen capture library",
  screenshot_permission_required: false,
  screenshot_permission_note:
    "Local Windows desktop capture usually runs without a separate OS permission prompt, but secure desktops and protected windows can block pixels.",
  control_backend: "local_windows_input_control",
  control_available: true,
  control_requires_approval: true,
  control_note:
    "mouse and keyboard control is routed through the local Windows input library",
  control_permission_required: false,
  control_permission_note:
    "Local Windows input control runs against the foreground desktop and can be blocked by secure desktop prompts or elevated target windows.",
  codex_bridge: {
    required: false,
    transport_env_var: "DEEPSEEK_AGENT_OS_CODEX_BRIDGE_TRANSPORT",
    transport: null,
    transport_decision_required: false,
    transport_options: [
      {
        value: "http",
        label: "External HTTP bridge",
        note:
          "Use an external loopback HTTP service with health, screenshot, control, and network-search endpoints.",
      },
    ],
    endpoint_env_var: "DEEPSEEK_AGENT_OS_CODEX_BRIDGE_URL",
    endpoint_configured: false,
    connected: false,
    note:
      "Codex bridge runtime is not required for the selected local Computer Use route.",
  },
};

const fallbackComputerControlUnlockStatus: ComputerControlUnlockStatus = {
  challenge: "",
  unlocked: false,
  unlocked_until: null,
};

const fallbackModelDrivenToolStrategy: ModelDrivenToolStrategy = {
  large_model_provider: "deepseek",
  large_model_supports_network_search: false,
  network_search_source_model_required: true,
  network_search_source_model: null,
  free_network_search_source_model_options: [
    {
      value: "free_web_source",
      label: "Free web source model",
      note: "Use a free source-backed web-search adapter for evidence and citations.",
    },
    {
      value: "free_local_browser",
      label: "Free local browser search (alpha)",
      note:
        "Alpha preset: currently uses the shared source-backed HTTP adapter; reserved for local browser/search-page retrieval.",
    },
    {
      value: "free_source_aggregator",
      label: "Free source aggregator (alpha)",
      note:
        "Alpha preset: currently uses the shared source-backed HTTP adapter; reserved for pluggable source aggregation.",
    },
  ],
  network_search_backend: "source_backed_model",
  computer_screenshot_backend: "local_windows_screen_capture",
  computer_control_backend: "local_windows_input_control",
  runtime_platform: "windows",
  macos_supported: true,
  note:
    "Selected large model needs a separate source-backed NetworkSearch model before NetworkSearch can run.",
};

const fallbackLocalDirectoryState: LocalDirectoryState = {
  app_data_dir: "",
  settings_file: "",
  settings: null,
  needs_setup: true,
};

const fallbackDeepSeekPricingState: DeepSeekPricingState = {
  app_data_dir: "",
  settings_file: "",
  pricing_configured: false,
  note: "DeepSeek cost estimates are disabled until a local pricing table is configured",
  settings: {
    enabled: false,
    flash_prompt_usd_per_million_tokens: "",
    flash_completion_usd_per_million_tokens: "",
    pro_prompt_usd_per_million_tokens: "",
    pro_completion_usd_per_million_tokens: "",
  },
};

const LANGUAGE_STORAGE_KEY = "deepseek-agent-os:ui-language:v1";
const THEME_STORAGE_KEY = "deepseek-agent-os:theme-style:v1";

const memoryTypeValues: MemoryType[] = [
  "preference",
  "project_context",
  "workflow_rule",
  "artifact",
  "failure_pattern",
];

const memoryScopeValues: MemoryScope[] = ["workspace", "project", "organization", "user"];

const memorySensitivityValues: MemorySensitivity[] = ["normal", "sensitive"];

const memoryLifecycleValues: MemoryLifecycle[] = ["active", "archived", "expires"];

type MemoryEditDraft = {
  id: string;
  title: string;
  body: string;
  memory_type: MemoryType;
  scope: MemoryScope;
  sensitivity: MemorySensitivity;
  lifecycle: MemoryLifecycle;
  expires_at: string;
};

function isoToDateInputValue(value: string | null): string {
  return value ? value.slice(0, 10) : "";
}

function dateInputValueToIso(
  value: string,
  lifecycle: MemoryLifecycle,
): string | null {
  if (lifecycle !== "expires" || !value) {
    return null;
  }

  return `${value}T23:59:59.000Z`;
}

function readInitialLanguage(): Language {
  if (typeof window === "undefined") {
    return "zh";
  }

  const storedLanguage = window.localStorage.getItem(LANGUAGE_STORAGE_KEY);
  return storedLanguage === "en" ? "en" : "zh";
}

function readInitialThemeStyle(): ThemeStyle {
  if (typeof window === "undefined") {
    return "deep";
  }

  const storedTheme = window.localStorage.getItem(THEME_STORAGE_KEY);
  if (storedTheme === "ink" || storedTheme === "porcelain") {
    return storedTheme;
  }
  return "deep";
}

function formatTaskDate(value: string, language: Language) {
  return new Intl.DateTimeFormat(language === "zh" ? "zh-CN" : "en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(value));
}

function formatMicroUsd(value: number) {
  const dollars = value / 1_000_000;
  const formatted = dollars.toFixed(6).replace(/\.?0+$/, "");
  return `$${formatted || "0"}`;
}

function capabilityFamilyIcon(family: CapabilityFamily) {
  switch (family) {
    case "file":
      return FileText;
    case "network":
      return Network;
    case "browser":
      return Globe2;
    case "email":
      return Mail;
    case "drive":
      return Cloud;
    case "terminal":
      return TerminalSquare;
    case "computer_use":
      return MonitorCog;
  }
}

export function App() {
  const [state, setState] = useState<FoundationState>(fallbackState);
  const [deepSeekCredentialStatus, setDeepSeekCredentialStatus] =
    useState<DeepSeekCredentialStatus>(fallbackDeepSeekCredentialStatus);
  const [deepSeekChatCacheState, setDeepSeekChatCacheState] =
    useState<DeepSeekChatCacheState>(fallbackDeepSeekChatCacheState);
  const [deepSeekTelemetry, setDeepSeekTelemetry] = useState<DeepSeekChatTelemetry[]>([]);
  const [networkSearchRouteStatus, setNetworkSearchRouteStatus] =
    useState<NetworkSearchRouteStatus>(fallbackNetworkSearchRouteStatus);
  const [computerUseBackendStatus, setComputerUseBackendStatus] =
    useState<ComputerUseBackendStatus>(fallbackComputerUseBackendStatus);
  const [computerControlUnlockStatus, setComputerControlUnlockStatus] =
    useState<ComputerControlUnlockStatus>(fallbackComputerControlUnlockStatus);
  const [modelToolStrategy, setModelToolStrategy] =
    useState<ModelDrivenToolStrategy>(fallbackModelDrivenToolStrategy);
  const [localDirectoryState, setLocalDirectoryState] =
    useState<LocalDirectoryState>(fallbackLocalDirectoryState);
  const [deepSeekPricingState, setDeepSeekPricingState] =
    useState<DeepSeekPricingState>(fallbackDeepSeekPricingState);
  const [language, setLanguage] = useState<Language>(readInitialLanguage);
  const [themeStyle, setThemeStyle] = useState<ThemeStyle>(readInitialThemeStyle);
  const [taskRecords, setTaskRecords] = useState<TaskRecord[]>([]);
  const [memoryRecords, setMemoryRecords] = useState<MemoryRecord[]>([]);
  const [memoryCandidateRecords, setMemoryCandidateRecords] = useState<MemoryCandidateRecord[]>([]);
  const [permissionAudits, setPermissionAudits] = useState<PermissionAuditEntry[]>([]);
  const [capabilityCatalog, setCapabilityCatalog] = useState<CapabilityDescriptor[]>([]);
  const [capabilityRecords, setCapabilityRecords] = useState<CapabilityAccessRecord[]>([]);
  const [capabilityInvocations, setCapabilityInvocations] = useState<CapabilityInvocation[]>([]);
  const [operationsBriefingRuns, setOperationsBriefingRuns] = useState<OperationsBriefingRun[]>([]);
  const [memoryQuery, setMemoryQuery] = useState("");
  const [candidateTitle, setCandidateTitle] = useState("");
  const [candidateBody, setCandidateBody] = useState("");
  const [candidateMemoryType, setCandidateMemoryType] = useState<MemoryType>("preference");
  const [candidateMemoryScope, setCandidateMemoryScope] = useState<MemoryScope>("workspace");
  const [candidateSensitivity, setCandidateSensitivity] =
    useState<MemorySensitivity>("normal");
  const [candidateLifecycle, setCandidateLifecycle] = useState<MemoryLifecycle>("active");
  const [candidateExpiresAt, setCandidateExpiresAt] = useState("");
  const [memoryEditDraft, setMemoryEditDraft] = useState<MemoryEditDraft | null>(null);
  const [memoryMergePreview, setMemoryMergePreview] =
    useState<MemoryCandidateMergePreview | null>(null);
  const [memoryReplacePreview, setMemoryReplacePreview] =
    useState<MemoryCandidateReplacePreview | null>(null);
  const [browserUrl, setBrowserUrl] = useState("");
  const [browserSubmitUrl, setBrowserSubmitUrl] = useState("");
  const [browserSubmitSummary, setBrowserSubmitSummary] = useState("");
  const [networkSearchQuery, setNetworkSearchQuery] = useState("");
  const [networkSearchScope, setNetworkSearchScope] = useState("");
  const [filePath, setFilePath] = useState("");
  const [fileWritePath, setFileWritePath] = useState("");
  const [fileWriteSummary, setFileWriteSummary] = useState("");
  const [fileWriteContent, setFileWriteContent] = useState("");
  const [folderPath, setFolderPath] = useState("");
  const [briefingFolderPath, setBriefingFolderPath] = useState("");
  const [terminalCommand, setTerminalCommand] =
    useState<TerminalReadCommand>("git status --short");
  const [terminalWriteCommand, setTerminalWriteCommand] = useState("");
  const [emailMailbox, setEmailMailbox] = useState("");
  const [emailReadQuery, setEmailReadQuery] = useState("");
  const [driveLocation, setDriveLocation] = useState("");
  const [driveReadQuery, setDriveReadQuery] = useState("");
  const [driveWriteLocation, setDriveWriteLocation] = useState("");
  const [driveWriteSummary, setDriveWriteSummary] = useState("");
  const [draftEmailTo, setDraftEmailTo] = useState("");
  const [draftEmailSubject, setDraftEmailSubject] = useState("");
  const [draftEmailBody, setDraftEmailBody] = useState("");
  const [emailTo, setEmailTo] = useState("");
  const [emailSubject, setEmailSubject] = useState("");
  const [emailBody, setEmailBody] = useState("");
  const [computerControlTarget, setComputerControlTarget] = useState("");
  const [computerControlAction, setComputerControlAction] = useState("");
  const [computerControlUnlockToken, setComputerControlUnlockToken] = useState("");
  const [setupWorkspaceDir, setSetupWorkspaceDir] = useState("");
  const [setupEvidenceDir, setSetupEvidenceDir] = useState("");
  const [setupExportDir, setSetupExportDir] = useState("");
  const [deepSeekPricingEnabled, setDeepSeekPricingEnabled] = useState(false);
  const [deepSeekFlashPromptPrice, setDeepSeekFlashPromptPrice] = useState("");
  const [deepSeekFlashCompletionPrice, setDeepSeekFlashCompletionPrice] = useState("");
  const [deepSeekProPromptPrice, setDeepSeekProPromptPrice] = useState("");
  const [deepSeekProCompletionPrice, setDeepSeekProCompletionPrice] = useState("");
  const [taskTitle, setTaskTitle] = useState("");
  const [taskSummary, setTaskSummary] = useState("");
  const [exportedPackageJson, setExportedPackageJson] = useState("");
  const [importPackageJson, setImportPackageJson] = useState("");
  const [importPreview, setImportPreview] = useState<WorkPackageImportPreview | null>(null);
  const [packageNotice, setPackageNotice] = useState("");
  const [packageError, setPackageError] = useState("");
  const [memoryNotice, setMemoryNotice] = useState("");
  const [memoryError, setMemoryError] = useState("");
  const [memoryCandidateNotice, setMemoryCandidateNotice] = useState("");
  const [memoryCandidateError, setMemoryCandidateError] = useState("");
  const [auditError, setAuditError] = useState("");
  const [capabilityError, setCapabilityError] = useState("");
  const [browserNotice, setBrowserNotice] = useState("");
  const [browserError, setBrowserError] = useState("");
  const [browserSubmitNotice, setBrowserSubmitNotice] = useState("");
  const [browserSubmitError, setBrowserSubmitError] = useState("");
  const [networkSearchNotice, setNetworkSearchNotice] = useState("");
  const [networkSearchError, setNetworkSearchError] = useState("");
  const [fileNotice, setFileNotice] = useState("");
  const [fileError, setFileError] = useState("");
  const [fileWriteNotice, setFileWriteNotice] = useState("");
  const [fileWriteError, setFileWriteError] = useState("");
  const [folderNotice, setFolderNotice] = useState("");
  const [folderError, setFolderError] = useState("");
  const [terminalNotice, setTerminalNotice] = useState("");
  const [terminalError, setTerminalError] = useState("");
  const [terminalWriteNotice, setTerminalWriteNotice] = useState("");
  const [terminalWriteError, setTerminalWriteError] = useState("");
  const [emailReadNotice, setEmailReadNotice] = useState("");
  const [emailReadError, setEmailReadError] = useState("");
  const [driveReadNotice, setDriveReadNotice] = useState("");
  const [driveReadError, setDriveReadError] = useState("");
  const [driveWriteNotice, setDriveWriteNotice] = useState("");
  const [driveWriteError, setDriveWriteError] = useState("");
  const [emailDraftNotice, setEmailDraftNotice] = useState("");
  const [emailDraftError, setEmailDraftError] = useState("");
  const [emailNotice, setEmailNotice] = useState("");
  const [emailError, setEmailError] = useState("");
  const [computerNotice, setComputerNotice] = useState("");
  const [computerError, setComputerError] = useState("");
  const [computerControlNotice, setComputerControlNotice] = useState("");
  const [computerControlError, setComputerControlError] = useState("");
  const [computerControlUnlockNotice, setComputerControlUnlockNotice] = useState("");
  const [computerControlUnlockError, setComputerControlUnlockError] = useState("");
  const [briefingNotice, setBriefingNotice] = useState("");
  const [briefingError, setBriefingError] = useState("");
  const [setupNotice, setSetupNotice] = useState("");
  const [setupError, setSetupError] = useState("");
  const [deepSeekCacheNotice, setDeepSeekCacheNotice] = useState("");
  const [deepSeekCacheError, setDeepSeekCacheError] = useState("");
  const [deepSeekPricingNotice, setDeepSeekPricingNotice] = useState("");
  const [deepSeekPricingError, setDeepSeekPricingError] = useState("");
  const [packagePending, setPackagePending] = useState(false);
  const [memoryPending, setMemoryPending] = useState(false);
  const [memoryCandidatePending, setMemoryCandidatePending] = useState(false);
  const [memoryCandidateResolutionPending, setMemoryCandidateResolutionPending] =
    useState<string | null>(null);
  const [memoryMergePreviewPending, setMemoryMergePreviewPending] =
    useState<string | null>(null);
  const [memoryReplacePreviewPending, setMemoryReplacePreviewPending] =
    useState<string | null>(null);
  const [memoryUpdatePending, setMemoryUpdatePending] = useState<string | null>(null);
  const [memoryDeletionPending, setMemoryDeletionPending] = useState<string | null>(null);
  const [browserPending, setBrowserPending] = useState(false);
  const [browserSubmitPending, setBrowserSubmitPending] = useState(false);
  const [networkSearchPending, setNetworkSearchPending] = useState(false);
  const [filePending, setFilePending] = useState(false);
  const [fileWritePending, setFileWritePending] = useState(false);
  const [folderPending, setFolderPending] = useState(false);
  const [terminalPending, setTerminalPending] = useState(false);
  const [terminalWritePending, setTerminalWritePending] = useState(false);
  const [emailReadPending, setEmailReadPending] = useState(false);
  const [driveReadPending, setDriveReadPending] = useState(false);
  const [driveWritePending, setDriveWritePending] = useState(false);
  const [emailDraftPending, setEmailDraftPending] = useState(false);
  const [emailPending, setEmailPending] = useState(false);
  const [computerPending, setComputerPending] = useState(false);
  const [computerControlPending, setComputerControlPending] = useState(false);
  const [computerControlUnlockPending, setComputerControlUnlockPending] = useState(false);
  const [briefingPending, setBriefingPending] = useState(false);
  const [setupPending, setSetupPending] = useState(false);
  const [deepSeekCachePending, setDeepSeekCachePending] = useState(false);
  const [deepSeekPricingPending, setDeepSeekPricingPending] = useState(false);
  const [capabilityPending, setCapabilityPending] = useState<CapabilityKind | null>(null);
  const [resolutionPending, setResolutionPending] = useState<string | null>(null);
  const copy = translations[language];
  const networkSearchSourceModelMissing =
    modelToolStrategy.network_search_source_model_required &&
    !state.network_search_source_model;
  const latestDeepSeekTelemetry = deepSeekTelemetry[0] ?? null;
  const latestDeepSeekTelemetryCacheLabel = latestDeepSeekTelemetry
    ? latestDeepSeekTelemetry.cache_status === "hit"
      ? copy.backendLabels.cacheHit
      : latestDeepSeekTelemetry.cache_status === "miss"
        ? copy.backendLabels.cacheMiss
        : copy.backendLabels.cacheDisabled
    : "";
  const latestDeepSeekTelemetryText = latestDeepSeekTelemetry
    ? `${latestDeepSeekTelemetryCacheLabel} / ${latestDeepSeekTelemetry.elapsed_ms}ms / ${
        latestDeepSeekTelemetry.total_tokens ?? copy.backendLabels.notSelected
      } ${copy.backendLabels.tokens}${
        latestDeepSeekTelemetry.estimated_cost_micro_usd !== null
          ? ` / ${copy.backendLabels.cost} ${formatMicroUsd(
              latestDeepSeekTelemetry.estimated_cost_micro_usd,
            )}`
          : ""
      }`
    : copy.backendLabels.noTelemetry;
  const computerControlUnlockUntilText =
    computerControlUnlockStatus.unlocked && computerControlUnlockStatus.unlocked_until
      ? formatTaskDate(computerControlUnlockStatus.unlocked_until, language)
      : "";

  const hydrateLocalDirectoryInputs = (directoryState: LocalDirectoryState) => {
    if (!directoryState.settings) {
      return;
    }

    const { workspace_dir, evidence_dir, export_dir } = directoryState.settings;
    setSetupWorkspaceDir(workspace_dir);
    setSetupEvidenceDir(evidence_dir);
    setSetupExportDir(export_dir);
    setBriefingFolderPath((current) => current || evidence_dir);
    setFolderPath((current) => current || evidence_dir);
    setDriveLocation((current) => current || workspace_dir);
    setDriveWriteLocation((current) => current || export_dir);
  };

  const hydrateDeepSeekPricingInputs = (pricingState: DeepSeekPricingState) => {
    setDeepSeekPricingEnabled(pricingState.settings.enabled);
    setDeepSeekFlashPromptPrice(
      pricingState.settings.flash_prompt_usd_per_million_tokens,
    );
    setDeepSeekFlashCompletionPrice(
      pricingState.settings.flash_completion_usd_per_million_tokens,
    );
    setDeepSeekProPromptPrice(pricingState.settings.pro_prompt_usd_per_million_tokens);
    setDeepSeekProCompletionPrice(
      pricingState.settings.pro_completion_usd_per_million_tokens,
    );
  };

  useEffect(() => {
    void invoke<FoundationState>("get_foundation_state")
      .then(setState)
      .catch(() => setState(fallbackState));
    void invoke<DeepSeekCredentialStatus>("get_deepseek_credential_status")
      .then(setDeepSeekCredentialStatus)
      .catch(() => setDeepSeekCredentialStatus(fallbackDeepSeekCredentialStatus));
    void invoke<DeepSeekChatCacheState>("get_deepseek_chat_cache_state")
      .then(setDeepSeekChatCacheState)
      .catch(() => setDeepSeekChatCacheState(fallbackDeepSeekChatCacheState));
    void invoke<DeepSeekChatTelemetry[]>("list_deepseek_chat_telemetry")
      .then(setDeepSeekTelemetry)
      .catch(() => setDeepSeekTelemetry([]));
    void invoke<ComputerControlUnlockStatus>("get_computer_control_unlock_status")
      .then(setComputerControlUnlockStatus)
      .catch(() => setComputerControlUnlockStatus(fallbackComputerControlUnlockStatus));
    void invoke<LocalDirectoryState>("get_local_directory_state")
      .then((directoryState) => {
        setLocalDirectoryState(directoryState);
        hydrateLocalDirectoryInputs(directoryState);
      })
      .catch(() => {
        setLocalDirectoryState(fallbackLocalDirectoryState);
        setSetupError(copy.localSetup.loadFailed);
      });
    void invoke<DeepSeekPricingState>("get_deepseek_pricing_state")
      .then((pricingState) => {
        setDeepSeekPricingState(pricingState);
        hydrateDeepSeekPricingInputs(pricingState);
      })
      .catch(() => {
        setDeepSeekPricingState(fallbackDeepSeekPricingState);
        setDeepSeekPricingError(copy.deepSeekPricing.loadFailed);
      });
  }, []);

  useEffect(() => {
    let cancelled = false;
    const strategyRequest = {
      largeModelProvider: state.large_model_provider,
      networkSearchSourceModel: state.network_search_source_model,
    };

    void Promise.all([
      invoke<ModelDrivenToolStrategy>("get_model_driven_tool_strategy", strategyRequest),
      invoke<NetworkSearchRouteStatus>(
        "get_network_search_route_status_for_model",
        strategyRequest,
      ),
      invoke<ComputerUseBackendStatus>(
        "get_computer_use_backend_status_for_model",
        strategyRequest,
      ),
    ])
      .then(([strategy, networkSearchStatus, computerUseStatus]) => {
        if (cancelled) {
          return;
        }

        setModelToolStrategy(strategy);
        setNetworkSearchRouteStatus(networkSearchStatus);
        setComputerUseBackendStatus(computerUseStatus);
        setState((currentState) => ({
          ...currentState,
          tool_backends: {
            ...currentState.tool_backends,
            network_search: strategy.network_search_backend,
            computer_screenshot: strategy.computer_screenshot_backend,
            computer_control: strategy.computer_control_backend,
          },
        }));
      })
      .catch(() => {
        if (cancelled) {
          return;
        }

        setModelToolStrategy(fallbackModelDrivenToolStrategy);
        setNetworkSearchRouteStatus(fallbackNetworkSearchRouteStatus);
        setComputerUseBackendStatus(fallbackComputerUseBackendStatus);
      });

    return () => {
      cancelled = true;
    };
  }, [state.large_model_provider, state.network_search_source_model]);

  useEffect(() => {
    void Promise.all([
      invoke<TaskRecord[]>("list_task_records"),
      invoke<MemoryRecord[]>("list_memory_records"),
      invoke<MemoryCandidateRecord[]>("list_memory_candidate_records"),
      invoke<PermissionAuditEntry[]>("list_permission_audit_entries"),
      invoke<CapabilityDescriptor[]>("list_capability_catalog"),
      invoke<CapabilityAccessRecord[]>("list_capability_access_records"),
      invoke<CapabilityInvocation[]>("list_capability_invocations"),
      invoke<OperationsBriefingRun[]>("list_operations_briefing_runs"),
    ])
      .then(([
        records,
        memories,
        memoryCandidates,
        audits,
        catalog,
        capabilityAccessRecords,
        invocations,
        briefingRuns,
      ]) => {
        setTaskRecords(records);
        setMemoryRecords(memories);
        setMemoryCandidateRecords(memoryCandidates);
        setPermissionAudits(audits);
        setCapabilityCatalog(catalog);
        setCapabilityRecords(capabilityAccessRecords);
        setCapabilityInvocations(invocations);
        setOperationsBriefingRuns(briefingRuns);
      })
      .catch(() => {
        setPackageError(copy.package.loadFailed);
        setMemoryError(copy.memory.loadFailed);
        setMemoryCandidateError(copy.memory.loadFailed);
        setAuditError(copy.audit.loadFailed);
        setCapabilityError(copy.capabilities.loadFailed);
        setBriefingError(copy.operationsBriefing.loadFailed);
      });
  }, [
    copy.audit.loadFailed,
    copy.capabilities.loadFailed,
    copy.memory.loadFailed,
    copy.operationsBriefing.loadFailed,
    copy.package.loadFailed,
  ]);

  useEffect(() => {
    document.documentElement.lang = language === "zh" ? "zh-CN" : "en";
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, language);
  }, [language]);

  useEffect(() => {
    document.documentElement.dataset.theme = themeStyle;
    window.localStorage.setItem(THEME_STORAGE_KEY, themeStyle);
  }, [themeStyle]);

  const updateModelRoute = (event: ChangeEvent<HTMLSelectElement>) => {
    setState((currentState) => ({
      ...currentState,
      model_route: event.target.value as ModelRoute,
    }));
  };

  const updateLargeModelProvider = (event: ChangeEvent<HTMLSelectElement>) => {
    setState((currentState) => ({
      ...currentState,
      large_model_provider: event.target.value as LargeModelProvider,
    }));
  };

  const updateNetworkSearchSourceModel = (event: ChangeEvent<HTMLSelectElement>) => {
    const value = event.target.value;
    setState((currentState) => ({
      ...currentState,
      network_search_source_model: value
        ? (value as NetworkSearchSourceModel)
        : null,
    }));
  };

  const updateAccessMode = (event: ChangeEvent<HTMLSelectElement>) => {
    setState((currentState) => ({
      ...currentState,
      access_mode: event.target.value as AccessMode,
    }));
  };

  const updateThinkingLevel = (event: ChangeEvent<HTMLSelectElement>) => {
    setState((currentState) => ({
      ...currentState,
      thinking_level: event.target.value as ThinkingLevel,
    }));
  };

  const updateThemeStyle = (event: ChangeEvent<HTMLSelectElement>) => {
    setThemeStyle(event.target.value as ThemeStyle);
  };

  const switchLanguage = (nextLanguage: Language) => {
    setLanguage(nextLanguage);
  };

  const clearDeepSeekChatCache = async () => {
    setDeepSeekCachePending(true);
    setDeepSeekCacheNotice("");
    setDeepSeekCacheError("");

    try {
      const removed = await invoke<number>("clear_deepseek_chat_cache");
      const cacheState = await invoke<DeepSeekChatCacheState>(
        "get_deepseek_chat_cache_state",
      );
      setDeepSeekChatCacheState(cacheState);
      setDeepSeekCacheNotice(copy.backendLabels.cacheCleared(removed));
    } catch (error) {
      setDeepSeekCacheError(String(error) || copy.backendLabels.cacheClearFailed);
    } finally {
      setDeepSeekCachePending(false);
    }
  };

  const saveLocalDirectorySetup = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setSetupPending(true);
    setSetupError("");
    setSetupNotice("");

    try {
      const directoryState = await invoke<LocalDirectoryState>(
        "save_local_directory_settings",
        {
          workspaceDir: setupWorkspaceDir,
          evidenceDir: setupEvidenceDir,
          exportDir: setupExportDir,
        },
      );
      setLocalDirectoryState(directoryState);
      hydrateLocalDirectoryInputs(directoryState);
      setSetupNotice(copy.localSetup.saved);
    } catch (error) {
      setSetupError(String(error) || copy.localSetup.failed);
    } finally {
      setSetupPending(false);
    }
  };

  const saveDeepSeekPricingSetup = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setDeepSeekPricingPending(true);
    setDeepSeekPricingNotice("");
    setDeepSeekPricingError("");

    try {
      const pricingState = await invoke<DeepSeekPricingState>(
        "save_deepseek_pricing_settings",
        {
          enabled: deepSeekPricingEnabled,
          flashPromptUsdPerMillionTokens: deepSeekFlashPromptPrice,
          flashCompletionUsdPerMillionTokens: deepSeekFlashCompletionPrice,
          proPromptUsdPerMillionTokens: deepSeekProPromptPrice,
          proCompletionUsdPerMillionTokens: deepSeekProCompletionPrice,
        },
      );
      setDeepSeekPricingState(pricingState);
      hydrateDeepSeekPricingInputs(pricingState);
      setDeepSeekPricingNotice(copy.deepSeekPricing.saved);
    } catch (error) {
      setDeepSeekPricingError(String(error) || copy.deepSeekPricing.failed);
    } finally {
      setDeepSeekPricingPending(false);
    }
  };

  const chooseLocalDirectory = async (
    target: "workspace" | "evidence" | "export",
  ) => {
    const currentPath =
      target === "workspace"
        ? setupWorkspaceDir
        : target === "evidence"
          ? setupEvidenceDir
          : setupExportDir;
    const title =
      target === "workspace"
        ? copy.localSetup.workspaceDialogTitle
        : target === "evidence"
          ? copy.localSetup.evidenceDialogTitle
          : copy.localSetup.exportDialogTitle;

    setSetupError("");
    setSetupNotice("");

    try {
      const selected = await open({
        title,
        directory: true,
        multiple: false,
        defaultPath: currentPath || undefined,
      });

      if (!selected || Array.isArray(selected)) {
        return;
      }

      if (target === "workspace") {
        setSetupWorkspaceDir(selected);
      } else if (target === "evidence") {
        setSetupEvidenceDir(selected);
      } else {
        setSetupExportDir(selected);
      }
    } catch (error) {
      setSetupError(String(error) || copy.localSetup.chooseFailed);
    }
  };

  const loadMemoryRecords = async (query: string) => {
    const trimmedQuery = query.trim();
    if (!trimmedQuery) {
      return invoke<MemoryRecord[]>("list_memory_records");
    }

    return invoke<MemoryRecord[]>("search_memory_records", {
      query: trimmedQuery,
    });
  };

  const refreshCapabilityState = async () => {
    const [records, audits, invocations] = await Promise.all([
      invoke<CapabilityAccessRecord[]>("list_capability_access_records"),
      invoke<PermissionAuditEntry[]>("list_permission_audit_entries"),
      invoke<CapabilityInvocation[]>("list_capability_invocations"),
    ]);
    setCapabilityRecords(records);
    setPermissionAudits(audits);
    setCapabilityInvocations(invocations);
  };

  const refreshComputerControlUnlockStatus = async () => {
    const unlockStatus = await invoke<ComputerControlUnlockStatus>(
      "get_computer_control_unlock_status",
    );
    setComputerControlUnlockStatus(unlockStatus);
    return unlockStatus;
  };

  const refreshOperationsBriefingRuns = async () => {
    const runs = await invoke<OperationsBriefingRun[]>("list_operations_briefing_runs");
    setOperationsBriefingRuns(runs);
  };

  const refreshMemoryCandidateRecords = async () => {
    const candidates = await invoke<MemoryCandidateRecord[]>("list_memory_candidate_records");
    setMemoryCandidateRecords(candidates);
  };

  const searchMemoryRecords = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setMemoryPending(true);
    setMemoryError("");
    setMemoryNotice("");

    try {
      const memories = await loadMemoryRecords(memoryQuery);
      setMemoryRecords(memories);
    } catch {
      setMemoryError(copy.memory.loadFailed);
    } finally {
      setMemoryPending(false);
    }
  };

  const proposeMemoryCandidate = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!candidateTitle.trim() || !candidateBody.trim()) {
      setMemoryCandidateError(copy.memory.emptyCandidate);
      return;
    }
    if (candidateLifecycle === "expires" && !candidateExpiresAt) {
      setMemoryCandidateError(copy.memory.emptyExpiration);
      return;
    }

    setMemoryCandidatePending(true);
    setMemoryCandidateError("");
    setMemoryCandidateNotice("");

    try {
      const candidate = await invoke<MemoryCandidateRecord>("propose_memory_candidate", {
        title: candidateTitle,
        body: candidateBody,
        memoryType: candidateMemoryType,
        scope: candidateMemoryScope,
        sensitivity: candidateSensitivity,
        lifecycle: candidateLifecycle,
        expiresAt: dateInputValueToIso(candidateExpiresAt, candidateLifecycle),
      });
      setMemoryCandidateRecords((currentCandidates) => [candidate, ...currentCandidates]);
      setCandidateTitle("");
      setCandidateBody("");
      setCandidateExpiresAt("");
      setMemoryCandidateNotice(copy.memory.proposed);
      setMemoryMergePreview(null);
      setMemoryReplacePreview(null);
    } catch (error) {
      setMemoryCandidateError(String(error) || copy.memory.proposeFailed);
    } finally {
      setMemoryCandidatePending(false);
    }
  };

  const resolveMemoryCandidate = async (candidateId: string, accepted: boolean) => {
    setMemoryCandidateResolutionPending(candidateId);
    setMemoryCandidateError("");
    setMemoryCandidateNotice("");

    try {
      await invoke("resolve_memory_candidate", {
        candidateId,
        accepted,
        note: accepted ? copy.memory.accept : copy.memory.reject,
      });
      const [memories] = await Promise.all([
        loadMemoryRecords(memoryQuery),
        refreshMemoryCandidateRecords(),
      ]);
      setMemoryRecords(memories);
      setMemoryCandidateNotice(accepted ? copy.memory.accepted : copy.memory.rejected);
      setMemoryMergePreview(null);
      setMemoryReplacePreview(null);
    } catch (error) {
      setMemoryCandidateError(String(error) || copy.memory.resolveFailed);
    } finally {
      setMemoryCandidateResolutionPending(null);
    }
  };

  const linkMemoryCandidateToConflicts = async (
    candidateId: string,
    linkedMemoryIds: string[],
  ) => {
    setMemoryCandidateResolutionPending(candidateId);
    setMemoryCandidateError("");
    setMemoryCandidateNotice("");

    try {
      await invoke("link_memory_candidate_to_conflicts", {
        candidateId,
        linkedMemoryIds,
        note: copy.memory.linkAndAccept,
      });
      const [memories] = await Promise.all([
        loadMemoryRecords(memoryQuery),
        refreshMemoryCandidateRecords(),
      ]);
      setMemoryRecords(memories);
      setMemoryCandidateNotice(copy.memory.linked);
      setMemoryMergePreview(null);
      setMemoryReplacePreview(null);
    } catch (error) {
      setMemoryCandidateError(String(error) || copy.memory.linkFailed);
    } finally {
      setMemoryCandidateResolutionPending(null);
    }
  };

  const previewMemoryCandidateMerge = async (
    candidateId: string,
    sourceMemoryIds: string[],
  ) => {
    setMemoryMergePreviewPending(candidateId);
    setMemoryCandidateError("");

    try {
      const preview = await invoke<MemoryCandidateMergePreview>(
        "preview_memory_candidate_merge",
        {
          candidateId,
          sourceMemoryIds,
        },
      );
      setMemoryMergePreview(preview);
    } catch (error) {
      setMemoryCandidateError(String(error) || copy.memory.mergePreviewFailed);
    } finally {
      setMemoryMergePreviewPending(null);
    }
  };

  const previewMemoryCandidateReplace = async (
    candidateId: string,
    targetMemoryIds: string[],
  ) => {
    setMemoryReplacePreviewPending(candidateId);
    setMemoryCandidateError("");

    try {
      const preview = await invoke<MemoryCandidateReplacePreview>(
        "preview_memory_candidate_replace",
        {
          candidateId,
          targetMemoryIds,
        },
      );
      setMemoryReplacePreview(preview);
    } catch (error) {
      setMemoryCandidateError(String(error) || copy.memory.replacePreviewFailed);
    } finally {
      setMemoryReplacePreviewPending(null);
    }
  };

  const mergeMemoryCandidateWithConflicts = async (
    candidateId: string,
    sourceMemoryIds: string[],
  ) => {
    setMemoryCandidateResolutionPending(candidateId);
    setMemoryCandidateError("");
    setMemoryCandidateNotice("");

    try {
      await invoke("merge_memory_candidate_with_conflicts", {
        candidateId,
        sourceMemoryIds,
        note: copy.memory.mergeAndAccept,
      });
      const [memories] = await Promise.all([
        loadMemoryRecords(memoryQuery),
        refreshMemoryCandidateRecords(),
      ]);
      setMemoryRecords(memories);
      setMemoryCandidateNotice(copy.memory.merged);
      setMemoryMergePreview(null);
      setMemoryReplacePreview(null);
    } catch (error) {
      setMemoryCandidateError(String(error) || copy.memory.mergeFailed);
    } finally {
      setMemoryCandidateResolutionPending(null);
    }
  };

  const replaceMemoryCandidateConflicts = async (
    candidateId: string,
    targetMemoryIds: string[],
  ) => {
    setMemoryCandidateResolutionPending(candidateId);
    setMemoryCandidateError("");
    setMemoryCandidateNotice("");

    try {
      await invoke("replace_memory_candidate_conflicts", {
        candidateId,
        targetMemoryIds,
        note: copy.memory.replaceAndAccept,
      });
      const [memories] = await Promise.all([
        loadMemoryRecords(memoryQuery),
        refreshMemoryCandidateRecords(),
      ]);
      setMemoryRecords(memories);
      setMemoryCandidateNotice(copy.memory.replaced);
      setMemoryMergePreview(null);
      setMemoryReplacePreview(null);
    } catch (error) {
      setMemoryCandidateError(String(error) || copy.memory.replaceFailed);
    } finally {
      setMemoryCandidateResolutionPending(null);
    }
  };

  const beginMemoryRecordEdit = (memory: MemoryRecord) => {
    setMemoryEditDraft({
      id: memory.id,
      title: memory.title,
      body: memory.body,
      memory_type: memory.memory_type,
      scope: memory.scope,
      sensitivity: memory.sensitivity,
      lifecycle: memory.lifecycle,
      expires_at: isoToDateInputValue(memory.expires_at),
    });
    setMemoryError("");
    setMemoryNotice("");
  };

  const patchMemoryEditDraft = (patch: Partial<MemoryEditDraft>) => {
    setMemoryEditDraft((currentDraft) =>
      currentDraft
        ? {
            ...currentDraft,
            ...patch,
          }
        : currentDraft,
    );
  };

  const cancelMemoryRecordEdit = () => {
    setMemoryEditDraft(null);
    setMemoryError("");
  };

  const updateMemoryRecord = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!memoryEditDraft) {
      return;
    }
    if (!memoryEditDraft.title.trim() || !memoryEditDraft.body.trim()) {
      setMemoryError(copy.memory.emptyEdit);
      return;
    }
    if (memoryEditDraft.lifecycle === "expires" && !memoryEditDraft.expires_at) {
      setMemoryError(copy.memory.emptyExpiration);
      return;
    }

    setMemoryUpdatePending(memoryEditDraft.id);
    setMemoryError("");
    setMemoryNotice("");

    try {
      await invoke<MemoryRecordUpdate>("update_memory_record", {
        memoryId: memoryEditDraft.id,
        title: memoryEditDraft.title,
        body: memoryEditDraft.body,
        memoryType: memoryEditDraft.memory_type,
        scope: memoryEditDraft.scope,
        sensitivity: memoryEditDraft.sensitivity,
        lifecycle: memoryEditDraft.lifecycle,
        expiresAt: dateInputValueToIso(
          memoryEditDraft.expires_at,
          memoryEditDraft.lifecycle,
        ),
        note: copy.memory.edit,
      });
      const memories = await loadMemoryRecords(memoryQuery);
      setMemoryRecords(memories);
      setMemoryEditDraft(null);
      setMemoryNotice(copy.memory.updated);
    } catch (error) {
      setMemoryError(String(error) || copy.memory.updateFailed);
    } finally {
      setMemoryUpdatePending(null);
    }
  };

  const deleteMemoryRecord = async (memoryId: string) => {
    setMemoryDeletionPending(memoryId);
    setMemoryError("");
    setMemoryNotice("");

    try {
      await invoke<MemoryRecordDeletion>("delete_memory_record", {
        memoryId,
        note: copy.memory.delete,
      });
      const memories = await loadMemoryRecords(memoryQuery);
      setMemoryRecords(memories);
      if (memoryEditDraft?.id === memoryId) {
        setMemoryEditDraft(null);
      }
      setMemoryNotice(copy.memory.deleted);
    } catch (error) {
      setMemoryError(String(error) || copy.memory.deleteFailed);
    } finally {
      setMemoryDeletionPending(null);
    }
  };

  const requestCapabilityAccess = async (capability: CapabilityKind) => {
    setCapabilityPending(capability);
    setCapabilityError("");
    setAuditError("");

    try {
      await invoke<CapabilityAccessRecord>("request_capability_access", {
        accessMode: state.access_mode,
        capability,
      });
      await refreshCapabilityState();
    } catch (error) {
      setCapabilityError(String(error) || copy.capabilities.requestFailed);
    } finally {
      setCapabilityPending(null);
    }
  };

  const browseBrowserUrl = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedUrl = browserUrl.trim();
    if (!trimmedUrl) {
      setBrowserError(copy.browserTool.failed);
      return;
    }

    setBrowserPending(true);
    setBrowserError("");
    setBrowserNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("browse_url", {
        accessMode: state.access_mode,
        url: trimmedUrl,
      });
      await refreshCapabilityState();
      if (invocation.status === "pending_approval") {
        setBrowserNotice(copy.browserTool.pendingHint);
      }
    } catch (error) {
      setBrowserError(String(error) || copy.browserTool.failed);
    } finally {
      setBrowserPending(false);
    }
  };

  const submitBrowserBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedUrl = browserSubmitUrl.trim();
    const trimmedSummary = browserSubmitSummary.trim();
    if (!trimmedUrl || !trimmedSummary) {
      setBrowserSubmitError(copy.browserSubmitTool.failed);
      return;
    }

    setBrowserSubmitPending(true);
    setBrowserSubmitError("");
    setBrowserSubmitNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("submit_browser_boundary", {
        accessMode: state.access_mode,
        url: trimmedUrl,
        summary: trimmedSummary,
      });
      await refreshCapabilityState();
      setBrowserSubmitNotice(
        invocation.status === "pending_approval"
          ? copy.browserSubmitTool.pendingHint
          : copy.browserSubmitTool.blocked,
      );
    } catch (error) {
      setBrowserSubmitError(String(error) || copy.browserSubmitTool.failed);
    } finally {
      setBrowserSubmitPending(false);
    }
  };

  const searchNetworkBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedQuery = networkSearchQuery.trim();
    const trimmedScope = networkSearchScope.trim();
    if (!trimmedQuery) {
      setNetworkSearchError(copy.networkSearchTool.failed);
      return;
    }
    if (networkSearchSourceModelMissing) {
      setNetworkSearchError(copy.networkSearchTool.sourceModelMissing);
      return;
    }
    if (!networkSearchRouteStatus.network_requests_enabled) {
      setNetworkSearchError(copy.networkSearchTool.routeNotEnabled);
      return;
    }

    setNetworkSearchPending(true);
    setNetworkSearchError("");
    setNetworkSearchNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("search_network_boundary", {
        accessMode: state.access_mode,
        largeModelProvider: state.large_model_provider,
        query: trimmedQuery,
        scope: trimmedScope,
        networkSearchSourceModel: state.network_search_source_model,
      });
      await refreshCapabilityState();
      setNetworkSearchNotice(
        invocation.status === "pending_approval"
          ? copy.networkSearchTool.pendingHint
          : invocation.status === "succeeded"
            ? copy.networkSearchTool.completed
            : copy.networkSearchTool.blocked,
      );
    } catch (error) {
      setNetworkSearchError(String(error) || copy.networkSearchTool.failed);
    } finally {
      setNetworkSearchPending(false);
    }
  };

  const readLocalFilePath = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedPath = filePath.trim();
    if (!trimmedPath) {
      setFileError(copy.fileTool.failed);
      return;
    }

    setFilePending(true);
    setFileError("");
    setFileNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("read_local_file", {
        accessMode: state.access_mode,
        path: trimmedPath,
      });
      await refreshCapabilityState();
      if (invocation.status === "pending_approval") {
        setFileNotice(copy.fileTool.pendingHint);
      }
    } catch (error) {
      setFileError(String(error) || copy.fileTool.failed);
    } finally {
      setFilePending(false);
    }
  };

  const writeFileBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedPath = fileWritePath.trim();
    const trimmedSummary = fileWriteSummary.trim();
    const trimmedContent = fileWriteContent.trim();
    if (!trimmedPath || !trimmedSummary || !trimmedContent) {
      setFileWriteError(copy.fileWriteTool.failed);
      return;
    }

    setFileWritePending(true);
    setFileWriteError("");
    setFileWriteNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("write_file_boundary", {
        accessMode: state.access_mode,
        path: trimmedPath,
        summary: trimmedSummary,
        content: fileWriteContent,
      });
      await refreshCapabilityState();
      setFileWriteNotice(
        invocation.status === "pending_approval"
          ? copy.fileWriteTool.pendingHint
          : invocation.status === "succeeded"
            ? copy.fileWriteTool.completed
            : copy.fileWriteTool.failed,
      );
    } catch (error) {
      setFileWriteError(String(error) || copy.fileWriteTool.failed);
    } finally {
      setFileWritePending(false);
    }
  };

  const ingestEvidenceFolderPath = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedPath = folderPath.trim();
    if (!trimmedPath) {
      setFolderError(copy.folderTool.failed);
      return;
    }

    setFolderPending(true);
    setFolderError("");
    setFolderNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("ingest_evidence_folder", {
        accessMode: state.access_mode,
        folderPath: trimmedPath,
      });
      await refreshCapabilityState();
      if (invocation.status === "pending_approval") {
        setFolderNotice(copy.folderTool.pendingHint);
      }
    } catch (error) {
      setFolderError(String(error) || copy.folderTool.failed);
    } finally {
      setFolderPending(false);
    }
  };

  const runTerminalReadCommand = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setTerminalPending(true);
    setTerminalError("");
    setTerminalNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("run_terminal_read", {
        accessMode: state.access_mode,
        command: terminalCommand,
      });
      await refreshCapabilityState();
      if (invocation.status === "pending_approval") {
        setTerminalNotice(copy.terminalTool.pendingHint);
      }
    } catch (error) {
      setTerminalError(String(error) || copy.terminalTool.failed);
    } finally {
      setTerminalPending(false);
    }
  };

  const runTerminalWriteBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!terminalWriteCommand.trim()) {
      setTerminalWriteError(copy.terminalTool.writeFailed);
      return;
    }

    setTerminalWritePending(true);
    setTerminalWriteError("");
    setTerminalWriteNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("run_terminal_write", {
        accessMode: state.access_mode,
        command: terminalWriteCommand,
      });
      await refreshCapabilityState();
      setTerminalWriteNotice(
        invocation.status === "pending_approval"
          ? copy.terminalTool.writePendingHint
          : copy.terminalTool.writeBlocked,
      );
    } catch (error) {
      setTerminalWriteError(String(error) || copy.terminalTool.writeFailed);
    } finally {
      setTerminalWritePending(false);
    }
  };

  const sendEmailBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedTo = emailTo.trim();
    const trimmedSubject = emailSubject.trim();
    const trimmedBody = emailBody.trim();
    if (!trimmedTo || !trimmedSubject || !trimmedBody) {
      setEmailError(copy.emailTool.failed);
      return;
    }

    setEmailPending(true);
    setEmailError("");
    setEmailNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("send_email_boundary", {
        accessMode: state.access_mode,
        to: trimmedTo,
        subject: trimmedSubject,
        body: trimmedBody,
      });
      await refreshCapabilityState();
      setEmailNotice(
        invocation.status === "pending_approval"
          ? copy.emailTool.pendingHint
          : copy.emailTool.blocked,
      );
    } catch (error) {
      setEmailError(String(error) || copy.emailTool.failed);
    } finally {
      setEmailPending(false);
    }
  };

  const readEmailBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedMailbox = emailMailbox.trim();
    const trimmedQuery = emailReadQuery.trim();
    if (!trimmedMailbox || !trimmedQuery) {
      setEmailReadError(copy.emailReadTool.failed);
      return;
    }

    setEmailReadPending(true);
    setEmailReadError("");
    setEmailReadNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("read_email_boundary", {
        accessMode: state.access_mode,
        mailbox: trimmedMailbox,
        query: trimmedQuery,
      });
      await refreshCapabilityState();
      setEmailReadNotice(
        invocation.status === "pending_approval"
          ? copy.emailReadTool.pendingHint
          : copy.emailReadTool.blocked,
      );
    } catch (error) {
      setEmailReadError(String(error) || copy.emailReadTool.failed);
    } finally {
      setEmailReadPending(false);
    }
  };

  const createEmailDraftBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedTo = draftEmailTo.trim();
    const trimmedSubject = draftEmailSubject.trim();
    const trimmedBody = draftEmailBody.trim();
    if (!trimmedTo || !trimmedSubject || !trimmedBody) {
      setEmailDraftError(copy.emailDraftTool.failed);
      return;
    }

    setEmailDraftPending(true);
    setEmailDraftError("");
    setEmailDraftNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("create_email_draft_boundary", {
        accessMode: state.access_mode,
        to: trimmedTo,
        subject: trimmedSubject,
        body: trimmedBody,
      });
      await refreshCapabilityState();
      setEmailDraftNotice(
        invocation.status === "pending_approval"
          ? copy.emailDraftTool.pendingHint
          : copy.emailDraftTool.blocked,
      );
    } catch (error) {
      setEmailDraftError(String(error) || copy.emailDraftTool.failed);
    } finally {
      setEmailDraftPending(false);
    }
  };

  const readDriveBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedLocation = driveLocation.trim();
    const trimmedQuery = driveReadQuery.trim();
    if (!trimmedLocation || !trimmedQuery) {
      setDriveReadError(copy.driveReadTool.failed);
      return;
    }

    setDriveReadPending(true);
    setDriveReadError("");
    setDriveReadNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("read_drive_boundary", {
        accessMode: state.access_mode,
        location: trimmedLocation,
        query: trimmedQuery,
      });
      await refreshCapabilityState();
      setDriveReadNotice(
        invocation.status === "pending_approval"
          ? copy.driveReadTool.pendingHint
          : invocation.status === "succeeded"
            ? copy.driveReadTool.completed
            : copy.driveReadTool.blocked,
      );
    } catch (error) {
      setDriveReadError(String(error) || copy.driveReadTool.failed);
    } finally {
      setDriveReadPending(false);
    }
  };

  const writeDriveBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedLocation = driveWriteLocation.trim();
    const trimmedSummary = driveWriteSummary.trim();
    if (!trimmedLocation || !trimmedSummary) {
      setDriveWriteError(copy.driveWriteTool.failed);
      return;
    }

    setDriveWritePending(true);
    setDriveWriteError("");
    setDriveWriteNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("write_drive_boundary", {
        accessMode: state.access_mode,
        location: trimmedLocation,
        summary: trimmedSummary,
      });
      await refreshCapabilityState();
      setDriveWriteNotice(
        invocation.status === "pending_approval"
          ? copy.driveWriteTool.pendingHint
          : invocation.status === "succeeded"
            ? copy.driveWriteTool.completed
            : copy.driveWriteTool.blocked,
      );
    } catch (error) {
      setDriveWriteError(String(error) || copy.driveWriteTool.failed);
    } finally {
      setDriveWritePending(false);
    }
  };

  const captureComputerScreenshot = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setComputerPending(true);
    setComputerError("");
    setComputerNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("capture_computer_screenshot", {
        accessMode: state.access_mode,
        largeModelProvider: state.large_model_provider,
        networkSearchSourceModel: state.network_search_source_model,
      });
      await refreshCapabilityState();
      if (invocation.status === "pending_approval") {
        setComputerNotice(copy.computerTool.pendingHint);
      } else if (invocation.status === "succeeded") {
        setComputerNotice(copy.computerTool.captured);
      } else {
        setComputerNotice(copy.computerTool.unavailable);
      }
    } catch (error) {
      setComputerError(String(error) || copy.computerTool.failed);
    } finally {
      setComputerPending(false);
    }
  };

  const unlockComputerControl = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedToken = computerControlUnlockToken.trim();
    if (!trimmedToken) {
      setComputerControlUnlockError(copy.computerControlTool.unlockFailed);
      return;
    }

    setComputerControlUnlockPending(true);
    setComputerControlUnlockError("");
    setComputerControlUnlockNotice("");

    try {
      const unlockStatus = await invoke<ComputerControlUnlockStatus>(
        "unlock_computer_control",
        {
          token: trimmedToken,
        },
      );
      setComputerControlUnlockStatus(unlockStatus);
      setComputerControlUnlockToken("");
      setComputerControlUnlockNotice(copy.computerControlTool.unlockReady);
    } catch (error) {
      setComputerControlUnlockError(String(error) || copy.computerControlTool.unlockFailed);
    } finally {
      setComputerControlUnlockPending(false);
    }
  };

  const controlComputerBoundary = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedTarget = computerControlTarget.trim();
    const trimmedAction = computerControlAction.trim();
    if (!trimmedTarget || !trimmedAction) {
      setComputerControlError(copy.computerControlTool.failed);
      return;
    }

    setComputerControlPending(true);
    setComputerControlError("");
    setComputerControlNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("control_computer_boundary", {
        accessMode: state.access_mode,
        largeModelProvider: state.large_model_provider,
        networkSearchSourceModel: state.network_search_source_model,
        target: trimmedTarget,
        action: trimmedAction,
      });
      await refreshCapabilityState();
      setComputerControlNotice(
        invocation.status === "pending_approval"
          ? copy.computerControlTool.pendingHint
          : invocation.status === "succeeded"
            ? copy.computerControlTool.executed
            : copy.computerControlTool.blocked,
      );
    } catch (error) {
      setComputerControlError(String(error) || copy.computerControlTool.failed);
    } finally {
      void refreshComputerControlUnlockStatus().catch(() =>
        setComputerControlUnlockStatus(fallbackComputerControlUnlockStatus),
      );
      setComputerControlPending(false);
    }
  };

  const runOperationsBriefingWorkflow = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const trimmedPath = briefingFolderPath.trim();
    if (!trimmedPath) {
      setBriefingError(copy.operationsBriefing.failed);
      return;
    }

    setBriefingPending(true);
    setBriefingError("");
    setBriefingNotice("");

    try {
      const run = await invoke<OperationsBriefingRun>("run_operations_briefing", {
        accessMode: state.access_mode,
        evidenceFolderPath: trimmedPath,
        largeModelProvider: state.large_model_provider,
        modelRoute: state.model_route,
        thinkingLevel: state.thinking_level,
      });
      const [, , telemetry, cacheState] = await Promise.all([
        refreshCapabilityState(),
        refreshOperationsBriefingRuns(),
        invoke<DeepSeekChatTelemetry[]>("list_deepseek_chat_telemetry"),
        invoke<DeepSeekChatCacheState>("get_deepseek_chat_cache_state"),
      ]);
      setDeepSeekTelemetry(telemetry);
      setDeepSeekChatCacheState(cacheState);
      if (run.status === "pending_approval") {
        setBriefingNotice(copy.operationsBriefing.pendingHint);
      }
    } catch (error) {
      setBriefingError(String(error) || copy.operationsBriefing.failed);
    } finally {
      setBriefingPending(false);
    }
  };

  const seedOperationsBriefingEvidenceTemplates = async () => {
    setBriefingPending(true);
    setBriefingError("");
    setBriefingNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>(
        "seed_operations_briefing_evidence_templates",
        {
          accessMode: state.access_mode,
        },
      );
      await refreshCapabilityState();
      if (invocation.evidence_ref) {
        setBriefingFolderPath(invocation.evidence_ref);
        setFolderPath((current) => current || invocation.evidence_ref || "");
      }
      setBriefingNotice(
        invocation.status === "pending_approval"
          ? copy.operationsBriefing.seedPendingHint
          : invocation.status === "succeeded"
            ? copy.operationsBriefing.seededTemplates
            : copy.operationsBriefing.seedFailed,
      );
    } catch (error) {
      setBriefingError(String(error) || copy.operationsBriefing.seedFailed);
    } finally {
      setBriefingPending(false);
    }
  };

  const resolveCapabilityAccess = async (requestId: string, approved: boolean) => {
    setResolutionPending(requestId);
    setCapabilityError("");

    try {
      await invoke("resolve_capability_access_request", {
        requestId,
        approved,
        note: approved ? copy.capabilities.approve : copy.capabilities.reject,
      });
      await refreshCapabilityState();
    } catch (error) {
      setCapabilityError(String(error) || copy.capabilities.resolveFailed);
    } finally {
      setResolutionPending(null);
    }
  };

  const clearPackageStatus = () => {
    setPackageNotice("");
    setPackageError("");
  };

  const exportOperationsBriefingPackage = async () => {
    clearPackageStatus();
    setBriefingPending(true);
    setPackagePending(true);
    setBriefingError("");
    setBriefingNotice("");

    try {
      const workPackage = await invoke<WorkPackage>("export_work_package");
      setExportedPackageJson(JSON.stringify(workPackage, null, 2));
      setBriefingNotice(copy.operationsBriefing.exported);
      setPackageNotice(copy.package.exported);
    } catch (error) {
      setBriefingError(String(error) || copy.operationsBriefing.failed);
    } finally {
      setBriefingPending(false);
      setPackagePending(false);
    }
  };

  const exportOperationsBriefingReport = async () => {
    if (!latestOperationsBriefingRun) {
      return;
    }

    setBriefingPending(true);
    setBriefingError("");
    setBriefingNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("export_operations_briefing_report", {
        accessMode: state.access_mode,
        runId: latestOperationsBriefingRun.id,
      });
      await refreshCapabilityState();
      setBriefingNotice(
        invocation.status === "pending_approval"
          ? copy.operationsBriefing.reportPendingHint
          : invocation.status === "succeeded"
            ? copy.operationsBriefing.reportExported
            : copy.operationsBriefing.reportExportFailed,
      );
    } catch (error) {
      setBriefingError(String(error) || copy.operationsBriefing.reportExportFailed);
    } finally {
      setBriefingPending(false);
    }
  };

  const exportOperationsBriefingHtmlReport = async () => {
    if (!latestOperationsBriefingRun) {
      return;
    }

    setBriefingPending(true);
    setBriefingError("");
    setBriefingNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("export_operations_briefing_html_report", {
        accessMode: state.access_mode,
        runId: latestOperationsBriefingRun.id,
      });
      await refreshCapabilityState();
      setBriefingNotice(
        invocation.status === "pending_approval"
          ? copy.operationsBriefing.htmlReportPendingHint
          : invocation.status === "succeeded"
            ? copy.operationsBriefing.htmlReportExported
            : copy.operationsBriefing.htmlReportExportFailed,
      );
    } catch (error) {
      setBriefingError(String(error) || copy.operationsBriefing.htmlReportExportFailed);
    } finally {
      setBriefingPending(false);
    }
  };

  const exportOperationsBriefingPdfReport = async () => {
    if (!latestOperationsBriefingRun) {
      return;
    }

    setBriefingPending(true);
    setBriefingError("");
    setBriefingNotice("");

    try {
      const invocation = await invoke<CapabilityInvocation>("export_operations_briefing_pdf_report", {
        accessMode: state.access_mode,
        runId: latestOperationsBriefingRun.id,
      });
      await refreshCapabilityState();
      setBriefingNotice(
        invocation.status === "pending_approval"
          ? copy.operationsBriefing.pdfReportPendingHint
          : invocation.status === "succeeded"
            ? copy.operationsBriefing.pdfReportExported
            : copy.operationsBriefing.pdfReportExportFailed,
      );
    } catch (error) {
      setBriefingError(String(error) || copy.operationsBriefing.pdfReportExportFailed);
    } finally {
      setBriefingPending(false);
    }
  };

  const createTaskRecord = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    clearPackageStatus();

    if (!taskTitle.trim()) {
      setPackageError(copy.package.emptyTitle);
      return;
    }

    setPackagePending(true);
    try {
      const record = await invoke<TaskRecord>("create_task_record", {
        title: taskTitle,
        summary: taskSummary,
      });
      const memories = await loadMemoryRecords(memoryQuery);
      setTaskRecords((currentRecords) => [record, ...currentRecords]);
      setMemoryRecords(memories);
      setTaskTitle("");
      setTaskSummary("");
      setPackageNotice(copy.package.created);
    } catch (error) {
      setPackageError(String(error));
    } finally {
      setPackagePending(false);
    }
  };

  const exportCurrentWorkPackage = async () => {
    clearPackageStatus();
    setPackagePending(true);
    try {
      const workPackage = await invoke<WorkPackage>("export_work_package");
      setExportedPackageJson(JSON.stringify(workPackage, null, 2));
      setPackageNotice(copy.package.exported);
    } catch (error) {
      setPackageError(String(error));
    } finally {
      setPackagePending(false);
    }
  };

  const copyCurrentWorkPackage = async () => {
    clearPackageStatus();
    setPackagePending(true);
    try {
      let packageJson = exportedPackageJson;
      if (!packageJson) {
        const workPackage = await invoke<WorkPackage>("export_work_package");
        packageJson = JSON.stringify(workPackage, null, 2);
        setExportedPackageJson(packageJson);
      }
      await navigator.clipboard.writeText(packageJson);
      setPackageNotice(copy.package.copied);
    } catch {
      setPackageError(copy.package.copyFailed);
    } finally {
      setPackagePending(false);
    }
  };

  const previewWorkPackageImport = async () => {
    clearPackageStatus();

    if (!importPackageJson.trim()) {
      setPackageError(copy.package.emptyImport);
      setImportPreview(null);
      return;
    }

    setPackagePending(true);
    try {
      const preview = await invoke<WorkPackageImportPreview>("preview_work_package_import", {
        packageJson: importPackageJson,
      });
      setImportPreview(preview);
      setPackageNotice(copy.package.previewReady);
    } catch (error) {
      setImportPreview(null);
      setPackageError(String(error) || copy.package.previewFailed);
    } finally {
      setPackagePending(false);
    }
  };

  const importWorkPackageJson = async () => {
    clearPackageStatus();

    if (!importPackageJson.trim()) {
      setPackageError(copy.package.emptyImport);
      return;
    }

    setPackagePending(true);
    try {
      const summary = await invoke<WorkPackageImportSummary>("import_work_package", {
        packageJson: importPackageJson,
      });
      const [records, memories, memoryCandidates, briefingRuns] = await Promise.all([
        invoke<TaskRecord[]>("list_task_records"),
        loadMemoryRecords(memoryQuery),
        invoke<MemoryCandidateRecord[]>("list_memory_candidate_records"),
        invoke<OperationsBriefingRun[]>("list_operations_briefing_runs"),
      ]);
      setTaskRecords(records);
      setMemoryRecords(memories);
      setMemoryCandidateRecords(memoryCandidates);
      setOperationsBriefingRuns(briefingRuns);
      setImportPackageJson("");
      setImportPreview(null);
      setPackageNotice(
        copy.package.imported(
          summary.imported,
          summary.skipped,
          summary.memory_candidates.imported,
          summary.memory_candidates.skipped,
          summary.operations_briefing_runs.imported,
          summary.operations_briefing_runs.skipped,
          summary.workflow_templates.imported,
          summary.workflow_templates.skipped,
        ),
      );
    } catch (error) {
      setPackageError(String(error));
    } finally {
      setPackagePending(false);
    }
  };

  const pendingCapabilityRecords = capabilityRecords.filter(
    (record) => record.effective_status === "pending_approval",
  );
  const latestOperationsBriefingRun = operationsBriefingRuns[0];

  return (
    <main className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">D</div>
          <div>
            <strong>{state.app_name}</strong>
            <span>{copy.brandTagline}</span>
          </div>
        </div>
        <div className="sidebar-preferences">
          <div className="language-switch" role="group" aria-label={copy.controls.language}>
            <Languages size={16} aria-hidden="true" />
            <button
              className={language === "zh" ? "language-option active" : "language-option"}
              type="button"
              aria-pressed={language === "zh"}
              onClick={() => switchLanguage("zh")}
            >
              中
            </button>
            <button
              className={language === "en" ? "language-option active" : "language-option"}
              type="button"
              aria-pressed={language === "en"}
              onClick={() => switchLanguage("en")}
            >
              EN
            </button>
          </div>
        </div>
        <nav className="nav-list" aria-label={copy.navLabel}>
          <button className="nav-item active" type="button">
            <FolderOpen size={18} /> {copy.nav.workbench}
          </button>
          <button className="nav-item" type="button">
            <Database size={18} /> {copy.nav.memory}
          </button>
          <button className="nav-item" type="button">
            <ShieldCheck size={18} /> {copy.nav.approvals}
          </button>
        </nav>
      </aside>

      <section className="workspace">
        <header className="toolbar">
          <select
            value={state.large_model_provider}
            aria-label={copy.controls.largeModelProvider}
            onChange={updateLargeModelProvider}
          >
            <option value="deepseek">{copy.largeModelOptions.deepseek}</option>
            <option value="chatgpt">{copy.largeModelOptions.chatgpt}</option>
            <option value="codex">{copy.largeModelOptions.codex}</option>
            <option value="custom">{copy.largeModelOptions.custom}</option>
          </select>
          <select value={state.model_route} aria-label={copy.controls.modelRoute} onChange={updateModelRoute}>
            <option value="auto">{copy.modelOptions.auto}</option>
            <option value="flash">{copy.modelOptions.flash}</option>
            <option value="pro">{copy.modelOptions.pro}</option>
          </select>
          <select value={state.access_mode} aria-label={copy.controls.accessMode} onChange={updateAccessMode}>
            <option value="ask_every_step">{copy.accessOptions.ask_every_step}</option>
            <option value="ask_on_risk">{copy.accessOptions.ask_on_risk}</option>
            <option value="limited_auto">{copy.accessOptions.limited_auto}</option>
            <option value="full_access">{copy.accessOptions.full_access}</option>
          </select>
          <select value={state.thinking_level} aria-label={copy.controls.thinkingLevel} onChange={updateThinkingLevel}>
            <option value="auto">{copy.thinkingOptions.auto}</option>
            <option value="fast">{copy.thinkingOptions.fast}</option>
            <option value="standard">{copy.thinkingOptions.standard}</option>
            <option value="deep">{copy.thinkingOptions.deep}</option>
          </select>
          <select value={themeStyle} aria-label={copy.controls.themeStyle} onChange={updateThemeStyle}>
            <option value="deep">{copy.themeOptions.deep}</option>
            <option value="ink">{copy.themeOptions.ink}</option>
            <option value="porcelain">{copy.themeOptions.porcelain}</option>
          </select>
        </header>

        <section className="workbench">
          <div className="timeline">
            <p className="eyebrow">{copy.workbench.stage}</p>
            <h1>{copy.workbench.title}</h1>
            <p className="summary">{copy.workbench.summary}</p>

            <section
              className={
                localDirectoryState.needs_setup
                  ? "setup-panel setup-required"
                  : "setup-panel"
              }
              aria-labelledby="local-directory-setup-title"
            >
              <div className="section-heading">
                <FolderOpen size={18} aria-hidden="true" />
                <h2 id="local-directory-setup-title">{copy.localSetup.title}</h2>
              </div>
              <p className="setup-status">
                {localDirectoryState.needs_setup
                  ? copy.localSetup.required
                  : copy.localSetup.ready}
              </p>
              <dl className="setup-meta">
                <div>
                  <dt>{copy.localSetup.appData}</dt>
                  <dd>{localDirectoryState.app_data_dir || copy.backendLabels.notSelected}</dd>
                </div>
                <div>
                  <dt>{copy.localSetup.settingsFile}</dt>
                  <dd>{localDirectoryState.settings_file || copy.backendLabels.notSelected}</dd>
                </div>
              </dl>
              <form className="setup-form" onSubmit={saveLocalDirectorySetup}>
                <label>
                  <span>{copy.localSetup.workspaceDir}</span>
                  <div className="setup-field">
                    <input
                      value={setupWorkspaceDir}
                      aria-label={copy.localSetup.workspaceDir}
                      placeholder={copy.localSetup.workspacePlaceholder}
                      onChange={(event) => setSetupWorkspaceDir(event.target.value)}
                    />
                    <button
                      type="button"
                      onClick={() => void chooseLocalDirectory("workspace")}
                    >
                      <FolderOpen size={14} aria-hidden="true" />
                      {copy.localSetup.choose}
                    </button>
                  </div>
                </label>
                <label>
                  <span>{copy.localSetup.evidenceDir}</span>
                  <div className="setup-field">
                    <input
                      value={setupEvidenceDir}
                      aria-label={copy.localSetup.evidenceDir}
                      placeholder={copy.localSetup.evidencePlaceholder}
                      onChange={(event) => setSetupEvidenceDir(event.target.value)}
                    />
                    <button
                      type="button"
                      onClick={() => void chooseLocalDirectory("evidence")}
                    >
                      <FolderOpen size={14} aria-hidden="true" />
                      {copy.localSetup.choose}
                    </button>
                  </div>
                </label>
                <label>
                  <span>{copy.localSetup.exportDir}</span>
                  <div className="setup-field">
                    <input
                      value={setupExportDir}
                      aria-label={copy.localSetup.exportDir}
                      placeholder={copy.localSetup.exportPlaceholder}
                      onChange={(event) => setSetupExportDir(event.target.value)}
                    />
                    <button
                      type="button"
                      onClick={() => void chooseLocalDirectory("export")}
                    >
                      <FolderOpen size={14} aria-hidden="true" />
                      {copy.localSetup.choose}
                    </button>
                  </div>
                </label>
                <button type="submit" disabled={setupPending}>
                  <MousePointerClick size={14} aria-hidden="true" />
                  {setupPending ? copy.localSetup.saving : copy.localSetup.save}
                </button>
              </form>
              {setupNotice ? <p className="package-message">{setupNotice}</p> : null}
              {setupError ? <p className="package-error">{setupError}</p> : null}
            </section>

            <section className="setup-panel" aria-labelledby="deepseek-pricing-title">
              <div className="section-heading">
                <Database size={18} aria-hidden="true" />
                <h2 id="deepseek-pricing-title">{copy.deepSeekPricing.title}</h2>
              </div>
              <p className="setup-status" title={deepSeekPricingState.note}>
                {deepSeekPricingState.pricing_configured
                  ? copy.deepSeekPricing.statusConfigured
                  : copy.deepSeekPricing.statusNotConfigured}
              </p>
              <dl className="setup-meta">
                <div>
                  <dt>{copy.deepSeekPricing.settingsFile}</dt>
                  <dd>{deepSeekPricingState.settings_file || copy.backendLabels.notSelected}</dd>
                </div>
              </dl>
              <form className="setup-form" onSubmit={saveDeepSeekPricingSetup}>
                <label className="setup-checkbox">
                  <input
                    type="checkbox"
                    checked={deepSeekPricingEnabled}
                    onChange={(event) => setDeepSeekPricingEnabled(event.target.checked)}
                  />
                  <span>{copy.deepSeekPricing.enabled}</span>
                </label>
                <p className="setup-status">{copy.deepSeekPricing.help}</p>
                <label>
                  <span>{copy.deepSeekPricing.flashPrompt}</span>
                  <input
                    type="number"
                    min="0"
                    step="0.000001"
                    value={deepSeekFlashPromptPrice}
                    aria-label={copy.deepSeekPricing.flashPrompt}
                    placeholder={copy.deepSeekPricing.pricePlaceholder}
                    onChange={(event) => setDeepSeekFlashPromptPrice(event.target.value)}
                  />
                </label>
                <label>
                  <span>{copy.deepSeekPricing.flashCompletion}</span>
                  <input
                    type="number"
                    min="0"
                    step="0.000001"
                    value={deepSeekFlashCompletionPrice}
                    aria-label={copy.deepSeekPricing.flashCompletion}
                    placeholder={copy.deepSeekPricing.pricePlaceholder}
                    onChange={(event) => setDeepSeekFlashCompletionPrice(event.target.value)}
                  />
                </label>
                <label>
                  <span>{copy.deepSeekPricing.proPrompt}</span>
                  <input
                    type="number"
                    min="0"
                    step="0.000001"
                    value={deepSeekProPromptPrice}
                    aria-label={copy.deepSeekPricing.proPrompt}
                    placeholder={copy.deepSeekPricing.pricePlaceholder}
                    onChange={(event) => setDeepSeekProPromptPrice(event.target.value)}
                  />
                </label>
                <label>
                  <span>{copy.deepSeekPricing.proCompletion}</span>
                  <input
                    type="number"
                    min="0"
                    step="0.000001"
                    value={deepSeekProCompletionPrice}
                    aria-label={copy.deepSeekPricing.proCompletion}
                    placeholder={copy.deepSeekPricing.pricePlaceholder}
                    onChange={(event) => setDeepSeekProCompletionPrice(event.target.value)}
                  />
                </label>
                <button type="submit" disabled={deepSeekPricingPending}>
                  <MousePointerClick size={14} aria-hidden="true" />
                  {deepSeekPricingPending
                    ? copy.deepSeekPricing.saving
                    : copy.deepSeekPricing.save}
                </button>
              </form>
              {deepSeekPricingNotice ? (
                <p className="package-message">{deepSeekPricingNotice}</p>
              ) : null}
              {deepSeekPricingError ? (
                <p className="package-error">{deepSeekPricingError}</p>
              ) : null}
            </section>

            <section className="workflow-panel" aria-labelledby="operations-briefing-title">
              <div className="section-heading">
                <ClipboardList size={18} aria-hidden="true" />
                <h2 id="operations-briefing-title">{copy.operationsBriefing.title}</h2>
              </div>

              <form className="workflow-form" onSubmit={runOperationsBriefingWorkflow}>
                <input
                  value={briefingFolderPath}
                  aria-label={copy.operationsBriefing.folderPlaceholder}
                  placeholder={copy.operationsBriefing.folderPlaceholder}
                  onChange={(event) => setBriefingFolderPath(event.target.value)}
                />
                <button
                  type="button"
                  disabled={briefingPending || capabilityPending !== null}
                  onClick={() => void seedOperationsBriefingEvidenceTemplates()}
                >
                  <FileText size={14} aria-hidden="true" />
                  {copy.operationsBriefing.seedTemplates}
                </button>
                <button type="submit" disabled={briefingPending || capabilityPending !== null}>
                  <MousePointerClick size={14} aria-hidden="true" />
                  {briefingPending ? copy.operationsBriefing.running : copy.operationsBriefing.run}
                </button>
              </form>

              {briefingNotice ? <p className="package-message">{briefingNotice}</p> : null}
              {briefingError ? <p className="package-error">{briefingError}</p> : null}

              <div className="workflow-run-list" aria-live="polite">
                {!latestOperationsBriefingRun ? (
                  <p className="empty-state">{copy.operationsBriefing.noRuns}</p>
                ) : (
                  <article className="workflow-run">
                    <header className="workflow-run-header">
                      <div>
                        <span>{copy.operationsBriefing.latestRun}</span>
                        <strong>{latestOperationsBriefingRun.title}</strong>
                      </div>
                      <span className={`access-status ${latestOperationsBriefingRun.status}`}>
                        {copy.operationsBriefing.status[latestOperationsBriefingRun.status]}
                      </span>
                    </header>
                    <p>{latestOperationsBriefingRun.summary}</p>
                    {latestOperationsBriefingRun.warnings.length > 0 ? (
                      <p>{latestOperationsBriefingRun.warnings.join(" ")}</p>
                    ) : null}
                    <footer>
                      <span>{formatTaskDate(latestOperationsBriefingRun.created_at, language)}</span>
                      {latestOperationsBriefingRun.archived_from_package ? (
                        <span className="archive-label">{copy.operationsBriefing.archived}</span>
                      ) : null}
                      {latestOperationsBriefingRun.evidence_folder_path ? (
                        <span>
                          {copy.operationsBriefing.evidence}:{" "}
                          {latestOperationsBriefingRun.evidence_folder_path}
                        </span>
                      ) : null}
                    </footer>
                    <div className="workflow-run-sections">
                      <section>
                        <strong>{copy.operationsBriefing.anomalies}</strong>
                        {latestOperationsBriefingRun.anomalies.length === 0 ? (
                          <p className="empty-state">{copy.operationsBriefing.noAnomalies}</p>
                        ) : (
                          <ul>
                            {latestOperationsBriefingRun.anomalies.map((anomaly) => (
                              <li key={`${latestOperationsBriefingRun.id}-${anomaly.area}`}>
                                <span>{anomaly.area}</span>
                                {anomaly.signal}
                              </li>
                            ))}
                          </ul>
                        )}
                      </section>
                      <section>
                        <strong>{copy.operationsBriefing.actions}</strong>
                        {latestOperationsBriefingRun.action_plan.length === 0 ? (
                          <p className="empty-state">{copy.operationsBriefing.noActions}</p>
                        ) : (
                          <ul>
                            {latestOperationsBriefingRun.action_plan.map((action) => (
                              <li key={`${latestOperationsBriefingRun.id}-${action.owner}`}>
                                <span>{action.owner}</span>
                                {action.action}
                              </li>
                            ))}
                          </ul>
                        )}
                      </section>
                    </div>
                    <div className="workflow-run-actions">
                      <button
                        type="button"
                        onClick={exportOperationsBriefingPackage}
                        disabled={briefingPending || packagePending}
                      >
                        <PackageOpen size={14} aria-hidden="true" />
                        {copy.operationsBriefing.exportPackage}
                      </button>
                      <button
                        type="button"
                        onClick={() => void exportOperationsBriefingReport()}
                        disabled={briefingPending}
                      >
                        <FileText size={14} aria-hidden="true" />
                        {copy.operationsBriefing.exportReport}
                      </button>
                      <button
                        type="button"
                        onClick={() => void exportOperationsBriefingHtmlReport()}
                        disabled={briefingPending}
                      >
                        <FileText size={14} aria-hidden="true" />
                        {copy.operationsBriefing.exportHtmlReport}
                      </button>
                      <button
                        type="button"
                        onClick={() => void exportOperationsBriefingPdfReport()}
                        disabled={briefingPending}
                      >
                        <FileText size={14} aria-hidden="true" />
                        {copy.operationsBriefing.exportPdfReport}
                      </button>
                    </div>
                  </article>
                )}
              </div>
            </section>

            <section className="package-panel" aria-labelledby="work-package-title">
              <div className="section-heading">
                <PackageOpen size={18} aria-hidden="true" />
                <h2 id="work-package-title">{copy.package.title}</h2>
              </div>

              <form className="task-form" onSubmit={createTaskRecord}>
                <input
                  value={taskTitle}
                  aria-label={copy.package.taskTitle}
                  placeholder={copy.package.taskTitle}
                  onChange={(event) => setTaskTitle(event.target.value)}
                />
                <textarea
                  value={taskSummary}
                  aria-label={copy.package.taskSummary}
                  placeholder={copy.package.taskSummary}
                  rows={3}
                  onChange={(event) => setTaskSummary(event.target.value)}
                />
                <button className="primary-action" type="submit" disabled={packagePending}>
                  <Plus size={16} aria-hidden="true" />
                  {copy.package.addRecord}
                </button>
              </form>

              <section className="memory-panel inline" aria-labelledby="memory-panel-title">
                <div className="inspector-header compact">
                  <Database size={18} aria-hidden="true" />
                  <strong id="memory-panel-title">{copy.memory.title}</strong>
                </div>
                <form className="memory-search" onSubmit={searchMemoryRecords}>
                  <input
                    value={memoryQuery}
                    aria-label={copy.memory.searchPlaceholder}
                    placeholder={copy.memory.searchPlaceholder}
                    onChange={(event) => setMemoryQuery(event.target.value)}
                  />
                  <button type="submit" disabled={memoryPending}>
                    <Search size={15} aria-hidden="true" />
                    {copy.memory.search}
                  </button>
                </form>
                {memoryNotice ? <p className="package-message">{memoryNotice}</p> : null}
                {memoryError ? <p className="package-error">{memoryError}</p> : null}
                {memoryRecords.length === 0 ? (
                  <p className="empty-state">{copy.memory.noMemories}</p>
                ) : (
                  <div className="memory-list">
                    {memoryRecords.slice(0, 3).map((memory) => {
                      const isEditing = memoryEditDraft?.id === memory.id;

                      return (
                        <article className="memory-row" key={memory.id}>
                          {isEditing && memoryEditDraft ? (
                            <form className="memory-edit-form" onSubmit={updateMemoryRecord}>
                              <input
                                value={memoryEditDraft.title}
                                aria-label={copy.memory.editTitle}
                                placeholder={copy.memory.editTitle}
                                onChange={(event) =>
                                  patchMemoryEditDraft({ title: event.target.value })
                                }
                              />
                              <textarea
                                value={memoryEditDraft.body}
                                aria-label={copy.memory.editBody}
                                placeholder={copy.memory.editBody}
                                onChange={(event) =>
                                  patchMemoryEditDraft({ body: event.target.value })
                                }
                              />
                              <div className="memory-candidate-metadata">
                                <label>
                                  <span>{copy.memory.candidateType}</span>
                                  <select
                                    value={memoryEditDraft.memory_type}
                                    onChange={(event) =>
                                      patchMemoryEditDraft({
                                        memory_type: event.target.value as MemoryType,
                                      })
                                    }
                                  >
                                    {memoryTypeValues.map((value) => (
                                      <option value={value} key={value}>
                                        {copy.memory.typeOptions[value]}
                                      </option>
                                    ))}
                                  </select>
                                </label>
                                <label>
                                  <span>{copy.memory.candidateScope}</span>
                                  <select
                                    value={memoryEditDraft.scope}
                                    onChange={(event) =>
                                      patchMemoryEditDraft({
                                        scope: event.target.value as MemoryScope,
                                      })
                                    }
                                  >
                                    {memoryScopeValues.map((value) => (
                                      <option value={value} key={value}>
                                        {copy.memory.scopeOptions[value]}
                                      </option>
                                    ))}
                                  </select>
                                </label>
                                <label>
                                  <span>{copy.memory.candidateSensitivity}</span>
                                  <select
                                    value={memoryEditDraft.sensitivity}
                                    onChange={(event) =>
                                      patchMemoryEditDraft({
                                        sensitivity: event.target.value as MemorySensitivity,
                                      })
                                    }
                                  >
                                    {memorySensitivityValues.map((value) => (
                                      <option value={value} key={value}>
                                        {copy.memory.sensitivityOptions[value]}
                                      </option>
                                    ))}
                                  </select>
                                </label>
                                <label>
                                  <span>{copy.memory.candidateLifecycle}</span>
                                  <select
                                    value={memoryEditDraft.lifecycle}
                                    onChange={(event) =>
                                      patchMemoryEditDraft({
                                        lifecycle: event.target.value as MemoryLifecycle,
                                      })
                                    }
                                  >
                                    {memoryLifecycleValues.map((value) => (
                                      <option value={value} key={value}>
                                        {copy.memory.lifecycleOptions[value]}
                                      </option>
                                    ))}
                                  </select>
                                </label>
                                {memoryEditDraft.lifecycle === "expires" ? (
                                  <label>
                                    <span>{copy.memory.expiresAt}</span>
                                    <input
                                      type="date"
                                      value={memoryEditDraft.expires_at}
                                      onChange={(event) =>
                                        patchMemoryEditDraft({
                                          expires_at: event.target.value,
                                        })
                                      }
                                    />
                                  </label>
                                ) : null}
                              </div>
                              <div className="candidate-actions">
                                <button type="submit" disabled={memoryUpdatePending !== null}>
                                  <Check size={14} aria-hidden="true" />
                                  {memoryUpdatePending === memory.id
                                    ? copy.memory.saving
                                    : copy.memory.save}
                                </button>
                                <button
                                  type="button"
                                  onClick={cancelMemoryRecordEdit}
                                  disabled={memoryUpdatePending !== null}
                                >
                                  <X size={14} aria-hidden="true" />
                                  {copy.memory.cancel}
                                </button>
                              </div>
                            </form>
                          ) : (
                            <>
                              <strong>{memory.title}</strong>
                              <p>{memory.body}</p>
                              <div className="memory-meta" aria-label={copy.memory.metadata}>
                                <span>{copy.memory.typeOptions[memory.memory_type]}</span>
                                <span>{copy.memory.scopeOptions[memory.scope]}</span>
                                <span>{copy.memory.sensitivityOptions[memory.sensitivity]}</span>
                                <span>{copy.memory.lifecycleOptions[memory.lifecycle]}</span>
                                {memory.expires_at ? (
                                  <span>
                                    {copy.memory.expiresAt}:{" "}
                                    {formatTaskDate(memory.expires_at, language)}
                                  </span>
                                ) : null}
                              </div>
                              {memory.linked_memories.length > 0 ? (
                                <div className="memory-linked-list">
                                  <span>{copy.memory.linkedMemories(memory.linked_memories.length)}</span>
                                  {memory.linked_memories.map((linkedMemory) => (
                                    <span className="memory-link-pill" key={linkedMemory.id}>
                                      <Link2 size={12} aria-hidden="true" />
                                      {linkedMemory.title}
                                    </span>
                                  ))}
                                </div>
                              ) : null}
                              <div className="candidate-actions">
                                <button
                                  type="button"
                                  onClick={() => beginMemoryRecordEdit(memory)}
                                  disabled={
                                    memoryUpdatePending !== null || memoryDeletionPending !== null
                                  }
                                >
                                  <Pencil size={14} aria-hidden="true" />
                                  {copy.memory.edit}
                                </button>
                                <button
                                  type="button"
                                  onClick={() => void deleteMemoryRecord(memory.id)}
                                  disabled={
                                    memoryDeletionPending !== null || memoryUpdatePending !== null
                                  }
                                >
                                  <X size={14} aria-hidden="true" />
                                  {memoryDeletionPending === memory.id
                                    ? copy.memory.deleting
                                    : copy.memory.delete}
                                </button>
                              </div>
                            </>
                          )}
                          <span>
                            {copy.memory.autoCapture} · {formatTaskDate(memory.created_at, language)}
                          </span>
                        </article>
                      );
                    })}
                  </div>
                )}
                <section className="memory-candidate-panel" aria-labelledby="memory-candidates-title">
                  <div className="queue-heading">
                    <strong id="memory-candidates-title">{copy.memory.candidates}</strong>
                    <span>{memoryCandidateRecords.length}</span>
                  </div>
                  <form className="memory-candidate-form" onSubmit={proposeMemoryCandidate}>
                    <input
                      value={candidateTitle}
                      aria-label={copy.memory.candidateTitle}
                      placeholder={copy.memory.candidateTitle}
                      onChange={(event) => setCandidateTitle(event.target.value)}
                    />
                    <textarea
                      value={candidateBody}
                      aria-label={copy.memory.candidateBody}
                      placeholder={copy.memory.candidateBody}
                      rows={3}
                      onChange={(event) => setCandidateBody(event.target.value)}
                    />
                    <div className="memory-candidate-metadata">
                      <label>
                        <span>{copy.memory.candidateType}</span>
                        <select
                          value={candidateMemoryType}
                          onChange={(event) =>
                            setCandidateMemoryType(event.target.value as MemoryType)
                          }
                        >
                          {memoryTypeValues.map((value) => (
                            <option key={value} value={value}>
                              {copy.memory.typeOptions[value]}
                            </option>
                          ))}
                        </select>
                      </label>
                      <label>
                        <span>{copy.memory.candidateScope}</span>
                        <select
                          value={candidateMemoryScope}
                          onChange={(event) =>
                            setCandidateMemoryScope(event.target.value as MemoryScope)
                          }
                        >
                          {memoryScopeValues.map((value) => (
                            <option key={value} value={value}>
                              {copy.memory.scopeOptions[value]}
                            </option>
                          ))}
                        </select>
                      </label>
                      <label>
                        <span>{copy.memory.candidateSensitivity}</span>
                        <select
                          value={candidateSensitivity}
                          onChange={(event) =>
                            setCandidateSensitivity(event.target.value as MemorySensitivity)
                          }
                        >
                          {memorySensitivityValues.map((value) => (
                            <option key={value} value={value}>
                              {copy.memory.sensitivityOptions[value]}
                            </option>
                          ))}
                        </select>
                      </label>
                      <label>
                        <span>{copy.memory.candidateLifecycle}</span>
                        <select
                          value={candidateLifecycle}
                          onChange={(event) =>
                            setCandidateLifecycle(event.target.value as MemoryLifecycle)
                          }
                        >
                          {memoryLifecycleValues.map((value) => (
                            <option key={value} value={value}>
                              {copy.memory.lifecycleOptions[value]}
                            </option>
                          ))}
                        </select>
                      </label>
                      {candidateLifecycle === "expires" ? (
                        <label>
                          <span>{copy.memory.expiresAt}</span>
                          <input
                            type="date"
                            value={candidateExpiresAt}
                            onChange={(event) => setCandidateExpiresAt(event.target.value)}
                          />
                        </label>
                      ) : null}
                    </div>
                    <button type="submit" disabled={memoryCandidatePending}>
                      <Plus size={15} aria-hidden="true" />
                      {memoryCandidatePending ? copy.memory.proposing : copy.memory.propose}
                    </button>
                  </form>
                  {memoryCandidateNotice ? (
                    <p className="package-message">{memoryCandidateNotice}</p>
                  ) : null}
                  {memoryCandidateError ? (
                    <p className="package-error">{memoryCandidateError}</p>
                  ) : null}
                  {memoryCandidateRecords.length === 0 ? (
                    <p className="empty-state">{copy.memory.noCandidates}</p>
                  ) : (
                    <div className="memory-list">
                      {memoryCandidateRecords.slice(0, 3).map((record) => (
                        <article className="memory-row candidate-row" key={record.candidate.id}>
                          <div className="candidate-row-header">
                            <strong>{record.candidate.title}</strong>
                            <span className={`access-status ${record.effective_status}`}>
                              {copy.memory.candidateStatus[record.effective_status]}
                            </span>
                          </div>
                          <p>{record.candidate.body}</p>
                          <div className="memory-meta" aria-label={copy.memory.metadata}>
                            <span>{copy.memory.typeOptions[record.candidate.memory_type]}</span>
                            <span>{copy.memory.scopeOptions[record.candidate.scope]}</span>
                            <span>
                              {copy.memory.sensitivityOptions[record.candidate.sensitivity]}
                            </span>
                            <span>{copy.memory.lifecycleOptions[record.candidate.lifecycle]}</span>
                            {record.candidate.expires_at ? (
                              <span>
                                {copy.memory.expiresAt}:{" "}
                                {formatTaskDate(record.candidate.expires_at, language)}
                              </span>
                            ) : null}
                          </div>
                          {record.conflicting_memory_ids.length > 0 ? (
                            <p className="memory-conflict">
                              {copy.memory.conflictWarning(
                                record.conflicting_memory_ids.length,
                              )}
                            </p>
                          ) : null}
                          {record.conflicting_memories.length > 0 ? (
                            <div className="memory-conflict-details">
                              <strong>{copy.memory.conflictDetails}</strong>
                              {record.conflicting_memories.map((memory) => (
                                <article className="memory-conflict-item" key={memory.id}>
                                  <div className="candidate-row-header">
                                    <span>{memory.title}</span>
                                    <span>
                                      {copy.memory.updatedAt}:{" "}
                                      {formatTaskDate(memory.updated_at, language)}
                                    </span>
                                  </div>
                                  <p>{memory.body}</p>
                                  <div className="memory-meta">
                                    <span>{copy.memory.typeOptions[memory.memory_type]}</span>
                                    <span>{copy.memory.scopeOptions[memory.scope]}</span>
                                    <span>
                                      {copy.memory.sensitivityOptions[memory.sensitivity]}
                                    </span>
                                    <span>{copy.memory.lifecycleOptions[memory.lifecycle]}</span>
                                    {memory.expires_at ? (
                                      <span>
                                        {copy.memory.expiresAt}:{" "}
                                        {formatTaskDate(memory.expires_at, language)}
                                      </span>
                                    ) : null}
                                  </div>
                                </article>
                              ))}
                            </div>
                          ) : null}
                          {memoryMergePreview?.candidate_id === record.candidate.id ? (
                            <div className="memory-merge-preview">
                              <strong>{copy.memory.mergePreviewTitle}</strong>
                              <div className="memory-meta">
                                <span>{copy.memory.typeOptions[memoryMergePreview.memory_type]}</span>
                                <span>{copy.memory.scopeOptions[memoryMergePreview.scope]}</span>
                                <span>
                                  {copy.memory.sensitivityOptions[memoryMergePreview.sensitivity]}
                                </span>
                                <span>
                                  {copy.memory.lifecycleOptions[memoryMergePreview.lifecycle]}
                                </span>
                              </div>
                              <span>{copy.memory.mergePreviewDraft}</span>
                              <p>{memoryMergePreview.body}</p>
                              <div className="candidate-actions">
                                <button
                                  type="button"
                                  onClick={() =>
                                    void mergeMemoryCandidateWithConflicts(
                                      record.candidate.id,
                                      memoryMergePreview.source_memory_ids,
                                    )
                                  }
                                  disabled={memoryCandidateResolutionPending !== null}
                                >
                                  <Check size={14} aria-hidden="true" />
                                  {memoryCandidateResolutionPending === record.candidate.id
                                    ? copy.memory.resolving
                                    : copy.memory.mergeAndAccept}
                                </button>
                              </div>
                            </div>
                          ) : null}
                          {memoryReplacePreview?.candidate_id === record.candidate.id ? (
                            <div className="memory-replace-preview">
                              <strong>{copy.memory.replacePreviewTitle}</strong>
                              <div className="memory-meta">
                                <span>
                                  {copy.memory.typeOptions[memoryReplacePreview.memory_type]}
                                </span>
                                <span>{copy.memory.scopeOptions[memoryReplacePreview.scope]}</span>
                                <span>
                                  {copy.memory.sensitivityOptions[memoryReplacePreview.sensitivity]}
                                </span>
                                <span>
                                  {copy.memory.lifecycleOptions[memoryReplacePreview.lifecycle]}
                                </span>
                              </div>
                              <span>{copy.memory.replacePreviewDraft}</span>
                              <p>{memoryReplacePreview.replacement_body}</p>
                              <span>{copy.memory.replacePreviewTargets}</span>
                              <div className="memory-linked-list">
                                {memoryReplacePreview.target_memories.map((memory) => (
                                  <span className="memory-link-pill" key={memory.id}>
                                    <X size={12} aria-hidden="true" />
                                    {memory.title}
                                  </span>
                                ))}
                              </div>
                              <div className="candidate-actions">
                                <button
                                  type="button"
                                  onClick={() =>
                                    void replaceMemoryCandidateConflicts(
                                      record.candidate.id,
                                      memoryReplacePreview.target_memory_ids,
                                    )
                                  }
                                  disabled={memoryCandidateResolutionPending !== null}
                                >
                                  <Check size={14} aria-hidden="true" />
                                  {memoryCandidateResolutionPending === record.candidate.id
                                    ? copy.memory.resolving
                                    : copy.memory.replaceAndAccept}
                                </button>
                              </div>
                            </div>
                          ) : null}
                          {record.effective_status === "pending" ? (
                            <div className="candidate-actions">
                              {record.conflicting_memory_ids.length > 0 ? (
                                <button
                                  type="button"
                                  onClick={() =>
                                    void previewMemoryCandidateMerge(
                                      record.candidate.id,
                                      record.conflicting_memory_ids,
                                    )
                                  }
                                  disabled={
                                    memoryCandidateResolutionPending !== null ||
                                    memoryMergePreviewPending !== null ||
                                    memoryReplacePreviewPending !== null
                                  }
                                >
                                  <FileText size={14} aria-hidden="true" />
                                  {memoryMergePreviewPending === record.candidate.id
                                    ? copy.memory.previewingMerge
                                    : copy.memory.previewMerge}
                                </button>
                              ) : null}
                              {record.conflicting_memory_ids.length > 0 ? (
                                <button
                                  type="button"
                                  onClick={() =>
                                    void previewMemoryCandidateReplace(
                                      record.candidate.id,
                                      record.conflicting_memory_ids,
                                    )
                                  }
                                  disabled={
                                    memoryCandidateResolutionPending !== null ||
                                    memoryMergePreviewPending !== null ||
                                    memoryReplacePreviewPending !== null
                                  }
                                >
                                  <X size={14} aria-hidden="true" />
                                  {memoryReplacePreviewPending === record.candidate.id
                                    ? copy.memory.previewingReplace
                                    : copy.memory.previewReplace}
                                </button>
                              ) : null}
                              {record.conflicting_memory_ids.length > 0 ? (
                                <button
                                  type="button"
                                  onClick={() =>
                                    void linkMemoryCandidateToConflicts(
                                      record.candidate.id,
                                      record.conflicting_memory_ids,
                                    )
                                  }
                                  disabled={memoryCandidateResolutionPending !== null}
                                >
                                  <Link2 size={14} aria-hidden="true" />
                                  {memoryCandidateResolutionPending === record.candidate.id
                                    ? copy.memory.resolving
                                    : copy.memory.linkAndAccept}
                                </button>
                              ) : null}
                              <button
                                type="button"
                                onClick={() => void resolveMemoryCandidate(record.candidate.id, true)}
                                disabled={memoryCandidateResolutionPending !== null}
                              >
                                <Check size={14} aria-hidden="true" />
                                {memoryCandidateResolutionPending === record.candidate.id
                                  ? copy.memory.resolving
                                  : copy.memory.accept}
                              </button>
                              <button
                                type="button"
                                onClick={() => void resolveMemoryCandidate(record.candidate.id, false)}
                                disabled={memoryCandidateResolutionPending !== null}
                              >
                                <X size={14} aria-hidden="true" />
                                {copy.memory.reject}
                              </button>
                            </div>
                          ) : null}
                        </article>
                      ))}
                    </div>
                  )}
                </section>
              </section>

              <div className="task-list" aria-live="polite">
                {taskRecords.length === 0 ? (
                  <p className="empty-state">{copy.package.noRecords}</p>
                ) : (
                  taskRecords.map((record) => (
                    <article className="task-row" key={record.id}>
                      <div>
                        <strong>{record.title}</strong>
                        {record.summary ? <p>{record.summary}</p> : null}
                      </div>
                      <time dateTime={record.created_at}>{formatTaskDate(record.created_at, language)}</time>
                    </article>
                  ))
                )}
              </div>

              <div className="package-actions">
                <button type="button" onClick={exportCurrentWorkPackage} disabled={packagePending}>
                  <PackageOpen size={16} aria-hidden="true" />
                  {copy.package.exportPackage}
                </button>
                <button type="button" onClick={copyCurrentWorkPackage} disabled={packagePending}>
                  <Clipboard size={16} aria-hidden="true" />
                  {copy.package.copyPackage}
                </button>
              </div>

              <textarea
                className="package-json"
                value={exportedPackageJson}
                aria-label={copy.package.packageJson}
                placeholder={copy.package.packageJson}
                rows={5}
                readOnly
              />

              <div className="import-row">
                <textarea
                  value={importPackageJson}
                  aria-label={copy.package.importJson}
                  placeholder={copy.package.importJson}
                  rows={4}
                  onChange={(event) => {
                    setImportPackageJson(event.target.value);
                    setImportPreview(null);
                  }}
                />
                <button type="button" onClick={previewWorkPackageImport} disabled={packagePending}>
                  <Search size={16} aria-hidden="true" />
                  {packagePending ? copy.package.previewing : copy.package.previewImport}
                </button>
                <button type="button" onClick={importWorkPackageJson} disabled={packagePending}>
                  <ArchiveRestore size={16} aria-hidden="true" />
                  {copy.package.importPackage}
                </button>
              </div>

              {importPreview ? (
                <section className="import-preview" aria-labelledby="import-preview-title">
                  <strong id="import-preview-title">{copy.package.previewTitle}</strong>
                  <div>
                    <span>
                      {copy.package.previewTotalTasks}: {importPreview.task_records.total}
                    </span>
                    <span>
                      {copy.package.previewNewTasks}: {importPreview.task_records.new}
                    </span>
                    <span>
                      {copy.package.previewSkippedTasks}: {importPreview.task_records.skipped}
                    </span>
                    <span>
                      {copy.package.previewMemoryCandidates}:{" "}
                      {importPreview.memory_candidates.total}
                    </span>
                    <span>
                      {copy.package.previewArchivedRuns}:{" "}
                      {importPreview.operations_briefing_runs.total}
                    </span>
                    <span>
                      {copy.package.previewWorkflowTemplates}:{" "}
                      {importPreview.workflow_templates.total}
                    </span>
                    <span>
                      {copy.package.previewNewWorkflowTemplates}:{" "}
                      {importPreview.workflow_templates.new}
                    </span>
                    <span>
                      {copy.package.previewSkippedWorkflowTemplates}:{" "}
                      {importPreview.workflow_templates.skipped}
                    </span>
                  </div>
                  <p>{copy.package.previewMemoryCandidateHint}</p>
                  <p>{copy.package.previewArchiveHint}</p>
                  <p>{copy.package.previewWorkflowTemplateHint}</p>
                </section>
              ) : null}

              {packageNotice ? <p className="package-message">{packageNotice}</p> : null}
              {packageError ? <p className="package-error">{packageError}</p> : null}
            </section>
          </div>
          <aside className="inspector">
            <div className="inspector-header">
              <Brain size={18} />
              <strong>{copy.inspector.title}</strong>
            </div>
            <section className="audit-panel" aria-labelledby="audit-panel-title">
              <div className="inspector-header compact">
                <ShieldCheck size={18} aria-hidden="true" />
                <strong id="audit-panel-title">{copy.capabilities.title}</strong>
              </div>
              {capabilityError ? <p className="package-error">{capabilityError}</p> : null}
              <form className="browser-tool" onSubmit={browseBrowserUrl}>
                <div className="tool-heading">
                  <Globe2 size={16} aria-hidden="true" />
                  <strong>{copy.browserTool.title}</strong>
                </div>
                <div className="browser-tool-row">
                  <input
                    value={browserUrl}
                    aria-label={copy.browserTool.urlPlaceholder}
                    placeholder={copy.browserTool.urlPlaceholder}
                    onChange={(event) => setBrowserUrl(event.target.value)}
                  />
                  <button type="submit" disabled={browserPending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {browserPending ? copy.browserTool.browsing : copy.browserTool.browse}
                  </button>
                </div>
                {browserNotice ? <p className="package-message">{browserNotice}</p> : null}
                {browserError ? <p className="package-error">{browserError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={submitBrowserBoundary}>
                <div className="tool-heading">
                  <MousePointerClick size={16} aria-hidden="true" />
                  <strong>{copy.browserSubmitTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={browserSubmitUrl}
                    aria-label={copy.browserSubmitTool.urlPlaceholder}
                    placeholder={copy.browserSubmitTool.urlPlaceholder}
                    onChange={(event) => setBrowserSubmitUrl(event.target.value)}
                  />
                  <input
                    value={browserSubmitSummary}
                    aria-label={copy.browserSubmitTool.summaryPlaceholder}
                    placeholder={copy.browserSubmitTool.summaryPlaceholder}
                    onChange={(event) => setBrowserSubmitSummary(event.target.value)}
                  />
                  <button
                    type="submit"
                    disabled={browserSubmitPending || capabilityPending !== null}
                  >
                    <MousePointerClick size={14} aria-hidden="true" />
                    {browserSubmitPending
                      ? copy.browserSubmitTool.requestingSubmit
                      : copy.browserSubmitTool.requestSubmit}
                  </button>
                </div>
                {browserSubmitNotice ? (
                  <p className="package-message">{browserSubmitNotice}</p>
                ) : null}
                {browserSubmitError ? (
                  <p className="package-error">{browserSubmitError}</p>
                ) : null}
              </form>
              <form className="browser-tool" onSubmit={searchNetworkBoundary}>
                <div className="tool-heading">
                  <Globe2 size={16} aria-hidden="true" />
                  <strong>{copy.networkSearchTool.title}</strong>
                </div>
                {modelToolStrategy.network_search_source_model_required ? (
                  <div
                    className="source-model-prompt"
                    role={networkSearchSourceModelMissing ? "alert" : undefined}
                  >
                    <strong>{copy.networkSearchTool.sourceModelRequiredTitle}</strong>
                    <p>{copy.networkSearchTool.sourceModelRequiredBody}</p>
                    <select
                      value={state.network_search_source_model ?? ""}
                      aria-label={copy.controls.networkSearchSourceModel}
                      onChange={updateNetworkSearchSourceModel}
                    >
                      <option value="">
                        {copy.networkSearchTool.sourceModelPlaceholder}
                      </option>
                      {modelToolStrategy.free_network_search_source_model_options.map(
                        (option) => (
                          <option
                            key={option.value}
                            value={option.value}
                            title={option.note}
                          >
                            {copy.networkSearchSourceOptions[option.value]}
                          </option>
                        ),
                      )}
                    </select>
                  </div>
                ) : null}
                <div className="email-tool-fields">
                  <input
                    value={networkSearchQuery}
                    aria-label={copy.networkSearchTool.queryPlaceholder}
                    placeholder={copy.networkSearchTool.queryPlaceholder}
                    onChange={(event) => setNetworkSearchQuery(event.target.value)}
                  />
                  <input
                    value={networkSearchScope}
                    aria-label={copy.networkSearchTool.scopePlaceholder}
                    placeholder={copy.networkSearchTool.scopePlaceholder}
                    onChange={(event) => setNetworkSearchScope(event.target.value)}
                  />
                  <button
                    type="submit"
                    disabled={
                      networkSearchPending ||
                      capabilityPending !== null ||
                      networkSearchSourceModelMissing ||
                      !networkSearchRouteStatus.network_requests_enabled
                    }
                  >
                    <MousePointerClick size={14} aria-hidden="true" />
                    {networkSearchPending
                      ? copy.networkSearchTool.requestingSearch
                      : copy.networkSearchTool.requestSearch}
                  </button>
                </div>
                {networkSearchNotice ? (
                  <p className="package-message">{networkSearchNotice}</p>
                ) : null}
                {networkSearchError ? (
                  <p className="package-error">{networkSearchError}</p>
                ) : null}
              </form>
              <form className="browser-tool" onSubmit={readLocalFilePath}>
                <div className="tool-heading">
                  <FileText size={16} aria-hidden="true" />
                  <strong>{copy.fileTool.title}</strong>
                </div>
                <div className="browser-tool-row">
                  <input
                    value={filePath}
                    aria-label={copy.fileTool.pathPlaceholder}
                    placeholder={copy.fileTool.pathPlaceholder}
                    onChange={(event) => setFilePath(event.target.value)}
                  />
                  <button type="submit" disabled={filePending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {filePending ? copy.fileTool.reading : copy.fileTool.read}
                  </button>
                </div>
                {fileNotice ? <p className="package-message">{fileNotice}</p> : null}
                {fileError ? <p className="package-error">{fileError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={writeFileBoundary}>
                <div className="tool-heading">
                  <FileText size={16} aria-hidden="true" />
                  <strong>{copy.fileWriteTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={fileWritePath}
                    aria-label={copy.fileWriteTool.pathPlaceholder}
                    placeholder={copy.fileWriteTool.pathPlaceholder}
                    onChange={(event) => setFileWritePath(event.target.value)}
                  />
                  <input
                    value={fileWriteSummary}
                    aria-label={copy.fileWriteTool.summaryPlaceholder}
                    placeholder={copy.fileWriteTool.summaryPlaceholder}
                    onChange={(event) => setFileWriteSummary(event.target.value)}
                  />
                  <textarea
                    value={fileWriteContent}
                    aria-label={copy.fileWriteTool.contentPlaceholder}
                    placeholder={copy.fileWriteTool.contentPlaceholder}
                    onChange={(event) => setFileWriteContent(event.target.value)}
                  />
                  <button type="submit" disabled={fileWritePending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {fileWritePending
                      ? copy.fileWriteTool.requestingWrite
                      : copy.fileWriteTool.requestWrite}
                  </button>
                </div>
                {fileWriteNotice ? <p className="package-message">{fileWriteNotice}</p> : null}
                {fileWriteError ? <p className="package-error">{fileWriteError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={ingestEvidenceFolderPath}>
                <div className="tool-heading">
                  <FolderOpen size={16} aria-hidden="true" />
                  <strong>{copy.folderTool.title}</strong>
                </div>
                <div className="browser-tool-row">
                  <input
                    value={folderPath}
                    aria-label={copy.folderTool.pathPlaceholder}
                    placeholder={copy.folderTool.pathPlaceholder}
                    onChange={(event) => setFolderPath(event.target.value)}
                  />
                  <button type="submit" disabled={folderPending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {folderPending ? copy.folderTool.ingesting : copy.folderTool.ingest}
                  </button>
                </div>
                {folderNotice ? <p className="package-message">{folderNotice}</p> : null}
                {folderError ? <p className="package-error">{folderError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={runTerminalReadCommand}>
                <div className="tool-heading">
                  <TerminalSquare size={16} aria-hidden="true" />
                  <strong>{copy.terminalTool.title}</strong>
                </div>
                <div className="browser-tool-row">
                  <select
                    value={terminalCommand}
                    aria-label={copy.terminalTool.commandLabel}
                    onChange={(event) =>
                      setTerminalCommand(event.target.value as TerminalReadCommand)
                    }
                  >
                    <option value="git status --short">
                      {copy.terminalTool.options["git status --short"]}
                    </option>
                    <option value="git diff --stat">
                      {copy.terminalTool.options["git diff --stat"]}
                    </option>
                    <option value="git branch --show-current">
                      {copy.terminalTool.options["git branch --show-current"]}
                    </option>
                    <option value="pwd">{copy.terminalTool.options.pwd}</option>
                  </select>
                  <button type="submit" disabled={terminalPending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {terminalPending ? copy.terminalTool.running : copy.terminalTool.run}
                  </button>
                </div>
                {terminalNotice ? <p className="package-message">{terminalNotice}</p> : null}
                {terminalError ? <p className="package-error">{terminalError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={runTerminalWriteBoundary}>
                <div className="tool-heading">
                  <TerminalSquare size={16} aria-hidden="true" />
                  <strong>{copy.terminalTool.writeTitle}</strong>
                </div>
                <div className="browser-tool-row">
                  <input
                    value={terminalWriteCommand}
                    aria-label={copy.terminalTool.writeCommandLabel}
                    placeholder={copy.terminalTool.writePlaceholder}
                    onChange={(event) => setTerminalWriteCommand(event.target.value)}
                  />
                  <button
                    type="submit"
                    disabled={terminalWritePending || capabilityPending !== null}
                  >
                    <MousePointerClick size={14} aria-hidden="true" />
                    {terminalWritePending
                      ? copy.terminalTool.requestingWrite
                      : copy.terminalTool.requestWrite}
                  </button>
                </div>
                {terminalWriteNotice ? (
                  <p className="package-message">{terminalWriteNotice}</p>
                ) : null}
                {terminalWriteError ? (
                  <p className="package-error">{terminalWriteError}</p>
                ) : null}
              </form>
              <form className="browser-tool" onSubmit={readEmailBoundary}>
                <div className="tool-heading">
                  <Mail size={16} aria-hidden="true" />
                  <strong>{copy.emailReadTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={emailMailbox}
                    aria-label={copy.emailReadTool.mailboxPlaceholder}
                    placeholder={copy.emailReadTool.mailboxPlaceholder}
                    onChange={(event) => setEmailMailbox(event.target.value)}
                  />
                  <input
                    value={emailReadQuery}
                    aria-label={copy.emailReadTool.queryPlaceholder}
                    placeholder={copy.emailReadTool.queryPlaceholder}
                    onChange={(event) => setEmailReadQuery(event.target.value)}
                  />
                  <button type="submit" disabled={emailReadPending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {emailReadPending
                      ? copy.emailReadTool.requestingRead
                      : copy.emailReadTool.requestRead}
                  </button>
                </div>
                {emailReadNotice ? <p className="package-message">{emailReadNotice}</p> : null}
                {emailReadError ? <p className="package-error">{emailReadError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={createEmailDraftBoundary}>
                <div className="tool-heading">
                  <Mail size={16} aria-hidden="true" />
                  <strong>{copy.emailDraftTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={draftEmailTo}
                    aria-label={copy.emailDraftTool.toPlaceholder}
                    placeholder={copy.emailDraftTool.toPlaceholder}
                    onChange={(event) => setDraftEmailTo(event.target.value)}
                  />
                  <input
                    value={draftEmailSubject}
                    aria-label={copy.emailDraftTool.subjectPlaceholder}
                    placeholder={copy.emailDraftTool.subjectPlaceholder}
                    onChange={(event) => setDraftEmailSubject(event.target.value)}
                  />
                  <textarea
                    value={draftEmailBody}
                    aria-label={copy.emailDraftTool.bodyPlaceholder}
                    placeholder={copy.emailDraftTool.bodyPlaceholder}
                    rows={3}
                    onChange={(event) => setDraftEmailBody(event.target.value)}
                  />
                  <button
                    type="submit"
                    disabled={emailDraftPending || capabilityPending !== null}
                  >
                    <MousePointerClick size={14} aria-hidden="true" />
                    {emailDraftPending
                      ? copy.emailDraftTool.requestingDraft
                      : copy.emailDraftTool.requestDraft}
                  </button>
                </div>
                {emailDraftNotice ? (
                  <p className="package-message">{emailDraftNotice}</p>
                ) : null}
                {emailDraftError ? <p className="package-error">{emailDraftError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={sendEmailBoundary}>
                <div className="tool-heading">
                  <Mail size={16} aria-hidden="true" />
                  <strong>{copy.emailTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={emailTo}
                    aria-label={copy.emailTool.toPlaceholder}
                    placeholder={copy.emailTool.toPlaceholder}
                    onChange={(event) => setEmailTo(event.target.value)}
                  />
                  <input
                    value={emailSubject}
                    aria-label={copy.emailTool.subjectPlaceholder}
                    placeholder={copy.emailTool.subjectPlaceholder}
                    onChange={(event) => setEmailSubject(event.target.value)}
                  />
                  <textarea
                    value={emailBody}
                    aria-label={copy.emailTool.bodyPlaceholder}
                    placeholder={copy.emailTool.bodyPlaceholder}
                    rows={3}
                    onChange={(event) => setEmailBody(event.target.value)}
                  />
                  <button type="submit" disabled={emailPending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {emailPending ? copy.emailTool.requestingSend : copy.emailTool.requestSend}
                  </button>
                </div>
                {emailNotice ? <p className="package-message">{emailNotice}</p> : null}
                {emailError ? <p className="package-error">{emailError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={readDriveBoundary}>
                <div className="tool-heading">
                  <Cloud size={16} aria-hidden="true" />
                  <strong>{copy.driveReadTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={driveLocation}
                    aria-label={copy.driveReadTool.locationPlaceholder}
                    placeholder={copy.driveReadTool.locationPlaceholder}
                    onChange={(event) => setDriveLocation(event.target.value)}
                  />
                  <input
                    value={driveReadQuery}
                    aria-label={copy.driveReadTool.queryPlaceholder}
                    placeholder={copy.driveReadTool.queryPlaceholder}
                    onChange={(event) => setDriveReadQuery(event.target.value)}
                  />
                  <button type="submit" disabled={driveReadPending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {driveReadPending
                      ? copy.driveReadTool.requestingRead
                      : copy.driveReadTool.requestRead}
                  </button>
                </div>
                {driveReadNotice ? <p className="package-message">{driveReadNotice}</p> : null}
                {driveReadError ? <p className="package-error">{driveReadError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={writeDriveBoundary}>
                <div className="tool-heading">
                  <Cloud size={16} aria-hidden="true" />
                  <strong>{copy.driveWriteTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={driveWriteLocation}
                    aria-label={copy.driveWriteTool.locationPlaceholder}
                    placeholder={copy.driveWriteTool.locationPlaceholder}
                    onChange={(event) => setDriveWriteLocation(event.target.value)}
                  />
                  <input
                    value={driveWriteSummary}
                    aria-label={copy.driveWriteTool.summaryPlaceholder}
                    placeholder={copy.driveWriteTool.summaryPlaceholder}
                    onChange={(event) => setDriveWriteSummary(event.target.value)}
                  />
                  <button type="submit" disabled={driveWritePending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {driveWritePending
                      ? copy.driveWriteTool.requestingWrite
                      : copy.driveWriteTool.requestWrite}
                  </button>
                </div>
                {driveWriteNotice ? <p className="package-message">{driveWriteNotice}</p> : null}
                {driveWriteError ? <p className="package-error">{driveWriteError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={captureComputerScreenshot}>
                <div className="tool-heading">
                  <MonitorCog size={16} aria-hidden="true" />
                  <strong>{copy.computerTool.title}</strong>
                </div>
                <div className="browser-tool-row">
                  <button type="submit" disabled={computerPending || capabilityPending !== null}>
                    <MousePointerClick size={14} aria-hidden="true" />
                    {computerPending ? copy.computerTool.capturing : copy.computerTool.capture}
                  </button>
                </div>
                {computerNotice ? <p className="package-message">{computerNotice}</p> : null}
                {computerError ? <p className="package-error">{computerError}</p> : null}
              </form>
              <form className="browser-tool" onSubmit={unlockComputerControl}>
                <div className="tool-heading">
                  <ShieldCheck size={16} aria-hidden="true" />
                  <strong>{copy.computerControlTool.unlockTitle}</strong>
                </div>
                <div
                  className="computer-control-unlock-status"
                  data-unlocked={computerControlUnlockStatus.unlocked}
                >
                  <span>{copy.computerControlTool.unlockChallengeLabel}</span>
                  <code>{computerControlUnlockStatus.challenge || "------"}</code>
                  <span>
                    {computerControlUnlockStatus.unlocked && computerControlUnlockUntilText
                      ? `${copy.computerControlTool.unlockReady} ${copy.computerControlTool.unlockExpires} ${computerControlUnlockUntilText}`
                      : copy.computerControlTool.unlockRequired}
                  </span>
                </div>
                <div className="browser-tool-row">
                  <input
                    value={computerControlUnlockToken}
                    aria-label={copy.computerControlTool.unlockTokenPlaceholder}
                    placeholder={copy.computerControlTool.unlockTokenPlaceholder}
                    onChange={(event) => setComputerControlUnlockToken(event.target.value)}
                  />
                  <button
                    type="submit"
                    disabled={computerControlUnlockPending || capabilityPending !== null}
                  >
                    <ShieldCheck size={14} aria-hidden="true" />
                    {computerControlUnlockPending
                      ? copy.computerControlTool.unlockingControl
                      : copy.computerControlTool.unlockControl}
                  </button>
                </div>
                {computerControlUnlockNotice ? (
                  <p className="package-message">{computerControlUnlockNotice}</p>
                ) : null}
                {computerControlUnlockError ? (
                  <p className="package-error">{computerControlUnlockError}</p>
                ) : null}
              </form>
              <form className="browser-tool" onSubmit={controlComputerBoundary}>
                <div className="tool-heading">
                  <MonitorCog size={16} aria-hidden="true" />
                  <strong>{copy.computerControlTool.title}</strong>
                </div>
                <div className="email-tool-fields">
                  <input
                    value={computerControlTarget}
                    aria-label={copy.computerControlTool.targetPlaceholder}
                    placeholder={copy.computerControlTool.targetPlaceholder}
                    onChange={(event) => setComputerControlTarget(event.target.value)}
                  />
                  <input
                    value={computerControlAction}
                    aria-label={copy.computerControlTool.actionPlaceholder}
                    placeholder={copy.computerControlTool.actionPlaceholder}
                    onChange={(event) => setComputerControlAction(event.target.value)}
                  />
                  <button
                    type="submit"
                    disabled={computerControlPending || capabilityPending !== null}
                  >
                    <MousePointerClick size={14} aria-hidden="true" />
                    {computerControlPending
                      ? copy.computerControlTool.requestingControl
                      : copy.computerControlTool.requestControl}
                  </button>
                </div>
                {computerControlNotice ? (
                  <p className="package-message">{computerControlNotice}</p>
                ) : null}
                {computerControlError ? (
                  <p className="package-error">{computerControlError}</p>
                ) : null}
              </form>
              <div className="capability-grid">
                {capabilityCatalog.map((capability) => {
                  const Icon = capabilityFamilyIcon(capability.family);
                  const latestRecord = capabilityRecords.find(
                    (record) => record.request.capability === capability.capability,
                  );

                  return (
                    <article className="capability-card" key={capability.capability}>
                      <div className="capability-card-header">
                        <Icon size={16} aria-hidden="true" />
                        <div>
                          <strong>{copy.capabilityOptions[capability.capability]}</strong>
                          <span>{copy.capabilityFamilyOptions[capability.family]}</span>
                        </div>
                        {capability.experimental ? (
                          <span className="experimental-label">{copy.capabilities.experimental}</span>
                        ) : null}
                      </div>
                      <p>{copy.capabilitySummaries[capability.capability]}</p>
                      <div className="capability-meta">
                        <span className={`risk ${capability.risk_level}`}>
                          {copy.riskOptions[capability.risk_level]}
                        </span>
                        {latestRecord ? (
                          <span className={`access-status ${latestRecord.effective_status}`}>
                            {copy.accessStatusOptions[latestRecord.effective_status]}
                          </span>
                        ) : null}
                        {latestRecord && latestRecord.grant_state !== "not_granted" ? (
                          <span className={`access-status ${latestRecord.grant_state}`}>
                            {copy.accessGrantOptions[latestRecord.grant_state]}
                          </span>
                        ) : null}
                      </div>
                      <button
                        type="button"
                        onClick={() => void requestCapabilityAccess(capability.capability)}
                        disabled={capabilityPending !== null || resolutionPending !== null}
                      >
                        <MousePointerClick size={14} aria-hidden="true" />
                        {capabilityPending === capability.capability
                          ? copy.capabilities.requesting
                          : copy.capabilities.request}
                      </button>
                    </article>
                  );
                })}
              </div>

              <div className="approval-queue">
                <div className="queue-heading">
                  <strong>{copy.capabilities.pendingTitle}</strong>
                  <span>{pendingCapabilityRecords.length}</span>
                </div>
                {pendingCapabilityRecords.length === 0 ? (
                  <p className="empty-state">{copy.capabilities.noPending}</p>
                ) : (
                  <div className="approval-list">
                    {pendingCapabilityRecords.map((record) => (
                      <article className="approval-row" key={record.request.id}>
                        <div>
                          <strong>{copy.capabilityOptions[record.request.capability]}</strong>
                          <p>
                            {copy.riskOptions[record.request.risk_level]} ·{" "}
                            {copy.accessOptions[record.request.access_mode]}
                          </p>
                        </div>
                        <div className="approval-actions">
                          <button
                            type="button"
                            aria-label={copy.capabilities.approve}
                            onClick={() => void resolveCapabilityAccess(record.request.id, true)}
                            disabled={resolutionPending !== null || capabilityPending !== null}
                          >
                            <Check size={14} aria-hidden="true" />
                            {resolutionPending === record.request.id
                              ? copy.capabilities.resolving
                              : copy.capabilities.approve}
                          </button>
                          <button
                            type="button"
                            aria-label={copy.capabilities.reject}
                            onClick={() => void resolveCapabilityAccess(record.request.id, false)}
                            disabled={resolutionPending !== null || capabilityPending !== null}
                          >
                            <X size={14} aria-hidden="true" />
                            {copy.capabilities.reject}
                          </button>
                        </div>
                      </article>
                    ))}
                  </div>
                )}
              </div>

              {auditError ? <p className="package-error">{auditError}</p> : null}
              <div className="tool-output">
                <div className="recent-audit-heading">{copy.browserTool.outputTitle}</div>
                {capabilityInvocations.length === 0 ? (
                  <p className="empty-state">{copy.browserTool.noOutput}</p>
                ) : (
                  <div className="tool-output-list">
                    {capabilityInvocations.slice(0, 4).map((invocation) => (
                      <article className="tool-output-row" key={invocation.id}>
                        <div>
                          <strong>
                            {invocation.title || copy.capabilityOptions[invocation.capability]}
                          </strong>
                          <span className={`access-status ${invocation.status}`}>
                            {copy.invocationStatusOptions[invocation.status]}
                          </span>
                        </div>
                        {invocation.excerpt ? <p>{invocation.excerpt}</p> : null}
                        {invocation.warnings.length > 0 ? (
                          <p>{invocation.warnings.join(" ")}</p>
                        ) : null}
                        <footer>
                          <span>{formatTaskDate(invocation.created_at, language)}</span>
                          {invocation.approval_request_id ? (
                            <span>
                              {copy.browserTool.approvalRequest}:{" "}
                              {invocation.approval_request_id}
                            </span>
                          ) : null}
                          {invocation.evidence_url || invocation.evidence_ref ? (
                            <span>{invocation.evidence_url || invocation.evidence_ref}</span>
                          ) : null}
                        </footer>
                      </article>
                    ))}
                  </div>
                )}
              </div>
              <div className="recent-audit-heading">{copy.capabilities.auditTitle}</div>
              {permissionAudits.length === 0 ? (
                <p className="empty-state">{copy.audit.empty}</p>
              ) : (
                <div className="audit-list">
                  {permissionAudits.slice(0, 4).map((entry) => (
                    <article className="audit-row" key={entry.id}>
                      <strong>{copy.capabilityOptions[entry.capability]}</strong>
                      <span className={`decision ${entry.decision}`}>{copy.decisionOptions[entry.decision]}</span>
                      <p>
                        {copy.riskOptions[entry.risk_level]} · {copy.accessOptions[entry.access_mode]} ·{" "}
                        {formatTaskDate(entry.created_at, language)}
                      </p>
                    </article>
                  ))}
                </div>
              )}
            </section>
            <dl>
              <div>
                <dt>{copy.inspector.largeModel}</dt>
                <dd>{copy.largeModelOptions[state.large_model_provider]}</dd>
              </div>
              <div>
                <dt>{copy.inspector.model}</dt>
                <dd>{copy.modelOptions[state.model_route]}</dd>
              </div>
              <div>
                <dt>{copy.inspector.access}</dt>
                <dd>{copy.accessOptions[state.access_mode]}</dd>
              </div>
              <div>
                <dt>{copy.inspector.thinking}</dt>
                <dd>{copy.thinkingOptions[state.thinking_level]}</dd>
              </div>
              <div>
                <dt>{copy.inspector.scope}</dt>
                <dd>{copy.scopeOptions[state.workspace_scope]}</dd>
              </div>
              <div>
                <dt>{copy.inspector.theme}</dt>
                <dd>{copy.themeOptions[themeStyle]}</dd>
              </div>
            </dl>
            <section className="tool-backend-list" aria-labelledby="tool-backend-title">
              <strong id="tool-backend-title">{copy.backendLabels.title}</strong>
              <dl>
                <div>
                  <dt>{copy.backendLabels.networkSearch}</dt>
                  <dd title={modelToolStrategy.note}>
                    {
                      copy.backendOptions.network_search[
                        modelToolStrategy.network_search_backend
                      ]
                    }
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.largeModelProvider}</dt>
                  <dd title={modelToolStrategy.note}>
                    {copy.largeModelOptions[modelToolStrategy.large_model_provider]}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.networkSearchSupport}</dt>
                  <dd title={modelToolStrategy.note}>
                    {modelToolStrategy.large_model_supports_network_search
                      ? copy.backendLabels.nativeSupported
                      : copy.backendLabels.sourceModelRequired}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.networkSearchSourceModel}</dt>
                  <dd title={modelToolStrategy.note}>
                    {modelToolStrategy.network_search_source_model
                      ? copy.networkSearchSourceOptions[
                          modelToolStrategy.network_search_source_model
                        ]
                      : copy.backendLabels.notSelected}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.networkSearchRoute}</dt>
                  <dd title={networkSearchRouteStatus.note}>
                    {copy.backendOptions.network_search[networkSearchRouteStatus.backend]}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.networkSearchExecution}</dt>
                  <dd title={networkSearchRouteStatus.note}>
                    {
                      copy.backendOptions.network_search_execution[
                        networkSearchRouteStatus.execution_mode
                      ]
                    }
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.networkSearchEvidence}</dt>
                  <dd title={networkSearchRouteStatus.note}>
                    {
                      copy.backendOptions.network_search_evidence[
                        networkSearchRouteStatus.evidence_policy
                      ]
                    }
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.networkRequests}</dt>
                  <dd title={networkSearchRouteStatus.note}>
                    {networkSearchRouteStatus.network_requests_enabled
                      ? copy.backendLabels.enabled
                      : copy.backendLabels.disabled}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.deepSeekOrchestration}</dt>
                  <dd title={networkSearchRouteStatus.note}>
                    {networkSearchRouteStatus.deepseek_orchestration_ready
                      ? copy.backendLabels.chatReady
                      : copy.backendLabels.chatNotReady}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.confirmationGate}</dt>
                  <dd title={networkSearchRouteStatus.note}>
                    {networkSearchRouteStatus.requires_user_confirmation
                      ? copy.backendLabels.confirmationRequired
                      : copy.backendLabels.confirmationNotRequired}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.deepSeekApi}</dt>
                  <dd>
                    {deepSeekCredentialStatus.api_key_configured
                      ? copy.backendLabels.apiKeyConfigured
                      : copy.backendLabels.apiKeyMissing}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.deepSeekChatApi}</dt>
                  <dd title={deepSeekCredentialStatus.readiness_note}>
                    {deepSeekCredentialStatus.chat_completion_ready
                      ? copy.backendLabels.chatReady
                      : copy.backendLabels.chatNotReady}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.apiBaseUrl}</dt>
                  <dd>{deepSeekCredentialStatus.base_url}</dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.chatEndpoint}</dt>
                  <dd>{deepSeekCredentialStatus.chat_completions_url}</dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.deepSeekModels}</dt>
                  <dd>
                    {deepSeekCredentialStatus.flash_model} /{" "}
                    {deepSeekCredentialStatus.pro_model}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.deepSeekTelemetry}</dt>
                  <dd
                    title={
                      latestDeepSeekTelemetry
                        ? `${latestDeepSeekTelemetry.model} / ${latestDeepSeekTelemetry.request_hash}`
                        : ""
                    }
                  >
                    {latestDeepSeekTelemetryText}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.cacheEntries}</dt>
                  <dd className="backend-cache-control">
                    <span>{deepSeekChatCacheState.entries}</span>
                    <button
                      type="button"
                      onClick={() => void clearDeepSeekChatCache()}
                      disabled={deepSeekCachePending}
                    >
                      {deepSeekCachePending
                        ? copy.backendLabels.clearingCache
                        : copy.backendLabels.clearCache}
                    </button>
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.apiKeyEnv}</dt>
                  <dd>{deepSeekCredentialStatus.api_key_env_var}</dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.email}</dt>
                  <dd>{copy.backendOptions.email[state.tool_backends.email]}</dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.drive}</dt>
                  <dd>{copy.backendOptions.drive[state.tool_backends.drive]}</dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.computerScreenshot}</dt>
                  <dd>
                    {
                      copy.backendOptions.computer_screenshot[
                        modelToolStrategy.computer_screenshot_backend
                      ]
                    }
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.screenshotBackendStatus}</dt>
                  <dd title={computerUseBackendStatus.screenshot_note}>
                    {copy.backendOptions.computer_screenshot[
                      computerUseBackendStatus.screenshot_backend
                    ]}{" "}
                    /{" "}
                    {computerUseBackendStatus.screenshot_available
                      ? copy.backendLabels.backendAvailable
                      : copy.backendLabels.backendUnavailable}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.screenshotPermission}</dt>
                  <dd className="backend-permission-note">
                    <span>
                      {computerUseBackendStatus.screenshot_permission_required
                        ? copy.backendLabels.osPermissionRequired
                        : copy.backendLabels.osPermissionNotRequired}
                    </span>
                    <small>{computerUseBackendStatus.screenshot_permission_note}</small>
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.computerControl}</dt>
                  <dd>
                    {
                      copy.backendOptions.computer_control[
                        modelToolStrategy.computer_control_backend
                      ]
                    }
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.controlBackendStatus}</dt>
                  <dd title={computerUseBackendStatus.control_note}>
                    {copy.backendOptions.computer_control[computerUseBackendStatus.control_backend]}{" "}
                    /{" "}
                    {computerUseBackendStatus.control_available
                      ? copy.backendLabels.backendAvailable
                      : copy.backendLabels.backendUnavailable}
                    {computerUseBackendStatus.control_requires_approval
                      ? ` / ${copy.backendLabels.approvalRequired}`
                      : ""}
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.controlPermission}</dt>
                  <dd className="backend-permission-note">
                    <span>
                      {computerUseBackendStatus.control_permission_required
                        ? copy.backendLabels.osPermissionRequired
                        : copy.backendLabels.osPermissionNotRequired}
                    </span>
                    <small>{computerUseBackendStatus.control_permission_note}</small>
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.codexBridgeRuntime}</dt>
                  <dd className="backend-permission-note">
                    <span>
                      {computerUseBackendStatus.codex_bridge.required
                        ? copy.backendLabels.bridgeRequired
                        : copy.backendLabels.bridgeNotRequired}
                      {" / "}
                      {computerUseBackendStatus.codex_bridge.transport
                        ? copy.codexBridgeTransportOptions[
                            computerUseBackendStatus.codex_bridge.transport
                          ]
                        : copy.backendLabels.bridgeTransportMissing}
                      {" / "}
                      {computerUseBackendStatus.codex_bridge.endpoint_configured
                        ? copy.backendLabels.bridgeEndpointConfigured
                        : copy.backendLabels.bridgeEndpointMissing}
                      {" / "}
                      {computerUseBackendStatus.codex_bridge.connected
                        ? copy.backendLabels.bridgeConnected
                        : copy.backendLabels.bridgeNotConnected}
                    </span>
                    <small>
                      {computerUseBackendStatus.codex_bridge.transport_env_var} /{" "}
                      {computerUseBackendStatus.codex_bridge.endpoint_env_var}:{" "}
                      {computerUseBackendStatus.codex_bridge.note}
                    </small>
                  </dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.runtimePlatform}</dt>
                  <dd>{copy.runtimePlatformOptions[modelToolStrategy.runtime_platform]}</dd>
                </div>
                <div>
                  <dt>{copy.backendLabels.macosPath}</dt>
                  <dd>
                    {modelToolStrategy.macos_supported
                      ? copy.backendLabels.enabled
                      : copy.backendLabels.disabled}
                  </dd>
                </div>
              </dl>
              {deepSeekCacheNotice ? (
                <p className="package-message">{deepSeekCacheNotice}</p>
              ) : null}
              {deepSeekCacheError ? (
                <p className="package-error">{deepSeekCacheError}</p>
              ) : null}
            </section>
          </aside>
        </section>
      </section>
    </main>
  );
}
