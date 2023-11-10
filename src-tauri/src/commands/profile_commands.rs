use crate::errors::window_manager::Error as WindowManagerError;
use crate::errors::profile::Error as ProfileError;
use crate::profile::{self, Profile};
use crate::window_manager;
use tauri::{AppHandle, Runtime};
use winapi::shared::minwindef::DWORD;

#[tauri::command]
pub fn profile_get<R: Runtime>(app_handle: AppHandle<R>) -> Result<Vec<Profile>, ProfileError> {
    profile::load_profiles(&app_handle)
}

#[tauri::command]
pub fn profile_apply(profile: Profile, pid: Option<u32>) -> Result<(), WindowManagerError> {
    window_manager::apply_profile(&profile, pid.map(|p| p as DWORD), true)
}

#[tauri::command]
pub fn profile_add<R: Runtime>(profile: Profile, app_handle: AppHandle<R>) -> Result<(), ProfileError> {
    profile::add_profile(profile, &app_handle)
}

#[tauri::command]
pub fn profile_update<R: Runtime>(profile: Profile, app_handle: AppHandle<R>) -> Result<(), ProfileError> {
    profile::update_profile(profile, &app_handle)
}

#[tauri::command]
pub fn profile_delete<R: Runtime>(profile: Profile, app_handle: AppHandle<R>) -> Result<(), ProfileError> {
    profile::delete_profile(profile, &app_handle)
}