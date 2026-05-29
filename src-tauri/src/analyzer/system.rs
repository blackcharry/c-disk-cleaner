// src-tauri/src/analyzer/system.rs

use std::path::Path;
use crate::models::fingerprint::SystemPathsConfig;

pub struct SystemFileDetector {
    config: SystemPathsConfig,
}

impl SystemFileDetector {
    pub fn new(config: SystemPathsConfig) -> Self {
        Self { config }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let config: SystemPathsConfig = serde_json::from_str(json)?;
        Ok(Self { config })
    }

    /// 判断文件是否为系统文件（基于白名单）
    /// 返回 true = 🔴 禁止删除
    pub fn is_system_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();

        // 1. 匹配系统目录
        for dir in &self.config.system_dirs {
            if path_str.starts_with(&dir.to_lowercase()) {
                return true;
            }
        }

        // 2. 匹配受保护扩展名
        if let Some(ext) = path.extension() {
            let ext = format!(".{}", ext.to_string_lossy().to_lowercase());
            if self.config.protected_extensions.iter().any(|e| e.to_lowercase() == ext) {
                return true;
            }
        }

        // 3. 匹配受保护文件名
        if let Some(name) = path.file_name() {
            let name = name.to_string_lossy().to_lowercase();
            if self.config.protected_filenames.iter().any(|f| f.to_lowercase() == name) {
                return true;
            }
        }

        false
    }

    /// 检查路径是否匹配临时文件模式
    pub fn is_temp_pattern(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();
        self.config.temp_path_patterns.iter().any(|p| {
            let pattern = p.to_lowercase();
            path_str.contains(&pattern.replace("*", ""))
        })
    }

    /// 检查路径是否匹配缓存文件模式
    pub fn is_cache_pattern(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();
        self.config.cache_path_patterns.iter().any(|p| {
            let pattern = p.to_lowercase();
            path_str.contains(&pattern.replace("*", ""))
        })
    }
}
