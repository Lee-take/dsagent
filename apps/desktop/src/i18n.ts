import type {
  AccessMode,
  CapabilityAccessStatus,
  CapabilityFamily,
  CapabilityGrantState,
  CapabilityInvocationStatus,
  CapabilityKind,
  CodexBridgeTransport,
  ComputerControlBackend,
  ComputerScreenshotBackend,
  DriveBackend,
  EmailBackend,
  LargeModelProvider,
  Language,
  MemoryLifecycle,
  MemoryScope,
  MemorySensitivity,
  MemoryCandidateStatus,
  MemoryType,
  ModelRoute,
  NetworkSearchBackend,
  NetworkSearchEvidencePolicy,
  NetworkSearchExecutionMode,
  NetworkSearchSourceModel,
  OperationsBriefingRunStatus,
  PolicyDecision,
  RiskLevel,
  RuntimePlatform,
  TerminalReadCommand,
  ThemeStyle,
  ThinkingLevel,
  WorkspaceScope,
} from "./types";

type TranslationSet = {
  brandTagline: string;
  navLabel: string;
  nav: {
    workbench: string;
    memory: string;
    approvals: string;
  };
  controls: {
    modelRoute: string;
    largeModelProvider: string;
    accessMode: string;
    thinkingLevel: string;
    themeStyle: string;
    language: string;
    networkSearchSourceModel: string;
  };
  largeModelOptions: Record<LargeModelProvider, string>;
  modelOptions: Record<ModelRoute, string>;
  accessOptions: Record<AccessMode, string>;
  thinkingOptions: Record<ThinkingLevel, string>;
  scopeOptions: Record<WorkspaceScope, string>;
  themeOptions: Record<ThemeStyle, string>;
  networkSearchSourceOptions: Record<NetworkSearchSourceModel, string>;
  runtimePlatformOptions: Record<RuntimePlatform, string>;
  codexBridgeTransportOptions: Record<CodexBridgeTransport, string>;
  backendOptions: {
    network_search: Record<NetworkSearchBackend, string>;
    network_search_execution: Record<NetworkSearchExecutionMode, string>;
    network_search_evidence: Record<NetworkSearchEvidencePolicy, string>;
    email: Record<EmailBackend, string>;
    drive: Record<DriveBackend, string>;
    computer_screenshot: Record<ComputerScreenshotBackend, string>;
    computer_control: Record<ComputerControlBackend, string>;
  };
  backendLabels: {
    title: string;
    largeModelProvider: string;
    networkSearch: string;
    networkSearchSupport: string;
    networkSearchSourceModel: string;
    networkSearchRoute: string;
    networkSearchExecution: string;
    networkSearchEvidence: string;
    networkRequests: string;
    deepSeekOrchestration: string;
    confirmationGate: string;
    email: string;
    drive: string;
    computerScreenshot: string;
    computerControl: string;
    deepSeekApi: string;
    deepSeekChatApi: string;
    deepSeekTelemetry: string;
    apiBaseUrl: string;
    chatEndpoint: string;
    deepSeekModels: string;
    apiKeyEnv: string;
    apiKeyConfigured: string;
    apiKeyMissing: string;
    chatReady: string;
    chatNotReady: string;
    enabled: string;
    disabled: string;
    confirmationRequired: string;
    confirmationNotRequired: string;
    screenshotBackendStatus: string;
    screenshotPermission: string;
    controlBackendStatus: string;
    controlPermission: string;
    codexBridgeRuntime: string;
    backendAvailable: string;
    backendUnavailable: string;
    approvalRequired: string;
    osPermissionRequired: string;
    osPermissionNotRequired: string;
    bridgeRequired: string;
    bridgeNotRequired: string;
    bridgeEndpointConfigured: string;
    bridgeEndpointMissing: string;
    bridgeConnected: string;
    bridgeNotConnected: string;
    bridgeTransportMissing: string;
    nativeSupported: string;
    sourceModelRequired: string;
    notSelected: string;
    noTelemetry: string;
    cacheHit: string;
    cacheMiss: string;
    cacheDisabled: string;
    cacheEntries: string;
    clearCache: string;
    clearingCache: string;
    cacheCleared: (count: number) => string;
    cacheClearFailed: string;
    tokens: string;
    cost: string;
    runtimePlatform: string;
    macosPath: string;
  };
  capabilityFamilyOptions: Record<CapabilityFamily, string>;
  capabilityOptions: Record<CapabilityKind, string>;
  capabilitySummaries: Record<CapabilityKind, string>;
  riskOptions: Record<RiskLevel, string>;
  decisionOptions: Record<PolicyDecision, string>;
  accessStatusOptions: Record<CapabilityAccessStatus, string>;
  accessGrantOptions: Record<CapabilityGrantState, string>;
  invocationStatusOptions: Record<CapabilityInvocationStatus, string>;
  workbench: {
    stage: string;
    title: string;
    summary: string;
  };
  localSetup: {
    title: string;
    required: string;
    ready: string;
    appData: string;
    settingsFile: string;
    workspaceDir: string;
    evidenceDir: string;
    exportDir: string;
    workspacePlaceholder: string;
    evidencePlaceholder: string;
    exportPlaceholder: string;
    choose: string;
    chooseFailed: string;
    workspaceDialogTitle: string;
    evidenceDialogTitle: string;
    exportDialogTitle: string;
    save: string;
    saving: string;
    saved: string;
    failed: string;
    loadFailed: string;
  };
  deepSeekPricing: {
    title: string;
    enabled: string;
    disabled: string;
    statusConfigured: string;
    statusNotConfigured: string;
    help: string;
    settingsFile: string;
    flashPrompt: string;
    flashCompletion: string;
    proPrompt: string;
    proCompletion: string;
    pricePlaceholder: string;
    save: string;
    saving: string;
    saved: string;
    failed: string;
    loadFailed: string;
  };
  operationsBriefing: {
    title: string;
    folderPlaceholder: string;
    run: string;
    running: string;
    seedTemplates: string;
    seededTemplates: string;
    seedPendingHint: string;
    seedFailed: string;
    exportPackage: string;
    exportReport: string;
    exportHtmlReport: string;
    exportPdfReport: string;
    exported: string;
    reportExported: string;
    reportPendingHint: string;
    reportExportFailed: string;
    htmlReportExported: string;
    htmlReportPendingHint: string;
    htmlReportExportFailed: string;
    pdfReportExported: string;
    pdfReportPendingHint: string;
    pdfReportExportFailed: string;
    latestRun: string;
    noRuns: string;
    pendingHint: string;
    failed: string;
    loadFailed: string;
    anomalies: string;
    actions: string;
    noAnomalies: string;
    noActions: string;
    evidence: string;
    archived: string;
    status: Record<OperationsBriefingRunStatus, string>;
  };
  package: {
    title: string;
    taskTitle: string;
    taskSummary: string;
    addRecord: string;
    exportPackage: string;
    copyPackage: string;
    importPackage: string;
    previewImport: string;
    previewing: string;
    previewTitle: string;
    previewReady: string;
    previewFailed: string;
    previewTotalTasks: string;
    previewNewTasks: string;
    previewSkippedTasks: string;
    previewMemoryCandidates: string;
    previewMemoryCandidateHint: string;
    previewArchivedRuns: string;
    previewArchiveHint: string;
    previewWorkflowTemplates: string;
    previewNewWorkflowTemplates: string;
    previewSkippedWorkflowTemplates: string;
    previewWorkflowTemplateHint: string;
    packageJson: string;
    importJson: string;
    emptyTitle: string;
    emptyImport: string;
    created: string;
    exported: string;
    copied: string;
    imported: (
      imported: number,
      skipped: number,
      candidateImported: number,
      candidateSkipped: number,
      briefingImported: number,
      briefingSkipped: number,
      templateImported: number,
      templateSkipped: number,
    ) => string;
    noRecords: string;
    copyFailed: string;
    loadFailed: string;
  };
  memory: {
    title: string;
    autoCapture: string;
    noMemories: string;
    loadFailed: string;
    search: string;
    searchPlaceholder: string;
    candidateTitle: string;
    candidateBody: string;
    candidateType: string;
    candidateScope: string;
    candidateSensitivity: string;
    candidateLifecycle: string;
    expiresAt: string;
    metadata: string;
    propose: string;
    proposing: string;
    candidates: string;
    noCandidates: string;
    accept: string;
    reject: string;
    edit: string;
    editTitle: string;
    editBody: string;
    save: string;
    saving: string;
    cancel: string;
    delete: string;
    deleting: string;
    resolving: string;
    proposed: string;
    accepted: string;
    rejected: string;
    updated: string;
    deleted: string;
    emptyCandidate: string;
    emptyEdit: string;
    emptyExpiration: string;
    proposeFailed: string;
    resolveFailed: string;
    updateFailed: string;
    deleteFailed: string;
    conflictWarning: (count: number) => string;
    conflictDetails: string;
    previewMerge: string;
    previewingMerge: string;
    mergePreviewTitle: string;
    mergePreviewDraft: string;
    mergePreviewFailed: string;
    mergeAndAccept: string;
    merged: string;
    mergeFailed: string;
    previewReplace: string;
    previewingReplace: string;
    replacePreviewTitle: string;
    replacePreviewDraft: string;
    replacePreviewTargets: string;
    replacePreviewFailed: string;
    replaceAndAccept: string;
    replaced: string;
    replaceFailed: string;
    linkAndAccept: string;
    linked: string;
    linkFailed: string;
    linkedMemories: (count: number) => string;
    updatedAt: string;
    candidateStatus: Record<MemoryCandidateStatus, string>;
    typeOptions: Record<MemoryType, string>;
    scopeOptions: Record<MemoryScope, string>;
    sensitivityOptions: Record<MemorySensitivity, string>;
    lifecycleOptions: Record<MemoryLifecycle, string>;
  };
  audit: {
    title: string;
    browser: string;
    emailSend: string;
    computerControl: string;
    empty: string;
    loadFailed: string;
    pending: string;
  };
  capabilities: {
    title: string;
    request: string;
    requesting: string;
    experimental: string;
    pendingTitle: string;
    noPending: string;
    approve: string;
    reject: string;
    resolving: string;
    loadFailed: string;
    requestFailed: string;
    resolveFailed: string;
    auditTitle: string;
  };
  browserTool: {
    title: string;
    urlPlaceholder: string;
    browse: string;
    browsing: string;
    outputTitle: string;
    noOutput: string;
    approvalRequest: string;
    pendingHint: string;
    failed: string;
  };
  browserSubmitTool: {
    title: string;
    urlPlaceholder: string;
    summaryPlaceholder: string;
    requestSubmit: string;
    requestingSubmit: string;
    pendingHint: string;
    blocked: string;
    failed: string;
  };
  networkSearchTool: {
    title: string;
    queryPlaceholder: string;
    scopePlaceholder: string;
    requestSearch: string;
    requestingSearch: string;
    pendingHint: string;
    completed: string;
    blocked: string;
    failed: string;
    sourceModelRequiredTitle: string;
    sourceModelRequiredBody: string;
    sourceModelPlaceholder: string;
    sourceModelMissing: string;
    routeNotEnabled: string;
  };
  fileTool: {
    title: string;
    pathPlaceholder: string;
    read: string;
    reading: string;
    pendingHint: string;
    failed: string;
  };
  fileWriteTool: {
    title: string;
    pathPlaceholder: string;
    summaryPlaceholder: string;
    contentPlaceholder: string;
    requestWrite: string;
    requestingWrite: string;
    pendingHint: string;
    completed: string;
    blocked: string;
    failed: string;
  };
  folderTool: {
    title: string;
    pathPlaceholder: string;
    ingest: string;
    ingesting: string;
    pendingHint: string;
    failed: string;
  };
  terminalTool: {
    title: string;
    commandLabel: string;
    run: string;
    running: string;
    pendingHint: string;
    failed: string;
    writeTitle: string;
    writeCommandLabel: string;
    writePlaceholder: string;
    requestWrite: string;
    requestingWrite: string;
    writePendingHint: string;
    writeBlocked: string;
    writeFailed: string;
    options: Record<TerminalReadCommand, string>;
  };
  computerTool: {
    title: string;
    capture: string;
    capturing: string;
    pendingHint: string;
    captured: string;
    unavailable: string;
    failed: string;
  };
  computerControlTool: {
    title: string;
    unlockTitle: string;
    unlockChallengeLabel: string;
    unlockTokenPlaceholder: string;
    unlockControl: string;
    unlockingControl: string;
    unlockReady: string;
    unlockRequired: string;
    unlockExpires: string;
    unlockFailed: string;
    targetPlaceholder: string;
    actionPlaceholder: string;
    requestControl: string;
    requestingControl: string;
    pendingHint: string;
    executed: string;
    blocked: string;
    failed: string;
  };
  emailTool: {
    title: string;
    toPlaceholder: string;
    subjectPlaceholder: string;
    bodyPlaceholder: string;
    requestSend: string;
    requestingSend: string;
    pendingHint: string;
    blocked: string;
    failed: string;
  };
  emailDraftTool: {
    title: string;
    toPlaceholder: string;
    subjectPlaceholder: string;
    bodyPlaceholder: string;
    requestDraft: string;
    requestingDraft: string;
    pendingHint: string;
    blocked: string;
    failed: string;
  };
  emailReadTool: {
    title: string;
    mailboxPlaceholder: string;
    queryPlaceholder: string;
    requestRead: string;
    requestingRead: string;
    pendingHint: string;
    blocked: string;
    failed: string;
  };
  driveReadTool: {
    title: string;
    locationPlaceholder: string;
    queryPlaceholder: string;
    requestRead: string;
    requestingRead: string;
    pendingHint: string;
    completed: string;
    blocked: string;
    failed: string;
  };
  driveWriteTool: {
    title: string;
    locationPlaceholder: string;
    summaryPlaceholder: string;
    requestWrite: string;
    requestingWrite: string;
    pendingHint: string;
    completed: string;
    blocked: string;
    failed: string;
  };
  inspector: {
    title: string;
    largeModel: string;
    model: string;
    access: string;
    thinking: string;
    scope: string;
    theme: string;
  };
};

export const translations: Record<Language, TranslationSet> = {
  zh: {
    brandTagline: "本地优先 Agent OS",
    navLabel: "主导航",
    nav: {
      workbench: "工作台",
      memory: "记忆",
      approvals: "审批",
    },
    controls: {
      modelRoute: "模型路线",
      largeModelProvider: "大模型",
      accessMode: "访问权限",
      thinkingLevel: "思考强度",
      themeStyle: "界面风格",
      language: "界面语言",
      networkSearchSourceModel: "NetworkSearch 来源模型",
    },
    largeModelOptions: {
      deepseek: "DeepSeek",
      chatgpt: "ChatGPT",
      codex: "Codex",
      custom: "自定义模型",
    },
    modelOptions: {
      auto: "DeepSeek 自动",
      flash: "DeepSeek 快速",
      pro: "DeepSeek 专业",
    },
    accessOptions: {
      ask_every_step: "每步询问",
      ask_on_risk: "风险时询问",
      limited_auto: "有限自动",
      full_access: "完全访问",
    },
    thinkingOptions: {
      auto: "自动思考",
      fast: "快速",
      standard: "标准",
      deep: "深入",
    },
    scopeOptions: {
      workspace: "工作区",
    },
    themeOptions: {
      deep: "深色默认",
      ink: "水墨山水",
      porcelain: "青花瓷",
    },
    networkSearchSourceOptions: {
      free_web_source: "免费 Web 来源模型",
      free_local_browser: "免费本地浏览器搜索（alpha）",
      free_source_aggregator: "免费来源聚合器（alpha）",
    },
    runtimePlatformOptions: {
      windows: "Windows",
      macos: "macOS",
      other: "其它平台",
    },
    codexBridgeTransportOptions: {
      http: "外部 HTTP bridge",
      stdio: "stdio sidecar（暂缓）",
    },
    backendOptions: {
      network_search: {
        deepseek: "DeepSeek 搜索编排",
        native_large_model: "大模型原生 NetworkSearch",
        source_backed_model: "来源支撑 NetworkSearch 模型",
      },
      network_search_execution: {
        permission_audit_only: "仅权限与审计",
        source_backed_adapter: "来源适配器执行",
        native_bridge_contract: "原生桥接执行",
      },
      network_search_evidence: {
        pending_user_confirmation: "待确认",
        source_links_required: "必须保留来源链接",
      },
      email: {
        architecture_only: "仅保留架构",
      },
      drive: {
        local_folder_export_package: "本地文件夹与导出包",
      },
      computer_screenshot: {
        codex_style_screen_capture: "Codex 式屏幕像素读取",
        codex_bridge_screen_capture: "Codex Bridge 屏幕像素读取",
        local_windows_screen_capture: "本地 Windows 屏幕像素读取",
        local_macos_screen_capture: "本地 macOS 屏幕像素读取",
      },
      computer_control: {
        codex_style_input_control: "Codex 式鼠标键盘控制",
        codex_bridge_input_control: "Codex Bridge 鼠标键盘控制",
        local_windows_input_control: "本地 Windows 鼠标键盘控制",
        local_macos_input_control: "本地 macOS 鼠标键盘控制",
      },
    },
    backendLabels: {
      title: "工具后端策略",
      largeModelProvider: "大模型",
      networkSearch: "联网搜索",
      networkSearchSupport: "搜索支持",
      networkSearchSourceModel: "搜索来源模型",
      networkSearchRoute: "搜索路线",
      networkSearchExecution: "搜索执行",
      networkSearchEvidence: "证据策略",
      networkRequests: "真实联网",
      deepSeekOrchestration: "DeepSeek 编排",
      confirmationGate: "确认门",
      email: "邮件",
      drive: "Drive",
      computerScreenshot: "屏幕读取",
      computerControl: "电脑控制",
      deepSeekApi: "DeepSeek API",
      deepSeekChatApi: "DeepSeek Chat API",
      deepSeekTelemetry: "DeepSeek 遥测",
      apiBaseUrl: "API 地址",
      chatEndpoint: "Chat 接口",
      deepSeekModels: "DeepSeek 模型",
      apiKeyEnv: "Key 环境变量",
      apiKeyConfigured: "已配置",
      apiKeyMissing: "未配置",
      chatReady: "已就绪",
      chatNotReady: "未就绪",
      enabled: "已启用",
      disabled: "未启用",
      confirmationRequired: "需确认",
      confirmationNotRequired: "无需确认",
      screenshotBackendStatus: "屏幕后端状态",
      screenshotPermission: "屏幕权限",
      controlBackendStatus: "控制后端状态",
      controlPermission: "控制权限",
      codexBridgeRuntime: "Codex bridge 运行时",
      backendAvailable: "可用",
      backendUnavailable: "未接通",
      approvalRequired: "需审批",
      osPermissionRequired: "需系统授权",
      osPermissionNotRequired: "无需显式系统授权",
      bridgeRequired: "需 bridge",
      bridgeNotRequired: "无需 bridge",
      bridgeEndpointConfigured: "Endpoint 已配置",
      bridgeEndpointMissing: "Endpoint 未配置",
      bridgeConnected: "已连接",
      bridgeNotConnected: "未连接",
      bridgeTransportMissing: "Transport 未选择",
      nativeSupported: "原生支持",
      sourceModelRequired: "需来源模型",
      notSelected: "未选择",
      noTelemetry: "暂无调用",
      cacheHit: "缓存命中",
      cacheMiss: "缓存未命中",
      cacheDisabled: "缓存关闭",
      cacheEntries: "缓存条目",
      clearCache: "清空缓存",
      clearingCache: "清空中",
      cacheCleared: (count) => `已清空 ${count} 条缓存。`,
      cacheClearFailed: "DeepSeek 缓存清空失败。",
      tokens: "tokens",
      cost: "费用",
      runtimePlatform: "运行平台",
      macosPath: "macOS 路径",
    },
    capabilityFamilyOptions: {
      file: "文件",
      network: "联网",
      browser: "浏览器",
      email: "邮件",
      drive: "网盘",
      terminal: "终端",
      computer_use: "电脑控制",
    },
    capabilityOptions: {
      file_read: "读取文件",
      file_write: "写入文件",
      network_search: "联网搜索",
      browser_browse: "浏览网页",
      browser_submit: "提交网页",
      email_read: "读取邮件",
      email_draft: "起草邮件",
      email_send: "发送邮件",
      drive_read: "读取网盘",
      drive_write: "写入网盘",
      terminal_read: "读取终端",
      terminal_write: "写入终端",
      computer_screenshot: "屏幕截图",
      computer_control: "控制电脑",
    },
    capabilitySummaries: {
      file_read: "读取工作区内经授权的本地文件。",
      file_write: "在授权范围内创建或修改草稿与导出物。",
      network_search: "联网检索公开资料并保留来源。",
      browser_browse: "打开并检查网页内容。",
      browser_submit: "填写或提交网页表单，默认需要审批。",
      email_read: "读取选定邮件线程作为任务证据。",
      email_draft: "生成邮件草稿，不直接发送。",
      email_send: "发送已审批的外发邮件。",
      drive_read: "读取选定网盘文件与文件夹。",
      drive_write: "上传或导出成果到网盘。",
      terminal_read: "运行只读诊断命令并收集输出。",
      terminal_write: "运行可能修改文件或系统状态的命令。",
      computer_screenshot: "截取或检查当前屏幕。",
      computer_control: "执行鼠标和键盘动作，每步审批。",
    },
    riskOptions: {
      low: "低风险",
      medium: "中风险",
      high: "高风险",
      critical: "关键风险",
    },
    decisionOptions: {
      allow: "允许",
      ask: "询问",
      deny: "拒绝",
    },
    accessStatusOptions: {
      auto_approved: "自动通过",
      pending_approval: "待审批",
      approved: "已批准",
      rejected: "已拒绝",
      denied: "已拦截",
    },
    accessGrantOptions: {
      not_granted: "未授权",
      reusable: "可复用",
      one_shot_available: "一次可用",
      one_shot_consumed: "已消耗",
    },
    invocationStatusOptions: {
      succeeded: "成功",
      pending_approval: "待审批",
      failed: "失败",
    },
    workbench: {
      stage: "基础 MVP",
      title: "运营简报工作台",
      summary:
        "第一版已打通桌面工作台、权限控制、DeepSeek 路由默认值与本地内核边界。",
    },
    localSetup: {
      title: "本地目录设置",
      required: "首次运行需要设置本机工作目录。",
      ready: "本地目录已设置。",
      appData: "应用数据目录",
      settingsFile: "目录配置",
      workspaceDir: "默认工作区",
      evidenceDir: "默认证据文件夹",
      exportDir: "默认导出文件夹",
      workspacePlaceholder: "输入你本机的默认工作区路径",
      evidencePlaceholder: "输入你本机的默认证据文件夹路径",
      exportPlaceholder: "输入你本机的默认导出文件夹路径",
      choose: "选择",
      chooseFailed: "打开目录选择器失败。",
      workspaceDialogTitle: "选择默认工作区",
      evidenceDialogTitle: "选择默认证据文件夹",
      exportDialogTitle: "选择默认导出文件夹",
      save: "保存目录",
      saving: "保存中",
      saved: "本地目录设置已保存。",
      failed: "本地目录设置保存失败。",
      loadFailed: "本地目录设置加载失败。",
    },
    deepSeekPricing: {
      title: "DeepSeek 价格表",
      enabled: "启用成本估算",
      disabled: "未启用成本估算",
      statusConfigured: "已配置本地价格表",
      statusNotConfigured: "未配置本地价格表",
      help: "按 USD / 1M tokens 填写。价格保存在本机应用数据目录，不写死到开源代码。",
      settingsFile: "价格配置",
      flashPrompt: "Flash 输入",
      flashCompletion: "Flash 输出",
      proPrompt: "Pro 输入",
      proCompletion: "Pro 输出",
      pricePlaceholder: "例如 0.14",
      save: "保存价格表",
      saving: "保存中",
      saved: "DeepSeek 价格表已保存。",
      failed: "DeepSeek 价格表保存失败。",
      loadFailed: "DeepSeek 价格表加载失败。",
    },
    operationsBriefing: {
      title: "运营简报工作流",
      folderPlaceholder: "输入你本机的证据文件夹路径",
      run: "运行",
      running: "运行中",
      seedTemplates: "写入模板",
      seededTemplates: "样例证据模板已写入本地证据文件夹。",
      seedPendingHint: "已创建 FileWrite 待审批请求，批准后再次写入即可生成模板。",
      seedFailed: "样例证据模板写入失败。",
      exportPackage: "导出简报包",
      exportReport: "导出报告",
      exportHtmlReport: "导出 HTML",
      exportPdfReport: "导出 PDF",
      exported: "简报工作包已生成。",
      reportExported: "运营简报 Markdown 报告已导出到本地导出文件夹。",
      reportPendingHint: "已创建 DriveWrite 待审批请求，批准后再次导出即可写入报告。",
      reportExportFailed: "运营简报报告导出失败。",
      htmlReportExported: "运营简报 HTML 报告已导出到本地导出文件夹。",
      htmlReportPendingHint: "已创建 DriveWrite 待审批请求，批准后再次导出即可写入 HTML 报告。",
      htmlReportExportFailed: "运营简报 HTML 报告导出失败。",
      pdfReportExported: "运营简报 PDF 报告已导出到本地导出文件夹。",
      pdfReportPendingHint: "已创建 DriveWrite 待审批请求，批准后再次导出即可写入 PDF 报告。",
      pdfReportExportFailed: "运营简报 PDF 报告导出失败。",
      latestRun: "最近运行",
      noRuns: "暂无运营简报运行记录",
      pendingHint: "已创建待审批请求，批准后再次运行即可执行。",
      failed: "运营简报运行失败。",
      loadFailed: "运营简报运行记录加载失败。",
      anomalies: "异常线索",
      actions: "行动项",
      noAnomalies: "暂无异常线索",
      noActions: "暂无行动项",
      evidence: "证据",
      archived: "归档",
      status: {
        pending_approval: "待审批",
        draft_ready: "草稿就绪",
        failed: "失败",
      },
    },
    package: {
      title: "任务记录与工作包",
      taskTitle: "任务标题",
      taskSummary: "任务摘要",
      addRecord: "记录任务",
      exportPackage: "导出工作包",
      copyPackage: "复制",
      importPackage: "导入",
      previewImport: "预览",
      previewing: "预览中",
      previewTitle: "导入预览",
      previewReady: "工作包预览已生成。",
      previewFailed: "工作包预览失败。",
      previewTotalTasks: "任务总数",
      previewNewTasks: "新增任务",
      previewSkippedTasks: "跳过任务",
      previewMemoryCandidates: "候选记忆",
      previewMemoryCandidateHint: "候选记忆将进入本地确认队列，不会直接写入长期记忆。",
      previewArchivedRuns: "简报归档",
      previewArchiveHint: "简报运行将作为只读归档导入，不会重新执行工具。",
      previewWorkflowTemplates: "工作流模板",
      previewNewWorkflowTemplates: "新增模板",
      previewSkippedWorkflowTemplates: "跳过模板",
      previewWorkflowTemplateHint: "工作流模板包将登记为本地可用资产，不会写入用户目录。",
      packageJson: "工作包 JSON",
      importJson: "导入 JSON",
      emptyTitle: "请先填写任务标题。",
      emptyImport: "请先粘贴工作包 JSON。",
      created: "任务记录已写入本地事件库。",
      exported: "工作包已生成。",
      copied: "工作包 JSON 已复制。",
      imported: (
        imported,
        skipped,
        candidateImported,
        candidateSkipped,
        briefingImported,
        briefingSkipped,
        templateImported,
        templateSkipped,
      ) =>
        `导入完成：任务新增 ${imported} 条，跳过 ${skipped} 条；候选记忆新增 ${candidateImported} 条，跳过 ${candidateSkipped} 条；简报归档新增 ${briefingImported} 条，跳过 ${briefingSkipped} 条；工作流模板新增 ${templateImported} 个，跳过 ${templateSkipped} 个。`,
      noRecords: "暂无任务记录",
      copyFailed: "复制失败，请手动选择 JSON。",
      loadFailed: "任务记录加载失败。",
    },
    memory: {
      title: "自动记忆",
      autoCapture: "由任务记录自动沉淀",
      noMemories: "暂无自动记忆",
      loadFailed: "记忆加载失败。",
      search: "搜索",
      searchPlaceholder: "搜索记忆",
      candidateTitle: "候选记忆标题",
      candidateBody: "候选记忆内容",
      candidateType: "类型",
      candidateScope: "范围",
      candidateSensitivity: "敏感度",
      candidateLifecycle: "生命周期",
      expiresAt: "过期日期",
      metadata: "记忆元数据",
      propose: "提议记忆",
      proposing: "提议中",
      candidates: "候选记忆",
      noCandidates: "暂无候选记忆",
      accept: "接受",
      reject: "拒绝",
      edit: "编辑",
      editTitle: "记忆标题",
      editBody: "记忆内容",
      save: "保存",
      saving: "保存中",
      cancel: "取消",
      delete: "删除",
      deleting: "删除中",
      resolving: "处理中",
      proposed: "候选记忆已提交，等待确认。",
      accepted: "候选记忆已接受并写入长期记忆。",
      rejected: "候选记忆已拒绝。",
      updated: "长期记忆已更新。",
      deleted: "长期记忆已删除。",
      emptyCandidate: "请填写候选记忆标题和内容。",
      emptyEdit: "请填写记忆标题和内容。",
      emptyExpiration: "请选择过期日期。",
      proposeFailed: "候选记忆提交失败。",
      resolveFailed: "候选记忆处理失败。",
      updateFailed: "长期记忆更新失败。",
      deleteFailed: "长期记忆删除失败。",
      conflictWarning: (count) => `可能与 ${count} 条长期记忆重叠`,
      conflictDetails: "重叠记忆",
      previewMerge: "预览合并",
      previewingMerge: "生成中",
      mergePreviewTitle: "合并草稿",
      mergePreviewDraft: "草稿内容",
      mergePreviewFailed: "合并草稿生成失败。",
      mergeAndAccept: "合并并接受",
      merged: "候选记忆已合并，旧记忆已归档。",
      mergeFailed: "候选记忆合并失败。",
      previewReplace: "预览替换",
      previewingReplace: "生成中",
      replacePreviewTitle: "替换预览",
      replacePreviewDraft: "替换草稿",
      replacePreviewTargets: "将被替换",
      replacePreviewFailed: "替换预览生成失败。",
      replaceAndAccept: "替换并接受",
      replaced: "候选记忆已接受，被替换记忆已归档。",
      replaceFailed: "候选记忆替换失败。",
      linkAndAccept: "关联并接受",
      linked: "候选记忆已接受，并与重叠记忆建立关联。",
      linkFailed: "候选记忆关联失败。",
      linkedMemories: (count) => `关联 ${count} 条记忆`,
      updatedAt: "更新于",
      candidateStatus: {
        pending: "待确认",
        accepted: "已接受",
        rejected: "已拒绝",
      },
      typeOptions: {
        preference: "偏好",
        project_context: "项目上下文",
        workflow_rule: "工作流规则",
        artifact: "成果物",
        failure_pattern: "失败模式",
      },
      scopeOptions: {
        workspace: "工作区",
        project: "项目",
        organization: "组织",
        user: "用户",
      },
      sensitivityOptions: {
        normal: "普通",
        sensitive: "敏感",
      },
      lifecycleOptions: {
        active: "启用",
        archived: "归档",
        expires: "会过期",
      },
    },
    audit: {
      title: "权限预检",
      browser: "浏览器",
      emailSend: "发邮件",
      computerControl: "控电脑",
      empty: "暂无权限审计",
      loadFailed: "权限审计加载失败。",
      pending: "检查中",
    },
    capabilities: {
      title: "工具能力与权限闭环",
      request: "请求",
      requesting: "请求中",
      experimental: "实验",
      pendingTitle: "待处理审批",
      noPending: "暂无待审批请求",
      approve: "批准",
      reject: "拒绝",
      resolving: "处理中",
      loadFailed: "工具能力加载失败。",
      requestFailed: "权限请求失败。",
      resolveFailed: "审批处理失败。",
      auditTitle: "最近审计",
    },
    browserTool: {
      title: "浏览器工具",
      urlPlaceholder: "https://example.com/report",
      browse: "浏览",
      browsing: "浏览中",
      outputTitle: "工具输出",
      noOutput: "暂无工具输出",
      approvalRequest: "审批",
      pendingHint: "已创建待审批请求，批准后再次浏览即可执行。",
      failed: "浏览失败。",
    },
    browserSubmitTool: {
      title: "浏览器提交边界",
      urlPlaceholder: "目标表单网址",
      summaryPlaceholder: "提交动作说明",
      requestSubmit: "请求提交",
      requestingSubmit: "请求中",
      pendingHint: "已创建 BrowserSubmit 待审批请求；v1 不会提交表单。",
      blocked: "BrowserSubmit v1 仅记录审批与审计，未提交任何表单。",
      failed: "BrowserSubmit 请求失败。",
    },
    networkSearchTool: {
      title: "网络搜索边界",
      queryPlaceholder: "搜索关键词",
      scopePlaceholder: "来源范围，例如：公开网页",
      requestSearch: "请求搜索",
      requestingSearch: "请求中",
      pendingHint: "已创建 NetworkSearch 待审批请求；批准前不会访问网络。",
      completed: "NetworkSearch 已执行，并记录了来源链接。",
      blocked: "NetworkSearch 未完成。请查看最近工具输出中的失败原因。",
      failed: "NetworkSearch 请求失败。",
      sourceModelRequiredTitle: "需要 NetworkSearch 来源模型",
      sourceModelRequiredBody:
        "当前开源 alpha 使用免费来源适配器执行真实搜索，请先选择来源模型以保留可审计链接；本地浏览器和聚合器预设目前共用来源型 HTTP 适配器。",
      sourceModelPlaceholder: "选择免费来源模型",
      sourceModelMissing: "请先选择 NetworkSearch 来源模型。",
      routeNotEnabled: "当前 NetworkSearch 路线尚未启用真实搜索。",
    },
    fileTool: {
      title: "文件工具",
      pathPlaceholder: "输入你本机的文件路径",
      read: "读取",
      reading: "读取中",
      pendingHint: "已创建待审批请求，批准后再次读取即可执行。",
      failed: "读取失败。",
    },
    fileWriteTool: {
      title: "文件写入边界",
      pathPlaceholder: "目标文件路径",
      summaryPlaceholder: "写入或修改说明",
      contentPlaceholder: "写入内容",
      requestWrite: "请求写入",
      requestingWrite: "请求中",
      pendingHint: "已创建 FileWrite 待审批请求；批准后再次提交即可写入。",
      completed: "文件已写入本地工作区。",
      blocked: "FileWrite 未执行。",
      failed: "FileWrite 请求失败。",
    },
    folderTool: {
      title: "证据文件夹",
      pathPlaceholder: "输入你本机的证据文件夹路径",
      ingest: "导入",
      ingesting: "导入中",
      pendingHint: "已创建待审批请求，批准后再次导入即可执行。",
      failed: "证据文件夹导入失败。",
    },
    terminalTool: {
      title: "终端只读工具",
      commandLabel: "只读诊断命令",
      run: "运行",
      running: "运行中",
      pendingHint: "已创建待审批请求，批准后再次运行即可执行。",
      failed: "终端只读命令失败。",
      writeTitle: "终端写入边界",
      writeCommandLabel: "待审批写入命令",
      writePlaceholder: "例如：npm install",
      requestWrite: "请求",
      requestingWrite: "请求中",
      writePendingHint: "已创建 TerminalWrite 待审批请求；v1 不会直接执行写命令。",
      writeBlocked: "TerminalWrite v1 仅记录审批与审计，未执行命令。",
      writeFailed: "TerminalWrite 请求失败。",
      options: {
        pwd: "当前目录",
        "git status --short": "Git 状态",
        "git diff --stat": "Git 变更统计",
        "git branch --show-current": "当前分支",
      },
    },
    computerTool: {
      title: "屏幕截图边界",
      capture: "检查屏幕",
      capturing: "检查中",
      pendingHint: "已创建屏幕截图待审批请求，批准后再次检查即可执行。",
      captured: "屏幕截图已保存为本地证据文件。",
      unavailable: "屏幕截图未完成；请检查系统截屏权限或显示器可用性。",
      failed: "屏幕截图请求失败。",
    },
    computerControlTool: {
      title: "桌面控制边界",
      unlockTitle: "桌面控制本地解锁",
      unlockChallengeLabel: "本机码",
      unlockTokenPlaceholder: "输入本机码",
      unlockControl: "解锁",
      unlockingControl: "解锁中",
      unlockReady: "本地控制已短时解锁。",
      unlockRequired: "执行前需要本机短时解锁。",
      unlockExpires: "有效至",
      unlockFailed: "桌面控制解锁失败。",
      targetPlaceholder: "目标窗口、页面或控件",
      actionPlaceholder: "click:120,340 或 hotkey:ctrl+shift+p",
      requestControl: "请求控制",
      requestingControl: "请求中",
      pendingHint: "已创建 ComputerControl 待审批请求；批准后重试将执行结构化动作。",
      executed: "ComputerControl 已执行，并已记录审批与审计。",
      blocked: "ComputerControl 未执行；请检查动作格式、系统权限或本地输入后端。",
      failed: "ComputerControl 请求失败。",
    },
    emailTool: {
      title: "邮件发送边界",
      toPlaceholder: "收件人",
      subjectPlaceholder: "主题",
      bodyPlaceholder: "正文",
      requestSend: "请求发送",
      requestingSend: "请求中",
      pendingHint: "已创建 EmailSend 待审批请求；批准后可重试一次，v1 不会直接发送邮件。",
      blocked: "EmailSend v1 仅记录审批与审计，未发送邮件。",
      failed: "EmailSend 请求失败。",
    },
    emailDraftTool: {
      title: "邮件草稿边界",
      toPlaceholder: "草稿收件人",
      subjectPlaceholder: "草稿主题",
      bodyPlaceholder: "草稿正文",
      requestDraft: "请求草稿",
      requestingDraft: "请求中",
      pendingHint: "已创建 EmailDraft 待审批请求；v1 不会创建真实邮箱草稿。",
      blocked: "EmailDraft v1 仅记录审批与审计，未创建邮箱草稿。",
      failed: "EmailDraft 请求失败。",
    },
    emailReadTool: {
      title: "邮件读取边界",
      mailboxPlaceholder: "邮箱或文件夹",
      queryPlaceholder: "读取条件或线索",
      requestRead: "请求读取",
      requestingRead: "请求中",
      pendingHint: "已创建 EmailRead 待审批请求；v1 不会读取真实邮箱。",
      blocked: "EmailRead v1 仅记录审批与审计，未读取邮箱。",
      failed: "EmailRead 请求失败。",
    },
    driveReadTool: {
      title: "Drive 本地文件夹读取",
      locationPlaceholder: "本地文件夹路径",
      queryPlaceholder: "文件名或内容关键词",
      requestRead: "请求读取",
      requestingRead: "请求中",
      pendingHint: "已创建 DriveRead 待审批请求；批准后会读取本地文件夹。",
      completed: "DriveRead 已读取本地文件夹并记录结果。",
      blocked: "DriveRead 未完成；请查看最近工具输出中的失败原因。",
      failed: "DriveRead 请求失败。",
    },
    driveWriteTool: {
      title: "Drive 导出工作包",
      locationPlaceholder: "本地导出文件夹路径",
      summaryPlaceholder: "导出工作包说明",
      requestWrite: "导出工作包",
      requestingWrite: "请求中",
      pendingHint: "已创建 DriveWrite 待审批请求；批准后会导出当前工作包 JSON。",
      completed: "DriveWrite 已将当前工作包 JSON 导出到本地文件夹。",
      blocked: "DriveWrite 未完成；请查看最近工具输出中的失败原因。",
      failed: "DriveWrite 请求失败。",
    },
    inspector: {
      title: "运行控制",
      largeModel: "大模型",
      model: "模型",
      access: "权限",
      thinking: "思考",
      scope: "范围",
      theme: "风格",
    },
  },
  en: {
    brandTagline: "Local-first Agent OS",
    navLabel: "Primary",
    nav: {
      workbench: "Workbench",
      memory: "Memory",
      approvals: "Approvals",
    },
    controls: {
      modelRoute: "Model route",
      largeModelProvider: "Large model",
      accessMode: "Access mode",
      thinkingLevel: "Thinking level",
      themeStyle: "Interface style",
      language: "Interface language",
      networkSearchSourceModel: "NetworkSearch source model",
    },
    largeModelOptions: {
      deepseek: "DeepSeek",
      chatgpt: "ChatGPT",
      codex: "Codex",
      custom: "Custom model",
    },
    modelOptions: {
      auto: "DeepSeek Auto",
      flash: "DeepSeek Flash",
      pro: "DeepSeek Pro",
    },
    accessOptions: {
      ask_every_step: "Every step asks",
      ask_on_risk: "Ask on risk",
      limited_auto: "Limited auto",
      full_access: "Full access",
    },
    thinkingOptions: {
      auto: "Thinking auto",
      fast: "Fast",
      standard: "Standard",
      deep: "Deep",
    },
    scopeOptions: {
      workspace: "Workspace",
    },
    themeOptions: {
      deep: "Deep default",
      ink: "Ink landscape",
      porcelain: "Blue porcelain",
    },
    networkSearchSourceOptions: {
      free_web_source: "Free web source model",
      free_local_browser: "Free local browser search (alpha)",
      free_source_aggregator: "Free source aggregator (alpha)",
    },
    runtimePlatformOptions: {
      windows: "Windows",
      macos: "macOS",
      other: "Other platform",
    },
    codexBridgeTransportOptions: {
      http: "External HTTP bridge",
      stdio: "stdio sidecar (deferred)",
    },
    backendOptions: {
      network_search: {
        deepseek: "DeepSeek search orchestration",
        native_large_model: "Native large-model NetworkSearch",
        source_backed_model: "Source-backed NetworkSearch model",
      },
      network_search_execution: {
        permission_audit_only: "Permission and audit only",
        source_backed_adapter: "Source adapter execution",
        native_bridge_contract: "Native bridge contract",
      },
      network_search_evidence: {
        pending_user_confirmation: "Pending confirmation",
        source_links_required: "Source links required",
      },
      email: {
        architecture_only: "Architecture only",
      },
      drive: {
        local_folder_export_package: "Local folders and export packages",
      },
      computer_screenshot: {
        codex_style_screen_capture: "Codex-style screen pixel capture",
        codex_bridge_screen_capture: "Codex Bridge screen pixel capture",
        local_windows_screen_capture: "Local Windows screen pixel capture",
        local_macos_screen_capture: "Local macOS screen pixel capture",
      },
      computer_control: {
        codex_style_input_control: "Codex-style mouse and keyboard control",
        codex_bridge_input_control: "Codex Bridge mouse and keyboard control",
        local_windows_input_control: "Local Windows mouse and keyboard control",
        local_macos_input_control: "Local macOS mouse and keyboard control",
      },
    },
    backendLabels: {
      title: "Tool Backend Strategy",
      largeModelProvider: "Large model",
      networkSearch: "Network search",
      networkSearchSupport: "Search support",
      networkSearchSourceModel: "Search source model",
      networkSearchRoute: "Search route",
      networkSearchExecution: "Search execution",
      networkSearchEvidence: "Evidence policy",
      networkRequests: "Network requests",
      deepSeekOrchestration: "DeepSeek orchestration",
      confirmationGate: "Confirmation gate",
      email: "Email",
      drive: "Drive",
      computerScreenshot: "Screen read",
      computerControl: "Computer control",
      deepSeekApi: "DeepSeek API",
      deepSeekChatApi: "DeepSeek Chat API",
      deepSeekTelemetry: "DeepSeek telemetry",
      apiBaseUrl: "API base",
      chatEndpoint: "Chat endpoint",
      deepSeekModels: "DeepSeek models",
      apiKeyEnv: "Key environment",
      apiKeyConfigured: "Configured",
      apiKeyMissing: "Missing",
      chatReady: "Ready",
      chatNotReady: "Not ready",
      enabled: "Enabled",
      disabled: "Disabled",
      confirmationRequired: "Required",
      confirmationNotRequired: "Not required",
      screenshotBackendStatus: "Screen backend status",
      screenshotPermission: "Screen permission",
      controlBackendStatus: "Control backend status",
      controlPermission: "Control permission",
      codexBridgeRuntime: "Codex bridge runtime",
      backendAvailable: "Available",
      backendUnavailable: "Not connected",
      approvalRequired: "Approval required",
      osPermissionRequired: "OS permission required",
      osPermissionNotRequired: "No explicit OS permission",
      bridgeRequired: "Bridge required",
      bridgeNotRequired: "Bridge not required",
      bridgeEndpointConfigured: "Endpoint configured",
      bridgeEndpointMissing: "Endpoint missing",
      bridgeConnected: "Connected",
      bridgeNotConnected: "Not connected",
      bridgeTransportMissing: "Transport missing",
      nativeSupported: "Native",
      sourceModelRequired: "Source model required",
      notSelected: "Not selected",
      noTelemetry: "No calls yet",
      cacheHit: "Cache hit",
      cacheMiss: "Cache miss",
      cacheDisabled: "Cache disabled",
      cacheEntries: "Cache entries",
      clearCache: "Clear cache",
      clearingCache: "Clearing",
      cacheCleared: (count) => `Cleared ${count} cache ${count === 1 ? "entry" : "entries"}.`,
      cacheClearFailed: "DeepSeek cache clear failed.",
      tokens: "tokens",
      cost: "Cost",
      runtimePlatform: "Runtime platform",
      macosPath: "macOS path",
    },
    capabilityFamilyOptions: {
      file: "Files",
      network: "Network",
      browser: "Browser",
      email: "Email",
      drive: "Drive",
      terminal: "Terminal",
      computer_use: "Computer Use",
    },
    capabilityOptions: {
      file_read: "Read files",
      file_write: "Write files",
      network_search: "Network search",
      browser_browse: "Browse web",
      browser_submit: "Submit web",
      email_read: "Read email",
      email_draft: "Draft email",
      email_send: "Send email",
      drive_read: "Read drive",
      drive_write: "Write drive",
      terminal_read: "Read terminal",
      terminal_write: "Write terminal",
      computer_screenshot: "Screenshot",
      computer_control: "Control computer",
    },
    capabilitySummaries: {
      file_read: "Read approved local files in the workspace.",
      file_write: "Create or modify drafts and exported artifacts.",
      network_search: "Search public sources and preserve citations.",
      browser_browse: "Open and inspect web pages.",
      browser_submit: "Fill or submit web forms with approval.",
      email_read: "Read selected email threads for task evidence.",
      email_draft: "Prepare drafts without sending.",
      email_send: "Send approved outbound email.",
      drive_read: "Read selected cloud-drive files and folders.",
      drive_write: "Upload or export artifacts to cloud drive.",
      terminal_read: "Run read-only diagnostics and collect output.",
      terminal_write: "Run commands that can change files or system state.",
      computer_screenshot: "Capture or inspect the visible desktop.",
      computer_control: "Use mouse and keyboard actions with per-step approval.",
    },
    riskOptions: {
      low: "Low risk",
      medium: "Medium risk",
      high: "High risk",
      critical: "Critical risk",
    },
    decisionOptions: {
      allow: "Allow",
      ask: "Ask",
      deny: "Deny",
    },
    accessStatusOptions: {
      auto_approved: "Auto approved",
      pending_approval: "Pending",
      approved: "Approved",
      rejected: "Rejected",
      denied: "Blocked",
    },
    accessGrantOptions: {
      not_granted: "Not granted",
      reusable: "Reusable",
      one_shot_available: "One-shot available",
      one_shot_consumed: "Consumed",
    },
    invocationStatusOptions: {
      succeeded: "Succeeded",
      pending_approval: "Pending",
      failed: "Failed",
    },
    workbench: {
      stage: "Foundation MVP",
      title: "Operations Briefing Workbench",
      summary:
        "The first runnable slice proves the desktop shell, policy controls, DeepSeek routing defaults, and local kernel boundary.",
    },
    localSetup: {
      title: "Local Directory Setup",
      required: "First run needs local working directories.",
      ready: "Local directories are configured.",
      appData: "App data directory",
      settingsFile: "Directory settings",
      workspaceDir: "Default workspace",
      evidenceDir: "Default evidence folder",
      exportDir: "Default export folder",
      workspacePlaceholder: "Enter a default local workspace path",
      evidencePlaceholder: "Enter a default local evidence folder path",
      exportPlaceholder: "Enter a default local export folder path",
      choose: "Choose",
      chooseFailed: "Folder picker failed to open.",
      workspaceDialogTitle: "Choose default workspace",
      evidenceDialogTitle: "Choose default evidence folder",
      exportDialogTitle: "Choose default export folder",
      save: "Save directories",
      saving: "Saving",
      saved: "Local directory settings saved.",
      failed: "Local directory setup failed.",
      loadFailed: "Local directory settings failed to load.",
    },
    deepSeekPricing: {
      title: "DeepSeek Pricing",
      enabled: "Enable cost estimates",
      disabled: "Cost estimates disabled",
      statusConfigured: "Local pricing configured",
      statusNotConfigured: "Local pricing not configured",
      help: "Enter USD / 1M tokens. Prices are stored in local app data, not hardcoded into the open-source project.",
      settingsFile: "Pricing settings",
      flashPrompt: "Flash input",
      flashCompletion: "Flash output",
      proPrompt: "Pro input",
      proCompletion: "Pro output",
      pricePlaceholder: "e.g. 0.14",
      save: "Save pricing",
      saving: "Saving",
      saved: "DeepSeek pricing saved.",
      failed: "DeepSeek pricing save failed.",
      loadFailed: "DeepSeek pricing failed to load.",
    },
    operationsBriefing: {
      title: "Operations Briefing Workflow",
      folderPlaceholder: "Enter a local evidence folder path",
      run: "Run",
      running: "Running",
      seedTemplates: "Seed templates",
      seededTemplates: "Sample evidence templates seeded into the local evidence folder.",
      seedPendingHint: "A FileWrite approval request was created. Approve it, then seed again.",
      seedFailed: "Evidence template seeding failed.",
      exportPackage: "Export brief package",
      exportReport: "Export report",
      exportHtmlReport: "Export HTML",
      exportPdfReport: "Export PDF",
      exported: "Briefing work package generated.",
      reportExported: "Operations briefing Markdown report exported to the local export folder.",
      reportPendingHint:
        "A DriveWrite approval request was created. Approve it, then export again.",
      reportExportFailed: "Operations briefing report export failed.",
      htmlReportExported: "Operations briefing HTML report exported to the local export folder.",
      htmlReportPendingHint:
        "A DriveWrite approval request was created. Approve it, then export HTML again.",
      htmlReportExportFailed: "Operations briefing HTML report export failed.",
      pdfReportExported: "Operations briefing PDF report exported to the local export folder.",
      pdfReportPendingHint:
        "A DriveWrite approval request was created. Approve it, then export PDF again.",
      pdfReportExportFailed: "Operations briefing PDF report export failed.",
      latestRun: "Latest Run",
      noRuns: "No operations briefing runs yet",
      pendingHint: "A pending approval request was created. Approve it, then run again.",
      failed: "Operations briefing run failed.",
      loadFailed: "Operations briefing runs failed to load.",
      anomalies: "Anomaly Leads",
      actions: "Actions",
      noAnomalies: "No anomaly leads",
      noActions: "No actions",
      evidence: "Evidence",
      archived: "Archived",
      status: {
        pending_approval: "Pending",
        draft_ready: "Draft ready",
        failed: "Failed",
      },
    },
    package: {
      title: "Task Records and Work Packages",
      taskTitle: "Task title",
      taskSummary: "Task summary",
      addRecord: "Add record",
      exportPackage: "Export package",
      copyPackage: "Copy",
      importPackage: "Import",
      previewImport: "Preview",
      previewing: "Previewing",
      previewTitle: "Import Preview",
      previewReady: "Work package preview generated.",
      previewFailed: "Work package preview failed.",
      previewTotalTasks: "Total tasks",
      previewNewTasks: "New tasks",
      previewSkippedTasks: "Skipped tasks",
      previewMemoryCandidates: "Memory candidates",
      previewMemoryCandidateHint:
        "Memory candidates import into local review and do not write long-term memory.",
      previewArchivedRuns: "Brief archives",
      previewArchiveHint:
        "Briefing runs import as read-only archives and do not rerun tools.",
      previewWorkflowTemplates: "Workflow templates",
      previewNewWorkflowTemplates: "New templates",
      previewSkippedWorkflowTemplates: "Skipped templates",
      previewWorkflowTemplateHint:
        "Workflow template packages import as local available assets and do not write user folders.",
      packageJson: "Work package JSON",
      importJson: "Import JSON",
      emptyTitle: "Add a task title first.",
      emptyImport: "Paste work package JSON first.",
      created: "Task record saved to the local event store.",
      exported: "Work package generated.",
      copied: "Work package JSON copied.",
      imported: (
        imported,
        skipped,
        candidateImported,
        candidateSkipped,
        briefingImported,
        briefingSkipped,
        templateImported,
        templateSkipped,
      ) =>
        `Import complete: ${imported} tasks added, ${skipped} skipped; ${candidateImported} memory candidates added, ${candidateSkipped} skipped; ${briefingImported} brief archives added, ${briefingSkipped} skipped; ${templateImported} workflow templates added, ${templateSkipped} skipped.`,
      noRecords: "No task records yet",
      copyFailed: "Copy failed. Select the JSON manually.",
      loadFailed: "Task records failed to load.",
    },
    memory: {
      title: "Auto Memory",
      autoCapture: "Captured from task records",
      noMemories: "No auto memories yet",
      loadFailed: "Memories failed to load.",
      search: "Search",
      searchPlaceholder: "Search memories",
      candidateTitle: "Memory candidate title",
      candidateBody: "Memory candidate body",
      candidateType: "Type",
      candidateScope: "Scope",
      candidateSensitivity: "Sensitivity",
      candidateLifecycle: "Lifecycle",
      expiresAt: "Expiration date",
      metadata: "Memory metadata",
      propose: "Propose memory",
      proposing: "Proposing",
      candidates: "Memory Candidates",
      noCandidates: "No memory candidates",
      accept: "Accept",
      reject: "Reject",
      edit: "Edit",
      editTitle: "Memory title",
      editBody: "Memory body",
      save: "Save",
      saving: "Saving",
      cancel: "Cancel",
      delete: "Delete",
      deleting: "Deleting",
      resolving: "Resolving",
      proposed: "Memory candidate submitted for review.",
      accepted: "Memory candidate accepted and saved to long-term memory.",
      rejected: "Memory candidate rejected.",
      updated: "Long-term memory updated.",
      deleted: "Long-term memory deleted.",
      emptyCandidate: "Add a memory candidate title and body first.",
      emptyEdit: "Add a memory title and body first.",
      emptyExpiration: "Choose an expiration date.",
      proposeFailed: "Memory candidate proposal failed.",
      resolveFailed: "Memory candidate update failed.",
      updateFailed: "Long-term memory update failed.",
      deleteFailed: "Long-term memory delete failed.",
      conflictWarning: (count) =>
        `May overlap with ${count} long-term ${count === 1 ? "memory" : "memories"}`,
      conflictDetails: "Overlapping memories",
      previewMerge: "Preview merge",
      previewingMerge: "Previewing",
      mergePreviewTitle: "Merge draft",
      mergePreviewDraft: "Draft body",
      mergePreviewFailed: "Memory merge preview failed.",
      mergeAndAccept: "Merge and accept",
      merged: "Memory candidate merged and older memories archived.",
      mergeFailed: "Memory candidate merge failed.",
      previewReplace: "Preview replace",
      previewingReplace: "Previewing",
      replacePreviewTitle: "Replace preview",
      replacePreviewDraft: "Replacement draft",
      replacePreviewTargets: "Would replace",
      replacePreviewFailed: "Memory replace preview failed.",
      replaceAndAccept: "Replace and accept",
      replaced: "Memory candidate accepted and replaced memories archived.",
      replaceFailed: "Memory candidate replace failed.",
      linkAndAccept: "Link and accept",
      linked: "Memory candidate accepted and linked to overlapping memories.",
      linkFailed: "Memory candidate link failed.",
      linkedMemories: (count) => `Linked to ${count} ${count === 1 ? "memory" : "memories"}`,
      updatedAt: "Updated",
      candidateStatus: {
        pending: "Pending",
        accepted: "Accepted",
        rejected: "Rejected",
      },
      typeOptions: {
        preference: "Preference",
        project_context: "Project context",
        workflow_rule: "Workflow rule",
        artifact: "Artifact",
        failure_pattern: "Failure pattern",
      },
      scopeOptions: {
        workspace: "Workspace",
        project: "Project",
        organization: "Organization",
        user: "User",
      },
      sensitivityOptions: {
        normal: "Normal",
        sensitive: "Sensitive",
      },
      lifecycleOptions: {
        active: "Active",
        archived: "Archived",
        expires: "Expires",
      },
    },
    audit: {
      title: "Permission Check",
      browser: "Browser",
      emailSend: "Email",
      computerControl: "Computer",
      empty: "No permission audits yet",
      loadFailed: "Permission audits failed to load.",
      pending: "Checking",
    },
    capabilities: {
      title: "Tools and Permission Loop",
      request: "Request",
      requesting: "Requesting",
      experimental: "Experimental",
      pendingTitle: "Pending Approvals",
      noPending: "No pending approval requests",
      approve: "Approve",
      reject: "Reject",
      resolving: "Resolving",
      loadFailed: "Tool capabilities failed to load.",
      requestFailed: "Permission request failed.",
      resolveFailed: "Approval update failed.",
      auditTitle: "Recent Audit",
    },
    browserTool: {
      title: "Browser Tool",
      urlPlaceholder: "https://example.com/report",
      browse: "Browse",
      browsing: "Browsing",
      outputTitle: "Tool Output",
      noOutput: "No tool output yet",
      approvalRequest: "Approval",
      pendingHint: "A pending approval request was created. Approve it, then browse again.",
      failed: "Browse failed.",
    },
    browserSubmitTool: {
      title: "Browser Submit Boundary",
      urlPlaceholder: "Target form URL",
      summaryPlaceholder: "Submission action summary",
      requestSubmit: "Request submit",
      requestingSubmit: "Requesting",
      pendingHint:
        "A BrowserSubmit approval request was created. v1 will not submit any form.",
      blocked:
        "BrowserSubmit v1 only records approval and audit evidence. No form was submitted.",
      failed: "BrowserSubmit request failed.",
    },
    networkSearchTool: {
      title: "Network Search Boundary",
      queryPlaceholder: "Search query",
      scopePlaceholder: "Source scope, for example: public web",
      requestSearch: "Request search",
      requestingSearch: "Requesting",
      pendingHint:
        "A NetworkSearch approval request was created. It will not access the network before approval.",
      completed: "NetworkSearch ran and recorded source links.",
      blocked:
        "NetworkSearch did not complete. Check recent tool output for the failure reason.",
      failed: "NetworkSearch request failed.",
      sourceModelRequiredTitle: "NetworkSearch source model required",
      sourceModelRequiredBody:
        "This open-source alpha uses a free source adapter for live search. Choose a source model first so auditable links are preserved; local-browser and aggregator presets currently share the source-backed HTTP adapter.",
      sourceModelPlaceholder: "Choose a free source model",
      sourceModelMissing: "Choose a NetworkSearch source model first.",
      routeNotEnabled: "The current NetworkSearch route is not enabled for live search.",
    },
    fileTool: {
      title: "File Tool",
      pathPlaceholder: "Enter a local file path",
      read: "Read",
      reading: "Reading",
      pendingHint: "A pending approval request was created. Approve it, then read again.",
      failed: "File read failed.",
    },
    fileWriteTool: {
      title: "File Write Boundary",
      pathPlaceholder: "Target file path",
      summaryPlaceholder: "Write or change summary",
      contentPlaceholder: "File content",
      requestWrite: "Request write",
      requestingWrite: "Requesting",
      pendingHint: "A FileWrite approval request was created. Approve it, then submit again.",
      completed: "File written to the local workspace.",
      blocked: "FileWrite did not execute.",
      failed: "FileWrite request failed.",
    },
    folderTool: {
      title: "Evidence Folder",
      pathPlaceholder: "Enter a local evidence folder path",
      ingest: "Ingest",
      ingesting: "Ingesting",
      pendingHint: "A pending approval request was created. Approve it, then ingest again.",
      failed: "Evidence folder ingest failed.",
    },
    terminalTool: {
      title: "Terminal Read Tool",
      commandLabel: "Read-only diagnostic command",
      run: "Run",
      running: "Running",
      pendingHint: "A pending approval request was created. Approve it, then run again.",
      failed: "Terminal read command failed.",
      writeTitle: "Terminal Write Boundary",
      writeCommandLabel: "Write command for approval",
      writePlaceholder: "Example: npm install",
      requestWrite: "Request",
      requestingWrite: "Requesting",
      writePendingHint:
        "A TerminalWrite approval request was created. v1 will not execute write commands directly.",
      writeBlocked:
        "TerminalWrite v1 only records approval and audit evidence. No command was run.",
      writeFailed: "TerminalWrite request failed.",
      options: {
        pwd: "Current directory",
        "git status --short": "Git status",
        "git diff --stat": "Git change stats",
        "git branch --show-current": "Current branch",
      },
    },
    computerTool: {
      title: "Screenshot Boundary",
      capture: "Inspect screen",
      capturing: "Inspecting",
      pendingHint:
        "A screenshot approval request was created. Approve it, then inspect again.",
      captured: "Screenshot saved as local evidence.",
      unavailable:
        "Screenshot was not captured. Check OS screen-capture permission or display availability.",
      failed: "Screenshot request failed.",
    },
    computerControlTool: {
      title: "Computer Control Boundary",
      unlockTitle: "Local Computer Control Unlock",
      unlockChallengeLabel: "Local code",
      unlockTokenPlaceholder: "Enter local code",
      unlockControl: "Unlock",
      unlockingControl: "Unlocking",
      unlockReady: "Local control is unlocked for a short window.",
      unlockRequired: "Local short-window unlock is required before execution.",
      unlockExpires: "Valid until",
      unlockFailed: "Computer control unlock failed.",
      targetPlaceholder: "Target window, page, or control",
      actionPlaceholder: "click:120,340 or hotkey:ctrl+shift+p",
      requestControl: "Request control",
      requestingControl: "Requesting",
      pendingHint:
        "A ComputerControl approval request was created. After approval, retry to execute the structured action.",
      executed: "ComputerControl executed and recorded approval and audit evidence.",
      blocked:
        "ComputerControl was not executed. Check action format, OS permission, or the local input backend.",
      failed: "ComputerControl request failed.",
    },
    emailTool: {
      title: "Email Send Boundary",
      toPlaceholder: "Recipient",
      subjectPlaceholder: "Subject",
      bodyPlaceholder: "Body",
      requestSend: "Request send",
      requestingSend: "Requesting",
      pendingHint:
        "An EmailSend approval request was created. After approval, retry once; v1 will not send email directly.",
      blocked: "EmailSend v1 only records approval and audit evidence. No email was sent.",
      failed: "EmailSend request failed.",
    },
    emailDraftTool: {
      title: "Email Draft Boundary",
      toPlaceholder: "Draft recipient",
      subjectPlaceholder: "Draft subject",
      bodyPlaceholder: "Draft body",
      requestDraft: "Request draft",
      requestingDraft: "Requesting",
      pendingHint:
        "An EmailDraft approval request was created. v1 will not create mailbox drafts.",
      blocked:
        "EmailDraft v1 only records approval and audit evidence. No mailbox draft was created.",
      failed: "EmailDraft request failed.",
    },
    emailReadTool: {
      title: "Email Read Boundary",
      mailboxPlaceholder: "Mailbox or folder",
      queryPlaceholder: "Read query or evidence clue",
      requestRead: "Request read",
      requestingRead: "Requesting",
      pendingHint:
        "An EmailRead approval request was created. v1 will not read a real mailbox.",
      blocked: "EmailRead v1 only records approval and audit evidence. No mailbox was read.",
      failed: "EmailRead request failed.",
    },
    driveReadTool: {
      title: "Drive Read Local Folder",
      locationPlaceholder: "Local folder path",
      queryPlaceholder: "File name or content keyword",
      requestRead: "Request read",
      requestingRead: "Requesting",
      pendingHint:
        "A DriveRead approval request was created. After approval, it will read a local folder.",
      completed: "DriveRead read the local folder and recorded the result.",
      blocked: "DriveRead did not complete. Check recent tool output for the failure reason.",
      failed: "DriveRead request failed.",
    },
    driveWriteTool: {
      title: "Drive Export Package",
      locationPlaceholder: "Target local export folder",
      summaryPlaceholder: "Export package summary",
      requestWrite: "Export package",
      requestingWrite: "Requesting",
      pendingHint:
        "A DriveWrite approval request was created. After approval, it will export the current work package JSON.",
      completed: "DriveWrite exported the current work package JSON to the local folder.",
      blocked: "DriveWrite did not complete. Check recent tool output for the failure reason.",
      failed: "DriveWrite request failed.",
    },
    inspector: {
      title: "Runtime Controls",
      largeModel: "Large model",
      model: "Model",
      access: "Access",
      thinking: "Thinking",
      scope: "Scope",
      theme: "Style",
    },
  },
};
