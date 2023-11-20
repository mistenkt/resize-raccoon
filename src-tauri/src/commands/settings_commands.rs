use std::sync::atomic::Ordering;

use crate::setup::state::AppState;
use crate::user_settings::{self, UserSettings};
use tauri::{AppHandle, Manager, Runtime};
use crate::errors::settings::Error as SettingsError;

#[tauri::command]
pub fn settings_get<R: Runtime>(app: AppHandle<R>) -> Result<UserSettings, SettingsError> {
    user_settings::get_user_settings(&app)
}

#[tauri::command]
pub fn settings_update<R: Runtime>(
    app: AppHandle<R>,
    settings: UserSettings,
) -> Result<(), SettingsError> {
    // Access the managed state from the application
    let app_state = app.state::<AppState>();

    // Update the running flag within the state
    app_state
        .process_watcher_enabled
        .store(settings.process_watcher_enabled, Ordering::SeqCst);
    app_state
        .poll_rate
        .store(settings.poll_rate, Ordering::SeqCst);

    user_settings::update_user_settings(settings, &app)
}

#[tauri::command]
pub fn settings_toggle_launch_on_start<R: Runtime>(app: AppHandle<R>, launch_on_start: bool) -> Result<bool, SettingsError> {
    user_settings::toggle_launch_on_start(launch_on_start, &app)
}
