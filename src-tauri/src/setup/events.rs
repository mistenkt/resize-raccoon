use tauri::{api::dialog, Builder, Manager, PhysicalSize, Runtime, WindowEvent};

use crate::{debug_log, operations::user_settings::{self, UserSettings}};

pub fn handle_events<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.on_window_event(move |event| match event.event() {
        WindowEvent::Resized(PhysicalSize {
            width: _,
            height: _,
        }) => {
            let window = event.window();
            if window.is_minimized().unwrap_or(false) {
                // The window was minimized, perform your actions here
                window.hide().unwrap(); // Hide the window to move it to the system tray
            }
        }
        WindowEvent::CloseRequested { api, .. } => {
            let app_handle = event.window().app_handle().clone();
            let window_id = event.window().label().to_string();

            // Check if we have setting that says to close to tray
            let user_settings = user_settings::get_user_settings(&app_handle).unwrap_or_else(|e| {
                debug_log!("Error loading user settings: {}", e);
                UserSettings::default()
            });

            if !user_settings.close_to_tray {
                return;
            }

            let window = app_handle.get_window(&window_id).unwrap();
            window.hide().expect("Could not hide the window");
            api.prevent_close();
        }
        _ => {}
    })
}
