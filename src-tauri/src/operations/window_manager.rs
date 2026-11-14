use std::cell::RefCell;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::debug_log;
use crate::errors::window_manager::Error as WindowManagerError;
use crate::process::is_process_running;
use crate::profile::Profile;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPARAM, TRUE};
use winapi::shared::windef::{HWND, RECT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{
    EnumWindows, GetClassNameW, GetSystemMetrics, GetWindowLongPtrW, GetWindowLongW, GetWindowRect,
    GetWindowThreadProcessId, IsWindow, MoveWindow, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE,
    GWL_STYLE, SM_CMONITORS, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
    SM_YVIRTUALSCREEN, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOOWNERZORDER,
    SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER, WS_BORDER, WS_DLGFRAME, WS_EX_CLIENTEDGE,
    WS_EX_DLGMODALFRAME, WS_EX_STATICEDGE, WS_EX_WINDOWEDGE, WS_THICKFRAME, WS_VISIBLE,
};

thread_local! {
    static HOOK_DATA: RefCell<Option<Profile>> = RefCell::new(None);
}

/// The newest override watcher per PID. Starting a watcher for a PID retires the previous one,
/// so two profiles applied to the same process never fight each other.
static ACTIVE_WATCHERS: Mutex<Vec<(DWORD, u64)>> = Mutex::new(Vec::new());
static WATCHER_GENERATION: AtomicU64 = AtomicU64::new(0);

fn register_watcher(pid: DWORD) -> u64 {
    let generation = WATCHER_GENERATION.fetch_add(1, Ordering::SeqCst) + 1;
    let mut watchers = ACTIVE_WATCHERS.lock().unwrap();
    watchers.retain(|(watched_pid, _)| *watched_pid != pid);
    watchers.push((pid, generation));
    generation
}

fn is_current_watcher(pid: DWORD, generation: u64) -> bool {
    ACTIVE_WATCHERS
        .lock()
        .unwrap()
        .iter()
        .any(|(watched_pid, watched_generation)| {
            *watched_pid == pid && *watched_generation == generation
        })
}

fn unregister_watcher(pid: DWORD, generation: u64) {
    ACTIVE_WATCHERS
        .lock()
        .unwrap()
        .retain(|(watched_pid, watched_generation)| {
            !(*watched_pid == pid && *watched_generation == generation)
        });
}

/// Monitor count plus the bounds of the virtual desktop. Changes whenever a display is
/// connected, disconnected or rearranged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DisplayLayout {
    monitors: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn current_display_layout() -> DisplayLayout {
    unsafe {
        DisplayLayout {
            monitors: GetSystemMetrics(SM_CMONITORS),
            x: GetSystemMetrics(SM_XVIRTUALSCREEN),
            y: GetSystemMetrics(SM_YVIRTUALSCREEN),
            width: GetSystemMetrics(SM_CXVIRTUALSCREEN),
            height: GetSystemMetrics(SM_CYVIRTUALSCREEN),
        }
    }
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

fn remove_window_borders(hwnd: HWND, profile: &Profile) {
    if profile.remove_borders == false {
        return;
    }

    // Remove standard window styles
    let mut style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) };
    style |= (WS_THICKFRAME | WS_DLGFRAME | WS_BORDER) as isize;
    style ^= (WS_THICKFRAME | WS_DLGFRAME | WS_BORDER) as isize;
    unsafe { SetWindowLongPtrW(hwnd, GWL_STYLE, style) };

    // Remove extended window styles
    let mut ex_style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
    ex_style |=
        (WS_EX_DLGMODALFRAME | WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE | WS_EX_STATICEDGE) as isize;
    ex_style ^=
        (WS_EX_DLGMODALFRAME | WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE | WS_EX_STATICEDGE) as isize;
    unsafe { SetWindowLongPtrW(hwnd, GWL_EXSTYLE, ex_style) };

    // Adjust window position to refresh the appearance
    let u_flags = SWP_NOSIZE
        | SWP_NOMOVE
        | SWP_NOZORDER
        | SWP_NOACTIVATE
        | SWP_NOOWNERZORDER
        | SWP_NOSENDCHANGING
        | SWP_FRAMECHANGED;
    unsafe { SetWindowPos(hwnd, ptr::null_mut(), 0, 0, 0, 0, u_flags) };
}

fn move_and_validate_window(
    hwnd: HWND,
    profile: &Profile,
    pid: DWORD,
) -> Result<(), WindowManagerError> {
    remove_window_borders(hwnd, profile);

    // SWP_NOSENDCHANGING skips WM_WINDOWPOSCHANGING, so neither the target's own handler nor
    // the default min/max tracking limits get a chance to clamp the rect we ask for.
    let flags = SWP_NOZORDER | SWP_NOACTIVATE | SWP_NOOWNERZORDER | SWP_NOSENDCHANGING;
    let mut moved = unsafe {
        SetWindowPos(
            hwnd,
            ptr::null_mut(),
            profile.window_pos_x,
            profile.window_pos_y,
            profile.window_width,
            profile.window_height,
            flags,
        )
    };

    if moved != 0 && !validate_window_rect(hwnd, profile) {
        debug_log!(
            "Window for PID {} did not keep the requested rect, retrying with MoveWindow.",
            pid
        );
        debug_window(hwnd);

        moved = unsafe {
            MoveWindow(
                hwnd,
                profile.window_pos_x,
                profile.window_pos_y,
                profile.window_width,
                profile.window_height,
                TRUE,
            )
        };
    }

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
            "Failed to move and resize window for PID {}: Window was not moved correctly. Wanted x {} y {} w {} h {}, display layout is {:?}.",
            pid,
            profile.window_pos_x,
            profile.window_pos_y,
            profile.window_width,
            profile.window_height,
            current_display_layout()
        );
        debug_window(hwnd);
        return Err(WindowManagerError::ApplyFailed);
    }

    Ok(())
}

fn watch_for_profile_overrides(hwnd: HWND, profile: &Profile, pid: DWORD) {
    let profile_clone = profile.clone();
    let hwnd_as_int = hwnd as usize;
    let generation = register_watcher(pid);

    thread::Builder::new()
        .name("Profile override watcher".to_string())
        .spawn(move || {
            // 10 checks one second apart, then 5 checks five seconds apart. After that the
            // window is left alone until the display layout changes, which starts a new round.
            let burst_polls: u32 = 10;
            let total_polls: u32 = burst_polls + 5;

            let hwnd_clone = hwnd_as_int as HWND;
            let mut polls: u32 = 0;
            let mut layout = current_display_layout();

            loop {
                if !is_process_running(pid) || unsafe { IsWindow(hwnd_clone) } == 0 {
                    debug_log!(
                        "[{}] Process or window is gone, exiting override polling",
                        profile_clone.name
                    );
                    break;
                }

                if !is_current_watcher(pid, generation) {
                    debug_log!(
                        "[{}] A newer profile was applied to PID {}, exiting override polling",
                        profile_clone.name,
                        pid
                    );
                    break;
                }

                // A display being connected, disconnected or rearranged makes Windows (and
                // often the game) reposition the window, so check the profile again.
                let new_layout = current_display_layout();
                if new_layout != layout {
                    debug_log!(
                        "[{}] Display layout changed from {:?} to {:?}, checking the profile again",
                        profile_clone.name,
                        layout,
                        new_layout
                    );
                    layout = new_layout;
                    polls = 0;
                    // Give Windows and the game a moment to finish reacting to the change
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }

                if polls < total_polls {
                    polls += 1;
                    debug_log!(
                        "[{}] Polling for profile overrides: [{}/{}]",
                        profile_clone.name,
                        polls,
                        total_polls
                    );

                    if !validate_window_rect(hwnd_clone, &profile_clone) {
                        debug_log!("Window for PID {} was moved or resized.", pid);
                        debug_window(hwnd_clone);

                        let result = move_and_validate_window(hwnd_clone, &profile_clone, pid);
                        if let Err(error) = result {
                            debug_log!("Failed to re-apply profile: {:?}", error);
                        }
                    }

                    if polls == total_polls {
                        debug_log!(
                            "[{}] Done polling for profile overrides, waiting for display changes.",
                            profile_clone.name
                        );
                    }
                }

                if polls < burst_polls {
                    thread::sleep(Duration::from_secs(1));
                } else if polls < total_polls {
                    thread::sleep(Duration::from_secs(5));
                } else {
                    thread::sleep(Duration::from_secs(2));
                }
            }

            unregister_watcher(pid, generation);
        })
        .unwrap();
}

extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let (pid, profile_ptr, mut success_ptr, mut error_ptr, monitor): (
        DWORD,
        *const Profile,
        NonNull<bool>,
        NonNull<Option<WindowManagerError>>,
        bool,
    ) = unsafe {
        *(lparam
            as *const (
                DWORD,
                *const Profile,
                NonNull<bool>,
                NonNull<Option<WindowManagerError>>,
                bool,
            ))
    };

    let profile = unsafe { &*profile_ptr };

    if is_target_window(hwnd, pid) {
        let result = move_and_validate_window(hwnd, profile, pid);
        debug_window(hwnd);
        match result {
            Ok(()) => {
                unsafe { *success_ptr.as_mut() = true };
                if monitor {
                    watch_for_profile_overrides(hwnd, profile, pid);
                }
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

#[derive(Debug)]
pub struct ApplyConfig {
    pid: Option<u32>,
    retry: bool,
    retries: u8,
    monitor: bool,
}

impl Default for ApplyConfig {
    fn default() -> Self {
        ApplyConfig {
            pid: None,
            retry: false,
            retries: 0,
            monitor: false,
        }
    }
}

impl ApplyConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pid(mut self, pid: Option<DWORD>) -> Self {
        self.pid = pid;
        self
    }

    pub fn retry(mut self, retry: bool) -> Self {
        self.retry = retry;
        self
    }

    pub fn retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }

    pub fn monitor(mut self, monitor: bool) -> Self {
        self.monitor = monitor;
        self
    }
}

pub fn apply_profile(profile: &Profile, config: ApplyConfig) -> Result<(), WindowManagerError> {
    let pid = config
        .pid
        .unwrap_or_else(|| crate::process::get_pid_from_profile(profile).unwrap_or(0));

    debug_log!("Profile config {:#?}", config);

    if pid == 0 {
        return Err(WindowManagerError::ProcessNotFound);
    }

    let mut success = false; // Here is our success variable
    let success_ptr = NonNull::new(&mut success).unwrap();

    let mut error = None;
    let error_ptr = NonNull::new(&mut error).unwrap();

    let callback_data = (
        pid,
        profile as *const _,
        success_ptr,
        error_ptr,
        config.monitor,
    );

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
        if config.retry && config.retries < 2 {
            debug_log!(
                "[{}] Faild to find active window, retrying in 5 seconds...",
                profile.name
            );
            thread::sleep(Duration::from_secs(5));

            let retry_config = ApplyConfig::new()
                .pid(Some(pid))
                .retry(true)
                .monitor(config.monitor)
                .retries(config.retries + 1);

            return apply_profile(profile, retry_config);
        } else {
            debug_log!(
                "[{}] Failed to find active window after 3 attempts.",
                profile.name
            );
            Err(WindowManagerError::ApplyFailed)
        }
    }
}
