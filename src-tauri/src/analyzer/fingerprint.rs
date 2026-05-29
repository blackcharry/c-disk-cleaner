// src-tauri/src/analyzer/fingerprint.rs

use std::path::Path;
use crate::models::fingerprint::FingerprintEntry;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub name: String,
    pub description: String,
    pub risk: String,
    pub cleanable: bool,
}

pub struct FingerprintEngine {
    entries: Vec<FingerprintEntry>,
}

impl FingerprintEngine {
    pub fn new(entries: Vec<FingerprintEntry>) -> Self {
        Self { entries }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let entries: Vec<FingerprintEntry> = serde_json::from_str(json)?;
        Ok(Self { entries })
    }

    /// 匹配文件路径，返回匹配到的软件名称和清理建议
    pub fn match_path(&self, path: &Path) -> Option<MatchResult> {
        let path_str = path.to_string_lossy();

        for entry in &self.entries {
            // 检查 cache_dirs（最高优先级，匹配最精确）
            for pattern in &entry.cache_dirs {
                if Self::path_matches(&path_str, pattern) {
                    return Some(MatchResult {
                        name: entry.name.clone(),
                        description: format!("{} 的缓存文件", entry.name),
                        risk: "low".to_string(),
                        cleanable: true,
                    });
                }
            }

            // 检查 data_dirs
            for pattern in &entry.data_dirs {
                if Self::path_matches(&path_str, pattern) {
                    return Some(MatchResult {
                        name: entry.name.clone(),
                        description: format!("{} 的数据目录", entry.name),
                        risk: "medium".to_string(),
                        cleanable: false,
                    });
                }
            }

            // 检查主路径
            for pattern in &entry.paths {
                if Self::path_matches(&path_str, pattern) {
                    return Some(MatchResult {
                        name: entry.name.clone(),
                        description: entry.cleanable_notes.clone(),
                        risk: entry.risk.clone(),
                        cleanable: entry.known_cleanable,
                    });
                }
            }
        }

        None
    }

    /// 路径通配符匹配（* 匹配任意字符）
    fn path_matches(path: &str, pattern: &str) -> bool {
        let path_lower = path.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        let pattern_parts: Vec<&str> = pattern_lower.split('*').collect();

        if pattern_parts.len() == 1 {
            return path_lower.contains(&pattern_lower);
        }

        let first = pattern_parts[0];
        let last = pattern_parts[pattern_parts.len() - 1];

        if !first.is_empty() && !path_lower.starts_with(first) {
            return false;
        }
        if !last.is_empty() && !path_lower.ends_with(last) {
            return false;
        }

        let mut pos = 0;
        for (i, part) in pattern_parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }
            if i == 0 {
                pos = part.len();
                continue;
            }
            if let Some(found) = path_lower[pos..].find(part) {
                pos += found + part.len();
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_matches() {
        assert!(FingerprintEngine::path_matches(
            r"C:\Users\xxx\AppData\Roaming\Tencent\WeChat\data",
            r"C:\Users\*\AppData\Roaming\Tencent\WeChat"
        ));
        assert!(FingerprintEngine::path_matches(
            r"C:\Users\xxx\Documents\WeChat Files\FileStorage",
            r"WeChat Files\*\FileStorage"
        ));
    }
}
