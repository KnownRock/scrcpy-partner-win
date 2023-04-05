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

use crate::wins::{self, set_window_loc_and_size_by_hwnd, set_window_loc_by_hwnd};

struct Hooks {
    loc: HWINEVENTHOOK,
    order: HWINEVENTHOOK,
    close: HWINEVENTHOOK,
}

struct WatcherState<'a> {
    app_state: &'a mut AppState,
    hooks: Hooks,
}

impl WatcherState<'_> {
    fn unhook_all_window_events(&self) {
        wins::unhook_all_window_events(vec![
            Some(self.hooks.loc),
            Some(self.hooks.order),
            Some(self.hooks.close),
        ]);
    }

    fn on_close(&self) {
        self.unhook_all_window_events();

        std::process::exit(0);
    }

    fn on_move(&mut self, hwnd: usize) {
        if hwnd == 0 {
            return;
        }
        let app_state = &mut self.app_state;

        let self_hwnd = app_state.hwnd;

        if hwnd != self_hwnd {
            return;
        }

        if !app_state.tool_window.is_none() {
            set_window_loc_by_hwnd(
                app_state.hwnd,
                app_state.tool_window.as_mut().unwrap(),
                app_state.is_window_borderless,
            );
        }

        if !app_state.record_window.is_none() {
            set_window_loc_and_size_by_hwnd(
                app_state.hwnd,
                app_state.record_window.as_mut().unwrap(),
                app_state.is_window_borderless,
                app_state.is_record_panel_with_motion_record,
            );
        }
    }

    fn on_order(&mut self) {
        let app_state = &mut self.app_state;

        if !app_state.tool_window.is_none() {
            let tool_window = app_state.tool_window.as_mut().unwrap();
            tool_window.set_always_on_top(true).unwrap();
            tool_window.set_always_on_top(false).unwrap();
        }

        if !app_state.record_window.is_none() {
            let record_window = app_state.record_window.as_mut().unwrap();
            record_window.set_always_on_top(true).unwrap();
            record_window.set_always_on_top(false).unwrap();
        }
    }
}

// FIXME: remove static
static mut WATCHER_STATE: Option<WatcherState> = None;

// pub fn set_watcher(app_state: &mut AppState) {
//     WATCHER_STATE = Some(WatcherState {
//         app_state: app_state,
//         hooks: Hooks {
//             loc: win_event_loc_hook,
//             order: win_event_close_hook,
//             close: win_event_close_hook,
//         },
//     });
// }

unsafe extern "system" fn win_event_close_callback(
    _hwin_event_hook: HWINEVENTHOOK,
    _event: winapi::shared::minwindef::DWORD,
    _hwnd: winapi::shared::windef::HWND,
    _id_object: LONG,
    _id_child: LONG,
    _id_event_thread: winapi::shared::minwindef::DWORD,
    _dwms_event_time: winapi::shared::minwindef::DWORD,
) {
    if WATCHER_STATE.is_none() {
        return;
    }

    let watch_state = &mut WATCHER_STATE.as_mut().unwrap();
    watch_state.on_close();
}

unsafe extern "system" fn win_event_loc_callback(
    _hwin_event_hook: HWINEVENTHOOK,
    _event: winapi::shared::minwindef::DWORD,
    _hwnd: winapi::shared::windef::HWND,
    _id_object: LONG,
    _id_child: LONG,
    _id_event_thread: winapi::shared::minwindef::DWORD,
    _dwms_event_time: winapi::shared::minwindef::DWORD,
) {
    let hwnd_usize = _hwnd as usize;

    if hwnd_usize == 0 {
        return;
    }

    if WATCHER_STATE.is_none() {
        return;
    }

    let watch_state = &mut WATCHER_STATE.as_mut().unwrap();
    watch_state.on_move(hwnd_usize);
}

unsafe extern "system" fn win_event_order_callback(
    _hwin_event_hook: HWINEVENTHOOK,
    _event: winapi::shared::minwindef::DWORD,
    _hwnd: winapi::shared::windef::HWND,
    _id_object: LONG,
    _id_child: LONG,
    _id_event_thread: winapi::shared::minwindef::DWORD,
    _dwms_event_time: winapi::shared::minwindef::DWORD,
) {
    let hwnd_usize = _hwnd as usize;

    if hwnd_usize == 0 {
        return;
    }

    if WATCHER_STATE.is_none() {
        return;
    }

    let watch_state = &mut WATCHER_STATE.as_mut().unwrap();
    let app_state = &mut *watch_state.app_state;

    if !app_state.tool_window.is_none() {
        let tool_window = app_state.tool_window.as_mut().unwrap();
        tool_window.set_always_on_top(true).unwrap();
        tool_window.set_always_on_top(false).unwrap();
    }

    if !app_state.record_window.is_none() {
        let record_window = app_state.record_window.as_mut().unwrap();
        record_window.set_always_on_top(true).unwrap();
        record_window.set_always_on_top(false).unwrap();
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
    is_record_panel_with_motion_record: bool,
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
            is_record_panel_with_motion_record: false,
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
            self.stop();

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

            let win_event_loc_hook = SetWinEventHook(
                EVENT_OBJECT_LOCATIONCHANGE,
                EVENT_OBJECT_LOCATIONCHANGE,
                std::ptr::null_mut(),
                Some(win_event_loc_callback),
                self.pid as DWORD,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
            );

            let win_event_order_hook = SetWinEventHook(
                EVENT_OBJECT_REORDER,
                EVENT_OBJECT_REORDER,
                std::ptr::null_mut(),
                Some(win_event_order_callback),
                self.pid as DWORD,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
            );

            unsafe {
                let app_s = self;
                WATCHER_STATE = Some(WatcherState {
                    app_state: &mut AppState::default(),
                    hooks: Hooks {
                        loc: win_event_loc_hook,
                        close: win_event_close_hook,
                        order: win_event_order_hook,
                    },
                });
            }

            // WIN_EVENT_CLOSE_HOOK = Some(win_event_close_hook);
        };
    }

    pub fn stop(&mut self) {
        unsafe {
            match &mut WATCHER_STATE {
                Some(watch_state) => {
                    watch_state.unhook_all_window_events();
                }
                None => {}
            }
        }
    }
}
