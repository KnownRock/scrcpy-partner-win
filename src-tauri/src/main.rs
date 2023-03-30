#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use tauri::{LogicalSize, Manager};

use std::os::windows::process::CommandExt;
use std::process::Command;

use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::HWINEVENTHOOK;

mod cmds;
mod sendkey;
mod tauri_funcs;
mod wins;

use cmds::{kill_process, run_scrcpy, save_size_and_position};
use tauri_funcs::{init_main_window, init_record_window, init_tool_window};
use winapi::shared::windef::RECT;
use wins::{get_hwnd_by_pid, set_window_loc_and_size_by_hwnd, set_window_loc_by_hwnd};
static mut TOOL_WINDOW: Option<tauri::Window> = None;
static mut RECORD_WINDOW: Option<tauri::Window> = None;

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

static mut fuc_loc_callback: Option<
    fn(rect: RECT, window: &mut tauri::Window, is_borderless: bool),
> = None;

static mut IS_RECORD_PANEL_WITH_MOTION_RECORD: bool = false;

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
            set_window_loc_by_hwnd(HWND, window, IS_WINDOW_BORDERLESS);
        }
        None => {}
    }

    match &mut RECORD_WINDOW {
        Some(window) => {
            set_window_loc_and_size_by_hwnd(
                HWND,
                window,
                IS_WINDOW_BORDERLESS,
                IS_RECORD_PANEL_WITH_MOTION_RECORD,
            );
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

    match &mut RECORD_WINDOW {
        Some(window) => {
            window.set_always_on_top(true).unwrap();
            window.set_always_on_top(false).unwrap();
        }
        None => {}
    }

    // match &mut TOOL_WINDOW {
    //     Some(window) => {
    //         set_window_loc_by_hwnd(HWND, window, IS_WINDOW_BORDERLESS);
    //     }
    //     None => {}
    // }

    // match &mut RECORD_WINDOW {
    //     Some(window) => {
    //         set_window_loc_by_hwnd(HWND, window, IS_WINDOW_BORDERLESS);
    //     }
    //     None => {}
    // }
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
                save_size_and_position(rect, IS_WINDOW_BORDERLESS, CONFIG_ID.clone());
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

    // fn hello_world(rect: RECT, window: &mut tauri::Window, is_borderless: bool) {
    //     println!("hello world");
    //     println!("rect: {:?}", rect.top);
    // }

    // unsafe {
    //     fuc_loc_callback = Some(hello_world);
    // }

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

// IS_RECORD_PANEL_WITH_MOTION_ROCORD
#[tauri::command]
fn set_record_panel_with_motion_record(record_panel_with_motion_record: bool) {
    unsafe {
        IS_RECORD_PANEL_WITH_MOTION_RECORD = record_panel_with_motion_record;

        match &mut RECORD_WINDOW {
            Some(window) => set_window_loc_and_size_by_hwnd(
                HWND,
                window,
                IS_WINDOW_BORDERLESS,
                IS_RECORD_PANEL_WITH_MOTION_RECORD,
            ),
            None => {}
        }
    }
}

#[tauri::command]
fn create_ms_link(link: String, args: Vec<String>) {
    let home_path = std::env::var("USERPROFILE").unwrap();

    let target = std::env::current_exe().unwrap();
    let mut sl = mslnk::ShellLink::new(target.clone()).unwrap();
    sl.set_arguments(Some(args.join(" ")));
    sl.create_lnk(&format!("{}\\Desktop\\{}.lnk", home_path, link))
        .unwrap();
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
async fn open(exec: String, args: Vec<String>, cwd: String) {
    Command::new(exec)
        .args(args)
        .current_dir(cwd)
        .spawn()
        .unwrap();
}

#[tauri::command]
async fn start(exec: String, cwd: String) {
    Command::new("cmd")
        .args(["/C", "start", "/d", cwd.as_str(), exec.as_str()])
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
                set_window_loc_by_hwnd(HWND, window, IS_WINDOW_BORDERLESS);
                window.show().unwrap();
            }
            None => {}
        }
    }
}

#[tauri::command]
async fn get_config_id() -> String {
    unsafe { CONFIG_ID.clone() }
}

#[tauri::command]
async fn get_env_args() -> Vec<String> {
    env::args().collect()
}

#[tauri::command]
fn exit() {
    unsafe {
        if PID != 0 {
            kill_process(PID);
            PID = 0;
        }
    }

    std::process::exit(0);
}

#[tauri::command]
async fn get_process_hwnd(pid: u32) -> usize {
    get_hwnd_by_pid(pid) as usize
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
static mut IS_WINDOW_BORDERLESS: bool = false;
static mut CONFIG_ID: String = String::new();
// static mut C: Option<FnMut(usize, usize, usize, usize)> = None;

#[tauri::command]
async fn init(
    app: tauri::AppHandle,
    pid: usize,
    hwnd: usize,
    is_tool_mode: bool,
    is_auto_save_location_and_size: bool,
    // isWindowBorderless
    is_window_borderless: bool,
    config_id: String,
) -> String {
    if is_tool_mode {
        unsafe {
            PID = pid as u32;
            HWND = hwnd;

            assert!(HWND != 0, "Failed to get hwnd");
            assert!(PID != 0, "Failed to get pid");

            println!("Mode: tool");

            IS_AUTO_SAVE_LOCATION_AND_SIZE = is_auto_save_location_and_size;
            IS_WINDOW_BORDERLESS = is_window_borderless;
            CONFIG_ID = config_id;

            dbg!(&IS_AUTO_SAVE_LOCATION_AND_SIZE);
            dbg!(&CONFIG_ID);

            init_tool_window(&app);

            println!("*** tool init done ***")
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

#[tauri::command]
async fn get_current_exe_path() -> String {
    std::env::current_exe()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[tauri::command]
async fn get_current_exe_dir() -> String {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[tauri::command]
async fn close_record_window() {
    unsafe {
        if let Some(window) = &RECORD_WINDOW {
            window.close().unwrap();
            RECORD_WINDOW = None;
        }
    }
}

#[tauri::command]
async fn open_record_window(app: tauri::AppHandle) {
    unsafe {
        if let Some(window) = &RECORD_WINDOW {
            window.show().unwrap();
        } else {
            // will set RECORD_WINDOW in on_page_load
            init_record_window(&app);
        }
    }
}

fn main() {
    let app_handle = tauri::Builder::default();
    app_handle
        .on_page_load(|window, _payload| {
            println!("page loaded, window: {:?}", window.label());
            if window.label() == "tool" {
                init_tool_hooks(window);
                println!("*** tool window loaded ***")
            } else if window.label() == "record" {
                unsafe {
                    RECORD_WINDOW = Some(window);
                    match &mut RECORD_WINDOW {
                        Some(window) => {
                            set_window_loc_by_hwnd(HWND, window, IS_WINDOW_BORDERLESS);
                            window.show().unwrap();
                        }
                        None => {}
                    }

                    println!("record window loaded");
                }
            } else {
                println!("*** non tool window loaded ***");
            }
        })
        .invoke_handler(tauri::generate_handler![
            adb_devices_l,
            sendkey,
            lanuch_self,
            init,
            call_prisma,
            connect_tcpip_device,
            get_env_args,
            run_scrcpy_command,
            create_ms_link,
            exit,
            open,
            start,
            get_config_id,
            get_current_exe_path,
            get_current_exe_dir,
            set_record_panel_with_motion_record,
            close_record_window,
            open_record_window,
            get_process_hwnd
        ])
        .run(tauri::generate_context!())
        .expect("***********************\nerror while running tauri application");

    println!("End");
}
