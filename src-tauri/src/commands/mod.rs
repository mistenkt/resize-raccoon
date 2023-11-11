use tauri::{generate_handler, Builder, Runtime};

pub mod process_commands;
pub mod profile_commands;
pub mod settings_commands;

pub fn register_commands<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.invoke_handler(generate_handler![
        profile_commands::profile_get,
        profile_commands::profile_apply,
        profile_commands::profile_add,
        profile_commands::profile_update,
        profile_commands::profile_delete,
        process_commands::process_get,
        settings_commands::settings_get,
        settings_commands::settings_update,
    ])
}
