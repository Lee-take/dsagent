use crate::kernel::computer_use::{
    computer_use_backend_status_for_strategy, ComputerUseBackendStatus,
};
use crate::kernel::deepseek::{deepseek_credential_status_from_env, DeepSeekCredentialStatus};
use crate::kernel::local_directory::LocalDirectoryReadinessStatus;
use crate::kernel::models::{FoundationState, MemoryCandidate, TaskRecord};
use crate::kernel::network_search::{
    network_search_route_status_for_strategy, NetworkSearchRouteStatus,
};
use crate::kernel::tool_strategy::{
    model_driven_tool_strategy_for_current_platform, ModelDrivenToolStrategy,
};
use crate::kernel::workflow::{
    operations_briefing_workflow_template_package, OperationsBriefingRun, WorkflowTemplatePackage,
};

pub const WORK_PACKAGE_VERSION: &str = "deepseek-agent-os.work-package.v1";

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum WorkPackageError {
    #[error("invalid work package json: {0}")]
    InvalidJson(String),

    #[error("unsupported work package version: {0}")]
    UnsupportedVersion(String),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackage {
    pub version: String,
    pub exported_at: chrono::DateTime<chrono::Utc>,
    pub foundation_state: FoundationState,
    #[serde(default)]
    pub tool_readiness: WorkPackageToolReadiness,
    pub task_records: Vec<TaskRecord>,
    #[serde(default)]
    pub memory_candidates: Vec<MemoryCandidate>,
    #[serde(default)]
    pub operations_briefing_runs: Vec<OperationsBriefingRun>,
    #[serde(default)]
    pub workflow_templates: Vec<WorkflowTemplatePackage>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageToolReadiness {
    pub deepseek: DeepSeekCredentialStatus,
    pub network_search: NetworkSearchRouteStatus,
    pub computer_use: ComputerUseBackendStatus,
    #[serde(default)]
    pub local_directories: LocalDirectoryReadinessStatus,
    #[serde(default)]
    pub tool_strategy: ModelDrivenToolStrategy,
}

impl Default for WorkPackageToolReadiness {
    fn default() -> Self {
        let deepseek = deepseek_credential_status_from_env(|_| None);
        let foundation_state = FoundationState::default();
        let tool_strategy = model_driven_tool_strategy_for_current_platform(
            foundation_state.large_model_provider,
            foundation_state.network_search_source_model,
        );
        Self {
            network_search: network_search_route_status_for_strategy(
                &tool_strategy,
                deepseek.chat_completion_ready,
            ),
            deepseek,
            computer_use: computer_use_backend_status_for_strategy(&tool_strategy),
            local_directories: LocalDirectoryReadinessStatus::default(),
            tool_strategy,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageImportSummary {
    pub imported: usize,
    pub skipped: usize,
    pub memory_candidates: WorkPackageMemoryCandidateImportSummary,
    pub operations_briefing_runs: WorkPackageOperationsBriefingImportSummary,
    pub workflow_templates: WorkPackageWorkflowTemplateImportSummary,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageMemoryCandidateImportSummary {
    pub imported: usize,
    pub skipped: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageOperationsBriefingImportSummary {
    pub imported: usize,
    pub skipped: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageWorkflowTemplateImportSummary {
    pub imported: usize,
    pub skipped: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageTaskImportPreview {
    pub total: usize,
    pub new: usize,
    pub skipped: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageOperationsBriefingImportPreview {
    pub total: usize,
    pub replay_supported: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageWorkflowTemplateImportPreview {
    pub total: usize,
    pub new: usize,
    pub skipped: usize,
    pub import_supported: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageMemoryCandidateImportPreview {
    pub total: usize,
    pub new: usize,
    pub skipped: usize,
    pub review_supported: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WorkPackageImportPreview {
    pub task_records: WorkPackageTaskImportPreview,
    pub memory_candidates: WorkPackageMemoryCandidateImportPreview,
    pub operations_briefing_runs: WorkPackageOperationsBriefingImportPreview,
    pub workflow_templates: WorkPackageWorkflowTemplateImportPreview,
}

#[cfg(test)]
pub fn export_work_package(
    foundation_state: FoundationState,
    task_records: Vec<TaskRecord>,
    memory_candidates: Vec<MemoryCandidate>,
    operations_briefing_runs: Vec<OperationsBriefingRun>,
) -> WorkPackage {
    export_work_package_with_tool_readiness(
        foundation_state,
        task_records,
        memory_candidates,
        operations_briefing_runs,
        WorkPackageToolReadiness::default(),
    )
}

pub fn export_work_package_with_tool_readiness(
    foundation_state: FoundationState,
    task_records: Vec<TaskRecord>,
    memory_candidates: Vec<MemoryCandidate>,
    operations_briefing_runs: Vec<OperationsBriefingRun>,
    tool_readiness: WorkPackageToolReadiness,
) -> WorkPackage {
    WorkPackage {
        version: WORK_PACKAGE_VERSION.to_string(),
        exported_at: chrono::Utc::now(),
        foundation_state,
        tool_readiness,
        task_records,
        memory_candidates,
        operations_briefing_runs,
        workflow_templates: vec![operations_briefing_workflow_template_package()],
    }
}

pub fn parse_work_package_json(package_json: &str) -> Result<WorkPackage, WorkPackageError> {
    let package = serde_json::from_str::<WorkPackage>(package_json)
        .map_err(|error| WorkPackageError::InvalidJson(error.to_string()))?;

    if package.version != WORK_PACKAGE_VERSION {
        return Err(WorkPackageError::UnsupportedVersion(package.version));
    }

    Ok(package)
}

#[cfg(test)]
mod tests {
    use super::{
        export_work_package, export_work_package_with_tool_readiness, parse_work_package_json,
        WorkPackageError, WorkPackageToolReadiness,
    };
    use crate::kernel::deepseek::{
        deepseek_credential_status_from_env, DeepSeekCredentialStatus, DEEPSEEK_API_KEY_ENV,
    };
    use crate::kernel::local_directory::{
        local_directory_readiness_from_state, LocalDirectorySettings, LocalDirectoryState,
    };
    use crate::kernel::models::{
        FoundationState, MemoryCandidate, MemoryCandidateSource, MemoryLifecycle, MemoryScope,
        MemorySensitivity, MemoryType, TaskRecord,
    };
    use crate::kernel::workflow::{
        OperationsBriefingAction, OperationsBriefingAnomaly, OperationsBriefingRun,
        OperationsBriefingRunStatus, OPERATIONS_BRIEFING_WORKFLOW_ID,
    };

    fn sample_operations_briefing_run() -> OperationsBriefingRun {
        OperationsBriefingRun {
            id: uuid::Uuid::new_v4(),
            workflow_id: OPERATIONS_BRIEFING_WORKFLOW_ID.to_string(),
            status: OperationsBriefingRunStatus::DraftReady,
            archived_from_package: false,
            evidence_folder_path: Some("fixtures/evidence".to_string()),
            evidence_invocation_id: Some(uuid::Uuid::new_v4()),
            title: "Operations Briefing Draft".to_string(),
            summary: "Draft ready from evidence manifest.".to_string(),
            anomalies: vec![OperationsBriefingAnomaly {
                area: "Revenue".to_string(),
                signal: "Room revenue improved by 6 percent.".to_string(),
                evidence_ref: Some("fixtures/evidence".to_string()),
            }],
            action_plan: vec![OperationsBriefingAction {
                owner: "Operations owner".to_string(),
                action: "Confirm owner follow-up items.".to_string(),
                due_hint: "Next briefing cycle".to_string(),
            }],
            warnings: vec!["One evidence file was skipped.".to_string()],
            created_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn exports_versioned_work_package_with_task_records() {
        let record = TaskRecord::new(
            "Prepare sales briefing".to_string(),
            "Summarize inbox and drive evidence for Monday review.".to_string(),
        )
        .expect("record is valid");

        let package = export_work_package(
            FoundationState::default(),
            vec![record.clone()],
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(package.version, "deepseek-agent-os.work-package.v1");
        assert!(package.exported_at <= chrono::Utc::now());
        assert_eq!(package.foundation_state, FoundationState::default());
        assert_eq!(package.task_records, vec![record]);
        assert!(package.operations_briefing_runs.is_empty());
    }

    #[test]
    fn operations_export_package_includes_briefing_runs() {
        let run = sample_operations_briefing_run();

        let package = export_work_package(
            FoundationState::default(),
            Vec::new(),
            Vec::new(),
            vec![run.clone()],
        );

        assert_eq!(package.operations_briefing_runs, vec![run]);
    }

    #[test]
    fn workflow_template_export_package_includes_default_operations_templates() {
        let package = export_work_package(
            FoundationState::default(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(package.workflow_templates.len(), 1);
        assert_eq!(
            package.workflow_templates[0].workflow_id,
            OPERATIONS_BRIEFING_WORKFLOW_ID
        );
        assert_eq!(package.workflow_templates[0].files.len(), 4);
    }

    #[test]
    fn imported_memory_candidate_export_package_includes_candidates() {
        let candidate = MemoryCandidate::new_with_metadata(
            "Review-safe project rule".to_string(),
            "Imported package candidates must stay pending until accepted locally.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "Package export should preserve review candidates.".to_string(),
            MemoryType::WorkflowRule,
            MemoryScope::Project,
            MemorySensitivity::Normal,
            MemoryLifecycle::Active,
        )
        .expect("candidate is valid");

        let package = export_work_package(
            FoundationState::default(),
            Vec::new(),
            vec![candidate.clone()],
            Vec::new(),
        );

        assert_eq!(package.memory_candidates, vec![candidate]);
    }

    #[test]
    fn imported_memory_candidate_legacy_package_defaults_candidates() {
        let package_json = serde_json::json!({
            "version": "deepseek-agent-os.work-package.v1",
            "exported_at": chrono::Utc::now(),
            "foundation_state": FoundationState::default(),
            "task_records": [],
            "operations_briefing_runs": []
        })
        .to_string();

        let package = parse_work_package_json(&package_json).expect("legacy package parses");

        assert!(package.memory_candidates.is_empty());
    }

    #[test]
    fn tool_readiness_export_package_stays_secret_safe() {
        let deepseek_status = deepseek_credential_status_from_env(|name| {
            if name == DEEPSEEK_API_KEY_ENV {
                Some("test-secret-token".to_string())
            } else {
                None
            }
        });
        let package = export_work_package_with_tool_readiness(
            FoundationState::default(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            WorkPackageToolReadiness {
                deepseek: deepseek_status,
                ..WorkPackageToolReadiness::default()
            },
        );

        let package_json = serde_json::to_string(&package).expect("package serializes");

        assert!(package.tool_readiness.deepseek.api_key_configured);
        assert!(package_json.contains(DEEPSEEK_API_KEY_ENV));
        assert!(package_json.contains("pending_user_confirmation"));
        assert!(package_json.contains("\"local_directories\""));
        if cfg!(target_os = "macos") {
            assert!(package_json.contains("local_macos_screen_capture"));
        } else {
            assert!(package_json.contains("local_windows_screen_capture"));
        }
        assert!(!package_json.contains("test-secret-token"));
    }

    #[test]
    fn tool_readiness_local_directories_reports_setup_without_serializing_paths() {
        let state = LocalDirectoryState {
            app_data_dir: "C:\\Users\\alice\\AppData\\Roaming\\deepseek-agent-os".to_string(),
            settings_file:
                "C:\\Users\\alice\\AppData\\Roaming\\deepseek-agent-os\\local-directories.json"
                    .to_string(),
            settings: Some(
                LocalDirectorySettings::new(
                    "D:\\Private\\workspace".to_string(),
                    "D:\\Private\\evidence".to_string(),
                    "D:\\Private\\exports".to_string(),
                )
                .expect("settings validate"),
            ),
            needs_setup: false,
        };
        let package = export_work_package_with_tool_readiness(
            FoundationState::default(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            WorkPackageToolReadiness {
                local_directories: local_directory_readiness_from_state(&state),
                ..WorkPackageToolReadiness::default()
            },
        );

        let package_json = serde_json::to_string(&package).expect("package serializes");

        assert!(!package.tool_readiness.local_directories.needs_setup);
        assert!(
            package
                .tool_readiness
                .local_directories
                .workspace_configured
        );
        assert!(package.tool_readiness.local_directories.evidence_configured);
        assert!(package.tool_readiness.local_directories.export_configured);
        assert!(package.tool_readiness.local_directories.paths_redacted);
        assert!(!package_json.contains("D:\\Private"));
        assert!(!package_json.contains("alice"));
    }

    #[test]
    fn tool_readiness_legacy_package_json_defaults_readiness() {
        let package_json = serde_json::json!({
            "version": "deepseek-agent-os.work-package.v1",
            "exported_at": chrono::Utc::now(),
            "foundation_state": FoundationState::default(),
            "task_records": [],
            "memory_candidates": [],
            "operations_briefing_runs": []
        })
        .to_string();

        let package = parse_work_package_json(&package_json).expect("legacy package parses");

        assert_eq!(
            package.tool_readiness.deepseek,
            DeepSeekCredentialStatus {
                base_url: "https://api.deepseek.com".to_string(),
                chat_completions_url: "https://api.deepseek.com/chat/completions".to_string(),
                api_key_env_var: DEEPSEEK_API_KEY_ENV.to_string(),
                api_key_configured: false,
                chat_completion_ready: false,
                flash_model: "deepseek-v4-flash".to_string(),
                pro_model: "deepseek-v4-pro".to_string(),
                readiness_note:
                    "set DEEPSEEK_API_KEY in the local process environment to enable Chat Completions requests"
                        .to_string(),
            }
        );
        assert_eq!(package.tool_readiness, WorkPackageToolReadiness::default());
    }

    #[test]
    fn operations_export_legacy_package_json_defaults_briefing_runs() {
        let package_json = serde_json::json!({
            "version": "deepseek-agent-os.work-package.v1",
            "exported_at": chrono::Utc::now(),
            "foundation_state": FoundationState::default(),
            "task_records": []
        })
        .to_string();

        let package = parse_work_package_json(&package_json).expect("legacy package parses");

        assert!(package.operations_briefing_runs.is_empty());
        assert!(package.workflow_templates.is_empty());
    }

    #[test]
    fn rejects_work_package_with_unknown_version() {
        let mut package = export_work_package(
            FoundationState::default(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        package.version = "deepseek-agent-os.work-package.v0".to_string();
        let package_json = serde_json::to_string(&package).expect("package serializes");

        let error = parse_work_package_json(&package_json).expect_err("version should fail");

        assert_eq!(
            error,
            WorkPackageError::UnsupportedVersion("deepseek-agent-os.work-package.v0".to_string())
        );
    }
}
