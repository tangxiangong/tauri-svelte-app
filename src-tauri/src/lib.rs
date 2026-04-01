pub mod commands;
pub mod memory;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_used_memory,
            commands::get_system_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
