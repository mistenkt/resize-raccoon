use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};

use std::thread;

use crate::debug_log;
use crate::operations::{
    process, profile,
    user_settings::{self, UserSettings},
};
use crate::setup::ipc;
use crate::setup::state::AppState;
use tauri::{Builder, Manager, Runtime};

pub fn setup<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.setup(move |app| {
        let watcher_flag = Arc::new(AtomicBool::new(false));
        let poll_rate_flag = Arc::new(AtomicU64::new(1000));
        let app_handle = app.handle();
        let user_settings = user_settings::get_user_settings(&app_handle).unwrap_or_else(|e| {
            debug_log!("Error loading user settings: {}", e);
            UserSettings::default()
        });

        let profiles = Arc::new(Mutex::new(
            profile::load_profiles(&app_handle).unwrap_or_else(|e| {
                debug_log!("Error loading profiles: {}", e);
                Vec::new()
            }),
        ));
        // Store the initial state of the process watcher
        watcher_flag.store(user_settings.process_watcher_enabled, Ordering::SeqCst);
        poll_rate_flag.store(user_settings.poll_rate, Ordering::SeqCst);

        let app_state = AppState {
            profiles: profiles.clone(),
            process_watcher_enabled: watcher_flag.clone(),
            poll_rate: poll_rate_flag.clone(),
        };
        app.manage(app_state);

        let profiles_clone = profiles.clone();

        thread::Builder::new().name("Process watcher".to_string()).spawn(move || {
            process::watcher(watcher_flag.clone(), poll_rate_flag.clone(), profiles_clone);
        }).unwrap();

        let profiles_clone = profiles.clone();

        thread::Builder::new().name("IPC Listener".to_string()).spawn(move || {
            ipc::listener(profiles_clone);
        }).unwrap();

        // Check if we should minimize to sys tray
        if user_settings.start_minimized {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
        }

        Ok(())
    })
}
