use std::ptr::NonNull;
use std::thread;
use std::time::Duration;

use crate::errors::window_manager::Error as WindowManagerError;
use crate::profile::Profile;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPARAM, TRUE};
use winapi::shared::windef::{HWND, RECT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, MoveWindow, GetWindowRect, GetWindowLongW, GWL_STYLE, WS_VISIBLE, GetClassNameW};

use crate::debug_log;

extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let (pid, profile_ptr, mut success_ptr, mut error_ptr): (
        DWORD,
        *const Profile,
        NonNull<bool>,
        NonNull<Option<WindowManagerError>>,
    ) = unsafe {
        *(lparam
            as *const (
                DWORD,
                *const Profile,
                NonNull<bool>,
                NonNull<Option<WindowManagerError>>,
            ))
    };

    let profile = unsafe { &*profile_ptr };
    let success_ref = unsafe { success_ptr.as_mut() }; // Safely get a mutable reference to success

    let mut window_pid = 0;
    
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut window_pid);
    }
    let style = unsafe { GetWindowLongW(hwnd, GWL_STYLE) as u32 };

    if window_pid == pid && style & WS_VISIBLE != 0 {
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

        let mut class_name = [0u16; 256]; // Adjust size as necessary
    unsafe {
        GetClassNameW(hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
    }
    let class_name = String::from_utf16_lossy(&class_name);
    debug_log!("Class name: {}", class_name);

        *success_ref = moved != 0; // Set success to true if MoveWindow succeeded

        if moved == 0 {
            let error_code = unsafe { GetLastError() };
            if error_code == 5 {
                unsafe {
                    *error_ptr.as_mut() = Some(WindowManagerError::AccessDenied);
                }
                debug_log!(
                    "Failed to move and resize window for PID {}: Access denied.",
                    pid
                );
            } else {
                unsafe {
                    *error_ptr.as_mut() = Some(WindowManagerError::ApplyFailed);
                }
                debug_log!(
                    "Failed to move and resize window for PID {}: Error code: {}",
                    pid,
                    error_code
                );
            }
        } else {
            thread::sleep(Duration::from_millis(50));
            let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
            unsafe {
                GetWindowRect(hwnd, &mut rect);
            }

            debug_log!(
                "RECT - Left: {}, Top: {}, Right: {}, Bottom: {}", 
                rect.left, rect.top, rect.right, rect.bottom
            );

            let actual_x = rect.left;
            let actual_y = rect.top;
            let actual_width = rect.right - rect.left;
            let actual_height = rect.bottom - rect.top;

            let intended_x = profile.window_pos_x;
            let intended_y = profile.window_pos_y;
            let intended_width = profile.window_width;
            let intended_height = profile.window_height;

            if actual_x == intended_x && actual_y == intended_y && actual_width == intended_width && actual_height == intended_height {
                debug_log!("Window moved to intended position and size.");
            } else {
                debug_log!("Window did not move to intended position and size.");
                *success_ref = false; // Update success_ref to false
            }
        }
        return FALSE; // Stop enumeration because we've found and moved the window
    }

    TRUE // Continue enumeration if this window did not match
}

pub fn apply_profile(profile: &Profile, pid: Option<DWORD>) -> Result<(), WindowManagerError> {
    let pid = pid.unwrap_or_else(|| crate::process::get_pid_from_profile(profile).unwrap_or(0));

    if pid == 0 {
        return Err(WindowManagerError::ProcessNotFound);
    }

    let mut success = false; // Here is our success variable
    let success_ptr = NonNull::new(&mut success).unwrap();

    let mut error = None;
    let error_ptr = NonNull::new(&mut error).unwrap();

    let callback_data = (pid, profile as *const _, success_ptr, error_ptr);

    unsafe {
        EnumWindows(
            Some(enum_windows_callback),
            &callback_data as *const _ as LPARAM,
        );
    }

    if success {
        Ok(())
    } else if let Some(error) = error {
        Err(error)
    } else {
        Err(WindowManagerError::ApplyFailed)
    }
}
