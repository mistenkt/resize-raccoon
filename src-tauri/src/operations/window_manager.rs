use std::ptr::NonNull;

use crate::profile::Profile;
use crate::errors::window_manager::Error as WindowManagerError;
use winapi::shared::minwindef::{BOOL, DWORD, LPARAM, TRUE, FALSE};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, MoveWindow};
use winapi::um::errhandlingapi::GetLastError;

extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let (pid, profile_ptr, mut success_ptr, debug_mode, mut error_ptr): 
    (DWORD, *const Profile, NonNull<bool>, bool, NonNull<Option<WindowManagerError>>) = 
        unsafe { *(lparam as *const (DWORD, *const Profile, NonNull<bool>, bool, NonNull<Option<WindowManagerError>>)) };

    let profile = unsafe { &*profile_ptr };
    let success_ref = unsafe { success_ptr.as_mut() }; // Safely get a mutable reference to success

    let mut window_pid = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut window_pid);
    }
    
    if window_pid == pid {
        let moved = unsafe {
            MoveWindow(
                hwnd,
                profile.window_pos_x,
                profile.window_pos_y,
                profile.window_width,
                profile.window_height,
                TRUE,
            )
        };

        *success_ref = moved != 0; // Set success to true if MoveWindow succeeded

        if moved == 0 {
            let error_code = unsafe { GetLastError() };
            if error_code == 5 {
                unsafe {
                    *error_ptr.as_mut() = Some(WindowManagerError::AccessDenied);
                }
                if debug_mode {
                    println!("Failed to move and resize window for PID {}: Access denied.", pid);
                }
            } else {
                unsafe {
                    *error_ptr.as_mut() = Some(WindowManagerError::ApplyFailed);
                }
                if debug_mode {
                    println!("Failed to move and resize window for PID {}: Error code: {}", pid, error_code);
                }
            }
        } else {
            if debug_mode {
                println!("Successfully moved and resized window for PID {}", pid);
            }
        }
        return FALSE; // Stop enumeration because we've found and moved the window
    }

    TRUE // Continue enumeration if this window did not match
}

pub fn apply_profile(profile: &Profile, pid: Option<DWORD>, debug_mode: bool) -> Result<(), WindowManagerError> {
    let pid = pid.unwrap_or_else(|| crate::process::get_pid_from_profile(profile).unwrap_or(0));

    if pid == 0 {
        return Err(WindowManagerError::ProcessNotFound);
    }

    let mut success = false; // Here is our success variable
    let success_ptr = NonNull::new(&mut success).unwrap(); 

    let mut error = None;
    let error_ptr = NonNull::new(&mut error).unwrap();

    let callback_data = (pid, profile as *const _, success_ptr, debug_mode, error_ptr);

    unsafe {
        EnumWindows(Some(enum_windows_callback), &callback_data as *const _ as LPARAM);
    }

    if success {
        Ok(())
    } else if let Some(error) = error {
        Err(error)
    } else {
        Err(WindowManagerError::ApplyFailed)
    }
}
