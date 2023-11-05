use crate::profile::{self, Profile};
use crate::window_manager;
use tauri::{AppHandle, Runtime};

#[tauri::command]
pub fn profile_get<R: Runtime>(app_handle: AppHandle<R>) -> Result<Vec<Profile>, String> {
    profile::load_profiles(&app_handle)
}

#[tauri::command]
pub fn profile_apply(profile: Profile, pid: Option<u32>) {
    window_manager::apply_profile(&profile, pid, false);
}

#[tauri::command]
pub fn profile_add<R: Runtime>(profile: Profile, app_handle: AppHandle<R>) -> Result<(), String> {
    profile::add_profile(profile, &app_handle)
}

#[tauri::command]
pub fn profile_update<R: Runtime>(profile: Profile, app_handle: AppHandle<R>) -> Result<(), String> {
    profile::update_profile(profile, &app_handle)
}

#[tauri::command]
pub fn profile_delete<R: Runtime>(profile: Profile, app_handle: AppHandle<R>) -> Result<(), String> {
    profile::delete_profile(profile, &app_handle)
}