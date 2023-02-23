#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

use async_process::Stdio;
use tauri::LogicalPosition;
use tauri::LogicalSize;
use tauri::Manager;
use tauri::Position;
use tauri::Size;

mod sendkey;

use std::os::windows::process::CommandExt;
use std::process::Command;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::TRUE;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::HWINEVENTHOOK;
use winapi::shared::windef::HWND;
use winapi::shared::windef::RECT;

// https://gist.github.com/daniel-abramov/5a460d9ca02948f1d2bfa53100c941da
fn enumerate_windows<F>(mut callback: F)
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

static mut TOOL_WINDOW: Option<tauri::Window> = None;

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

fn set_window_loc_by_hwnd(hwnd_usize: usize, window: &mut tauri::Window) {
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

    window
        .set_position(Position::Logical(LogicalPosition::new(
            (rect.right - 8) as f64,
            (rect.top + 40i32) as f64,
        )))
        .unwrap();
}

// TODO: refactor this, using closure to avoid global variable
static mut WIN_EVENT_LOC_HOOK: Option<HWINEVENTHOOK> = None;
static mut WIN_EVENT_ORDER_HOOK: Option<HWINEVENTHOOK> = None;
static mut WIN_EVENT_CLOSE_HOOK: Option<HWINEVENTHOOK> = None;

use winapi::um::winuser::{
    SetWinEventHook, UnhookWinEvent, EVENT_OBJECT_DESTROY, EVENT_OBJECT_LOCATIONCHANGE,
    EVENT_OBJECT_REORDER, WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WINEVENT_SKIPOWNTHREAD,
};

fn unhook_all_window_events() {
    unsafe {
        match WIN_EVENT_LOC_HOOK {
            Some(hook) => {
                UnhookWinEvent(hook);
                println!("unhook win event loc")
            }
            None => {}
        }

        match WIN_EVENT_ORDER_HOOK {
            Some(hook) => {
                UnhookWinEvent(hook);
                println!("unhook win event order")
            }
            None => {}
        }

        match WIN_EVENT_CLOSE_HOOK {
            Some(hook) => {
                UnhookWinEvent(hook);
                println!("unhook win event close")
            }
            None => {}
        }
    }
}

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

fn get_hwnd_by_pid(pid: DWORD) -> HWND {
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

#[tauri::command]
fn adb_devices_l() -> String {
    let output = std::process::Command::new("adb")
        .arg("devices")
        .arg("-l")
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("output: {}", &output);
    output
}

fn get_adb_devices() -> Vec<String> {
    let output = std::process::Command::new("adb")
        .arg("devices")
        .arg("-l")
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    dbg!("output: {}", &output);

    let mut devices = Vec::new();
    for line in output.lines() {
        if line.starts_with("List") {
            continue;
        }

        if line.len() == 0 {
            break;
        }

        let mut iter = line.split_whitespace();
        let device = iter.next().unwrap();
        if device == "List" {
            continue;
        }

        devices.push(device.to_string());
    }

    devices
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

fn run_scrcpy(pars: &Vec<String>) -> Option<(u32, usize)> {
    // noconsole
    let child = Command::new("scrcpy.exe")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(pars)
        .creation_flags(0x08000000)
        .spawn()
        .unwrap();

    println!("Launched scrcpy");

    let pid = child.id();

    let mut timeout = 20;
    let mut hwnd_usize: usize = 0;
    while timeout > 0 {
        sleep(Duration::from_millis(100));
        let hwnd = get_hwnd_by_pid(pid);
        println!("hwnd: {:?}", hwnd);

        timeout -= 1;

        if hwnd as usize != 0 {
            hwnd_usize = hwnd as usize;
            break;
        }
    }

    if hwnd_usize == 0 {
        return None;
    }

    println!("hwnd_usize: {:?}", hwnd_usize);

    Some((pid, hwnd_usize))
}

fn kill_process(pid: u32) {
    if pid != 0 {
        println!("kill {}", pid);
        let _ = Command::new("taskkill")
            .arg("/F")
            .arg("/T")
            .arg("/PID")
            .arg(pid.to_string())
            .output();
    }
}

fn is_device_valid_args() -> bool {
    let args: Vec<String> = env::args().collect();

    let devices_ids = get_adb_devices();
    if devices_ids.len() == 0 {
        return false;
    }

    let mut have_device_arg_flag = false;

    for arg in &args {
        for device_id in &devices_ids {
            if arg == &format!("--serial {}", device_id)
                || arg == &format!("--serial={}", device_id)
                || arg == &format!("-s{}", device_id)
                || arg.starts_with("--tcpip")
                // TODO: check next -s arg
                || arg == "-s"
            {
                have_device_arg_flag = true;
                break;
            }
        }
    }

    // if devices_ids.len() == 1 && !have_device_arg_flag {
    //     let mut no_device_flag = true;
    //     for arg in &args {
    //         // because if have a vaild device will have_device_arg_flag set true
    //         // so only check is have -s or --serial arg
    //         if arg.starts_with("--serial") || arg.starts_with("-s") {
    //             no_device_flag = false;
    //             break;
    //         }
    //     }
    //     if no_device_flag {
    //         have_device_arg_flag = true;
    //     }
    // }

    have_device_arg_flag
}

fn init_tool_window(app: &tauri::AppHandle) {
    let tool_window =
        tauri::WindowBuilder::new(app, "tool", tauri::WindowUrl::App("tool.html".into()))
            // .center()
            .visible(false)
            .decorations(false)
            .resizable(false)
            .skip_taskbar(true)
            .build()
            .unwrap();

    tool_window
        .set_size(Size::Logical(LogicalSize {
            width: 48.0,
            height: 600.0,
        }))
        .unwrap();
}

fn init_main_window(app: &tauri::AppHandle) {
    tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
        .center()
        .visible(false)
        .title("Scrcpy Partner")
        .build()
        .unwrap();
}

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
    let is_device_valid = is_device_valid_args();
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
            } else {
                dbg!("** non main window");
            }
            println!("page loaded");
        })
        .invoke_handler(tauri::generate_handler![
            adb_devices_l,
            // get_exec_mode,
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
