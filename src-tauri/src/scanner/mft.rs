// src-tauri/src/scanner/mft.rs

// MFT 直读模块 —— Windows 平台专用
// 当前版本使用递归遍历作为主力方案，MFT 直读作为后续优化

#[allow(dead_code)]
pub struct MftScanner {
    drive: String,
}

#[allow(dead_code)]
impl MftScanner {
    pub fn new(drive: &str) -> Self {
        Self {
            drive: drive.to_string(),
        }
    }

    /// 检查是否可以使用 MFT 直读
    pub fn can_use_mft() -> bool {
        // 后续实现：检查管理员权限 + NTFS 文件系统
        false
    }
}
