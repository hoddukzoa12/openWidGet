use serde::Serialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const ANCHOR_FILE_PREFIX: &str = "OpenWidGet Anchor";
const ANCHOR_ICON_BYTES: &[u8] = include_bytes!("../icons/anchor-transparent.ico");

#[derive(Debug, Clone, Serialize)]
pub struct AnchorShortcut {
    pub anchor_id: String,
    pub widget_id: String,
    pub session_id: String,
    pub row: u8,
    pub column: u8,
    pub filename: String,
    pub description: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnchorLifecycleStatus {
    pub platform: &'static str,
    pub enabled: bool,
    pub session_id: String,
    pub desktop_dir: String,
    pub state_dir: String,
    pub anchors_planned: usize,
    pub anchors_created: usize,
    pub stale_detected: usize,
    pub stale_deleted: usize,
    pub anchor_files: Vec<String>,
    pub last_error: Option<String>,
}

impl AnchorLifecycleStatus {
    pub fn unsupported(manager: &AnchorShortcutManager, anchors_planned: usize) -> Self {
        Self {
            platform: std::env::consts::OS,
            enabled: false,
            session_id: manager.session_id.clone(),
            desktop_dir: path_for_status(&manager.desktop_dir),
            state_dir: path_for_status(&manager.state_dir),
            anchors_planned,
            anchors_created: 0,
            stale_detected: 0,
            stale_deleted: 0,
            anchor_files: Vec::new(),
            last_error: Some(
                "Anchor Shortcut materialization is Windows-only; this platform only verifies the plan."
                    .to_string(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnchorShortcutManager {
    session_id: String,
    desktop_dir: PathBuf,
    state_dir: PathBuf,
    target_exe: PathBuf,
    icon_path: PathBuf,
}

impl AnchorShortcutManager {
    pub fn for_runtime() -> Result<Self, String> {
        let session_id = generate_session_id();
        let desktop_dir = resolve_desktop_dir();
        let state_dir = resolve_state_dir().join("anchor-shortcuts");
        let target_exe =
            env::current_exe().map_err(|error| format!("current_exe failed: {error}"))?;
        let icon_path = state_dir.join("openwidget-anchor-transparent.ico");

        Ok(Self {
            session_id,
            desktop_dir,
            state_dir,
            target_exe,
            icon_path,
        })
    }

    pub fn prepare_startup_sample(&self) -> AnchorLifecycleStatus {
        let anchors = build_anchor_plan("clock", 2, 2, &self.session_id);

        if !cfg!(windows) {
            return AnchorLifecycleStatus::unsupported(self, anchors.len());
        }

        match self.prepare_startup_sample_windows(&anchors) {
            Ok(status) => status,
            Err(error) => AnchorLifecycleStatus {
                platform: std::env::consts::OS,
                enabled: false,
                session_id: self.session_id.clone(),
                desktop_dir: path_for_status(&self.desktop_dir),
                state_dir: path_for_status(&self.state_dir),
                anchors_planned: anchors.len(),
                anchors_created: 0,
                stale_detected: 0,
                stale_deleted: 0,
                anchor_files: Vec::new(),
                last_error: Some(error),
            },
        }
    }

    fn prepare_startup_sample_windows(
        &self,
        anchors: &[AnchorShortcut],
    ) -> Result<AnchorLifecycleStatus, String> {
        fs::create_dir_all(&self.state_dir)
            .map_err(|error| format!("failed to create state dir {:?}: {error}", self.state_dir))?;
        fs::create_dir_all(&self.desktop_dir).map_err(|error| {
            format!(
                "failed to access desktop dir {:?}: {error}",
                self.desktop_dir
            )
        })?;
        fs::write(&self.icon_path, ANCHOR_ICON_BYTES)
            .map_err(|error| format!("failed to materialize transparent icon: {error}"))?;

        let stale = find_stale_anchor_shortcuts_in(&self.desktop_dir, &self.session_id)
            .map_err(|error| format!("failed to scan desktop anchors: {error}"))?;
        let stale_detected = stale.len();
        let mut stale_deleted = 0;
        for path in stale {
            if fs::remove_file(&path).is_ok() {
                stale_deleted += 1;
            }
        }

        let mut created = Vec::new();
        for anchor in anchors {
            let shortcut_path = self.desktop_dir.join(&anchor.filename);
            create_shortcut(
                &shortcut_path,
                &self.target_exe,
                &anchor.arguments,
                &anchor.description,
                &self.icon_path,
            )?;
            created.push(path_for_status(&shortcut_path));
        }

        Ok(AnchorLifecycleStatus {
            platform: std::env::consts::OS,
            enabled: true,
            session_id: self.session_id.clone(),
            desktop_dir: path_for_status(&self.desktop_dir),
            state_dir: path_for_status(&self.state_dir),
            anchors_planned: anchors.len(),
            anchors_created: created.len(),
            stale_detected,
            stale_deleted,
            anchor_files: created,
            last_error: None,
        })
    }

    pub fn cleanup_current_session(&self) -> Result<usize, String> {
        let anchors = build_anchor_plan("clock", 2, 2, &self.session_id);
        let mut deleted = 0;

        for anchor in anchors {
            let path = self.desktop_dir.join(anchor.filename);
            match fs::remove_file(&path) {
                Ok(()) => deleted += 1,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(format!("failed to delete {:?}: {error}", path)),
            }
        }

        Ok(deleted)
    }
}

pub fn build_anchor_plan(
    widget_id: &str,
    rows: u8,
    columns: u8,
    session_id: &str,
) -> Vec<AnchorShortcut> {
    let widget = sanitize_segment(widget_id);
    let session = sanitize_segment(session_id);
    let mut anchors = Vec::with_capacity((rows as usize) * (columns as usize));

    for row in 1..=rows {
        for column in 1..=columns {
            let slot = format!("r{row}c{column}");
            let anchor_id = format!("{widget}-{session}-{slot}");
            let filename = format!("{ANCHOR_FILE_PREFIX} - {widget} - {session} - {slot}.lnk");
            let description = format!(
                "OpenWidGet runtime Anchor Shortcut; widget={widget}; session={session}; slot={slot}; delete-on-exit=true"
            );
            let arguments = format!(
                "--open-anchor --widget-id {widget} --anchor-id {anchor_id} --session-id {session}"
            );

            anchors.push(AnchorShortcut {
                anchor_id,
                widget_id: widget.clone(),
                session_id: session.clone(),
                row,
                column,
                filename,
                description,
                arguments,
            });
        }
    }

    anchors
}

pub fn is_openwidget_anchor_shortcut(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
        return false;
    };

    if !extension.eq_ignore_ascii_case("lnk") {
        return false;
    }

    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|filename| filename.starts_with(ANCHOR_FILE_PREFIX))
}

pub fn find_stale_anchor_shortcuts_in(
    desktop_dir: &Path,
    active_session_id: &str,
) -> std::io::Result<Vec<PathBuf>> {
    let mut stale = Vec::new();

    if !desktop_dir.exists() {
        return Ok(stale);
    }

    for entry in fs::read_dir(desktop_dir)? {
        let path = entry?.path();
        if !is_openwidget_anchor_shortcut(&path) {
            continue;
        }

        let filename = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if !filename.contains(active_session_id) {
            stale.push(path);
        }
    }

    Ok(stale)
}

#[cfg(windows)]
fn create_shortcut(
    shortcut_path: &Path,
    target_path: &Path,
    arguments: &str,
    description: &str,
    icon_path: &Path,
) -> Result<(), String> {
    use std::process::Command;

    let script = format!(
        "$wsh = New-Object -ComObject WScript.Shell; \
         $lnk = $wsh.CreateShortcut('{}'); \
         $lnk.TargetPath = '{}'; \
         $lnk.Arguments = '{}'; \
         $lnk.Description = '{}'; \
         $lnk.IconLocation = '{}'; \
         $lnk.Save()",
        powershell_single_quote(&path_for_status(shortcut_path)),
        powershell_single_quote(&path_for_status(target_path)),
        powershell_single_quote(arguments),
        powershell_single_quote(description),
        powershell_single_quote(&format!("{},0", path_for_status(icon_path))),
    );

    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ])
        .output()
        .map_err(|error| format!("failed to execute PowerShell shortcut writer: {error}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "PowerShell shortcut writer failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

#[cfg(not(windows))]
fn create_shortcut(
    _shortcut_path: &Path,
    _target_path: &Path,
    _arguments: &str,
    _description: &str,
    _icon_path: &Path,
) -> Result<(), String> {
    Err("Windows shortcut creation is unavailable on this platform".to_string())
}

fn resolve_desktop_dir() -> PathBuf {
    let mut candidate_roots = Vec::new();
    for key in [
        "OneDrive",
        "OneDriveConsumer",
        "OneDriveCommercial",
        "USERPROFILE",
    ] {
        if let Some(path) = env_path(key) {
            candidate_roots.push(path);
        }
    }

    let fallback_desktop = env_path("USERPROFILE")
        .map(|home| home.join("Desktop"))
        .unwrap_or_else(|| {
            env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("Desktop")
        });

    resolve_desktop_dir_from_parts(
        resolve_known_desktop_dir(),
        candidate_roots,
        fallback_desktop,
    )
}

fn resolve_desktop_dir_from_parts(
    known_desktop: Option<PathBuf>,
    candidate_roots: impl IntoIterator<Item = PathBuf>,
    fallback_desktop: PathBuf,
) -> PathBuf {
    if let Some(path) = known_desktop.filter(|path| !path.as_os_str().is_empty()) {
        return path;
    }

    if let Some(path) = first_existing_desktop_dir(candidate_roots) {
        return path;
    }

    fallback_desktop
}

#[cfg(windows)]
fn resolve_known_desktop_dir() -> Option<PathBuf> {
    use std::process::Command;

    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "$OutputEncoding = [Console]::OutputEncoding = [System.Text.Encoding]::UTF8; [Environment]::GetFolderPath('Desktop')",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let desktop = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if desktop.is_empty() {
        None
    } else {
        Some(PathBuf::from(desktop))
    }
}

#[cfg(not(windows))]
fn resolve_known_desktop_dir() -> Option<PathBuf> {
    None
}

fn first_existing_desktop_dir(
    candidate_roots: impl IntoIterator<Item = PathBuf>,
) -> Option<PathBuf> {
    for root in candidate_roots {
        for folder_name in ["Desktop", "바탕 화면"] {
            let path = root.join(folder_name);
            if path.exists() {
                return Some(path);
            }
        }
    }

    None
}

fn resolve_state_dir() -> PathBuf {
    if let Some(local_app_data) = env_path("LOCALAPPDATA") {
        return local_app_data.join("OpenWidGet");
    }

    if let Some(app_data) = env_path("APPDATA") {
        return app_data.join("OpenWidGet");
    }

    if let Some(home) = env_path("HOME") {
        return home.join(".openwidget");
    }

    env::temp_dir().join("OpenWidGet")
}

fn env_path(key: &str) -> Option<PathBuf> {
    env::var_os(key).map(PathBuf::from)
}

fn generate_session_id() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    sanitize_segment(&format!("{seconds}-{}", std::process::id()))
}

fn sanitize_segment(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect();

    sanitized.trim_matches('-').to_string()
}

#[cfg(windows)]
fn powershell_single_quote(value: &str) -> String {
    value.replace('\'', "''")
}

fn path_for_status(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, time::Duration};

    #[test]
    fn two_by_two_widget_creates_four_anchor_definitions() {
        let anchors = build_anchor_plan("clock", 2, 2, "session-1");

        assert_eq!(anchors.len(), 4);
        assert_eq!(anchors[0].row, 1);
        assert_eq!(anchors[0].column, 1);
        assert_eq!(anchors[3].row, 2);
        assert_eq!(anchors[3].column, 2);
    }

    #[test]
    fn anchor_filenames_are_identifiable_runtime_artifacts() {
        let anchors = build_anchor_plan("clock widget", 1, 2, "session 2");

        for anchor in anchors {
            assert!(anchor.filename.starts_with(ANCHOR_FILE_PREFIX));
            assert!(anchor.filename.ends_with(".lnk"));
            assert!(anchor
                .description
                .contains("OpenWidGet runtime Anchor Shortcut"));
            assert!(anchor.description.contains("delete-on-exit=true"));
            assert!(anchor.arguments.contains("--open-anchor"));
            assert!(is_openwidget_anchor_shortcut(Path::new(&anchor.filename)));
        }
    }

    #[test]
    fn known_desktop_beats_stale_userprofile_desktop() {
        let root = unique_temp_root("desktop-known-folder");
        let known_desktop = root.join("OneDrive").join("바탕 화면");
        let stale_userprofile_desktop = root.join("UserProfile").join("Desktop");
        fs::create_dir_all(&known_desktop).unwrap();
        fs::create_dir_all(&stale_userprofile_desktop).unwrap();

        let resolved = resolve_desktop_dir_from_parts(
            Some(known_desktop.clone()),
            vec![root.join("UserProfile")],
            root.join("fallback").join("Desktop"),
        );

        assert_eq!(resolved, known_desktop);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn fallback_detects_localized_onedrive_desktop() {
        let root = unique_temp_root("desktop-localized-fallback");
        let onedrive_desktop = root.join("OneDrive").join("바탕 화면");
        let userprofile_desktop = root.join("UserProfile").join("Desktop");
        fs::create_dir_all(&onedrive_desktop).unwrap();
        fs::create_dir_all(&userprofile_desktop).unwrap();

        let resolved = resolve_desktop_dir_from_parts(
            None,
            vec![root.join("OneDrive"), root.join("UserProfile")],
            root.join("fallback").join("Desktop"),
        );

        assert_eq!(resolved, onedrive_desktop);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn stale_detection_ignores_active_session_and_non_anchor_files() {
        let root = unique_temp_root("stale-detection");
        fs::create_dir_all(&root).unwrap();

        let active = root.join("OpenWidGet Anchor - clock - active - r1c1.lnk");
        let stale = root.join("OpenWidGet Anchor - clock - old - r1c1.lnk");
        let unrelated = root.join("Normal Shortcut.lnk");
        File::create(&active).unwrap();
        File::create(&stale).unwrap();
        File::create(&unrelated).unwrap();

        let stale_results = find_stale_anchor_shortcuts_in(&root, "active").unwrap();

        assert_eq!(stale_results, vec![stale]);
        fs::remove_dir_all(root).unwrap();
    }

    fn unique_temp_root(label: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "openwidget-anchor-test-{label}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_nanos()
        ))
    }
}
