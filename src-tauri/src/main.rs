#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::{imageops, imageops::FilterType};
use screenshots::Screen;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Monitor {
    id: u32,
    name: String,
    width: u32,
    height: u32,
    is_primary: bool,
}

#[tauri::command]
fn list_monitors() -> Result<Vec<Monitor>, String> {
    let screens = Screen::all().map_err(|e| e.to_string())?;
    if screens.is_empty() {
        return Err("No monitors found.".to_string());
    }

    let monitors: Vec<Monitor> = screens
        .iter()
        .map(|screen| {
            let info = &screen.display_info;
            Monitor {
                id: info.id,
                name: format!(
                    "Monitor {} ({}x{}){}",
                    info.id,
                    info.width,
                    info.height,
                    if info.is_primary { " - Primary" } else { "" }
                ),
                width: info.width,
                height: info.height,
                is_primary: info.is_primary,
            }
        })
        .collect();

    Ok(monitors)
}

#[tauri::command]
fn capture_and_scale(monitor_id: u32, scale: f32, save_path: String) -> Result<String, String> {
    if !(1.0..=5.0).contains(&scale) {
        return Err(format!(
            "Scale factor must be between 1.0 and 5.0, but received {}.",
            scale
        ));
    }

    let screens = Screen::all().map_err(|e| e.to_string())?;
    let screen = screens
        .iter()
        .find(|s| s.display_info.id == monitor_id)
        .ok_or(format!("Monitor con ID {} no encontrado.", monitor_id))?;

    let rgba = screen.capture().map_err(|e| e.to_string())?;
    let (w, h) = (rgba.width(), rgba.height());

    let target_w = (w as f32 * scale).round() as u32;
    let target_h = (h as f32 * scale).round() as u32;

    if target_w == 0 || target_h == 0 {
        return Err(
            "El factor de escala es demasiado pequeño o el redondeo resultó en tamaño cero."
                .to_string(),
        );
    }
    let out_path = std::path::PathBuf::from(save_path);
    let scaled_image = imageops::resize(&rgba, target_w, target_h, FilterType::Lanczos3);
    scaled_image.save(&out_path).map_err(|e| e.to_string())?;
    Ok(out_path.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![list_monitors, capture_and_scale])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
