use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

/// Write raw bytes (base64-encoded over the IPC bridge) to an absolute path the
/// user already chose via the native save dialog. Keeping the write in a custom
/// command avoids the fs-plugin path-scope machinery entirely.
#[tauri::command]
fn write_file(path: String, data: String) -> Result<(), String> {
    let bytes = BASE64.decode(data.as_bytes()).map_err(|e| e.to_string())?;
    // Create parent directories so folder-mode writes (e.g. <dir>/Assets/x.png) succeed.
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![write_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
