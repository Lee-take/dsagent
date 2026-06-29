use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub const LOCAL_DIRECTORY_SETTINGS_FILE: &str = "local-directories.json";

#[derive(Debug, thiserror::Error)]
pub enum LocalDirectoryError {
    #[error("workspace directory is required")]
    MissingWorkspace,

    #[error("evidence directory is required")]
    MissingEvidence,

    #[error("export directory is required")]
    MissingExport,

    #[error("local directory settings could not be read: {0}")]
    Read(std::io::Error),

    #[error("local directory settings could not be written: {0}")]
    Write(std::io::Error),

    #[error("local directory settings are invalid json: {0}")]
    Json(serde_json::Error),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocalDirectorySettings {
    pub workspace_dir: String,
    pub evidence_dir: String,
    pub export_dir: String,
}

impl LocalDirectorySettings {
    pub fn new(
        workspace_dir: String,
        evidence_dir: String,
        export_dir: String,
    ) -> Result<Self, LocalDirectoryError> {
        let workspace_dir = workspace_dir.trim().to_string();
        let evidence_dir = evidence_dir.trim().to_string();
        let export_dir = export_dir.trim().to_string();

        if workspace_dir.is_empty() {
            return Err(LocalDirectoryError::MissingWorkspace);
        }
        if evidence_dir.is_empty() {
            return Err(LocalDirectoryError::MissingEvidence);
        }
        if export_dir.is_empty() {
            return Err(LocalDirectoryError::MissingExport);
        }

        Ok(Self {
            workspace_dir,
            evidence_dir,
            export_dir,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocalDirectoryState {
    pub app_data_dir: String,
    pub settings_file: String,
    pub settings: Option<LocalDirectorySettings>,
    pub needs_setup: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocalDirectoryReadinessStatus {
    pub needs_setup: bool,
    pub workspace_configured: bool,
    pub evidence_configured: bool,
    pub export_configured: bool,
    pub paths_redacted: bool,
    pub note: String,
}

impl Default for LocalDirectoryReadinessStatus {
    fn default() -> Self {
        Self {
            needs_setup: true,
            workspace_configured: false,
            evidence_configured: false,
            export_configured: false,
            paths_redacted: true,
            note: "local workspace, evidence, and export directories need setup on this machine"
                .to_string(),
        }
    }
}

pub fn local_directory_readiness_from_state(
    state: &LocalDirectoryState,
) -> LocalDirectoryReadinessStatus {
    let Some(settings) = state.settings.as_ref() else {
        return LocalDirectoryReadinessStatus::default();
    };
    let workspace_configured = !settings.workspace_dir.trim().is_empty();
    let evidence_configured = !settings.evidence_dir.trim().is_empty();
    let export_configured = !settings.export_dir.trim().is_empty();
    let needs_setup =
        state.needs_setup || !workspace_configured || !evidence_configured || !export_configured;

    LocalDirectoryReadinessStatus {
        needs_setup,
        workspace_configured,
        evidence_configured,
        export_configured,
        paths_redacted: true,
        note: if needs_setup {
            "local directory settings are incomplete on this machine".to_string()
        } else {
            "local workspace, evidence, and export directories are configured; paths are redacted"
                .to_string()
        },
    }
}

pub fn load_local_directory_state(
    app_data_dir: impl AsRef<Path>,
) -> Result<LocalDirectoryState, LocalDirectoryError> {
    let app_data_dir = app_data_dir.as_ref();
    let settings_file = app_data_dir.join(LOCAL_DIRECTORY_SETTINGS_FILE);
    let settings = if settings_file.exists() {
        let settings_json =
            fs::read_to_string(&settings_file).map_err(LocalDirectoryError::Read)?;
        Some(serde_json::from_str(&settings_json).map_err(LocalDirectoryError::Json)?)
    } else {
        None
    };

    Ok(LocalDirectoryState {
        app_data_dir: app_data_dir.to_string_lossy().to_string(),
        settings_file: settings_file.to_string_lossy().to_string(),
        needs_setup: settings.is_none(),
        settings,
    })
}

pub fn save_local_directory_settings(
    app_data_dir: impl AsRef<Path>,
    settings: LocalDirectorySettings,
) -> Result<LocalDirectoryState, LocalDirectoryError> {
    let app_data_dir = app_data_dir.as_ref();
    fs::create_dir_all(app_data_dir).map_err(LocalDirectoryError::Write)?;
    let settings_file = app_data_dir.join(LOCAL_DIRECTORY_SETTINGS_FILE);
    let settings_json =
        serde_json::to_string_pretty(&settings).map_err(LocalDirectoryError::Json)?;
    fs::write(&settings_file, settings_json).map_err(LocalDirectoryError::Write)?;

    load_local_directory_state(app_data_dir)
}

#[cfg(test)]
mod tests {
    use super::{
        load_local_directory_state, save_local_directory_settings, LocalDirectorySettings,
        LOCAL_DIRECTORY_SETTINGS_FILE,
    };

    #[test]
    fn missing_settings_requires_first_run_setup() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        let state = load_local_directory_state(temp_dir.path()).expect("state loads");

        assert!(state.needs_setup);
        assert!(state.settings.is_none());
        assert_eq!(state.app_data_dir, temp_dir.path().to_string_lossy());
        assert!(state.settings_file.ends_with(LOCAL_DIRECTORY_SETTINGS_FILE));
    }

    #[test]
    fn save_then_load_local_directory_settings() {
        let temp_dir = tempfile::tempdir().expect("temp dir");

        let saved = save_local_directory_settings(
            temp_dir.path(),
            LocalDirectorySettings::new(
                "  fixtures/workspace  ".to_string(),
                "  fixtures/evidence  ".to_string(),
                "  fixtures/exports  ".to_string(),
            )
            .expect("settings validate"),
        )
        .expect("settings save");

        assert!(!saved.needs_setup);
        assert_eq!(
            saved
                .settings
                .as_ref()
                .expect("saved settings")
                .workspace_dir,
            "fixtures/workspace"
        );

        let loaded = load_local_directory_state(temp_dir.path()).expect("state reloads");
        assert_eq!(loaded, saved);
    }

    #[test]
    fn local_directory_settings_reject_blank_required_paths() {
        let error = LocalDirectorySettings::new(
            " ".to_string(),
            "fixtures/evidence".to_string(),
            "fixtures/exports".to_string(),
        )
        .expect_err("blank workspace should fail");

        assert_eq!(error.to_string(), "workspace directory is required");
    }
}
