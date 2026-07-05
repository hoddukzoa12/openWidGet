mod anchor_shortcuts;
mod desktop_grid;

use anchor_shortcuts::{AnchorLifecycleStatus, AnchorShortcutManager};
use desktop_grid::{get_desktop_grid_status as resolve_desktop_grid_status, DesktopGridStatus};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};

type SharedAnchorLifecycle = Arc<Mutex<AnchorLifecycleStatus>>;

#[derive(Debug, Serialize)]
struct WidgetPreview {
    id: &'static str,
    name: &'static str,
    size: &'static str,
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct AppStatus {
    app_name: &'static str,
    version: &'static str,
    mode: &'static str,
    widgets: Vec<WidgetPreview>,
}

#[tauri::command]
fn get_app_status() -> AppStatus {
    AppStatus {
        app_name: "OpenWidGet",
        version: env!("CARGO_PKG_VERSION"),
        mode: "shell-preview",
        widgets: vec![
            WidgetPreview {
                id: "clock",
                name: "Clock",
                size: "2×2",
                status: "bundled-preview",
            },
            WidgetPreview {
                id: "memo",
                name: "Memo",
                size: "2×2",
                status: "bundled-preview",
            },
            WidgetPreview {
                id: "launcher",
                name: "Project Launcher",
                size: "4×2",
                status: "bundled-preview",
            },
        ],
    }
}

#[tauri::command]
fn get_anchor_lifecycle_status(
    state: tauri::State<'_, SharedAnchorLifecycle>,
) -> AnchorLifecycleStatus {
    state
        .lock()
        .map(|status| status.clone())
        .unwrap_or_else(|_| AnchorLifecycleStatus {
            platform: std::env::consts::OS,
            enabled: false,
            session_id: "state-poisoned".to_string(),
            desktop_dir: "unknown".to_string(),
            state_dir: "unknown".to_string(),
            anchors_planned: 0,
            anchors_created: 0,
            stale_detected: 0,
            stale_deleted: 0,
            anchor_files: Vec::new(),
            last_error: Some("anchor lifecycle state lock was poisoned".to_string()),
        })
}

#[tauri::command]
fn get_desktop_grid_status() -> DesktopGridStatus {
    resolve_desktop_grid_status()
}

fn build_tray(app: &tauri::App) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Show OpenWidGet", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

    let _tray = TrayIconBuilder::with_id("main")
        .tooltip("OpenWidGet v0.1-alpha")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { .. } = event {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

pub fn run() {
    let anchor_manager = AnchorShortcutManager::for_runtime()
        .expect("failed to initialize OpenWidGet Anchor Shortcut manager");
    let cleanup_manager = anchor_manager.clone();
    let lifecycle_state: SharedAnchorLifecycle = Arc::new(Mutex::new(
        AnchorLifecycleStatus::unsupported(&anchor_manager, 0),
    ));
    let lifecycle_state_for_setup = Arc::clone(&lifecycle_state);

    let result = tauri::Builder::default()
        .manage(Arc::clone(&lifecycle_state))
        .setup(move |app| {
            build_tray(app)?;
            let startup_status = anchor_manager.prepare_startup_sample();
            if let Ok(mut status) = lifecycle_state_for_setup.lock() {
                *status = startup_status;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_app_status,
            get_anchor_lifecycle_status,
            get_desktop_grid_status
        ])
        .run(tauri::generate_context!());

    if let Err(error) = cleanup_manager.cleanup_current_session() {
        eprintln!("OpenWidGet Anchor Shortcut cleanup failed: {error}");
    }

    result.expect("error while running OpenWidGet application");
}
