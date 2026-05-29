// src-tauri/src/models/file_entry.rs

use serde::{Deserialize, Serialize};

/// 风险等级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Safe,      // 🟢 可安全删除
    Caution,   // 🟡 谨慎删除
    Forbidden, // 🔴 禁止删除（系统文件）
}

/// 文件类型分类
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileCategory {
    SystemFile,       // Windows 系统文件
    SoftwareData,     // 软件数据文件
    TempFile,         // 临时文件
    CacheFile,        // 缓存文件
    LogFile,          // 日志文件
    UninstallResidue, // 卸载残留
    Unknown,          // 未知
}

/// 文件条目 —— 扫描结果的最小单元
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub file_count: u32,
    pub last_modified: String,
    pub last_accessed: String,
    pub risk_level: RiskLevel,
    pub category: FileCategory,
    pub software_name: Option<String>,
    pub description: Option<String>,
    pub cleanable_advice: Option<String>,
}

impl FileEntry {
    pub fn new(path: String, is_dir: bool, size_bytes: u64) -> Self {
        Self {
            path,
            is_dir,
            size_bytes,
            file_count: if is_dir { 0 } else { 1 },
            last_modified: String::new(),
            last_accessed: String::new(),
            risk_level: RiskLevel::Caution,
            category: FileCategory::Unknown,
            software_name: None,
            description: None,
            cleanable_advice: None,
        }
    }
}
