use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{Manager, PhysicalPosition};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

/// Tracks whether the title bar is currently visible.
static TITLEBAR_VISIBLE: AtomicBool = AtomicBool::new(false);

fn primary_hotkey() -> Shortcut {
    Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Space)
}

fn fallback_hotkey() -> Shortcut {
    Shortcut::new(Some(Modifiers::SUPER | Modifiers::ALT), Code::Space)
}

fn toggle_titlebar(app: &tauri::AppHandle) -> tauri::Result<()> {
    let window = app
        .get_webview_window("titlebar")
        .ok_or_else(|| tauri::Error::WebviewNotFound)?;

    if TITLEBAR_VISIBLE.swap(true, Ordering::SeqCst) {
        // Already visible → focus the input (no-op for v1).
        window.set_focus()?;
        return Ok(());
    }

    // Position the title bar 80px from the top, horizontally centered on the focused monitor.
    if let Ok(Some(monitor)) = window.current_monitor() {
        let scale = monitor.scale_factor();
        let mon_pos = monitor.position();
        let mon_size = monitor.size();
        let win_w = (700.0_f64 * scale).round() as i32;
        let x = mon_pos.x + ((mon_size.width as i32 - win_w) / 2).max(0);
        let y = mon_pos.y + (80.0_f64 * scale).round() as i32;
        window.set_position(PhysicalPosition::new(x, y))?;
    }

    window.show()?;
    window.set_focus()?;
    Ok(())
}

fn hide_titlebar(app: &tauri::AppHandle) -> tauri::Result<()> {
    let window = app
        .get_webview_window("titlebar")
        .ok_or_else(|| tauri::Error::WebviewNotFound)?;
    TITLEBAR_VISIBLE.store(false, Ordering::SeqCst);
    window.hide()?;
    Ok(())
}

#[tauri::command]
fn show_titlebar(app: tauri::AppHandle) -> Result<(), String> {
    toggle_titlebar(&app).map_err(|e| e.to_string())
}

#[tauri::command]
fn hide_titlebar_cmd(app: tauri::AppHandle) -> Result<(), String> {
    hide_titlebar(&app).map_err(|e| e.to_string())
}

/// Tauri command exposed to the frontend so the input can clear-then-close on Esc.
#[tauri::command]
fn close_titlebar(app: tauri::AppHandle) -> Result<(), String> {
    hide_titlebar(&app).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,telme_lib=debug")),
        )
        .with_target(false)
        .compact()
        .init();

    let primary = primary_hotkey();
    let fallback = fallback_hotkey();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    // Only react to whichever shortcut we actually registered.
                    if *shortcut == primary || *shortcut == fallback {
                        if let Err(err) = toggle_titlebar(app) {
                            tracing::warn!(?err, "failed to toggle titlebar from hotkey");
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            show_titlebar,
            hide_titlebar_cmd,
            close_titlebar,
        ])
        .setup(move |app| {
            // Try primary, fall back to alt if it collides with another registered shortcut.
            let gs = app.global_shortcut();
            match gs.register(primary) {
                Ok(_) => tracing::info!("registered hotkey: ⌘⇧Space"),
                Err(primary_err) => {
                    tracing::warn!(?primary_err, "primary hotkey unavailable, trying ⌘⌥Space");
                    gs.register(fallback)?;
                }
            }

            // Hide on focus loss so the title bar doesn't steal focus forever.
            if let Some(window) = app.get_webview_window("titlebar") {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        // Allow the user to click outside to dismiss.
                        TITLEBAR_VISIBLE.store(false, Ordering::SeqCst);
                        let _ = win.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
