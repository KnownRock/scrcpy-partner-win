use std::mem;
use tauri::LogicalPosition;
use tauri::Position;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::TRUE;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::shared::windef::HWINEVENTHOOK;
use winapi::shared::windef::HWND;
use winapi::shared::windef::RECT;
use winapi::um::winuser::UnhookWinEvent;
// https://gist.github.com/daniel-abramov/5a460d9ca02948f1d2bfa53100c941da
pub fn enumerate_windows<F>(mut callback: F)
where
    F: FnMut(HWND) -> bool,
{
    use winapi::um::winuser::EnumWindows;

    unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let closure: &mut &mut dyn FnMut(HWND) -> bool = mem::transmute(lparam as *mut c_void);
        if closure(hwnd) {
            TRUE
        } else {
            FALSE
        }
    }

    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };

    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumWindows(Some(enumerate_callback), lparam) };
}

#[test]
fn test_enumerate_windows() {
    enumerate_windows(|h| {
        println!("hwnd: {:?}", h);
        true
    });
}

pub fn set_window_loc_by_hwnd(hwnd_usize: usize, window: &mut tauri::Window) {
    if hwnd_usize == 0 {
        return;
    }

    let hwnd = hwnd_usize as HWND;

    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    // get rect of window
    unsafe {
        winapi::um::winuser::GetWindowRect(hwnd, &mut rect);
    }

    dbg!(rect.left, rect.top, rect.right, rect.bottom);

    window
        .set_position(Position::Logical(LogicalPosition::new(
            (rect.right - 8) as f64,
            (rect.top + 40i32) as f64,
        )))
        .unwrap();
}

pub fn get_hwnd_by_pid(pid: DWORD) -> HWND {
    fn get_window_thread_process_id(hwnd: HWND) -> DWORD {
        let mut pid: DWORD = 0;
        unsafe {
            winapi::um::winuser::GetWindowThreadProcessId(hwnd, &mut pid);
        }
        pid
    }

    let mut hwnd: HWND = std::ptr::null_mut();
    enumerate_windows(|h| {
        if get_window_thread_process_id(h) == pid {
            hwnd = h;
            return false;
        }
        true
    });
    hwnd
}

pub fn unhook_all_window_events(window_event_hooks: Vec<Option<HWINEVENTHOOK>>) {
    unsafe {
        for hook in window_event_hooks {
            match hook {
                Some(hook) => {
                    UnhookWinEvent(hook);
                }
                None => {}
            }
        }
    }
}
