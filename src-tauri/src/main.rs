#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::mem;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use tauri::{Manager};
use serde::{Serialize, Deserialize};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Device {
    name: String,
    id: String,
}

#[tauri::command]
fn list_devices() -> Vec<Device> {
    let output = std::process::Command::new("adb")
        .arg("devices")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("output: {}", &output);
    let mut devices = Vec::new();
    let lines = output.lines();

    for line in lines {
        if line.contains("device") {
            if line.contains("List of devices attached") {
                continue;
            }
            let device_name = line.split("\t").collect::<Vec<&str>>()[0];
            let device_id = line.split("\t").collect::<Vec<&str>>()[1];
            devices.push(Device {
                name: device_name.to_string(),
                id: device_id.to_string(),
            });
            
        }
    }
    devices
}

#[tauri::command]
fn adb_devices_l() -> String {
    let output = std::process::Command::new("adb")
        .arg("devices")
        .arg("-l")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("output: {}", &output);
    output
}

static mut SCRCPY_PROCESS: Vec<u32> = Vec::new();
// use std::process::Command;
use async_process::Command;

use winapi::shared::ntdef::LONG;
use winapi::shared::windef::HWINEVENTHOOK;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::TRUE;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::shared::windef::RECT;

// https://gist.github.com/daniel-abramov/5a460d9ca02948f1d2bfa53100c941da
fn enumerate_windows<F>(mut callback: F)
    where F: FnMut(HWND) -> bool
{
    use winapi::um::winuser::EnumWindows;
    use std::os::raw::c_void;

    unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let closure: &mut &mut dyn FnMut(HWND) -> bool = mem::transmute(lparam as *mut c_void);
        if closure(hwnd) { TRUE } else { FALSE }
    }

    let mut trait_obj: &mut dyn FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };

    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumWindows(Some(enumerate_callback), lparam) };
}

// IS_SCRCPY_HWND_MAP
use std::collections::HashMap;

lazy_static! {
    static ref IS_SCRCPY_HWND_MAP: Mutex<HashMap<usize, bool>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };    
}


fn watch_window_size_and_position() {
    // use winapi::um::winuser::GetWindowRect;
    // use winapi::um::winuser::GetClientRect;
    // use winapi::um::winuser::GetWindowLongPtrW;
    // use winapi::um::winuser::SetWindowLongPtrW;

    // use winapi::um::winuser::SetWinEventHook;

    println!("watch_window_size_and_position, pid: {}", 0);


    unsafe extern "system" fn win_event_callback(
        _hwin_event_hook: HWINEVENTHOOK,
        _event: winapi::shared::minwindef::DWORD,
        _hwnd: winapi::shared::windef::HWND,
        _id_object: LONG,
        _id_child: LONG,
        _id_event_thread: winapi::shared::minwindef::DWORD,
        _dwms_event_time: winapi::shared::minwindef::DWORD,
    ) {
        // let hwnd = _hwnd;
        
        let hwnd_usize = _hwnd as usize;

        // if IS_SCRCPY_HWND_MAP.lock().unwrap().contains_key(&hwnd_usize) {
            println!("window size changed");
            println!("hwnd: {:?}", _hwnd);
            println!("id event thread: {}", _id_event_thread);
            println!("** is scrpcy hwnd **");

            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            // get rect of window
            winapi::um::winuser::GetWindowRect(_hwnd, &mut rect);

            println!("rect: {:?}", rect.left);
            println!("rect: {:?}", rect.top);
            println!("rect: {:?}", rect.right);
            println!("rect: {:?}", rect.bottom);


        // }
    }

    use winapi::um::winuser::{SetWinEventHook ,EVENT_OBJECT_LOCATIONCHANGE,EVENT_SYSTEM_FOREGROUND, WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WINEVENT_SKIPOWNTHREAD}; 

    unsafe{
        let hook_handle = unsafe {
            SetWinEventHook(
                EVENT_OBJECT_LOCATIONCHANGE, 
                EVENT_OBJECT_LOCATIONCHANGE, 
                std::ptr::null_mut(), 
                Some(win_event_callback), 
                31612, 
                0, 
                WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS | WINEVENT_SKIPOWNTHREAD
            )
        };
    }
}


    
fn get_window_thread_process_id(hwnd: HWND) -> DWORD {
    let mut pid: DWORD = 0;
    unsafe {
        winapi::um::winuser::GetWindowThreadProcessId(hwnd, &mut pid);
    }
    pid
}


fn get_hwnd_by_pid(pid: DWORD) -> HWND {
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

fn get_title(hwnd: HWND) -> String {
    use winapi::um::winuser::GetWindowTextLengthW;
    use winapi::um::winuser::GetWindowTextW;
    // use std::os::raw::c_void;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    unsafe {
        let len = GetWindowTextLengthW(hwnd) as usize;
        let mut buf: Vec<u16> = Vec::with_capacity(len + 1);
        buf.set_len(len + 1);
        GetWindowTextW(hwnd, buf.as_mut_ptr(), len as i32 + 1);
        OsString::from_wide(&buf).into_string().unwrap()
    }
}



#[tauri::command]
async fn lanuch_scrcpy(handle: tauri::AppHandle) {

    let local_window = tauri::WindowBuilder::new(
        &handle,
        "local",
        tauri::WindowUrl::App("tool.html".into())
      ).build();


    
    let child = Command::new("scrcpy.exe")
        // .arg(" --window-x -1858 --window-y 31 --window-width 480 --window-height 1049 --max-size 2160  --turn-screen-off --stay-awake --window-title \"K30s\" -s 7dbe4499")
        // .args(&["--window-x", "0", "--window-y", "31", "--window-width", "480", "--window-height", "1049", "--max-size", "2160", "--turn-screen-off", "--stay-awake", "--window-title", "\"K30s\"", "-s", "7dbe4499"])
        .spawn()
        .expect("failed to execute child");

    println!("Launched scrcpy");
    
    let pid = child.id();

    unsafe {
        SCRCPY_PROCESS.push(pid);

        println!("SCRCPY_PROCESS S: {:?}", &SCRCPY_PROCESS);
    }

    // let scrcpy_hwnd;

    let mut scrcpy_hwnd = 0;

    unsafe {
        // TODO: add a loop to get the hwnd
        sleep(Duration::from_secs(1));
        let hwnd = get_hwnd_by_pid(pid);
        println!("hwnd: {:?}", hwnd);

        let title = get_title(hwnd);
        println!("title: {:?}", title);

        let hwnd_usize = hwnd as usize;

        IS_SCRCPY_HWND_MAP.lock().unwrap().insert(hwnd_usize, true);

        scrcpy_hwnd = hwnd_usize;
        // scrcpy_hwnd = hwnd;

        // watch_window_size_and_position();
    }



    child.output().await.expect("failed to wait on child");

    println!("scrcpy exited");

    unsafe {
        for i in 0..SCRCPY_PROCESS.len() {
            if SCRCPY_PROCESS[i] == pid {
                SCRCPY_PROCESS.remove(i);
                break;
            }
        }

        if scrcpy_hwnd != 0 {
            IS_SCRCPY_HWND_MAP.lock().unwrap().remove(&scrcpy_hwnd);
        }
        println!("IS_SCRCPY_HWND_MAP: {:?}", &IS_SCRCPY_HWND_MAP.lock().unwrap());

        println!("SCRCPY_PROCESS F: {:?}", &SCRCPY_PROCESS);
    }

    
}




fn main() {
    println!("Start");

    watch_window_size_and_position();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            
            // window.set_title("Hello World!!!");

            // window.set_size(Size::Logical(LogicalSize {
            //     width: 200.0,
            //     height: 200.0,
            // }));

            // let local_window = tauri::WindowBuilder::new(
            //     app,
            //     "local",
            //     tauri::WindowUrl::App("tool.html".into())
            //   ).build()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            list_devices,
            adb_devices_l,
            lanuch_scrcpy
            ])
        .run(tauri::generate_context!())
        .expect("***********************\nerror while running tauri application");

    println!("End");

    
}
