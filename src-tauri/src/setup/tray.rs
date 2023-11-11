use tauri::{
    Builder, CustomMenuItem, Manager, Runtime, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn setup_tray<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    let system_tray = SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("show", "Show"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("exit", "Exit")),
    );

    builder
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                }
                "exit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
}
