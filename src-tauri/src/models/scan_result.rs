// src-tauri/src/models/scan_result.rs

use serde::{Deserialize, Serialize};
use super::file_entry::FileEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub entries: Vec<FileEntry>,
    pub total_size: u64,
    pub used_size: u64,
    pub free_size: u64,
    pub scan_duration_ms: u64,
    pub entry_count: u32,
}
