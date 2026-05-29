// src-tauri/src/scanner/walker.rs

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use crate::models::file_entry::FileEntry;

/// 递归遍历扫描器
pub struct WalkScanner {
    drive: String,
    min_size_bytes: u64,
}

impl WalkScanner {
    pub fn new(drive: &str, min_size_bytes: u64) -> Self {
        Self {
            drive: drive.to_string(),
            min_size_bytes,
        }
    }

    /// 执行递归扫描，返回扁平的文件列表
    pub fn scan(&self) -> Vec<FileEntry> {
        let root = Path::new(&self.drive);
        let mut entries: Vec<FileEntry> = Vec::new();

        self.scan_dir(root, &mut entries);

        // 按大小降序排序
        entries.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

        entries
    }

    fn scan_dir(&self, dir: &Path, entries: &mut Vec<FileEntry>) {
        let read_dir = match fs::read_dir(dir) {
            Ok(r) => r,
            Err(_) => return, // 权限不足，跳过
        };

        for entry in read_dir.flatten() {
            let path = entry.path();
            let metadata = match fs::metadata(&path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            if metadata.is_dir() {
                // 计算目录总大小
                let dir_size = Self::dir_size(&path);
                if dir_size >= self.min_size_bytes {
                    if let Ok(modified) = metadata.modified() {
                        let entry = self.create_entry(
                            &path,
                            true,
                            dir_size,
                            modified,
                            metadata.accessed().ok(),
                        );
                        entries.push(entry);
                    }
                }

                // 递归进入子目录
                self.scan_dir(&path, entries);
            } else if metadata.is_file() {
                let size = metadata.len();
                if size >= self.min_size_bytes {
                    if let Ok(modified) = metadata.modified() {
                        let entry = self.create_entry(
                            &path,
                            false,
                            size,
                            modified,
                            metadata.accessed().ok(),
                        );
                        entries.push(entry);
                    }
                }
            }
        }
    }

    fn create_entry(
        &self,
        path: &Path,
        is_dir: bool,
        size: u64,
        modified: SystemTime,
        accessed: Option<SystemTime>,
    ) -> FileEntry {
        let last_modified = system_time_to_iso(modified);
        let last_accessed = accessed
            .map(system_time_to_iso)
            .unwrap_or_else(|| last_modified.clone());

        FileEntry {
            path: path.to_string_lossy().to_string(),
            is_dir,
            size_bytes: size,
            file_count: if is_dir { 1 } else { 1 },
            last_modified,
            last_accessed,
            risk_level: crate::models::file_entry::RiskLevel::Caution,
            category: crate::models::file_entry::FileCategory::Unknown,
            software_name: None,
            description: None,
            cleanable_advice: None,
        }
    }

    /// 递归计算目录总大小
    fn dir_size(path: &Path) -> u64 {
        let mut total: u64 = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if let Ok(meta) = fs::metadata(&p) {
                    if meta.is_dir() {
                        total += Self::dir_size(&p);
                    } else {
                        total += meta.len();
                    }
                }
            }
        }
        total
    }
}

fn system_time_to_iso(t: SystemTime) -> String {
    let dt: DateTime<Utc> = t.into();
    dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
