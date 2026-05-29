// src-tauri/src/analyzer/exe_scanner.rs

use std::path::{Path, PathBuf};
use std::fs;

/// 可执行文件的基本信息（跨平台兼容版本）
#[derive(Debug, Clone)]
pub struct ExeInfo {
    pub path: PathBuf,
    pub name: String,
    pub directory: String,
}

pub struct ExeScanner;

impl ExeScanner {
    /// 扫描指定目录下所有 .exe，提取基本信息
    pub fn scan_program_files() -> Vec<ExeInfo> {
        let dirs = vec![
            r"C:\Program Files",
            r"C:\Program Files (x86)",
        ];

        let mut results = Vec::new();
        for dir in dirs {
            let path = Path::new(dir);
            if path.exists() {
                Self::scan_dir_recursive(path, &mut results);
            }
        }
        results
    }

    fn scan_dir_recursive(dir: &Path, results: &mut Vec<ExeInfo>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    Self::scan_dir_recursive(&path, results);
                } else if path.extension().map_or(false, |e| e == "exe") {
                    let name = path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let directory = path.parent()
                        .unwrap_or(Path::new(""))
                        .to_string_lossy()
                        .to_string();

                    results.push(ExeInfo {
                        path,
                        name,
                        directory,
                    });
                }
            }
        }
    }

    /// 从 ExeInfo 列表构建软件目录索引
    /// 将 exe 所在目录作为软件安装路径索引
    pub fn build_directory_index(exes: &[ExeInfo]) -> Vec<(String, String)> {
        let mut index: Vec<(String, String)> = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for exe in exes {
            let dir_lower = exe.directory.to_lowercase();
            if seen.insert(dir_lower.clone()) {
                index.push((exe.name.clone(), exe.directory.clone()));
            }
        }

        index
    }
}
