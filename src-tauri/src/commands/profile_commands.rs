use crate::errors::profile::Error as ProfileError;
use crate::errors::window_manager::Error as WindowManagerError;
use crate::operations::window_manager::ApplyConfig;
use crate::profile::{self, Profile};
use crate::window_manager;
use tauri::{AppHandle, Runtime};

#[tauri::command]
pub fn profile_get<R: Runtime>(app_handle: AppHandle<R>) -> Result<Vec<Profile>, ProfileError> {
    profile::load_profiles(&app_handle)
}

#[tauri::command]
pub fn profile_apply(profile: Profile, pid: Option<u32>) -> Result<(), WindowManagerError> {
    let config = ApplyConfig::new().pid(pid).retry(true).monitor(true);
    window_manager::apply_profile(&profile, config)
}

#[tauri::command]
pub fn profile_test(profile: Profile, pid: Option<u32>) -> Result<(), WindowManagerError> {
    let config = ApplyConfig::new().pid(pid).retry(false).monitor(false);
    window_manager::apply_profile(&profile, config)
}

#[tauri::command]
pub fn profile_add<R: Runtime>(
    profile: Profile,
    app_handle: AppHandle<R>,
) -> Result<(), ProfileError> {
    profile::add_profile(profile, &app_handle)
}

#[tauri::command]
pub fn profile_update<R: Runtime>(
    profile: Profile,
    app_handle: AppHandle<R>,
) -> Result<(), ProfileError> {
    profile::update_profile(profile, &app_handle)
}

#[tauri::command]
pub fn profile_delete<R: Runtime>(
    profile: Profile,
    app_handle: AppHandle<R>,
) -> Result<(), ProfileError> {
    profile::delete_profile(profile, &app_handle)
}
