use std::cell::RefCell;
use std::ptr::NonNull;
use std::thread;
use std::time::Duration;

use crate::debug_log;
use crate::errors::window_manager::Error as WindowManagerError;
use crate::profile::Profile;
use crate::process::is_process_running;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPARAM, TRUE};
use winapi::shared::windef::{HWND, RECT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{
    EnumWindows, GetClassNameW, GetWindowLongW, GetWindowRect,
    GetWindowThreadProcessId, MoveWindow,
    GWL_STYLE, WS_VISIBLE,
};

thread_local! {
    static HOOK_DATA: RefCell<Option<Profile>> = RefCell::new(None);
}

fn is_target_window(hwnd: HWND, target_pid: DWORD) -> bool {
    let mut window_pid = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut window_pid);
    }
    let style = unsafe { GetWindowLongW(hwnd, GWL_STYLE) as u32 };

    window_pid == target_pid && style & WS_VISIBLE != 0
}

fn debug_window(hwnd: HWND) {
    let mut class_name = [0u16; 256]; // Adjust size as necessary
    unsafe {
        GetClassNameW(hwnd, class_name.as_mut_ptr(), class_name.len() as i32);
    }
    let class_name = String::from_utf16_lossy(&class_name);
    debug_log!("Class name: {}", class_name);

    let mut rect: RECT = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetWindowRect(hwnd, &mut rect);
    }

    debug_log!(
        "RECT - Left: {}, Top: {}, Right: {}, Bottom: {}",
        rect.left,
        rect.top,
        rect.right,
        rect.bottom
    );
}

fn validate_window_rect(hwnd: HWND, profile: &Profile) -> bool {
    let mut rect: RECT = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetWindowRect(hwnd, &mut rect);
    }

    let actual_x = rect.left;
    let actual_y = rect.top;
    let actual_width = rect.right - rect.left;
    let actual_height = rect.bottom - rect.top;

    let intended_x = profile.window_pos_x;
    let intended_y = profile.window_pos_y;
    let intended_width = profile.window_width;
    let intended_height = profile.window_height;

    let was_correctly_moved: bool = actual_x == intended_x
        && actual_y == intended_y
        && actual_width == intended_width
        && actual_height == intended_height;

    was_correctly_moved
}

fn move_and_validate_window(
    hwnd: HWND,
    profile: &Profile,
    pid: DWORD,
) -> Result<(), WindowManagerError> {
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

    if moved == 0 {
        let error_code = unsafe { GetLastError() };
        if error_code == 5 {
            debug_log!(
                "Failed to move and resize window for PID {}: Access denied.",
                pid
            );
            return Err(WindowManagerError::AccessDenied);
        } else {
            debug_log!(
                "Failed to move and resize window for PID {}: Error code: {}",
                pid,
                error_code
            );
            return Err(WindowManagerError::ApplyFailed);
        }
    }

    if !validate_window_rect(hwnd, profile) {
        debug_log!(
            "Failed to move and resize window for PID {}: Window was not moved correctly.",
            pid
        );
        return Err(WindowManagerError::ApplyFailed);
    }

    Ok(())
}

fn watch_for_profile_overrides(hwnd: HWND, profile: &Profile, pid: DWORD) {
    let profile_clone = profile.clone();
    let hwnd_as_int = hwnd as usize;

    thread::spawn(move || {
        let mut poll_counter = 0;
        let mut poll_extended_counter = 0;

        let poll_itterations = 10;
        let poll_extended_itterations = 5;

        let hwnd_clone = hwnd_as_int as HWND;

        loop {

            if !is_process_running(pid) {
                debug_log!("[{}] Process no longer running, exiting override polling", profile_clone.name);
                break;
            }

            // check pos
            let matches_profile = validate_window_rect(hwnd_clone, &profile_clone);
            poll_counter += 1;

            if poll_counter <= poll_itterations {
                debug_log!("[{}] Polling for profile overrides: [{}/10 x 1second]", profile_clone.name, poll_counter);
            } else {
                poll_extended_counter += 1;
                debug_log!("[{}] Polling for profile overrides: [{}/5 x 5seconds]", profile_clone.name, poll_extended_counter);
            }
            

            if !matches_profile {
                debug_log!("Window for PID {} was moved or resized.", pid);
                debug_window(hwnd_clone);

                let result = move_and_validate_window(hwnd_clone, &profile_clone, pid);
                if let Err(error) = result {
                    debug_log!("Failed to re-apply profile: {:?}", error);
                }
            }

            if poll_counter < poll_itterations {
                thread::sleep(Duration::from_secs(1));
            } else {
                if poll_extended_counter < poll_extended_itterations {
                    thread::sleep(Duration::from_secs(5));
                } else {
                    // we are done polling
                    debug_log!("Done polling for profile overrides.");
                    break;
                }
            }
        }
    });
}

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

    if is_target_window(hwnd, pid) {
        let result = move_and_validate_window(hwnd, profile, pid);
        debug_window(hwnd);
        match result {
            Ok(()) => {
                unsafe { *success_ptr.as_mut() = true };
                watch_for_profile_overrides(hwnd, profile, pid);
            }
            Err(error) => unsafe {
                *success_ptr.as_mut() = false;
                *error_ptr.as_mut() = Some(error);
            },
        }
        return FALSE; // Stop enumeration because we've found and moved the window
    }

    TRUE // Continue enumeration if this window did not match
}

pub fn apply_profile(profile: &Profile, pid: Option<DWORD>, retry: bool, retries: Option<u8>) -> Result<(), WindowManagerError> {
    let pid = pid.unwrap_or_else(|| crate::process::get_pid_from_profile(profile).unwrap_or(0));
    let retries = retries.unwrap_or(0);

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
        debug_log!("Successfully applied profile: {}", profile.name);
        Ok(())
    } else if let Some(error) = error {
        Err(error)
    } else {
        if retry && retries < 2 {
            debug_log!("[{}] Faild to find active window, retrying in 5 seconds...", profile.name);
            thread::sleep(Duration::from_secs(5));
            return apply_profile(profile, Some(pid), true, Some(retries + 1));
        } else {
            debug_log!("[{}] Failed to find active window after 3 attempts.", profile.name);
            Err(WindowManagerError::ApplyFailed)
        }
        
    }
}
