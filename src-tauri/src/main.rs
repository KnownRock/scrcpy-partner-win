#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::env;
use std::mem;
use std::sync::Mutex;
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

static mut MAIN_WINDOW: Option<tauri::Window> = None;

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

    match &mut MAIN_WINDOW {
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

    match &mut MAIN_WINDOW {
        Some(window) => {
            window.set_always_on_top(true).unwrap();
            window.set_always_on_top(false).unwrap();
        }
        None => {}
    }

    match &mut MAIN_WINDOW {
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

fn watch_window_size_and_position_and_order(pid: DWORD) {
    println!("watch_window_size_and_position, pid: {}", 0);

    use winapi::um::winuser::{
        SetWinEventHook, EVENT_OBJECT_DESTROY, EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_REORDER,
        WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WINEVENT_SKIPOWNTHREAD,
    };

    unsafe {
        SetWinEventHook(
            EVENT_OBJECT_LOCATIONCHANGE,
            EVENT_OBJECT_LOCATIONCHANGE,
            std::ptr::null_mut(),
            Some(win_event_loc_callback),
            pid,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
        );

        SetWinEventHook(
            EVENT_OBJECT_REORDER,
            EVENT_OBJECT_REORDER,
            std::ptr::null_mut(),
            Some(win_event_order_callback),
            pid,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
        );

        SetWinEventHook(
            EVENT_OBJECT_DESTROY,
            EVENT_OBJECT_DESTROY,
            std::ptr::null_mut(),
            Some(win_event_close_callback),
            pid,
            0,
            WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD,
        );
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

    Command::new(self_path).args(args).creation_flags(0x08000000).spawn().unwrap();
}

#[tauri::command]
fn get_exec_mode(app: tauri::AppHandle) -> String {
    println!("get_exec_mode");

    let main_window = app.get_window("main").unwrap();
    main_window.show().unwrap();

    // let splashscreen_window = app.get_window("splashscreen").unwrap();
    // splashscreen_window.close().unwrap();

    
    match app.get_window("splashscreen") {
        Some(splashscreen_window) => {
            splashscreen_window.close().unwrap();
        }
        None => {
            println!("get_exec_mode, splashscreen_window is none");
        }
    }

    unsafe {
        if IS_TOOL_MODE {
            match &mut MAIN_WINDOW {
                // TODO: set window size by items count
                Some(window) => {
                    window
                        .set_size(Size::Logical(LogicalSize {
                            width: 48.0,
                            height: 600.0,
                        }))
                        .unwrap();
                }
                None => {}
            }

            return "tool".to_string();
        } else {
            println!("get_exec_mode, home mode");

            match &mut MAIN_WINDOW {
                // TODO: set window size by items count
                Some(window) => {
                    println!("get_exec_mode, home mode, set window size");

                    // window
                    //     .set_size(Size::Logical(LogicalSize {
                    //         width: 800.0,
                    //         height: 600.0,
                    //     }))
                    //     .unwrap();

                    // window
                    //     .set_position(Position::Logical(LogicalPosition::new(0.0, 0.0)))
                    //     .unwrap();

                    window.set_decorations(true).unwrap();
                    window.set_resizable(true).unwrap();
                    window.set_skip_taskbar(false).unwrap();
                }
                None => {}
            }

            return "home".to_string();
        }
    }
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

static mut IS_TOOL_MODE: bool = false;
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

#[tauri::command]
async fn init(app:  tauri::AppHandle) -> String{
    // let loading_window = 

    return "ok".to_string();
}

fn init_handle(main_window:  tauri::Window) -> String{

    unsafe{
        // close scrcpy if exist
        let scrcpy_pid = PID;
        if scrcpy_pid != 0 {
            println!("kill scrcpy");
            let _ = Command::new("taskkill")
                .arg("/F")
                .arg("/T")
                .arg("/PID")
                .arg(scrcpy_pid.to_string())
                .output();
        }
        
    }

    // let loading_window = 

    // return "ok".to_string();
    let devices_ids = get_adb_devices();
    let mut window = main_window;
    let mut have_device_arg_flag = false;

    let mut args: Vec<String> = env::args().collect();
    if devices_ids.len() > 0 {
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
    } else if devices_ids.len() == 0 {
        println!("no device");
        have_device_arg_flag = false;
    }

    if devices_ids.len() == 1 {
        let mut no_device_flag = true;
        for arg in &args {
            if arg.starts_with("--serial") || arg.starts_with("-s") {
                no_device_flag = false;
                break;
            }
        }
        if no_device_flag {
            have_device_arg_flag = true;
        }
    }

    if have_device_arg_flag {
        dbg!("haveDeviceArgFlag");
    } else {
        dbg!("no haveDeviceArgFlag");
    }

    // have_device_arg_flag = true;

    if !have_device_arg_flag {
        unsafe {
            IS_TOOL_MODE = false;
        }
    } else {
        let mut pid = 0;
        unsafe {
            args.remove(0);
            println!("args: {:?}", &args);

            match run_scrcpy(&args) {
                Some((scrcpy_pid, hwnd)) => {
                    pid = scrcpy_pid;
                    HWND = hwnd;
                }
                None => {
                    pid = 0;
                    HWND = 0;
                }
            }
        }

        // using scrcpy running state to determine if it is tool mode
        

        unsafe {
            if HWND == 0 {
                // panic!("Failed to get hwnd");
                IS_TOOL_MODE = false;
            } else {
                IS_TOOL_MODE = true;
            }

            if IS_TOOL_MODE {
                PID = pid;

                window.set_title("SPW Tool").unwrap();
                // TODO: set window size by items count
                window
                    .set_size(Size::Logical(LogicalSize {
                        width: 1.0,
                        height: 1.0,
                    }))
                    .unwrap();
                window.set_decorations(false).unwrap();
                window.set_resizable(false).unwrap();

                window
                    .set_position(Position::Logical(LogicalPosition { x: 0.0, y: 0.0 }))
                    .unwrap();

                window.set_skip_taskbar(true).unwrap();

                println!("a3");
                println!("PID: {}", pid);

                watch_window_size_and_position_and_order(pid);
                set_window_loc_by_hwnd(HWND, &mut window);
                window.set_always_on_top(true).unwrap();
                window.set_always_on_top(false).unwrap();

                println!("HWND: 0x{:x}", HWND);
            }
        }

        
    }
    unsafe{
        MAIN_WINDOW = Some(window);
    }   

    "ok".to_string()

}


static mut IS_MAIN_WINDOW_LOADED : bool = false;

fn main() {
    tauri::Builder::default().setup(|app| {
        let splashscreen_window = app.get_window("splashscreen").unwrap();
        let main_window = app.get_window("main").unwrap();
        // we perform the initialization code on a new task so the app doesn't freeze
        tauri::async_runtime::spawn(async move {
          
        });
        Ok(())
      })
        .on_page_load(|window, payload| {
            println!("page loaded, window: {:?}", window.label());
            if window.label() == "main" {
                init_handle(window);
                // unsafe{
                //     MAIN_WINDOW_ON_LOAD = Some(window);
                // }
                dbg!("** main window");

                unsafe{
                    IS_MAIN_WINDOW_LOADED = true;
                }

            } else {
                dbg!("** non main window");
                unsafe{
                    if window.label() == "splashscreen" {
                        if IS_MAIN_WINDOW_LOADED {
                            window.close();
                        }
                    }
                }
               
            
            }

            
            println!("page loaded");
        })
        .invoke_handler(tauri::generate_handler![
            adb_devices_l,
            get_exec_mode,
            sendkey,
            lanuch_self,
            init
        ])
        .run(tauri::generate_context!())
        .expect("***********************\nerror while running tauri application");

    println!("End");
}
