use winapi::ctypes::c_void;
use winapi::um::winuser::{
    SetWinEventHook, EVENT_OBJECT_DESTROY, EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_REORDER,
    WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WINEVENT_SKIPOWNTHREAD,
};

use std::borrow::Borrow;
use std::os::windows::process::CommandExt;
use std::process::Command;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::{HWINEVENTHOOK, HWINEVENTHOOK__, HWND__};

use crate::wins;

static mut WIN_EVENT_LOC_HOOK: Option<HWINEVENTHOOK> = None;
static mut WIN_EVENT_ORDER_HOOK: Option<HWINEVENTHOOK> = None;
static mut WIN_EVENT_CLOSE_HOOK: Option<HWINEVENTHOOK> = None;

struct Hooks {
    loc: HWINEVENTHOOK,
    order: HWINEVENTHOOK,
    close: HWINEVENTHOOK,
}

struct WatchState {
    app_state: *mut AppState,
    hooks: Hooks,
}

impl WatchState {
    fn unhook_all_window_events(&self) {
        wins::unhook_all_window_events(vec![
            Some(self.hooks.loc),
            Some(self.hooks.order),
            Some(self.hooks.close),
        ]);
    }
}

// FIXME: remove static
static mut WATCH_STATE: Option<WatchState> = None;

unsafe extern "system" fn win_event_close_callback(
    _hwin_event_hook: HWINEVENTHOOK,
    _event: winapi::shared::minwindef::DWORD,
    _hwnd: winapi::shared::windef::HWND,
    _id_object: LONG,
    _id_child: LONG,
    _id_event_thread: winapi::shared::minwindef::DWORD,
    _dwms_event_time: winapi::shared::minwindef::DWORD,
) {
}

fn unhook_all_window_events() {
    unsafe {
        println!("unhook_all_window_events");
        wins::unhook_all_window_events(vec![
            WIN_EVENT_LOC_HOOK,
            WIN_EVENT_ORDER_HOOK,
            WIN_EVENT_CLOSE_HOOK,
        ]);
    }
}

struct HwndCallback {
    hwnd: usize,
    callback: fn(),
}

pub struct AppState {
    record_window: Option<tauri::Window>,
    tool_window: Option<tauri::Window>,

    pid: usize,
    hwnd: usize,

    is_auto_save_location_and_size: bool,
    is_window_borderless: bool,
    config_id: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            record_window: None,
            tool_window: None,

            pid: 0,
            hwnd: 0,

            is_auto_save_location_and_size: false,
            is_window_borderless: false,
            config_id: String::new(),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn set_record_window(&mut self, window: tauri::Window) {
        self.record_window = Some(window);
    }

    pub fn set_tool_window(&mut self, window: tauri::Window) {
        self.tool_window = Some(window);
    }

    pub fn set_pid_and_hwnd(&mut self, pid: usize, hwnd: usize) {
        self.pid = pid;
        self.hwnd = hwnd;
    }

    pub fn set_save_info(
        &mut self,
        is_auto_save_location_and_size: bool,
        is_window_borderless: bool,
        config_id: String,
    ) {
        self.is_auto_save_location_and_size = is_auto_save_location_and_size;
        self.is_window_borderless = is_window_borderless;
        self.config_id = config_id;
    }

    pub fn start(&mut self) {
        unsafe {
            match &mut WATCH_STATE {
                Some(watch_state) => {
                    watch_state.unhook_all_window_events();
                }
                None => {}
            }

            // WIN_EVENT_CLOSE_HOOK
            let win_event_close_hook = SetWinEventHook(
                EVENT_OBJECT_DESTROY,
                EVENT_OBJECT_DESTROY,
                std::ptr::null_mut(),
                Some(win_event_close_callback),
                self.pid as DWORD,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
            );

            // WIN_EVENT_CLOSE_HOOK = Some(win_event_close_hook);

            WATCH_STATE = Some(WatchState {
                app_state: self as *mut AppState,
                hooks: Hooks {
                    loc: win_event_close_hook,
                    order: win_event_close_hook,
                    close: win_event_close_hook,
                },
            });
        };
    }

    pub fn stop(&mut self) {
        unsafe {
            match &mut WATCH_STATE {
                Some(watch_state) => {
                    watch_state.unhook_all_window_events();
                }
                None => {}
            }
        }
    }
}
