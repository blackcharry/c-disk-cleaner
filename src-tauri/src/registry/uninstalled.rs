// src-tauri/src/registry/uninstalled.rs

use std::fs;
use std::path::Path;

/// 卸载残留条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResidueEntry {
    pub path: String,
    pub software_name: Option<String>,
    pub size_bytes: u64,
}

/// 比对注册表卸载信息与实际磁盘目录，标记疑似残留
///
/// 逻辑：遍历注册表中记录的 InstallLocation，检查目录是否仍存在。
/// 如果目录存在但其 DisplayName 不在当前已安装软件列表中 → 标记为残留。
pub fn check_residues(
    all_install_paths: &[(String, String)], // (DisplayName, InstallLocation)
) -> Vec<ResidueEntry> {
    let mut residues = Vec::new();

    for (name, location) in all_install_paths {
        if location.is_empty() {
            continue;
        }

        let path = Path::new(location);
        if !path.exists() {
            continue;
        }

        // 检查该目录下的 exe 是否还存在
        let exe_exists = has_executable(path);

        if !exe_exists {
            // 目录在但没 exe → 疑似卸载不干净
            let size = dir_size(path);
            residues.push(ResidueEntry {
                path: location.clone(),
                software_name: Some(name.clone()),
                size_bytes: size,
            });
        }
    }

    // 按大小降序
    residues.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
    residues
}

fn has_executable(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if has_executable(&path) {
                    return true;
                }
            } else if path.extension().map_or(false, |e| e == "exe") {
                return true;
            }
        }
    }
    false
}

fn dir_size(dir: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(meta) = fs::metadata(&path) {
                if meta.is_dir() {
                    total += dir_size(&path);
                } else {
                    total += meta.len();
                }
            }
        }
    }
    total
}
