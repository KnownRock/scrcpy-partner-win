#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use tauri::Manager;

use std::os::windows::process::CommandExt;
use std::process::Command;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::HWINEVENTHOOK;

mod cmds;
mod funcs;
mod sendkey;
mod tauri_funcs;
mod wins;

use cmds::{kill_process, run_scrcpy};
use funcs::is_device_valid_args;
use tauri_funcs::{init_main_window, init_tool_window};
use wins::set_window_loc_by_hwnd;
static mut TOOL_WINDOW: Option<tauri::Window> = None;

fn unhook_all_window_events() {
    unsafe {
        wins::unhook_all_window_events(vec![
            WIN_EVENT_LOC_HOOK,
            WIN_EVENT_ORDER_HOOK,
            WIN_EVENT_CLOSE_HOOK,
        ]);
    }
}

// TODO: refactor this, using closure to avoid global variable
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

    if HWND == 0 {
        return;
    }

    if hwnd_usize != HWND {
        return;
    }

    match &mut TOOL_WINDOW {
        Some(window) => {
            set_window_loc_by_hwnd(HWND, window);
        }
        None => {}
    }
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
    println!("window order changed");
    println!("hwnd: {:?}", _hwnd);

    match &mut TOOL_WINDOW {
        Some(window) => {
            window.set_always_on_top(true).unwrap();
            window.set_always_on_top(false).unwrap();
        }
        None => {}
    }

    match &mut TOOL_WINDOW {
        Some(window) => {
            set_window_loc_by_hwnd(HWND, window);
        }
        None => {}
    }
}

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

    if hwnd_usize == HWND {
        println!("window close");
        println!("hwnd: {:?}", _hwnd);

        unhook_all_window_events();

        std::process::exit(0);
    }
}

static mut WIN_EVENT_LOC_HOOK: Option<HWINEVENTHOOK> = None;
static mut WIN_EVENT_ORDER_HOOK: Option<HWINEVENTHOOK> = None;
static mut WIN_EVENT_CLOSE_HOOK: Option<HWINEVENTHOOK> = None;

use winapi::um::winuser::{
    SetWinEventHook, EVENT_OBJECT_DESTROY, EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_REORDER,
    WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WINEVENT_SKIPOWNTHREAD,
};

use crate::cmds::get_adb_devices_raw;

fn watch_window_size_and_position_and_order(pid: DWORD) {
    println!("watch_window_size_and_position, pid: {}", 0);

    unhook_all_window_events();

    unsafe {
        let win_event_loc_hook = SetWinEventHook(
            EVENT_OBJECT_LOCATIONCHANGE,
            EVENT_OBJECT_LOCATIONCHANGE,
            std::ptr::null_mut(),
            Some(win_event_loc_callback),
            pid,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
        );

        WIN_EVENT_LOC_HOOK = Some(win_event_loc_hook);

        let win_event_order_hook = SetWinEventHook(
            EVENT_OBJECT_REORDER,
            EVENT_OBJECT_REORDER,
            std::ptr::null_mut(),
            Some(win_event_order_callback),
            pid,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
        );

        WIN_EVENT_ORDER_HOOK = Some(win_event_order_hook);

        let win_event_close_hook = SetWinEventHook(
            EVENT_OBJECT_DESTROY,
            EVENT_OBJECT_DESTROY,
            std::ptr::null_mut(),
            Some(win_event_close_callback),
            pid,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
        );

        WIN_EVENT_CLOSE_HOOK = Some(win_event_close_hook);
    };
}

#[tauri::command]
fn adb_devices_l() -> String {
    get_adb_devices_raw()
}

#[tauri::command]
fn lanuch_self(args: Vec<String>) {
    let self_path = std::env::current_exe().unwrap();

    println!("lanuch_self: {:?}", self_path);

    Command::new(self_path)
        .args(args)
        .creation_flags(0x08000000)
        .spawn()
        .unwrap();
}

#[tauri::command]
async fn sendkey(
    key_code: usize,
    scan_code: usize,
    extend_key_flag: usize,
    is_alt: bool,
    is_shift: bool,
) {
    unsafe { sendkey::sendkey(HWND, key_code, scan_code, extend_key_flag, is_alt, is_shift) }
}

static mut HWND: usize = 0;
static mut PID: u32 = 0;

fn init_tool_hooks(tool_window: tauri::Window) {
    unsafe {
        TOOL_WINDOW = Some(tool_window);
        watch_window_size_and_position_and_order(PID);
        match &mut TOOL_WINDOW {
            Some(window) => {
                set_window_loc_by_hwnd(HWND, window);
            }
            None => {}
        }
    }
}

#[tauri::command]
async fn show_main_window(app: tauri::AppHandle) {
    match app.get_window("main") {
        Some(window) => {
            window.show().unwrap();
        }
        None => {
            // TODO: error handle
        }
    }
    match app.get_window("splashscreen") {
        Some(window) => {
            window.hide().unwrap();
        }
        None => {}
    }
}

#[tauri::command]
async fn show_tool_window(app: tauri::AppHandle) {
    match app.get_window("tool") {
        Some(window) => {
            window.show().unwrap();
        }
        None => {
            // TODO: error handle
        }
    }
    match app.get_window("splashscreen") {
        Some(window) => {
            window.hide().unwrap();
        }
        None => {}
    }
}

#[tauri::command]
async fn init(app: tauri::AppHandle) -> String {
    unsafe {
        kill_process(PID);
        PID = 0;
    }

    let mut args: Vec<String> = env::args().collect();

    let mut is_tool_mode = true;

    // if not have vaild device arg, will not run scrcpy to save start time
    let is_device_valid = is_device_valid_args(args.clone());
    if !is_device_valid {
        is_tool_mode = false;
    }

    // if have device arg, will try to run scrcpy
    if is_tool_mode {
        unsafe {
            args.remove(0);
            println!("args: {:?}", &args);

            match run_scrcpy(&args) {
                Some((scrcpy_pid, hwnd)) => {
                    PID = scrcpy_pid;
                    HWND = hwnd;
                    is_tool_mode = true;
                }
                None => {
                    PID = 0;
                    HWND = 0;
                    is_tool_mode = false;
                }
            }
        }
    }

    if is_tool_mode {
        unsafe {
            assert!(HWND != 0, "Failed to get hwnd");
            assert!(PID != 0, "Failed to get pid");

            println!("Mode: tool");

            init_tool_window(&app);
        }
    } else {
        println!("Mode: main");
        init_main_window(&app);
    }

    "ok".to_string()
}

// static mut TAURI_APP : Option<&mut tauri::App> = None;

fn main() {
    let app_handle = tauri::Builder::default();
    app_handle
        .on_page_load(|window, _payload| {
            println!("page loaded, window: {:?}", window.label());
            if window.label() == "tool" {
                init_tool_hooks(window);
                println!("*** tool window loaded ***")
            } else {
                println!("*** non tool window loaded ***");
            }
        })
        .invoke_handler(tauri::generate_handler![
            adb_devices_l,
            show_main_window,
            show_tool_window,
            sendkey,
            lanuch_self,
            init
        ])
        .run(tauri::generate_context!())
        .expect("***********************\nerror while running tauri application");

    println!("End");
}
