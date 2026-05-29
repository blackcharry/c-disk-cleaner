// src-tauri/src/registry/installed.rs

/// 已安装软件信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct InstalledSoftware {
    pub display_name: String,
    pub publisher: String,
    pub install_location: String,
    pub uninstall_string: Option<String>,
}

#[cfg(target_os = "windows")]
mod imp {
    use super::InstalledSoftware;
    use winreg::enums::*;
    use winreg::RegKey;

    pub fn get_installed_software() -> Vec<InstalledSoftware> {
        let mut software = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // 三个注册表路径都要查
        let keys = [
            (
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            ),
            (
                HKEY_LOCAL_MACHINE,
                r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
            ),
            (
                HKEY_CURRENT_USER,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
            ),
        ];

        for (hkey, path) in &keys {
            if let Ok(key) = RegKey::predef(*hkey).open_subkey_with_flags(path, KEY_READ) {
                for subkey_name in key.enum_keys().filter_map(|k| k.ok()) {
                    if let Ok(subkey) = key.open_subkey_with_flags(&subkey_name, KEY_READ) {
                        let display_name: String =
                            subkey.get_value("DisplayName").unwrap_or_default();
                        let publisher: String =
                            subkey.get_value("Publisher").unwrap_or_default();
                        let install_location: String =
                            subkey.get_value("InstallLocation").unwrap_or_default();
                        let uninstall_string: String =
                            subkey.get_value("UninstallString").unwrap_or_default();

                        if !display_name.is_empty() && seen.insert(display_name.clone()) {
                            software.push(InstalledSoftware {
                                display_name,
                                publisher,
                                install_location,
                                uninstall_string: if uninstall_string.is_empty() {
                                    None
                                } else {
                                    Some(uninstall_string)
                                },
                            });
                        }
                    }
                }
            }
        }

        software
    }
}

#[cfg(not(target_os = "windows"))]
mod imp {
    use super::InstalledSoftware;

    #[allow(dead_code)]
    pub fn get_installed_software() -> Vec<InstalledSoftware> {
        Vec::new()
    }
}

pub use imp::get_installed_software;
