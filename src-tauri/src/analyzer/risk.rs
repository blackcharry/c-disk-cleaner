// src-tauri/src/analyzer/risk.rs

use std::path::Path;
use crate::models::file_entry::{RiskLevel, FileCategory};

pub struct RiskAssessor;

impl RiskAssessor {
    /// 综合评估风险等级
    pub fn assess(
        path: &Path,
        is_system: bool,
        fingerprint_match: Option<&crate::analyzer::fingerprint::MatchResult>,
        last_accessed_days: u64,
        is_in_use: bool,
    ) -> (RiskLevel, FileCategory, String) {
        // 第一优先级：系统文件
        if is_system {
            return (
                RiskLevel::Forbidden,
                FileCategory::SystemFile,
                "Windows 系统文件，删除可能导致系统崩溃".to_string(),
            );
        }

        let path_str = path.to_string_lossy().to_lowercase();

        // 第二优先级：临时文件
        if path_str.contains("temp") || path_str.contains("crashdumps") {
            return (
                RiskLevel::Safe,
                FileCategory::TempFile,
                "临时文件，可安全删除".to_string(),
            );
        }

        // 缓存文件
        if path_str.contains("cache") || path_str.contains("prefetch") {
            return (
                RiskLevel::Safe,
                FileCategory::CacheFile,
                "缓存文件，删除后系统或软件会自动重建".to_string(),
            );
        }

        // 日志文件
        if path_str.ends_with(".log")
            || path_str.ends_with(".etl")
            || path_str.contains("logs")
            || path_str.contains("logfiles")
        {
            return (
                RiskLevel::Safe,
                FileCategory::LogFile,
                "日志文件，可安全删除".to_string(),
            );
        }

        // 错误转储
        if path_str.ends_with(".dmp") || path_str.ends_with(".hdmp") {
            return (
                RiskLevel::Safe,
                FileCategory::TempFile,
                "系统/程序崩溃转储文件，可安全删除".to_string(),
            );
        }

        // 第三优先级：指纹库匹配
        if let Some(matched) = fingerprint_match {
            let category = if matched.name == "临时文件" || matched.risk == "low" {
                FileCategory::CacheFile
            } else {
                FileCategory::SoftwareData
            };
            let risk = if matched.cleanable {
                RiskLevel::Safe
            } else {
                RiskLevel::Caution
            };
            return (
                risk,
                category,
                matched.description.clone(),
            );
        }

        // 第四优先级：时间 + 占用判断
        if last_accessed_days > 90 && !is_in_use {
            return (
                RiskLevel::Safe,
                FileCategory::Unknown,
                "超过 90 天未访问，可能是残留文件".to_string(),
            );
        }

        if is_in_use {
            return (
                RiskLevel::Caution,
                FileCategory::Unknown,
                "当前被进程占用，建议谨慎处理".to_string(),
            );
        }

        // 默认：未知文件
        (
            RiskLevel::Caution,
            FileCategory::Unknown,
            "无法确定归属，建议保留".to_string(),
        )
    }
}
