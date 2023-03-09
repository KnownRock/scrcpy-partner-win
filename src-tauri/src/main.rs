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
        println!("unhook_all_window_events");
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

        unsafe {
            let rect = wins::get_window_rect_by_hwnd(HWND);

            if IS_AUTO_SAVE_LOCATION_AND_SIZE {
                dbg!("save window size and position");
                println!("config id: {:?}", &CONFIG_ID);
                println!(
                    "rect: yt:{} xl:{} xr:{}",
                    &rect.top, &rect.left, &rect.right
                );

                fn get_prisma_json(config_id: String, key: String, value: String) -> String {
                    serde_json::json!({
                        "where": {
                            "deviceConfigId_key": {
                                "deviceConfigId": config_id,
                                "key": key
                            }
                        },
                        "update":{
                            "value": serde_json::json!(value).to_string()
                        },
                        "create": {
                            "deviceConfigId": config_id,
                            "key": key,
                            "value": serde_json::json!(value).to_string()
                        }
                    })
                    .to_string()
                }

                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async move {
                        call_prisma(
                            "deviceConfigValue".to_string(),
                            "upsert".to_string(),
                            get_prisma_json(
                                CONFIG_ID.to_string(),
                                "window-x".to_string(),
                                serde_json::json!(rect.left).to_string(),
                            ),
                        )
                        .await;

                        call_prisma(
                            "deviceConfigValue".to_string(),
                            "upsert".to_string(),
                            get_prisma_json(
                                CONFIG_ID.to_string(),
                                "window-y".to_string(),
                                serde_json::json!(rect.top + 25 + 4).to_string(),
                            ),
                        )
                        .await;

                        call_prisma(
                            "deviceConfigValue".to_string(),
                            "upsert".to_string(),
                            get_prisma_json(
                                CONFIG_ID.to_string(),
                                "window-width".to_string(),
                                serde_json::json!(rect.right - rect.left).to_string(),
                            ),
                        )
                        .await;

                        std::process::exit(0);
                    });
            } else {
                std::process::exit(0);
            }
        }
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
fn connect_tcpip_device(ip: String, is_connect: bool) {
    let mut adb = Command::new("adb");
    if is_connect {
        adb.arg("connect");
    } else {
        adb.arg("disconnect");
    }
    adb.arg(ip);

    adb.spawn().unwrap();
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
                window.show().unwrap();
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
            window.close().unwrap();
        }
        None => {}
    }
}

#[tauri::command]
async fn show_tool_window(app: tauri::AppHandle) {
    match app.get_window("tool") {
        Some(_window) => {
            // window.show().unwrap();
        }
        None => {
            // TODO: error handle
        }
    }
    match app.get_window("splashscreen") {
        Some(window) => {
            window.close().unwrap();
        }
        None => {}
    }
}

#[tauri::command]
async fn get_env_args() -> Vec<String> {
    env::args().collect()
}

#[tauri::command]
fn close_application() {
    unsafe {
        kill_process(PID);
        PID = 0;
    }
    std::process::exit(0);
}

#[tauri::command]
async fn run_scrcpy_command(args: Vec<String>) -> bool {
    dbg!(args.clone());

    unsafe {
        if PID != 0 {
            kill_process(PID);
            PID = 0;
        }
    }

    match run_scrcpy(&args) {
        Some((scrcpy_pid, hwnd)) => {
            unsafe {
                PID = scrcpy_pid;
                HWND = hwnd;
            }
            true
        }
        None => false,
    }
}

static mut IS_AUTO_SAVE_LOCATION_AND_SIZE: bool = false;
static mut CONFIG_ID: String = String::new();
// static mut C: Option<FnMut(usize, usize, usize, usize)> = None;

#[tauri::command]
async fn init(
    app: tauri::AppHandle,
    is_tool_mode: bool,
    is_auto_save_location_and_size: bool,
    config_id: String,
) -> String {
    if is_tool_mode {
        unsafe {
            assert!(HWND != 0, "Failed to get hwnd");
            assert!(PID != 0, "Failed to get pid");

            println!("Mode: tool");

            IS_AUTO_SAVE_LOCATION_AND_SIZE = is_auto_save_location_and_size;
            CONFIG_ID = config_id;

            dbg!(&IS_AUTO_SAVE_LOCATION_AND_SIZE);
            dbg!(&CONFIG_ID);

            init_tool_window(&app);
        }
    } else {
        println!("Mode: main");
        init_main_window(&app);
    }

    "ok".to_string()
}

#[tauri::command]
async fn call_prisma(table: String, func: String, arg_json: String) -> String {
    println!("call_prisma: {} {} {}", table, func, arg_json);
    cmds::call_prisma(table, func, arg_json).await.unwrap()
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
            init,
            call_prisma,
            connect_tcpip_device,
            get_env_args,
            close_application,
            run_scrcpy_command
        ])
        .run(tauri::generate_context!())
        .expect("***********************\nerror while running tauri application");

    println!("End");
}
