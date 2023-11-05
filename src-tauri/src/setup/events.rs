use tauri::{Runtime, Builder, WindowEvent, PhysicalSize, Manager, api::dialog};

pub fn handle_events<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder
        .on_window_event(move |event| match event.event() {
            WindowEvent::Resized(PhysicalSize { width: _, height: _ }) => {
                let window = event.window();
                if window.is_minimized().unwrap_or(false) {
                    // The window was minimized, perform your actions here
                    window.hide().unwrap(); // Hide the window to move it to the system tray
                }
            },
            WindowEvent::CloseRequested { api, .. } => {
                let app_handle = event.window().app_handle().clone();
                let window_id = event.window().label().to_string();
    
                // Ask the user if they want to close or minimize to system tray
                dialog::ask(
                    Some(event.window()),
                    "Confirm Close",
                    "Would you like to minimize to system tray instead of closing",
                    move |answer| {
                        let window = app_handle.get_window(&window_id).unwrap();
                        if answer {
                            window.hide().expect("Could not hide the window");
                        } else {
                            app_handle.exit(0);
                        }
                    }
                );
    
                // Prevent the default close behavior until the user has made their choice
                api.prevent_close();
            },
            _ => {}
        
        })
}