use std::path::{Path, PathBuf};
use std::sync::Mutex;

use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::kernel::capability::{
    run_browser_browse, run_browser_submit_boundary, run_computer_control_boundary,
    run_computer_screenshot, run_drive_read_boundary, run_drive_write_boundary,
    run_email_draft_boundary, run_email_read_boundary, run_email_send_boundary,
    run_evidence_folder_ingest, run_file_read, run_file_write_boundary,
    run_network_search_boundary, run_terminal_read as run_terminal_read_capability,
    run_terminal_write_boundary, BrowserBrowseRequest, BrowserSubmitRequest, CapabilityInvocation,
    CodexBridgeComputerControlClient, CodexBridgeComputerScreenshotClient,
    CodexBridgeNetworkSearchClient, ComputerControlRequest, ComputerScreenshotRequest,
    DriveReadRequest, DriveWriteExportFile, DriveWriteRequest, EmailDraftRequest, EmailReadRequest,
    EmailSendRequest, EvidenceFolderRequest, FileReadRequest, FileWriteRequest,
    HttpBrowserPageClient, HttpNetworkSearchClient, LocalComputerControlClient,
    LocalComputerScreenshotClient, LocalDriveFolderClient, LocalEvidenceFolderClient,
    LocalFileContentClient, LocalTerminalReadClient, LocalWorkspaceFileWriteClient,
    NetworkSearchRequest, TerminalReadRequest, TerminalWriteRequest,
};
use crate::kernel::computer_use::{
    computer_use_backend_status_for_strategy, ComputerUseBackendStatus,
};
use crate::kernel::deepseek::{
    current_deepseek_credential_status, DeepSeekChatCacheState, DeepSeekChatTelemetry,
    DeepSeekCredentialStatus, DeepSeekMemoryChatCompletionCache,
    DeepSeekOperationsBriefingSynthesizer, HttpDeepSeekChatCompletionTransport,
    DEEPSEEK_API_KEY_ENV,
};
use crate::kernel::deepseek_pricing::{
    estimate_deepseek_chat_cost_micro_usd, load_deepseek_pricing_state,
    save_deepseek_pricing_settings as persist_deepseek_pricing_settings, DeepSeekPricingSettings,
    DeepSeekPricingState,
};
use crate::kernel::event_store::EventStore;
use crate::kernel::local_directory::{
    load_local_directory_state, local_directory_readiness_from_state,
    save_local_directory_settings as persist_local_directory_settings,
    LocalDirectoryReadinessStatus, LocalDirectorySettings, LocalDirectoryState,
};
use crate::kernel::models::FoundationState;
use crate::kernel::models::TaskRecord;
use crate::kernel::models::{AccessMode, ModelRoute, ThinkingLevel};
use crate::kernel::models::{
    ComputerControlBackend, ComputerScreenshotBackend, LargeModelProvider, NetworkSearchBackend,
    NetworkSearchSourceModel,
};
use crate::kernel::models::{
    MemoryCandidate, MemoryCandidateMergePreview, MemoryCandidateRecord,
    MemoryCandidateReplacePreview, MemoryCandidateResolution, MemoryCandidateSource,
    MemoryLifecycle, MemoryRecord, MemoryRecordDeletion, MemoryRecordUpdate, MemoryScope,
    MemorySensitivity, MemoryType,
};
use crate::kernel::network_search::{
    network_search_route_status_for_strategy, NetworkSearchRouteStatus,
};
use crate::kernel::policy::{
    builtin_capability_catalog, request_capability_access as build_capability_access_request,
    CapabilityAccessRecord, CapabilityDescriptor, CapabilityGrantState, CapabilityKind,
    PermissionAuditEntry, PermissionResolution,
};
use crate::kernel::tool_strategy::{
    model_driven_tool_strategy_for_current_platform, ModelDrivenToolStrategy,
};
use crate::kernel::work_package::{
    export_work_package_with_tool_readiness as build_work_package_with_tool_readiness,
    parse_work_package_json, WorkPackage, WorkPackageImportPreview, WorkPackageImportSummary,
    WorkPackageToolReadiness,
};
use crate::kernel::workflow::{
    operations_briefing_html_report_file_name, operations_briefing_pdf_report_file_name,
    operations_briefing_report_file_name, render_operations_briefing_html_report,
    render_operations_briefing_pdf_report, render_operations_briefing_report,
    run_operations_briefing as build_operations_briefing_run,
    run_operations_briefing_template_seed as build_operations_briefing_template_seed,
    run_operations_briefing_with_synthesizer as build_operations_briefing_run_with_synthesizer,
    LocalOperationsBriefingTemplateSeeder, OperationsBriefingRequest, OperationsBriefingRun,
    OperationsBriefingTemplateSeedRequest,
};

pub struct AppState {
    event_store: Mutex<EventStore>,
    computer_control_unlock: Mutex<ComputerControlUnlockState>,
    deepseek_chat_cache: DeepSeekMemoryChatCompletionCache,
}

impl AppState {
    pub fn new(event_store: EventStore) -> Self {
        Self {
            event_store: Mutex::new(event_store),
            computer_control_unlock: Mutex::new(ComputerControlUnlockState::generated()),
            deepseek_chat_cache: DeepSeekMemoryChatCompletionCache::default(),
        }
    }
}

const COMPUTER_CONTROL_UNLOCK_TTL_MINUTES: i64 = 5;
const COMPUTER_CONTROL_UNLOCK_CHALLENGE_LENGTH: usize = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ComputerControlUnlockStatus {
    pub challenge: String,
    pub unlocked: bool,
    pub unlocked_until: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug)]
struct ComputerControlUnlockState {
    challenge: String,
    unlocked_until: Option<DateTime<Utc>>,
}

impl ComputerControlUnlockState {
    fn new(challenge: String) -> Self {
        Self {
            challenge,
            unlocked_until: None,
        }
    }

    fn generated() -> Self {
        Self::new(generate_computer_control_unlock_challenge())
    }

    fn status(&self, now: DateTime<Utc>) -> ComputerControlUnlockStatus {
        let unlocked_until = self
            .unlocked_until
            .as_ref()
            .filter(|until| **until > now)
            .cloned();
        ComputerControlUnlockStatus {
            challenge: self.challenge.clone(),
            unlocked: unlocked_until.is_some(),
            unlocked_until,
        }
    }

    fn unlock(
        &mut self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<ComputerControlUnlockStatus, String> {
        if normalize_computer_control_unlock_token(token)
            != normalize_computer_control_unlock_token(&self.challenge)
        {
            return Err("invalid computer control unlock token".to_string());
        }

        self.unlocked_until = Some(now + Duration::minutes(COMPUTER_CONTROL_UNLOCK_TTL_MINUTES));
        Ok(self.status(now))
    }

    fn is_unlocked(&self, now: DateTime<Utc>) -> bool {
        self.unlocked_until
            .as_ref()
            .is_some_and(|unlocked_until| *unlocked_until > now)
    }
}

fn generate_computer_control_unlock_challenge() -> String {
    Uuid::new_v4()
        .simple()
        .to_string()
        .to_uppercase()
        .chars()
        .take(COMPUTER_CONTROL_UNLOCK_CHALLENGE_LENGTH)
        .collect()
}

fn normalize_computer_control_unlock_token(token: &str) -> String {
    token
        .chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}

fn should_require_computer_control_unlock(approval_granted: bool) -> bool {
    approval_granted
}

fn event_store_error(error: impl std::fmt::Display) -> String {
    error.to_string()
}

fn lock_error() -> String {
    "event store lock is unavailable".to_string()
}

fn computer_control_unlock_lock_error() -> String {
    "computer control unlock lock is unavailable".to_string()
}

fn current_work_package_tool_readiness(
    local_directories: LocalDirectoryReadinessStatus,
) -> WorkPackageToolReadiness {
    let deepseek = current_deepseek_credential_status();
    let foundation_state = FoundationState::default();
    let tool_strategy = model_driven_tool_strategy_for_current_platform(
        foundation_state.large_model_provider,
        foundation_state.network_search_source_model,
    );
    WorkPackageToolReadiness {
        network_search: network_search_route_status_for_strategy(
            &tool_strategy,
            deepseek.chat_completion_ready,
        ),
        deepseek,
        computer_use: computer_use_backend_status_for_strategy(&tool_strategy),
        local_directories,
        tool_strategy,
    }
}

fn current_local_directory_readiness(
    app: &AppHandle,
) -> Result<LocalDirectoryReadinessStatus, String> {
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    let directory_state = load_local_directory_state(app_data_dir).map_err(event_store_error)?;
    Ok(local_directory_readiness_from_state(&directory_state))
}

fn computer_tool_strategy_for_command(
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
) -> ModelDrivenToolStrategy {
    model_driven_tool_strategy_for_current_platform(
        large_model_provider,
        network_search_source_model,
    )
}

fn computer_screenshot_evidence_base_dir(
    app_data_dir: &Path,
    directory_state: &LocalDirectoryState,
) -> PathBuf {
    directory_state
        .settings
        .as_ref()
        .map(|settings| PathBuf::from(&settings.evidence_dir))
        .unwrap_or_else(|| app_data_dir.to_path_buf())
}

fn operations_briefing_report_export_dir(
    app_data_dir: &Path,
    directory_state: &LocalDirectoryState,
) -> PathBuf {
    directory_state
        .settings
        .as_ref()
        .map(|settings| PathBuf::from(&settings.export_dir))
        .unwrap_or_else(|| app_data_dir.to_path_buf())
}

fn operations_briefing_template_seed_dir(
    app_data_dir: &Path,
    directory_state: &LocalDirectoryState,
) -> PathBuf {
    directory_state
        .settings
        .as_ref()
        .map(|settings| PathBuf::from(&settings.evidence_dir))
        .unwrap_or_else(|| app_data_dir.join("operations-briefing-evidence"))
}

fn file_write_workspace_base_dir(
    app_data_dir: &Path,
    directory_state: &LocalDirectoryState,
) -> PathBuf {
    directory_state
        .settings
        .as_ref()
        .map(|settings| PathBuf::from(&settings.workspace_dir))
        .unwrap_or_else(|| app_data_dir.join("workspace"))
}

fn operations_briefing_deepseek_api_key_for_provider(
    read_env: impl Fn(&str) -> Option<String>,
    large_model_provider: LargeModelProvider,
) -> Option<String> {
    if large_model_provider != LargeModelProvider::DeepSeek {
        return None;
    }

    read_env(DEEPSEEK_API_KEY_ENV)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn deepseek_telemetry_with_pricing(
    mut telemetry: Vec<DeepSeekChatTelemetry>,
    pricing_settings: Option<&DeepSeekPricingSettings>,
) -> Vec<DeepSeekChatTelemetry> {
    let Some(pricing_settings) = pricing_settings else {
        return telemetry;
    };

    for entry in &mut telemetry {
        entry.estimated_cost_micro_usd =
            estimate_deepseek_chat_cost_micro_usd(entry, pricing_settings);
    }

    telemetry
}

#[tauri::command]
pub fn get_foundation_state() -> FoundationState {
    FoundationState::default()
}

#[tauri::command]
pub fn get_deepseek_credential_status() -> DeepSeekCredentialStatus {
    current_deepseek_credential_status()
}

#[tauri::command]
pub fn list_deepseek_chat_telemetry(
    state: State<'_, AppState>,
) -> Result<Vec<DeepSeekChatTelemetry>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .list_deepseek_chat_telemetry()
        .map_err(event_store_error)
}

#[tauri::command]
pub fn get_deepseek_chat_cache_state(state: State<'_, AppState>) -> DeepSeekChatCacheState {
    state.deepseek_chat_cache.state()
}

#[tauri::command]
pub fn clear_deepseek_chat_cache(state: State<'_, AppState>) -> usize {
    state.deepseek_chat_cache.clear()
}

#[tauri::command]
pub fn get_deepseek_pricing_state(app: AppHandle) -> Result<DeepSeekPricingState, String> {
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    load_deepseek_pricing_state(app_data_dir).map_err(event_store_error)
}

#[tauri::command]
pub fn save_deepseek_pricing_settings(
    app: AppHandle,
    enabled: bool,
    flash_prompt_usd_per_million_tokens: String,
    flash_completion_usd_per_million_tokens: String,
    pro_prompt_usd_per_million_tokens: String,
    pro_completion_usd_per_million_tokens: String,
) -> Result<DeepSeekPricingState, String> {
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    persist_deepseek_pricing_settings(
        app_data_dir,
        DeepSeekPricingSettings {
            enabled,
            flash_prompt_usd_per_million_tokens,
            flash_completion_usd_per_million_tokens,
            pro_prompt_usd_per_million_tokens,
            pro_completion_usd_per_million_tokens,
        },
    )
    .map_err(event_store_error)
}

#[tauri::command]
pub fn get_network_search_route_status() -> NetworkSearchRouteStatus {
    let deepseek_status = current_deepseek_credential_status();
    let foundation_state = FoundationState::default();
    let tool_strategy = model_driven_tool_strategy_for_current_platform(
        foundation_state.large_model_provider,
        foundation_state.network_search_source_model,
    );
    network_search_route_status_for_strategy(&tool_strategy, deepseek_status.chat_completion_ready)
}

#[tauri::command]
pub fn get_computer_use_backend_status() -> ComputerUseBackendStatus {
    let foundation_state = FoundationState::default();
    let tool_strategy = model_driven_tool_strategy_for_current_platform(
        foundation_state.large_model_provider,
        foundation_state.network_search_source_model,
    );
    computer_use_backend_status_for_strategy(&tool_strategy)
}

#[tauri::command]
pub fn get_computer_control_unlock_status(
    state: State<'_, AppState>,
) -> Result<ComputerControlUnlockStatus, String> {
    let unlock_state = state
        .computer_control_unlock
        .lock()
        .map_err(|_| computer_control_unlock_lock_error())?;
    Ok(unlock_state.status(Utc::now()))
}

#[tauri::command]
pub fn unlock_computer_control(
    token: String,
    state: State<'_, AppState>,
) -> Result<ComputerControlUnlockStatus, String> {
    let mut unlock_state = state
        .computer_control_unlock
        .lock()
        .map_err(|_| computer_control_unlock_lock_error())?;
    unlock_state.unlock(&token, Utc::now())
}

#[tauri::command]
pub fn get_network_search_route_status_for_model(
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
) -> NetworkSearchRouteStatus {
    let deepseek_status = current_deepseek_credential_status();
    let tool_strategy = model_driven_tool_strategy_for_current_platform(
        large_model_provider,
        network_search_source_model,
    );
    network_search_route_status_for_strategy(&tool_strategy, deepseek_status.chat_completion_ready)
}

#[tauri::command]
pub fn get_computer_use_backend_status_for_model(
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
) -> ComputerUseBackendStatus {
    let tool_strategy = model_driven_tool_strategy_for_current_platform(
        large_model_provider,
        network_search_source_model,
    );
    computer_use_backend_status_for_strategy(&tool_strategy)
}

#[tauri::command]
pub fn get_model_driven_tool_strategy(
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
) -> ModelDrivenToolStrategy {
    model_driven_tool_strategy_for_current_platform(
        large_model_provider,
        network_search_source_model,
    )
}

#[tauri::command]
pub fn get_local_directory_state(app: AppHandle) -> Result<LocalDirectoryState, String> {
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    load_local_directory_state(app_data_dir).map_err(event_store_error)
}

#[tauri::command]
pub fn save_local_directory_settings(
    app: AppHandle,
    workspace_dir: String,
    evidence_dir: String,
    export_dir: String,
) -> Result<LocalDirectoryState, String> {
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    let settings = LocalDirectorySettings::new(workspace_dir, evidence_dir, export_dir)
        .map_err(event_store_error)?;
    persist_local_directory_settings(app_data_dir, settings).map_err(event_store_error)
}

#[tauri::command]
pub fn list_task_records(state: State<'_, AppState>) -> Result<Vec<TaskRecord>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store.list_task_records().map_err(event_store_error)
}

#[tauri::command]
pub fn list_memory_records(state: State<'_, AppState>) -> Result<Vec<MemoryRecord>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store.list_memory_records().map_err(event_store_error)
}

#[tauri::command]
pub fn list_memory_candidate_records(
    state: State<'_, AppState>,
) -> Result<Vec<MemoryCandidateRecord>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .list_memory_candidate_records()
        .map_err(event_store_error)
}

#[tauri::command]
pub fn search_memory_records(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<MemoryRecord>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .search_memory_records(&query)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn propose_memory_candidate(
    title: String,
    body: String,
    memory_type: MemoryType,
    scope: MemoryScope,
    sensitivity: MemorySensitivity,
    lifecycle: MemoryLifecycle,
    expires_at: Option<DateTime<Utc>>,
    state: State<'_, AppState>,
) -> Result<MemoryCandidateRecord, String> {
    let candidate = MemoryCandidate::new_with_metadata_and_expiration(
        title,
        body,
        MemoryCandidateSource::Manual,
        None,
        "User proposed this memory in Memory Studio.".to_string(),
        memory_type,
        scope,
        sensitivity,
        lifecycle,
        expires_at,
    )?;
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .append_memory_candidate(&candidate)
        .map_err(event_store_error)?;

    store
        .list_memory_candidate_records()
        .map_err(event_store_error)?
        .into_iter()
        .find(|record| record.candidate.id == candidate.id)
        .ok_or_else(|| "memory candidate was not found after append".to_string())
}

#[tauri::command]
pub fn resolve_memory_candidate(
    candidate_id: Uuid,
    accepted: bool,
    note: String,
    state: State<'_, AppState>,
) -> Result<MemoryCandidateResolution, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .resolve_memory_candidate(candidate_id, accepted, note)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn preview_memory_candidate_merge(
    candidate_id: Uuid,
    source_memory_ids: Vec<Uuid>,
    state: State<'_, AppState>,
) -> Result<MemoryCandidateMergePreview, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .preview_memory_candidate_merge(candidate_id, source_memory_ids)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn preview_memory_candidate_replace(
    candidate_id: Uuid,
    target_memory_ids: Vec<Uuid>,
    state: State<'_, AppState>,
) -> Result<MemoryCandidateReplacePreview, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .preview_memory_candidate_replace(candidate_id, target_memory_ids)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn merge_memory_candidate_with_conflicts(
    candidate_id: Uuid,
    source_memory_ids: Vec<Uuid>,
    note: String,
    state: State<'_, AppState>,
) -> Result<MemoryCandidateResolution, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .merge_memory_candidate_with_conflicts(candidate_id, source_memory_ids, note)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn replace_memory_candidate_conflicts(
    candidate_id: Uuid,
    target_memory_ids: Vec<Uuid>,
    note: String,
    state: State<'_, AppState>,
) -> Result<MemoryCandidateResolution, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .replace_memory_candidate_conflicts(candidate_id, target_memory_ids, note)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn link_memory_candidate_to_conflicts(
    candidate_id: Uuid,
    linked_memory_ids: Vec<Uuid>,
    note: String,
    state: State<'_, AppState>,
) -> Result<MemoryCandidateResolution, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .link_memory_candidate_to_conflicts(candidate_id, linked_memory_ids, note)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn update_memory_record(
    memory_id: Uuid,
    title: String,
    body: String,
    memory_type: MemoryType,
    scope: MemoryScope,
    sensitivity: MemorySensitivity,
    lifecycle: MemoryLifecycle,
    expires_at: Option<DateTime<Utc>>,
    note: String,
    state: State<'_, AppState>,
) -> Result<MemoryRecordUpdate, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .update_memory_record(
            memory_id,
            title,
            body,
            memory_type,
            scope,
            sensitivity,
            lifecycle,
            expires_at,
            note,
        )
        .map_err(event_store_error)
}

#[tauri::command]
pub fn delete_memory_record(
    memory_id: Uuid,
    note: String,
    state: State<'_, AppState>,
) -> Result<MemoryRecordDeletion, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .delete_memory_record(memory_id, note)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn list_permission_audit_entries(
    state: State<'_, AppState>,
) -> Result<Vec<PermissionAuditEntry>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .list_permission_audit_entries()
        .map_err(event_store_error)
}

#[tauri::command]
pub fn list_capability_catalog() -> Vec<CapabilityDescriptor> {
    builtin_capability_catalog()
}

#[tauri::command]
pub fn list_capability_access_records(
    state: State<'_, AppState>,
) -> Result<Vec<CapabilityAccessRecord>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .list_capability_access_records()
        .map_err(event_store_error)
}

#[tauri::command]
pub fn list_pending_capability_access_records(
    state: State<'_, AppState>,
) -> Result<Vec<CapabilityAccessRecord>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .list_pending_capability_access_records()
        .map_err(event_store_error)
}

#[tauri::command]
pub fn request_capability_access(
    access_mode: AccessMode,
    capability: CapabilityKind,
    state: State<'_, AppState>,
) -> Result<CapabilityAccessRecord, String> {
    let request = build_capability_access_request(access_mode, capability)?;
    let entry = PermissionAuditEntry::evaluate(access_mode, capability);
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .append_capability_access_request(&request)
        .map_err(event_store_error)?;
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;

    Ok(CapabilityAccessRecord {
        effective_status: request.status,
        grant_state: CapabilityGrantState::NotGranted,
        request,
        resolution: None,
    })
}

#[tauri::command]
pub fn resolve_capability_access_request(
    request_id: Uuid,
    approved: bool,
    note: String,
    state: State<'_, AppState>,
) -> Result<PermissionResolution, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .resolve_capability_access_request(request_id, approved, note)
        .map_err(event_store_error)
}

#[tauri::command]
pub fn list_capability_invocations(
    state: State<'_, AppState>,
) -> Result<Vec<CapabilityInvocation>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .list_capability_invocations()
        .map_err(event_store_error)
}

#[tauri::command]
pub fn list_operations_briefing_runs(
    state: State<'_, AppState>,
) -> Result<Vec<OperationsBriefingRun>, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .list_operations_briefing_runs()
        .map_err(event_store_error)
}

#[tauri::command]
pub fn browse_url(
    access_mode: AccessMode,
    url: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::BrowserBrowse)
            .map_err(event_store_error)?
    };
    let client = HttpBrowserPageClient::new()?;
    let outcome = run_browser_browse(
        BrowserBrowseRequest {
            access_mode,
            url,
            approval_granted,
        },
        &client,
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::BrowserBrowse);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn submit_browser_boundary(
    access_mode: AccessMode,
    url: String,
    summary: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::BrowserSubmit)
            .map_err(event_store_error)?
    };
    let outcome = run_browser_submit_boundary(BrowserSubmitRequest {
        access_mode,
        url,
        summary,
        approval_granted,
    })?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::BrowserSubmit);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn search_network_boundary(
    access_mode: AccessMode,
    large_model_provider: LargeModelProvider,
    query: String,
    scope: String,
    network_search_source_model: Option<NetworkSearchSourceModel>,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::NetworkSearch)
            .map_err(event_store_error)?
    };
    let strategy = model_driven_tool_strategy_for_current_platform(
        large_model_provider,
        network_search_source_model,
    );
    let request = NetworkSearchRequest {
        access_mode,
        query,
        scope,
        approval_granted,
    };
    let outcome = match strategy.network_search_backend {
        NetworkSearchBackend::NativeLargeModel => {
            let client = CodexBridgeNetworkSearchClient::from_env(large_model_provider);
            run_network_search_boundary(request, &client)?
        }
        NetworkSearchBackend::SourceBackedModel | NetworkSearchBackend::DeepSeek => {
            let source_model = strategy.network_search_source_model.ok_or_else(|| {
                "network search source model is required for local source-backed execution"
                    .to_string()
            })?;
            let client = HttpNetworkSearchClient::new(source_model)?;
            run_network_search_boundary(request, &client)?
        }
    };
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::NetworkSearch);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn read_local_file(
    access_mode: AccessMode,
    path: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::FileRead)
            .map_err(event_store_error)?
    };
    let client = LocalFileContentClient::new(512 * 1024);
    let outcome = run_file_read(
        FileReadRequest {
            access_mode,
            path,
            approval_granted,
        },
        &client,
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::FileRead);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn write_file_boundary(
    app: AppHandle,
    access_mode: AccessMode,
    path: String,
    summary: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::FileWrite)
            .map_err(event_store_error)?
    };
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    let directory_state = load_local_directory_state(&app_data_dir).map_err(event_store_error)?;
    let workspace_dir = file_write_workspace_base_dir(&app_data_dir, &directory_state);
    std::fs::create_dir_all(&workspace_dir).map_err(event_store_error)?;
    let client = LocalWorkspaceFileWriteClient::new(workspace_dir, 512 * 1024);
    let outcome = run_file_write_boundary(
        FileWriteRequest {
            access_mode,
            path,
            summary,
            content,
            approval_granted,
        },
        &client,
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::FileWrite);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn ingest_evidence_folder(
    access_mode: AccessMode,
    folder_path: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::FileRead)
            .map_err(event_store_error)?
    };
    let client = LocalEvidenceFolderClient::new(20, 512 * 1024);
    let outcome = run_evidence_folder_ingest(
        EvidenceFolderRequest {
            access_mode,
            folder_path,
            approval_granted,
        },
        &client,
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::FileRead);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn run_terminal_read(
    access_mode: AccessMode,
    command: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::TerminalRead)
            .map_err(event_store_error)?
    };
    let working_dir = std::env::current_dir().map_err(event_store_error)?;
    let client = LocalTerminalReadClient::new(working_dir, 4_000);
    let outcome = run_terminal_read_capability(
        TerminalReadRequest {
            access_mode,
            command,
            approval_granted,
        },
        &client,
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::TerminalRead);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn run_terminal_write(
    access_mode: AccessMode,
    command: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::TerminalWrite)
            .map_err(event_store_error)?
    };
    let outcome = run_terminal_write_boundary(TerminalWriteRequest {
        access_mode,
        command,
        approval_granted,
    })?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::TerminalWrite);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn capture_computer_screenshot(
    app: AppHandle,
    access_mode: AccessMode,
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::ComputerScreenshot)
            .map_err(event_store_error)?
    };
    let strategy =
        computer_tool_strategy_for_command(large_model_provider, network_search_source_model);
    let request = ComputerScreenshotRequest {
        access_mode,
        approval_granted,
    };
    let outcome = match strategy.computer_screenshot_backend {
        ComputerScreenshotBackend::LocalWindowsScreenCapture
        | ComputerScreenshotBackend::LocalMacosScreenCapture => {
            let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
            let directory_state =
                load_local_directory_state(&app_data_dir).map_err(event_store_error)?;
            let evidence_base_dir =
                computer_screenshot_evidence_base_dir(&app_data_dir, &directory_state);
            let client = LocalComputerScreenshotClient::new(evidence_base_dir);
            run_computer_screenshot(request, &client)?
        }
        ComputerScreenshotBackend::CodexBridgeScreenCapture
        | ComputerScreenshotBackend::CodexStyleScreenCapture => {
            let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
            let directory_state =
                load_local_directory_state(&app_data_dir).map_err(event_store_error)?;
            let evidence_base_dir =
                computer_screenshot_evidence_base_dir(&app_data_dir, &directory_state);
            let client = CodexBridgeComputerScreenshotClient::from_env(evidence_base_dir);
            run_computer_screenshot(request, &client)?
        }
    };
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::ComputerScreenshot);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn control_computer_boundary(
    access_mode: AccessMode,
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
    target: String,
    action: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_request_id = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .available_capability_grant_request_id(CapabilityKind::ComputerControl)
            .map_err(event_store_error)?
    };
    let approval_granted = approval_request_id.is_some();
    if should_require_computer_control_unlock(approval_granted) {
        let unlock_state = state
            .computer_control_unlock
            .lock()
            .map_err(|_| computer_control_unlock_lock_error())?;
        if !unlock_state.is_unlocked(Utc::now()) {
            return Err("computer control requires local unlock before execution".to_string());
        }
    }
    let strategy =
        computer_tool_strategy_for_command(large_model_provider, network_search_source_model);
    let request = ComputerControlRequest {
        access_mode,
        target,
        action,
        approval_granted,
    };
    let mut outcome = match strategy.computer_control_backend {
        ComputerControlBackend::LocalWindowsInputControl
        | ComputerControlBackend::LocalMacosInputControl => {
            let client = LocalComputerControlClient::new();
            run_computer_control_boundary(request, &client)?
        }
        ComputerControlBackend::CodexBridgeInputControl
        | ComputerControlBackend::CodexStyleInputControl => {
            let client = CodexBridgeComputerControlClient::from_env();
            run_computer_control_boundary(request, &client)?
        }
    };
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::ComputerControl);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    outcome.invocation.approval_request_id = approval_request_id
        .or_else(|| should_record_access_request.then_some(outcome.access_request.id));
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn send_email_boundary(
    access_mode: AccessMode,
    to: String,
    subject: String,
    body: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_request_id = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .available_capability_grant_request_id(CapabilityKind::EmailSend)
            .map_err(event_store_error)?
    };
    let approval_granted = approval_request_id.is_some();
    let mut outcome = run_email_send_boundary(EmailSendRequest {
        access_mode,
        to,
        subject,
        body,
        approval_granted,
    })?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::EmailSend);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    outcome.invocation.approval_request_id = approval_request_id
        .or_else(|| should_record_access_request.then_some(outcome.access_request.id));
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn create_email_draft_boundary(
    access_mode: AccessMode,
    to: String,
    subject: String,
    body: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::EmailDraft)
            .map_err(event_store_error)?
    };
    let outcome = run_email_draft_boundary(EmailDraftRequest {
        access_mode,
        to,
        subject,
        body,
        approval_granted,
    })?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::EmailDraft);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn read_email_boundary(
    access_mode: AccessMode,
    mailbox: String,
    query: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::EmailRead)
            .map_err(event_store_error)?
    };
    let outcome = run_email_read_boundary(EmailReadRequest {
        access_mode,
        mailbox,
        query,
        approval_granted,
    })?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::EmailRead);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn read_drive_boundary(
    access_mode: AccessMode,
    location: String,
    query: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::DriveRead)
            .map_err(event_store_error)?
    };
    let outcome = run_drive_read_boundary(
        DriveReadRequest {
            access_mode,
            location,
            query,
            approval_granted,
        },
        &LocalDriveFolderClient::new(20, 512 * 1024),
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::DriveRead);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn write_drive_boundary(
    app: AppHandle,
    access_mode: AccessMode,
    location: String,
    summary: String,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::DriveWrite)
            .map_err(event_store_error)?
    };
    let package_json = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        let task_records = store.list_task_records().map_err(event_store_error)?;
        let memory_candidates = store.list_memory_candidates().map_err(event_store_error)?;
        let operations_briefing_runs = store
            .list_operations_briefing_runs()
            .map_err(event_store_error)?;
        let package = build_work_package_with_tool_readiness(
            FoundationState::default(),
            task_records,
            memory_candidates,
            operations_briefing_runs,
            current_work_package_tool_readiness(current_local_directory_readiness(&app)?),
        );
        serde_json::to_string_pretty(&package).map_err(event_store_error)?
    };
    let outcome = run_drive_write_boundary(
        DriveWriteRequest {
            access_mode,
            location,
            summary,
            package_json: Some(package_json),
            export_file: None,
            approval_granted,
        },
        &LocalDriveFolderClient::new(20, 512 * 1024),
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::DriveWrite);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn run_operations_briefing(
    app: AppHandle,
    access_mode: AccessMode,
    evidence_folder_path: String,
    large_model_provider: LargeModelProvider,
    model_route: ModelRoute,
    thinking_level: ThinkingLevel,
    state: State<'_, AppState>,
) -> Result<OperationsBriefingRun, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::FileRead)
            .map_err(event_store_error)?
    };
    let client = LocalEvidenceFolderClient::new(20, 512 * 1024);
    let request = OperationsBriefingRequest {
        access_mode,
        evidence_folder_path,
        approval_granted,
    };
    let mut deepseek_telemetry = Vec::new();
    let outcome = if let Some(api_key) = operations_briefing_deepseek_api_key_for_provider(
        |name| std::env::var(name).ok(),
        large_model_provider,
    ) {
        let transport = HttpDeepSeekChatCompletionTransport::new()?;
        let synthesizer = DeepSeekOperationsBriefingSynthesizer::new_with_cache(
            &transport,
            &state.deepseek_chat_cache,
            api_key,
            model_route,
            thinking_level,
        );
        let outcome =
            build_operations_briefing_run_with_synthesizer(request, &client, &synthesizer)?;
        deepseek_telemetry = synthesizer.take_telemetry();
        outcome
    } else {
        build_operations_briefing_run(request, &client)?
    };
    let pricing_settings = app
        .path()
        .app_data_dir()
        .ok()
        .and_then(|app_data_dir| load_deepseek_pricing_state(app_data_dir).ok())
        .map(|pricing_state| pricing_state.settings);
    let deepseek_telemetry =
        deepseek_telemetry_with_pricing(deepseek_telemetry, pricing_settings.as_ref());
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::FileRead);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.evidence_invocation)
        .map_err(event_store_error)?;
    store
        .append_operations_briefing_run(&outcome.run)
        .map_err(event_store_error)?;
    for telemetry in deepseek_telemetry {
        store
            .append_deepseek_chat_telemetry(&telemetry)
            .map_err(event_store_error)?;
    }

    Ok(outcome.run)
}

#[tauri::command]
pub fn export_operations_briefing_report(
    app: AppHandle,
    access_mode: AccessMode,
    run_id: Uuid,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::DriveWrite)
            .map_err(event_store_error)?
    };
    let run = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .list_operations_briefing_runs()
            .map_err(event_store_error)?
            .into_iter()
            .find(|run| run.id == run_id)
            .ok_or_else(|| "operations briefing run was not found".to_string())?
    };
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    let directory_state = load_local_directory_state(&app_data_dir).map_err(event_store_error)?;
    let export_dir = operations_briefing_report_export_dir(&app_data_dir, &directory_state);
    let report_markdown = render_operations_briefing_report(&run);
    let outcome = run_drive_write_boundary(
        DriveWriteRequest {
            access_mode,
            location: export_dir.to_string_lossy().to_string(),
            summary: format!("Export Operations Briefing report {}", run.id),
            package_json: None,
            export_file: Some(DriveWriteExportFile {
                file_name: operations_briefing_report_file_name(&run),
                content: report_markdown,
                content_base64: None,
            }),
            approval_granted,
        },
        &LocalDriveFolderClient::new(20, 512 * 1024),
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::DriveWrite);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn export_operations_briefing_html_report(
    app: AppHandle,
    access_mode: AccessMode,
    run_id: Uuid,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::DriveWrite)
            .map_err(event_store_error)?
    };
    let run = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .list_operations_briefing_runs()
            .map_err(event_store_error)?
            .into_iter()
            .find(|run| run.id == run_id)
            .ok_or_else(|| "operations briefing run was not found".to_string())?
    };
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    let directory_state = load_local_directory_state(&app_data_dir).map_err(event_store_error)?;
    let export_dir = operations_briefing_report_export_dir(&app_data_dir, &directory_state);
    let report_html = render_operations_briefing_html_report(&run);
    let outcome = run_drive_write_boundary(
        DriveWriteRequest {
            access_mode,
            location: export_dir.to_string_lossy().to_string(),
            summary: format!("Export Operations Briefing HTML report {}", run.id),
            package_json: None,
            export_file: Some(DriveWriteExportFile {
                file_name: operations_briefing_html_report_file_name(&run),
                content: report_html,
                content_base64: None,
            }),
            approval_granted,
        },
        &LocalDriveFolderClient::new(20, 512 * 1024),
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::DriveWrite);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn export_operations_briefing_pdf_report(
    app: AppHandle,
    access_mode: AccessMode,
    run_id: Uuid,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::DriveWrite)
            .map_err(event_store_error)?
    };
    let run = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .list_operations_briefing_runs()
            .map_err(event_store_error)?
            .into_iter()
            .find(|run| run.id == run_id)
            .ok_or_else(|| "operations briefing run was not found".to_string())?
    };
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    let directory_state = load_local_directory_state(&app_data_dir).map_err(event_store_error)?;
    let export_dir = operations_briefing_report_export_dir(&app_data_dir, &directory_state);
    let report_pdf = render_operations_briefing_pdf_report(&run);
    let outcome = run_drive_write_boundary(
        DriveWriteRequest {
            access_mode,
            location: export_dir.to_string_lossy().to_string(),
            summary: format!("Export Operations Briefing PDF report {}", run.id),
            package_json: None,
            export_file: Some(DriveWriteExportFile {
                file_name: operations_briefing_pdf_report_file_name(&run),
                content: String::new(),
                content_base64: Some(general_purpose::STANDARD.encode(report_pdf)),
            }),
            approval_granted,
        },
        &LocalDriveFolderClient::new(20, 512 * 1024),
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::DriveWrite);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn seed_operations_briefing_evidence_templates(
    app: AppHandle,
    access_mode: AccessMode,
    state: State<'_, AppState>,
) -> Result<CapabilityInvocation, String> {
    let approval_granted = {
        let store = state.event_store.lock().map_err(|_| lock_error())?;
        store
            .has_user_approved_capability(CapabilityKind::FileWrite)
            .map_err(event_store_error)?
    };
    let app_data_dir = app.path().app_data_dir().map_err(event_store_error)?;
    let directory_state = load_local_directory_state(&app_data_dir).map_err(event_store_error)?;
    let seed_dir = operations_briefing_template_seed_dir(&app_data_dir, &directory_state);
    let outcome = build_operations_briefing_template_seed(
        OperationsBriefingTemplateSeedRequest {
            access_mode,
            evidence_folder_path: seed_dir.to_string_lossy().to_string(),
            approval_granted,
        },
        &LocalOperationsBriefingTemplateSeeder,
    )?;
    let should_record_access_request = !approval_granted
        || outcome.access_request.decision == crate::kernel::policy::PolicyDecision::Allow;
    let entry = PermissionAuditEntry::evaluate(access_mode, CapabilityKind::FileWrite);
    let store = state.event_store.lock().map_err(|_| lock_error())?;

    if should_record_access_request {
        store
            .append_capability_access_request(&outcome.access_request)
            .map_err(event_store_error)?;
    }
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    store
        .append_capability_invocation(&outcome.invocation)
        .map_err(event_store_error)?;

    Ok(outcome.invocation)
}

#[tauri::command]
pub fn record_permission_audit(
    access_mode: AccessMode,
    capability: CapabilityKind,
    state: State<'_, AppState>,
) -> Result<PermissionAuditEntry, String> {
    let entry = PermissionAuditEntry::evaluate(access_mode, capability);
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .append_permission_audit_entry(&entry)
        .map_err(event_store_error)?;
    Ok(entry)
}

#[tauri::command]
pub fn create_task_record(
    title: String,
    summary: String,
    state: State<'_, AppState>,
) -> Result<TaskRecord, String> {
    let record = TaskRecord::new(title, summary)?;
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .append_task_record(&record)
        .map_err(event_store_error)?;
    let memory = MemoryRecord::from_task_record(&record);
    store
        .append_memory_record(&memory)
        .map_err(event_store_error)?;
    Ok(record)
}

#[tauri::command]
pub fn export_work_package(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<WorkPackage, String> {
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    let task_records = store.list_task_records().map_err(event_store_error)?;
    let memory_candidates = store.list_memory_candidates().map_err(event_store_error)?;
    let operations_briefing_runs = store
        .list_operations_briefing_runs()
        .map_err(event_store_error)?;
    Ok(build_work_package_with_tool_readiness(
        FoundationState::default(),
        task_records,
        memory_candidates,
        operations_briefing_runs,
        current_work_package_tool_readiness(current_local_directory_readiness(&app)?),
    ))
}

#[tauri::command]
pub fn import_work_package(
    package_json: String,
    state: State<'_, AppState>,
) -> Result<WorkPackageImportSummary, String> {
    let package = parse_work_package_json(&package_json).map_err(event_store_error)?;
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    let mut summary = store
        .import_task_records(&package.task_records)
        .map_err(event_store_error)?;
    summary.memory_candidates = store
        .import_memory_candidates(&package.memory_candidates)
        .map_err(event_store_error)?;
    summary.operations_briefing_runs = store
        .import_operations_briefing_runs(&package.operations_briefing_runs)
        .map_err(event_store_error)?;
    summary.workflow_templates = store
        .import_workflow_template_packages(&package.workflow_templates)
        .map_err(event_store_error)?;
    Ok(summary)
}

#[tauri::command]
pub fn preview_work_package_import(
    package_json: String,
    state: State<'_, AppState>,
) -> Result<WorkPackageImportPreview, String> {
    let package = parse_work_package_json(&package_json).map_err(event_store_error)?;
    let store = state.event_store.lock().map_err(|_| lock_error())?;
    store
        .preview_work_package_import(&package)
        .map_err(event_store_error)
}

#[cfg(test)]
mod tests {
    use crate::commands::{
        computer_screenshot_evidence_base_dir, computer_tool_strategy_for_command,
        deepseek_telemetry_with_pricing, operations_briefing_deepseek_api_key_for_provider,
        operations_briefing_report_export_dir, operations_briefing_template_seed_dir,
        should_require_computer_control_unlock, ComputerControlUnlockState,
        COMPUTER_CONTROL_UNLOCK_TTL_MINUTES,
    };
    use crate::kernel::deepseek::{
        DeepSeekChatCacheStatus, DeepSeekChatTelemetry, DEEPSEEK_FLASH_MODEL,
    };
    use crate::kernel::deepseek_pricing::DeepSeekPricingSettings;
    use crate::kernel::local_directory::{
        LocalDirectorySettings, LocalDirectoryState, LOCAL_DIRECTORY_SETTINGS_FILE,
    };
    use crate::kernel::models::{
        ComputerControlBackend, ComputerScreenshotBackend, LargeModelProvider,
        NetworkSearchSourceModel,
    };
    use chrono::{Duration, TimeZone, Utc};
    use uuid::Uuid;

    #[test]
    fn computer_screenshot_evidence_base_prefers_user_evidence_dir() {
        let app_data_dir = std::path::PathBuf::from("fixtures/app-data");
        let state = LocalDirectoryState {
            app_data_dir: app_data_dir.to_string_lossy().to_string(),
            settings_file: app_data_dir
                .join(LOCAL_DIRECTORY_SETTINGS_FILE)
                .to_string_lossy()
                .to_string(),
            settings: Some(
                LocalDirectorySettings::new(
                    "fixtures/workspace".to_string(),
                    "fixtures/evidence".to_string(),
                    "fixtures/exports".to_string(),
                )
                .expect("settings validate"),
            ),
            needs_setup: false,
        };

        assert_eq!(
            computer_screenshot_evidence_base_dir(&app_data_dir, &state),
            std::path::PathBuf::from("fixtures/evidence")
        );
    }

    #[test]
    fn computer_screenshot_evidence_base_falls_back_to_app_data_before_setup() {
        let app_data_dir = std::path::PathBuf::from("fixtures/app-data");
        let state = LocalDirectoryState {
            app_data_dir: app_data_dir.to_string_lossy().to_string(),
            settings_file: app_data_dir
                .join(LOCAL_DIRECTORY_SETTINGS_FILE)
                .to_string_lossy()
                .to_string(),
            settings: None,
            needs_setup: true,
        };

        assert_eq!(
            computer_screenshot_evidence_base_dir(&app_data_dir, &state),
            app_data_dir
        );
    }

    #[test]
    fn operations_briefing_report_export_dir_prefers_user_export_dir() {
        let app_data_dir = std::path::PathBuf::from("fixtures/app-data");
        let state = LocalDirectoryState {
            app_data_dir: app_data_dir.to_string_lossy().to_string(),
            settings_file: app_data_dir
                .join(LOCAL_DIRECTORY_SETTINGS_FILE)
                .to_string_lossy()
                .to_string(),
            settings: Some(
                LocalDirectorySettings::new(
                    "fixtures/workspace".to_string(),
                    "fixtures/evidence".to_string(),
                    "fixtures/exports".to_string(),
                )
                .expect("settings validate"),
            ),
            needs_setup: false,
        };

        assert_eq!(
            operations_briefing_report_export_dir(&app_data_dir, &state),
            std::path::PathBuf::from("fixtures/exports")
        );
    }

    #[test]
    fn operations_briefing_report_export_dir_falls_back_to_app_data_before_setup() {
        let app_data_dir = std::path::PathBuf::from("fixtures/app-data");
        let state = LocalDirectoryState {
            app_data_dir: app_data_dir.to_string_lossy().to_string(),
            settings_file: app_data_dir
                .join(LOCAL_DIRECTORY_SETTINGS_FILE)
                .to_string_lossy()
                .to_string(),
            settings: None,
            needs_setup: true,
        };

        assert_eq!(
            operations_briefing_report_export_dir(&app_data_dir, &state),
            app_data_dir
        );
    }

    #[test]
    fn operations_briefing_template_seed_dir_prefers_user_evidence_dir() {
        let app_data_dir = std::path::PathBuf::from("fixtures/app-data");
        let state = LocalDirectoryState {
            app_data_dir: app_data_dir.to_string_lossy().to_string(),
            settings_file: app_data_dir
                .join(LOCAL_DIRECTORY_SETTINGS_FILE)
                .to_string_lossy()
                .to_string(),
            settings: Some(
                LocalDirectorySettings::new(
                    "fixtures/workspace".to_string(),
                    "fixtures/evidence".to_string(),
                    "fixtures/exports".to_string(),
                )
                .expect("settings validate"),
            ),
            needs_setup: false,
        };

        assert_eq!(
            operations_briefing_template_seed_dir(&app_data_dir, &state),
            std::path::PathBuf::from("fixtures/evidence")
        );
    }

    #[test]
    fn operations_briefing_template_seed_dir_falls_back_to_app_data_subdir_before_setup() {
        let app_data_dir = std::path::PathBuf::from("fixtures/app-data");
        let state = LocalDirectoryState {
            app_data_dir: app_data_dir.to_string_lossy().to_string(),
            settings_file: app_data_dir
                .join(LOCAL_DIRECTORY_SETTINGS_FILE)
                .to_string_lossy()
                .to_string(),
            settings: None,
            needs_setup: true,
        };

        assert_eq!(
            operations_briefing_template_seed_dir(&app_data_dir, &state),
            app_data_dir.join("operations-briefing-evidence")
        );
    }

    #[test]
    fn computer_control_unlock_state_starts_locked_with_local_challenge() {
        let now = Utc.with_ymd_and_hms(2026, 6, 29, 12, 0, 0).unwrap();
        let state = ComputerControlUnlockState::new("ABC123".to_string());

        let status = state.status(now);

        assert_eq!(status.challenge, "ABC123");
        assert!(!status.unlocked);
        assert_eq!(status.unlocked_until, None);
    }

    #[test]
    fn computer_control_unlock_rejects_wrong_token_without_unlocking() {
        let now = Utc.with_ymd_and_hms(2026, 6, 29, 12, 0, 0).unwrap();
        let mut state = ComputerControlUnlockState::new("ABC123".to_string());

        let error = state
            .unlock("WRONG", now)
            .expect_err("wrong token should not unlock control");

        assert!(error.contains("invalid computer control unlock token"));
        assert!(!state.status(now).unlocked);
    }

    #[test]
    fn computer_control_unlock_accepts_token_for_limited_window_and_expires() {
        let now = Utc.with_ymd_and_hms(2026, 6, 29, 12, 0, 0).unwrap();
        let mut state = ComputerControlUnlockState::new("ABC123".to_string());

        let status = state
            .unlock(" abc123 ", now)
            .expect("matching token unlocks control");

        assert!(status.unlocked);
        assert_eq!(
            status.unlocked_until,
            Some(now + Duration::minutes(COMPUTER_CONTROL_UNLOCK_TTL_MINUTES))
        );
        assert!(state.is_unlocked(now + Duration::minutes(COMPUTER_CONTROL_UNLOCK_TTL_MINUTES - 1)));
        assert!(
            !state.is_unlocked(now + Duration::minutes(COMPUTER_CONTROL_UNLOCK_TTL_MINUTES + 1))
        );
    }

    #[test]
    fn computer_control_execution_requires_unlock_only_after_approval() {
        assert!(!should_require_computer_control_unlock(false));
        assert!(should_require_computer_control_unlock(true));
    }

    #[test]
    fn computer_tool_strategy_for_command_routes_chatgpt_to_codex_bridge() {
        let strategy = computer_tool_strategy_for_command(LargeModelProvider::ChatGpt, None);

        assert_eq!(
            strategy.computer_screenshot_backend,
            ComputerScreenshotBackend::CodexBridgeScreenCapture
        );
        assert_eq!(
            strategy.computer_control_backend,
            ComputerControlBackend::CodexBridgeInputControl
        );
    }

    #[test]
    fn computer_tool_strategy_for_command_routes_deepseek_to_local_backend() {
        let strategy = computer_tool_strategy_for_command(
            LargeModelProvider::DeepSeek,
            Some(NetworkSearchSourceModel::FreeWebSource),
        );

        if cfg!(target_os = "macos") {
            assert_eq!(
                strategy.computer_screenshot_backend,
                ComputerScreenshotBackend::LocalMacosScreenCapture
            );
            assert_eq!(
                strategy.computer_control_backend,
                ComputerControlBackend::LocalMacosInputControl
            );
        } else {
            assert_eq!(
                strategy.computer_screenshot_backend,
                ComputerScreenshotBackend::LocalWindowsScreenCapture
            );
            assert_eq!(
                strategy.computer_control_backend,
                ComputerControlBackend::LocalWindowsInputControl
            );
        }
    }

    #[test]
    fn operations_briefing_uses_deepseek_key_only_for_deepseek_provider() {
        let key = operations_briefing_deepseek_api_key_for_provider(
            |name| (name == "DEEPSEEK_API_KEY").then_some("test-secret-token".to_string()),
            LargeModelProvider::DeepSeek,
        );
        let chatgpt_key = operations_briefing_deepseek_api_key_for_provider(
            |name| (name == "DEEPSEEK_API_KEY").then_some("test-secret-token".to_string()),
            LargeModelProvider::ChatGpt,
        );

        assert_eq!(key.as_deref(), Some("test-secret-token"));
        assert_eq!(chatgpt_key, None);
    }

    #[test]
    fn operations_briefing_ignores_blank_deepseek_key() {
        let key = operations_briefing_deepseek_api_key_for_provider(
            |name| (name == "DEEPSEEK_API_KEY").then_some("  ".to_string()),
            LargeModelProvider::DeepSeek,
        );

        assert_eq!(key, None);
    }

    #[test]
    fn deepseek_telemetry_with_pricing_sets_cost_estimate() {
        let telemetry = DeepSeekChatTelemetry {
            id: Uuid::new_v4(),
            request_hash: "abc123".to_string(),
            model: DEEPSEEK_FLASH_MODEL.to_string(),
            cache_status: DeepSeekChatCacheStatus::Miss,
            elapsed_ms: 25,
            prompt_tokens: Some(1_000_000),
            completion_tokens: Some(500_000),
            total_tokens: Some(1_500_000),
            estimated_cost_micro_usd: None,
            created_at: Utc::now(),
        };
        let settings = DeepSeekPricingSettings {
            enabled: true,
            flash_prompt_usd_per_million_tokens: "0.14".to_string(),
            flash_completion_usd_per_million_tokens: "0.28".to_string(),
            ..DeepSeekPricingSettings::default()
        };

        let entries = deepseek_telemetry_with_pricing(vec![telemetry], Some(&settings));

        assert_eq!(entries[0].estimated_cost_micro_usd, Some(280_000));
    }
}
