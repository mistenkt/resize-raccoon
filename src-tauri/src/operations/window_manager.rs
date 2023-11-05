use crate::profile::Profile;
use winapi::shared::minwindef::{BOOL, DWORD, LPARAM, TRUE, FALSE};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, MoveWindow};
use winapi::um::errhandlingapi::GetLastError;

extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let (pid, profile_ptr, debug_mode): (DWORD, *const Profile, bool) = unsafe { *(lparam as *const (DWORD, *const Profile, bool)) };
    let profile = unsafe { &*profile_ptr };
    
    let mut window_pid = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut window_pid);
    }
    
    if window_pid == pid {
        let success = unsafe {
            MoveWindow(
                hwnd,
                profile.window_pos_x,
                profile.window_pos_y,
                profile.window_width,
                profile.window_height,
                TRUE
            )
        };
        
        if debug_mode {
            if success == 0 {
                let error = unsafe { GetLastError() };
                println!("Failed to move and resize window for PID {}: Error code: {}", pid, error);
            } else {
                println!("Successfully moved and resized window for PID {}", pid);
            }
        }
        
        FALSE // Stop the enumeration because we've found and moved the window
    } else {
        TRUE // Continue enumeration if this window did not match
    }
}

pub fn apply_profile(profile: &Profile, pid: Option<DWORD>, debug_mode: bool) {
    
    let pid = match pid {
        Some(pid) => pid,
        None => crate::process::get_pid_from_profile(profile).unwrap_or(0),
    };

    if pid == 0 {
        if debug_mode {
            println!("Error: Could not find PID for process: {}", profile.process_name);
        }
        return;
    }

    if debug_mode {
        println!("Debug: Attempting to apply profile for process: {}", profile.process_name);
    }

    let callback_data = (pid, profile as *const _, debug_mode);

    unsafe {
        EnumWindows(Some(enum_windows_callback), &callback_data as *const _ as LPARAM);
    }
}
