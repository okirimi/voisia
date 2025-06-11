use log::LevelFilter;
use std::env;
use std::fs;
use std::path::PathBuf;
use tauri::Wry;
use tauri::plugin::TauriPlugin;
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};

/// Get logs directory for log files and create logs dir if it doesn't exist.
///
/// # Platform-specific behavior
///
/// `current_dir` and `create_dir_all` has specific behavior on windows.
///
/// # Returns
///
/// A `Result` containing the `PathBuf` to the logs directory, or an `std::io::Error` on failure.
///
fn get_logs_dir() -> std::io::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let logs_dir = current_dir.join("logs");

    if !logs_dir.exists() {
        match fs::create_dir_all(&logs_dir) {
            Ok(_) => println!("Logs directory successfully created: {:?}", logs_dir),
            Err(e) => {
                eprintln!("Failed to create logs directory at {:?}: {:?}", logs_dir, e);
                return Err(e);
            }
        }
    }
    Ok(logs_dir)
}

/// Build and initialize the application logger using `tauri_plugin_log`.
pub fn build_logger() -> TauriPlugin<Wry, ()> {
    let mut log_builder = tauri_plugin_log::Builder::new()
        .level(LevelFilter::Info)
        .timezone_strategy(TimezoneStrategy::UseLocal)
        // Exclude logs with target "hyper"
        .filter(|metadata| metadata.target() != "hyper")
        .max_file_size(1024 * 1024 * 50 /* 50MB */)
        // Keep all log files
        .rotation_strategy(RotationStrategy::KeepAll);

    match get_logs_dir() {
        Ok(logs_dir_path) => {
            log_builder = log_builder.target(Target::new(
                TargetKind::Folder {
                    path: logs_dir_path.clone(),
                    file_name: None,
                },
            ));
            println!("Logs will be written to: {:?}", logs_dir_path);
        }
        Err(e) => {
            eprintln!(
                "Failed to get or create logs directory. `Error: {:?}`",
                e
            );
            // stdout logging will be added below, regardless of folder logging success.
        }
    };

    // Stdout logging
    log_builder = log_builder.target(Target::new(
        TargetKind::Stdout,
    ));

    log_builder.build()
}
