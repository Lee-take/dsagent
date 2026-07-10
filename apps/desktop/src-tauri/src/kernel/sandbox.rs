use std::ffi::OsString;
use std::fmt;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandboxDenyRule {
    InvalidPath,
    FilesystemRoot,
    HomeRoot,
    ProtectedDirectory,
    SecretFile,
    AppConfig,
    InstallDirectory,
    UserProtected,
}

impl SandboxDenyRule {
    fn as_str(self) -> &'static str {
        match self {
            Self::InvalidPath => "invalid_path",
            Self::FilesystemRoot => "filesystem_root",
            Self::HomeRoot => "home_root",
            Self::ProtectedDirectory => "protected_directory",
            Self::SecretFile => "secret_file",
            Self::AppConfig => "app_config",
            Self::InstallDirectory => "install_directory",
            Self::UserProtected => "user_protected",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandboxViolation {
    pub rule: SandboxDenyRule,
    pub path: PathBuf,
    pub message: String,
}

impl fmt::Display for SandboxViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "DS Agent deny-first sandbox blocked `{}`: {} (rule={})",
            self.path.display(),
            self.message,
            self.rule.as_str()
        )
    }
}

impl std::error::Error for SandboxViolation {}

#[derive(Clone, Debug, Default)]
pub struct PathSandboxPolicy {
    home_dir: Option<PathBuf>,
    app_config_dirs: Vec<PathBuf>,
    install_dirs: Vec<PathBuf>,
    user_protected_dirs: Vec<PathBuf>,
}

impl PathSandboxPolicy {
    pub fn new(
        home_dir: Option<PathBuf>,
        app_config_dirs: Vec<PathBuf>,
        install_dirs: Vec<PathBuf>,
        user_protected_dirs: Vec<PathBuf>,
    ) -> Self {
        Self {
            home_dir: home_dir.map(|path| normalize_lexically(&path)),
            app_config_dirs: normalized_roots(app_config_dirs),
            install_dirs: normalized_roots(install_dirs),
            user_protected_dirs: normalized_roots(user_protected_dirs),
        }
    }

    pub fn for_current_process() -> Self {
        let home_dir = std::env::var_os("USERPROFILE")
            .or_else(|| std::env::var_os("HOME"))
            .map(PathBuf::from);
        let mut app_config_dirs = Vec::new();
        for base in ["APPDATA", "LOCALAPPDATA"] {
            if let Some(base) = std::env::var_os(base) {
                let base = PathBuf::from(base);
                app_config_dirs.push(base.join("ai.deepseek-agent-os.desktop"));
                app_config_dirs.push(base.join("DS Agent"));
            }
        }
        if let Some(path) = std::env::var_os("DEEPSEEK_AGENT_OS_APP_DATA_DIR") {
            app_config_dirs.push(PathBuf::from(path));
        }
        if let Some(home) = &home_dir {
            app_config_dirs.push(home.join(".config/deepseek-agent-os"));
            app_config_dirs.push(home.join(".config/ds-agent"));
        }

        let install_dirs = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(Path::to_path_buf))
            .into_iter()
            .collect();
        let user_protected_dirs = std::env::var_os("DS_AGENT_PROTECTED_PATHS")
            .map(|value| std::env::split_paths(&value).collect())
            .unwrap_or_default();

        Self::new(home_dir, app_config_dirs, install_dirs, user_protected_dirs)
    }

    pub fn validate_mutation_path(&self, path: &Path) -> Result<(), SandboxViolation> {
        self.validate_local_path(path)
    }

    pub fn validate_read_path(&self, path: &Path) -> Result<(), SandboxViolation> {
        self.validate_local_path(path)
    }

    fn validate_local_path(&self, path: &Path) -> Result<(), SandboxViolation> {
        if path.as_os_str().is_empty() || !path.is_absolute() {
            return Err(violation(
                SandboxDenyRule::InvalidPath,
                path,
                "local filesystem targets must be absolute local paths",
            ));
        }

        let mut candidates = vec![normalize_lexically(path)];
        if let Some(resolved) = resolve_existing_path(path) {
            let resolved = normalize_lexically(&resolved);
            if !candidates
                .iter()
                .any(|candidate| paths_equal(candidate, &resolved))
            {
                candidates.push(resolved);
            }
        }

        for candidate in candidates {
            self.validate_candidate(&candidate)?;
        }
        Ok(())
    }

    fn validate_candidate(&self, path: &Path) -> Result<(), SandboxViolation> {
        if path.file_name().is_none() {
            return Err(violation(
                SandboxDenyRule::FilesystemRoot,
                path,
                "filesystem roots are protected from direct local tool access",
            ));
        }
        if self
            .home_dir
            .as_ref()
            .is_some_and(|home| paths_equal(path, home))
        {
            return Err(violation(
                SandboxDenyRule::HomeRoot,
                path,
                "the user home root is protected from bulk local tool access",
            ));
        }
        if let Some(root) = matching_root(path, &self.app_config_dirs) {
            return Err(violation(
                SandboxDenyRule::AppConfig,
                path,
                format!(
                    "DS Agent application state is protected under `{}`",
                    root.display()
                ),
            ));
        }
        if let Some(root) = matching_root(path, &self.install_dirs) {
            return Err(violation(
                SandboxDenyRule::InstallDirectory,
                path,
                format!(
                    "the running DS Agent installation is protected under `{}`",
                    root.display()
                ),
            ));
        }
        if let Some(root) = matching_root(path, &self.user_protected_dirs) {
            return Err(violation(
                SandboxDenyRule::UserProtected,
                path,
                format!("the user protected this folder: `{}`", root.display()),
            ));
        }
        if let Some(component) = protected_directory_component(path) {
            return Err(violation(
                SandboxDenyRule::ProtectedDirectory,
                path,
                format!("protected directory component `{component}` cannot be accessed"),
            ));
        }
        if let Some(file_name) = protected_file_name(path) {
            return Err(violation(
                SandboxDenyRule::SecretFile,
                path,
                format!("secret or package-manager config file `{file_name}` cannot be accessed"),
            ));
        }
        Ok(())
    }
}

pub fn enforce_local_mutation_path(path: &Path) -> Result<(), String> {
    PathSandboxPolicy::for_current_process()
        .validate_mutation_path(path)
        .map_err(|violation| violation.to_string())
}

pub fn enforce_local_read_path(path: &Path) -> Result<(), String> {
    PathSandboxPolicy::for_current_process()
        .validate_read_path(path)
        .map_err(|violation| violation.to_string())
}

pub fn enforce_workspace_relative_mutation_path(path: &str) -> Result<(), String> {
    let path = Path::new(path.trim());
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(violation(
            SandboxDenyRule::InvalidPath,
            path,
            "workspace tool paths must be relative",
        )
        .to_string());
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(violation(
            SandboxDenyRule::InvalidPath,
            path,
            "workspace tool paths cannot contain parent-directory traversal",
        )
        .to_string());
    }
    if let Some(component) = protected_directory_component(path) {
        return Err(violation(
            SandboxDenyRule::ProtectedDirectory,
            path,
            format!("protected directory component `{component}` cannot be mutated"),
        )
        .to_string());
    }
    if let Some(file_name) = protected_file_name(path) {
        return Err(violation(
            SandboxDenyRule::SecretFile,
            path,
            format!("secret or package-manager config file `{file_name}` cannot be mutated"),
        )
        .to_string());
    }
    Ok(())
}

pub fn enforce_workspace_relative_read_path(path: &str) -> Result<(), String> {
    let path = Path::new(path.trim());
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(violation(
            SandboxDenyRule::InvalidPath,
            path,
            "workspace read paths must be relative",
        )
        .to_string());
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(violation(
            SandboxDenyRule::InvalidPath,
            path,
            "workspace read paths cannot contain parent-directory traversal",
        )
        .to_string());
    }
    if let Some(component) = protected_directory_component(path) {
        return Err(violation(
            SandboxDenyRule::ProtectedDirectory,
            path,
            format!("protected directory component `{component}` cannot be accessed"),
        )
        .to_string());
    }
    if let Some(file_name) = protected_file_name(path) {
        return Err(violation(
            SandboxDenyRule::SecretFile,
            path,
            format!("secret or package-manager config file `{file_name}` cannot be accessed"),
        )
        .to_string());
    }
    Ok(())
}

fn violation(rule: SandboxDenyRule, path: &Path, message: impl Into<String>) -> SandboxViolation {
    SandboxViolation {
        rule,
        path: path.to_path_buf(),
        message: message.into(),
    }
}

fn normalized_roots(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .into_iter()
        .filter(|path| path.is_absolute())
        .map(|path| normalize_lexically(&path))
        .collect()
}

fn normalize_lexically(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

fn resolve_existing_path(path: &Path) -> Option<PathBuf> {
    if path.exists() {
        return path.canonicalize().ok();
    }

    let mut ancestor = path;
    let mut suffix = Vec::<OsString>::new();
    while !ancestor.exists() {
        suffix.push(ancestor.file_name()?.to_os_string());
        ancestor = ancestor.parent()?;
    }
    let mut resolved = ancestor.canonicalize().ok()?;
    for component in suffix.into_iter().rev() {
        resolved.push(component);
    }
    Some(resolved)
}

fn matching_root<'a>(path: &Path, roots: &'a [PathBuf]) -> Option<&'a PathBuf> {
    roots
        .iter()
        .find(|root| path_is_same_or_descendant(path, root))
}

fn path_is_same_or_descendant(path: &Path, root: &Path) -> bool {
    let path = comparison_key(path);
    let root = comparison_key(root);
    path == root
        || path
            .strip_prefix(&root)
            .is_some_and(|suffix| suffix.starts_with(std::path::MAIN_SEPARATOR))
}

fn paths_equal(left: &Path, right: &Path) -> bool {
    comparison_key(left) == comparison_key(right)
}

fn comparison_key(path: &Path) -> String {
    let key = normalize_lexically(path).to_string_lossy().to_string();
    #[cfg(windows)]
    {
        key.to_ascii_lowercase()
    }
    #[cfg(not(windows))]
    {
        key
    }
}

fn protected_directory_component(path: &Path) -> Option<String> {
    const PROTECTED: &[&str] = &[
        ".git", ".ssh", ".gnupg", ".aws", ".azure", ".kube", ".docker",
    ];
    path.components().find_map(|component| {
        let Component::Normal(component) = component else {
            return None;
        };
        let component = component.to_string_lossy();
        PROTECTED
            .iter()
            .any(|protected| component.eq_ignore_ascii_case(protected))
            .then(|| component.to_string())
    })
}

fn protected_file_name(path: &Path) -> Option<String> {
    let file_name = path.file_name()?.to_string_lossy();
    let normalized = file_name.to_ascii_lowercase();
    let exact = matches!(
        normalized.as_str(),
        ".npmrc"
            | ".yarnrc"
            | ".yarnrc.yml"
            | ".pypirc"
            | "pip.conf"
            | "nuget.config"
            | "credentials"
            | "credentials.json"
            | "id_rsa"
            | "id_ed25519"
    );
    let env_file = normalized == ".env" || normalized.starts_with(".env.");
    let secret_extension = path
        .extension()
        .map(|extension| extension.to_string_lossy().to_ascii_lowercase())
        .is_some_and(|extension| {
            matches!(extension.as_str(), "pem" | "key" | "pfx" | "p12" | "kdbx")
        });
    (exact || env_file || secret_extension).then(|| file_name.to_string())
}

#[cfg(test)]
mod tests {
    use super::{PathSandboxPolicy, SandboxDenyRule};

    #[test]
    fn deny_first_path_policy_blocks_protected_roots_components_and_secret_files() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        let home_dir = temp_dir.path().join("home");
        let app_config_dir = temp_dir.path().join("app-config");
        let install_dir = temp_dir.path().join("install");
        let user_protected_dir = temp_dir.path().join("protected");
        let workspace_dir = temp_dir.path().join("workspace");
        let policy = PathSandboxPolicy::new(
            Some(home_dir.clone()),
            vec![app_config_dir.clone()],
            vec![install_dir.clone()],
            vec![user_protected_dir.clone()],
        );
        let filesystem_root = temp_dir.path().ancestors().last().expect("filesystem root");

        assert_eq!(
            policy
                .validate_mutation_path(filesystem_root)
                .expect_err("filesystem root is denied")
                .rule,
            SandboxDenyRule::FilesystemRoot
        );
        assert_eq!(
            policy
                .validate_mutation_path(&home_dir)
                .expect_err("home root is denied")
                .rule,
            SandboxDenyRule::HomeRoot
        );
        assert_eq!(
            policy
                .validate_mutation_path(&workspace_dir.join(".git/config"))
                .expect_err("git metadata is denied")
                .rule,
            SandboxDenyRule::ProtectedDirectory
        );
        assert_eq!(
            policy
                .validate_mutation_path(&home_dir.join(".ssh/id_ed25519"))
                .expect_err("credential directory is denied")
                .rule,
            SandboxDenyRule::ProtectedDirectory
        );
        assert_eq!(
            policy
                .validate_mutation_path(&workspace_dir.join(".env.local"))
                .expect_err("secret file is denied")
                .rule,
            SandboxDenyRule::SecretFile
        );
        assert_eq!(
            policy
                .validate_mutation_path(&app_config_dir.join("settings.json"))
                .expect_err("app config is denied")
                .rule,
            SandboxDenyRule::AppConfig
        );
        assert_eq!(
            policy
                .validate_mutation_path(&install_dir.join("DS Agent.exe"))
                .expect_err("install directory is denied")
                .rule,
            SandboxDenyRule::InstallDirectory
        );
        assert_eq!(
            policy
                .validate_mutation_path(&user_protected_dir.join("records.db"))
                .expect_err("user protected directory is denied")
                .rule,
            SandboxDenyRule::UserProtected
        );
        assert!(policy
            .validate_mutation_path(&workspace_dir.join("reports/briefing.md"))
            .is_ok());
    }

    #[test]
    fn deny_first_path_policy_blocks_package_manager_and_private_key_files() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        let policy = PathSandboxPolicy::new(None, Vec::new(), Vec::new(), Vec::new());

        for name in [
            ".npmrc",
            "pip.conf",
            "NuGet.Config",
            "signing.pem",
            "account.pfx",
        ] {
            let violation = policy
                .validate_mutation_path(&temp_dir.path().join(name))
                .expect_err("sensitive config is denied");
            assert_eq!(violation.rule, SandboxDenyRule::SecretFile, "{name}");
        }
    }
}
