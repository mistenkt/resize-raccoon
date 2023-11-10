// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod operations {
    pub mod profile;
    pub mod window_manager;
    pub mod user_settings;
    pub mod process;
}

mod setup {
    pub mod init;
    pub mod state;
    pub mod tray;
    pub mod events;
}

mod commands;
mod errors;

use crate::operations::{profile, process, user_settings, window_manager};

fn main() {
    let debug = true;

    let builder = tauri::Builder::default();
    let builder = commands::register_commands(builder);
    let builder = setup::init::setup(builder, debug);
    let builder = setup::tray::setup_tray(builder);
    let builder = setup::events::handle_events(builder);


    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
