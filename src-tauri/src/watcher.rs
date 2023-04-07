use winapi::um::winuser::{
    SetFocus, SetForegroundWindow, SetWinEventHook, EVENT_OBJECT_DESTROY,
    EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_REORDER, WINEVENT_OUTOFCONTEXT,
    WINEVENT_SKIPOWNPROCESS, WINEVENT_SKIPOWNTHREAD,
};

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::RECT;
use winapi::shared::windef::{HWINEVENTHOOK, HWND};

use crate::cmds::{kill_process, save_size_and_position};
use crate::sendkey;
use crate::tauri_funcs::init_record_window;
use crate::wins::{self, set_window_loc_and_size_by_hwnd, set_window_loc_by_hwnd};

struct Hooks {
    loc: HWINEVENTHOOK,
    order: HWINEVENTHOOK,
    close: HWINEVENTHOOK,
}

pub struct Watcher {
    pub app_state: Option<AppState>,
    hooks: Hooks,

    last_rect: Option<RECT>,
}

impl Watcher {
    fn unhook_all_window_events(&self) {
        wins::unhook_all_window_events(vec![
            Some(self.hooks.loc),
            Some(self.hooks.order),
            Some(self.hooks.close),
        ]);
    }

    pub fn sendkey(
        &self,
        key_code: usize,
        scan_code: usize,
        extend_key_flag: usize,
        is_alt: bool,
        is_shift: bool,
    ) {
        if self.app_state.is_none() {
            return;
        }

        let app_state = &self.app_state.as_ref().unwrap();
        let hwnd = app_state.hwnd;
        sendkey::sendkey(hwnd, key_code, scan_code, extend_key_flag, is_alt, is_shift)
    }

    pub fn close_record_window(&mut self) {
        if self.app_state.is_none() {
            return;
        }

        if self.app_state.as_ref().unwrap().record_window.is_none() {
            return;
        }

        let app_state = self.app_state.as_mut().unwrap();

        app_state.record_window.as_mut().unwrap().close().unwrap();
        app_state.record_window = None;
    }

    pub fn open_record_window(&mut self, app: &tauri::AppHandle) {
        if let Some(window) = self.app_state.as_ref().unwrap().record_window.as_ref() {
            window.show().unwrap();
        } else {
            // will set RECORD_WINDOW in on_page_load
            let win = init_record_window(&app);
            self.app_state.as_mut().unwrap().set_record_window(win);
        }
    }

    pub fn exit(&self) {
        if self.app_state.is_none() {
            return;
        }

        let app_state = &self.app_state.as_ref().unwrap();
        let pid = app_state.pid;

        kill_process(pid as u32);

        self.unhook_all_window_events();

        std::process::exit(0);
    }

    pub fn set_app_state(&mut self, app_state: AppState) {
        self.app_state = Some(app_state);
    }

    fn on_close(&self, hwnd: usize) {
        if hwnd == 0 {
            return;
        }
        if self.app_state.is_none() {
            return;
        }

        let app_state = self.app_state.as_ref().unwrap();

        let self_hwnd = app_state.hwnd;
        if hwnd != self_hwnd {
            return;
        }

        if app_state.is_auto_save_location_and_size {
            save_size_and_position(
                self.last_rect.unwrap(),
                app_state.is_window_borderless,
                app_state.config_id.clone(),
            );
        }

        self.unhook_all_window_events();

        std::process::exit(0);
    }

    fn on_move(&mut self, hwnd: usize) {
        if hwnd == 0 {
            return;
        }
        if self.app_state.is_none() {
            return;
        }

        let app_state = &mut self.app_state.as_mut().unwrap();

        let self_hwnd = app_state.hwnd;

        if hwnd != self_hwnd {
            return;
        }

        let rect = wins::get_window_rect_by_hwnd(hwnd);
        self.last_rect = Some(rect);

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
        if self.app_state.is_none() {
            return;
        }

        let app_state = &mut self.app_state.as_mut().unwrap();

        // prevent tool window and record window from being covered by other windows
        // prevent tool window and record window change focus
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

    pub fn fresh_loc_and_size(&mut self) {
        self.on_move(self.app_state.as_ref().unwrap().hwnd);
        self.on_order();
    }

    pub fn start(&mut self) {
        unsafe {
            if self.app_state.is_none() {
                panic!("app_state is none");
            }

            self.unhook_all_window_events();

            let app_state = &mut self.app_state.as_mut().unwrap();
            let pid = app_state.pid;

            // WIN_EVENT_CLOSE_HOOK
            let win_event_close_hook = SetWinEventHook(
                EVENT_OBJECT_DESTROY,
                EVENT_OBJECT_DESTROY,
                std::ptr::null_mut(),
                Some(win_event_close_callback),
                pid as DWORD,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
            );

            let win_event_loc_hook = SetWinEventHook(
                EVENT_OBJECT_LOCATIONCHANGE,
                EVENT_OBJECT_LOCATIONCHANGE,
                std::ptr::null_mut(),
                Some(win_event_loc_callback),
                pid as DWORD,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
            );

            let win_event_order_hook = SetWinEventHook(
                EVENT_OBJECT_REORDER,
                EVENT_OBJECT_REORDER,
                std::ptr::null_mut(),
                Some(win_event_order_callback),
                pid as DWORD,
                0,
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
            );

            self.hooks = Hooks {
                loc: win_event_loc_hook,
                order: win_event_order_hook,
                close: win_event_close_hook,
            };

            SetForegroundWindow(app_state.hwnd as HWND);
        };
    }
}

// FIXME: remove static
// pub static mut WATCHER: Option<Watcher> = None;
pub static mut WATCHER: Watcher = Watcher {
    app_state: None,
    hooks: Hooks {
        loc: std::ptr::null_mut(),
        order: std::ptr::null_mut(),
        close: std::ptr::null_mut(),
    },
    last_rect: None,
};

// pub fn start(app_state: Option<AppState>) {
//     unsafe {
//         WATCHER_STATE = Some(WatcherState {
//             app_state: app_state,
//             // hooks: hooks,
//         })
//     };
// }

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
    let hwnd_usize = _hwnd as usize;

    if hwnd_usize == 0 {
        return;
    }

    WATCHER.on_close(hwnd_usize);
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

    WATCHER.on_move(hwnd_usize);
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

    WATCHER.on_order();
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

    pub fn set_record_window(&mut self, window: tauri::Window) {
        self.record_window = Some(window);
    }

    pub fn get_config_id(&self) -> String {
        self.config_id.clone()
    }

    pub fn set_tool_window(&mut self, window: tauri::Window) {
        self.tool_window = Some(window);
    }

    pub fn set_pid_and_hwnd(&mut self, pid: usize, hwnd: usize) {
        self.pid = pid;
        self.hwnd = hwnd;
    }

    pub fn set_is_record_panel_with_motion_record(
        &mut self,
        is_record_panel_with_motion_record: bool,
    ) {
        self.is_record_panel_with_motion_record = is_record_panel_with_motion_record;
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
}
