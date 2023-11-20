use auto_launch::AutoLaunch;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};
use tauri::api::path;
use tauri::{AppHandle, Runtime};

use crate::debug_log;
use crate::errors::settings::Error as SettingsError;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct UserSettings {
    pub process_watcher_enabled: bool,
    pub poll_rate: u64,
    pub check_for_updates: bool,
    pub has_prompted_for_launch_on_start: bool,
    pub launch_on_start: bool,
}

#[derive(Serialize)]
pub struct SerializeableUserSettings {
    pub process_watcher_enabled: bool,
    pub poll_rate: u64,
    pub check_for_updates: bool,
    pub has_prompted_for_launch_on_start: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            process_watcher_enabled: false,
            poll_rate: 1000,
            check_for_updates: true,
            launch_on_start: false,
            has_prompted_for_launch_on_start: false,
        }
    }
}

impl UserSettings {
    pub fn to_serializeable(&self) -> SerializeableUserSettings {
        SerializeableUserSettings {
            process_watcher_enabled: self.process_watcher_enabled,
            poll_rate: self.poll_rate,
            check_for_updates: self.check_for_updates,
            has_prompted_for_launch_on_start: self.has_prompted_for_launch_on_start,
        }
    }
}

pub fn get_user_settings_path<R: Runtime>(
    app_handle: &AppHandle<R>,
) -> Result<PathBuf, SettingsError> {
    let settings_path =
        path::app_data_dir(&app_handle.config()).ok_or(SettingsError::SettingsPathError)?;

    if !settings_path.exists() {
        fs::create_dir_all(&settings_path)?;
    }

    Ok(settings_path.join("user_settings.json"))
}

pub fn get_user_settings<R: Runtime>(
    app_handle: &AppHandle<R>,
) -> Result<UserSettings, SettingsError> {
    // Return the user settings, if they dont exist create them with default settings before returning
    let user_settings_path = get_user_settings_path(app_handle)?;
    let mut user_settings = UserSettings::default();

    if !user_settings_path.exists() {
        let json_string: String = serde_json::to_string_pretty(&user_settings.to_serializeable())?;
        fs::write(user_settings_path, json_string)?;
    } else {
        let user_settings_json = fs::read_to_string(user_settings_path)?;
        user_settings = serde_json::from_str(&user_settings_json)?;
    }

    user_settings.launch_on_start = get_auto_launch_value(&get_auto_launch_instance(app_handle)?);
    Ok(user_settings)
}

pub fn update_user_settings<R: Runtime>(
    user_settings: UserSettings,
    app_handle: &AppHandle<R>,
) -> Result<(), SettingsError> {
    let user_settings_path = get_user_settings_path(app_handle)?;
    let json_string: String = serde_json::to_string_pretty(&user_settings.to_serializeable())?;

    fs::write(user_settings_path, json_string).map_err(Into::into)
}

fn get_auto_launch_instance<R: Runtime>(
    app_handle: &AppHandle<R>,
) -> Result<AutoLaunch, SettingsError> {
    let app_config = app_handle.config();

    let app_name = match &app_config.package.product_name {
        Some(name) => name.clone(),
        None => return Err(SettingsError::UnableToFetchAppData),
    };

    let app_path_buf = env::current_exe().map_err(|_| SettingsError::UnableToFetchAppData)?;

    let app_path = app_path_buf
        .to_str()
        .ok_or(SettingsError::UnableToFetchAppData)?;

    let auto = AutoLaunch::new(&app_name, &app_path, &[] as &[&str]);

    Ok(auto)
}

fn get_auto_launch_value(auto: &AutoLaunch) -> bool {
    auto.is_enabled().unwrap_or(false)
}

pub fn toggle_launch_on_start<R: Runtime>(
    launch_on_start: bool,
    app_handle: &AppHandle<R>,
) -> Result<bool, SettingsError> {
    debug_log!("toggle_launch_on_start: {}", launch_on_start);
    let auto = get_auto_launch_instance(app_handle).unwrap();

    // Check the current status
    let is_enabled = get_auto_launch_value(&auto);

    if launch_on_start != is_enabled {
        if launch_on_start {
            auto.enable()
                .map_err(|_| SettingsError::LaunchOnStartError)?;
        } else {
            auto.disable()
                .map_err(|_| SettingsError::LaunchOnStartError)?;
        }
    }

    let is_now_enabled = auto.is_enabled().unwrap_or(false);

    debug_log!("toggle launch on start flag: {}", is_now_enabled);

    if launch_on_start != is_now_enabled {
        Err(SettingsError::LaunchOnStartError)
    } else {
        Ok(is_now_enabled)
    }
}
