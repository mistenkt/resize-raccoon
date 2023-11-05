use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Runtime};
use tauri::api::path;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct UserSettings {
    pub process_watcher_enabled: bool,
    pub poll_rate: u64,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            process_watcher_enabled: false,
            poll_rate: 1000
        }
    }
}

pub fn get_user_settings_path<R: Runtime>(app_handle: &AppHandle<R>) -> Result<PathBuf, String> {
    let settings_path = path::app_data_dir(&app_handle.config())
    .ok_or_else(|| "Could not find the app data directory.".to_string())?;

    if !settings_path.exists() {
        fs::create_dir_all(&settings_path).map_err(|e| e.to_string())?;
    }

    Ok(settings_path.join("user_settings.json"))
}

pub fn get_user_settings<R: Runtime>(app_handle: &AppHandle<R>) -> Result<UserSettings, String> {
    // Return the user settings, if they dont exist create them with default settings before returning
    let user_settings_path = get_user_settings_path(app_handle)?;
    if !user_settings_path.exists() {
        let user_settings = UserSettings::default();
        let json_string: String = serde_json::to_string_pretty(&user_settings)
            .map_err(|e| e.to_string())?;
        fs::write(user_settings_path, json_string)
            .map_err(|e| e.to_string())?;
        Ok(user_settings)
    } else {
        let user_settings_json = fs::read_to_string(user_settings_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&user_settings_json)
            .map_err(|e| e.to_string())
    }
}

pub fn update_user_settings<R: Runtime>(user_settings: UserSettings, app_handle: &AppHandle<R>) -> Result<(), String> {
    let user_settings_path = get_user_settings_path(app_handle)?;
    let json_string: String = serde_json::to_string_pretty(&user_settings)
            .map_err(|e| e.to_string())?;
        fs::write(user_settings_path, json_string)
            .map_err(|e| e.to_string())
}