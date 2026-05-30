// src-tauri/src/commands/scan.rs

use std::time::Instant;
use crate::models::file_entry::{FileEntry, RiskLevel, FileCategory};
use crate::models::scan_result::ScanResult;
use crate::models::fingerprint::ExeFingerprint;
use crate::registry::uninstalled::{ResidueEntry, check_residues};
use crate::registry::installed::get_installed_software;
use crate::scanner::walker::WalkScanner;
use crate::analyzer::system::SystemFileDetector;
use crate::analyzer::fingerprint::FingerprintEngine;
use crate::analyzer::risk::RiskAssessor;

/// 启动扫描
#[tauri::command]
pub async fn start_scan(
    system_config_json: tauri::State<'_, String>,
    fingerprints_json: tauri::State<'_, String>,
    drive: String,
    min_size_mb: u64,
) -> Result<ScanResult, String> {
    let start = Instant::now();

    // 加载系统文件检测器
    let detector = SystemFileDetector::from_json(&system_config_json)
        .map_err(|e| format!("加载系统白名单失败: {}", e))?;

    // 加载指纹库
    let engine = FingerprintEngine::from_json(&fingerprints_json)
        .map_err(|e| format!("加载指纹库失败: {}", e))?;

    // 执行扫描
    let min_size_bytes = min_size_mb * 1024 * 1024;
    let scanner = WalkScanner::new(&drive, min_size_bytes);
    let mut entries = scanner.scan();

    // 对每个条目执行分析
    for entry in &mut entries {
        let path = std::path::Path::new(&entry.path);

        // 系统文件检测
        let is_system = detector.is_system_file(path);
        if is_system {
            entry.risk_level = RiskLevel::Forbidden;
            entry.category = FileCategory::SystemFile;
            entry.description = Some("Windows 系统文件，删除可能导致系统崩溃".to_string());
            entry.cleanable_advice = Some("禁止删除".to_string());
            entry.software_name = Some("Windows 系统".to_string());
            continue;
        }

        // 临时/缓存模式检测
        if detector.is_temp_pattern(path) {
            entry.risk_level = RiskLevel::Safe;
            entry.category = FileCategory::TempFile;
            entry.description = Some("临时文件".to_string());
            entry.cleanable_advice = Some("可安全删除".to_string());
            entry.software_name = Some("临时文件".to_string());
            continue;
        }

        if detector.is_cache_pattern(path) {
            entry.risk_level = RiskLevel::Safe;
            entry.category = FileCategory::CacheFile;
            entry.description = Some("缓存文件".to_string());
            entry.cleanable_advice = Some("可安全删除，系统或软件会自动重建".to_string());
            entry.software_name = Some("缓存文件".to_string());
            continue;
        }

        // 指纹库匹配
        if let Some(matched) = engine.match_path(path) {
            entry.software_name = Some(matched.name.clone());
            entry.description = Some(matched.description.clone());
            entry.cleanable_advice = Some(if matched.cleanable {
                "可安全删除".to_string()
            } else {
                "谨慎删除，删除后软件可能异常".to_string()
            });

            if matched.cleanable {
                entry.risk_level = RiskLevel::Safe;
            } else {
                entry.risk_level = RiskLevel::Caution;
            }
            entry.category = FileCategory::SoftwareData;
            continue;
        }

        // 未匹配 → 使用 RiskAssessor 做最终判断
        let last_accessed_days = 0u64; // TODO: 计算实际天差
        let (risk, category, description) = RiskAssessor::assess(
            path,
            false,
            None,
            last_accessed_days,
            false,
        );
        entry.risk_level = risk;
        entry.category = category;
        entry.description = Some(description);
        entry.cleanable_advice = Some("建议查看后自行判断".to_string());
    }

    let duration = start.elapsed();
    let entry_count = entries.len() as u32;

    // 统计可清理/谨慎/禁止的数量和大小
    let (safe_count, safe_size) = count_by_risk(&entries, RiskLevel::Safe);
    let (caution_count, caution_size) = count_by_risk(&entries, RiskLevel::Caution);
    let (forbidden_count, forbidden_size) = count_by_risk(&entries, RiskLevel::Forbidden);

    Ok(ScanResult {
        entries,
        total_size: 0,       // 磁盘总空间（后续获取）
        used_size: safe_size + caution_size + forbidden_size,
        free_size: 0,        // 可用空间（后续获取）
        scan_duration_ms: duration.as_millis() as u64,
        entry_count,
    })
}

fn count_by_risk(entries: &[FileEntry], risk: RiskLevel) -> (u32, u64) {
    let mut count = 0u32;
    let mut size = 0u64;
    for entry in entries {
        if entry.risk_level == risk {
            count += 1;
            size += entry.size_bytes;
        }
    }
    (count, size)
}

/// 获取可执行文件扫描结果（软件列表）
#[tauri::command]
pub async fn scan_executables() -> Result<Vec<ExeFingerprint>, String> {
    let exes = crate::analyzer::exe_scanner::ExeScanner::scan_program_files();
    let index = crate::analyzer::exe_scanner::ExeScanner::build_directory_index(&exes);

    let fingerprints: Vec<ExeFingerprint> = index
        .into_iter()
        .map(|(name, path)| {
            // 去重：取第一个匹配的最短路径
            ExeFingerprint {
                name,
                vendor: String::new(),
                install_path: path.clone(),
                exe_path: path,
                signed: false, // 签名检测后续实现
            }
        })
        .collect();

    Ok(fingerprints)
}

/// 检查卸载残留
#[tauri::command]
pub async fn check_uninstall_residue() -> Result<Vec<ResidueEntry>, String> {
    let installed = get_installed_software();
    let paths: Vec<(String, String)> = installed
        .iter()
        .map(|s| (s.display_name.clone(), s.install_location.clone()))
        .collect();
    Ok(check_residues(&paths))
}
