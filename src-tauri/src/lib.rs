pub mod api;
pub mod cmd;
pub mod dto;
pub mod llm;
pub mod log;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(log::build_logger())
        // Register all handlers to be invoked in React
        .invoke_handler(tauri::generate_handler![
            cmd::generate_anthropic_response,
            cmd::generate_openai_response,
            cmd::get_available_models,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
