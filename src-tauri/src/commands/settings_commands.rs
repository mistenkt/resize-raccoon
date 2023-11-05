use std::sync::atomic::Ordering;

use tauri::{Runtime, Manager, AppHandle};
use crate::user_settings::{self, UserSettings};
use crate::setup::state::AppState;


#[tauri::command]
pub fn settings_get<R: Runtime>(app: AppHandle<R>) -> Result<UserSettings, String> {
    // Call the get_user_settings function and directly return the Result<(), String>
    user_settings::get_user_settings(&app)
}

#[tauri::command]
pub fn settings_update<R: Runtime>(app: AppHandle<R>, settings: UserSettings) -> Result<(), String> {
    // Access the managed state from the application
    let app_state = app.state::<AppState>();

    // Update the running flag within the state
    app_state.process_watcher_enabled.store(settings.process_watcher_enabled, Ordering::SeqCst);
    app_state.poll_rate.store(settings.poll_rate, Ordering::SeqCst);

    user_settings::update_user_settings(settings, &app)
}