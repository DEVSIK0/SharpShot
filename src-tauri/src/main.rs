// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use screenshots::Screen;
// Usa SIEMPRE el image re-exportado por `screenshots` para evitar choques de tipos
use screenshots::image::{self, imageops::FilterType, RgbaImage};
use std::{error::Error, path::PathBuf};

fn pick_primary_screen() -> Option<Screen> {
    let mut screens = Screen::all().ok()?;
    if screens.is_empty() {
        return None;
    }
    if let Some(s) = screens.iter().find(|s| s.display_info.x == 0 && s.display_info.y == 0) {
        return Some(s.clone());
    }
    Some(screens.swap_remove(0))
}

/// Captura el monitor principal, escala x2 y guarda `screenshot_scaled.png` en el cwd.
/// Devuelve la ruta absoluta al archivo.
#[tauri::command]
fn capture_and_scale_x2() -> Result<String, String> {
    let screen = pick_primary_screen().ok_or("No se encontró ningún monitor")?;
    let rgba: RgbaImage = screen.capture().map_err(|e| e.to_string())?;

    let w = rgba.width();
    let h = rgba.height();

    let target_w = w.saturating_mul(2);
    let target_h = h.saturating_mul(2);

    // Para 3D/escenarios suele ir muy bien Lanczos3. Cambia a CatmullRom si ves halos en HUD.
    let filter = FilterType::Lanczos3;
    let up = image::imageops::resize(&rgba, target_w, target_h, filter);

    let mut out_path = std::env::current_dir().map_err(|e| e.to_string())?;
    out_path.push("screenshot_scaled.png");
    up.save(&out_path).map_err(|e| e.to_string())?;

    Ok(out_path.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture_and_scale_x2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
