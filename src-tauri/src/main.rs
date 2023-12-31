// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod operations {
    pub mod process;
    pub mod profile;
    pub mod user_settings;
    pub mod window_manager;
}

mod setup {
    pub mod events;
    pub mod init;
    pub mod state;
    pub mod tray;
    pub mod ipc;
}

mod commands;
mod errors;


mod debug_utils;

use crate::operations::{process, profile, user_settings, window_manager};

fn main() {
    dotenvy::dotenv().ok();

    let builder = tauri::Builder::default();
    let builder = commands::register_commands(builder);
    let builder = setup::init::setup(builder);
    let builder = setup::tray::setup_tray(builder);
    let builder = setup::events::handle_events(builder);

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
