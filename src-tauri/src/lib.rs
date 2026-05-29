pub mod models;
pub mod scanner;
pub mod analyzer;
pub mod registry;
pub mod commands;

use commands::scan::{start_scan, scan_executables, check_uninstall_residue};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 加载内置指纹库
    let system_config = include_str!("../data/system_paths.json");
    let fingerprints = include_str!("../data/fingerprints.json");

    tauri::Builder::default()
        .manage(system_config.to_string())
        .manage(fingerprints.to_string())
        .invoke_handler(tauri::generate_handler![
            start_scan,
            scan_executables,
            check_uninstall_residue,
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}
