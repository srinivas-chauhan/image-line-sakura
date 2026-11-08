//! Windows registry read helpers for Image Line Sakura.

pub const APP_KEY: &str = "Software\\Image\\Image Line Sakura";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Image Line Sakura";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Image Line Sakura") },
        RegValue { name: "Version",     value: String::from("v2.5.0") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Image Line Sakura") },
        RegValue { name: "Publisher",       value: String::from("Image") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Image Line Sakura\\uninstall.exe\"") },
    ]
}
