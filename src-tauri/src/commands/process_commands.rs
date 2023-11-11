use crate::process::{self, ProcessInfo};

#[tauri::command]
pub fn process_get() -> Vec<ProcessInfo> {
    process::get_process_list()
}
