use serde::Serialize;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};

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
    tauri::Builder::default()
        .setup(|app| {
            build_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_app_status])
        .run(tauri::generate_context!())
        .expect("error while running OpenWidGet application");
}
