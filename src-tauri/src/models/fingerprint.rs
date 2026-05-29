// src-tauri/src/models/fingerprint.rs

use serde::{Deserialize, Serialize};

/// 内置指纹条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintEntry {
    pub name: String,
    pub vendor: String,
    pub icon: Option<String>,
    pub paths: Vec<String>,
    pub data_dirs: Vec<String>,
    pub cache_dirs: Vec<String>,
    pub known_cleanable: bool,
    pub cleanable_notes: String,
    pub risk: String,
}

/// 从可执行文件自动提取的指纹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExeFingerprint {
    pub name: String,
    pub vendor: String,
    pub install_path: String,
    pub exe_path: String,
    pub signed: bool,
}

/// 系统路径白名单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPathsConfig {
    pub version: String,
    pub system_dirs: Vec<String>,
    pub protected_extensions: Vec<String>,
    pub protected_filenames: Vec<String>,
    pub temp_path_patterns: Vec<String>,
    pub cache_path_patterns: Vec<String>,
}
