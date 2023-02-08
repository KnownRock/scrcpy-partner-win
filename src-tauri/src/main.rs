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

use tauri::LogicalPosition;
use tauri::LogicalSize;
use tauri::Position;
use tauri::Size;
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

use winapi::ctypes::c_void;
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

static mut MAIN_WINDOWS : Vec<tauri::Window> = Vec::new();

unsafe extern "system" fn win_event_callback(
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


    // println!("{:?}", MAIN_WINDOWS.len());
    if MAIN_WINDOWS.len() == 1{
        let window = & mut MAIN_WINDOWS[0];

        window.set_position(Position::Logical(LogicalPosition::new((rect.right - 8) as f64, (rect.top + 30i32) as f64)));
    }


}

fn watch_window_size_and_position(pid: DWORD) {
    println!("watch_window_size_and_position, pid: {}", 0);

    

    // type MyFn = unsafe extern "system" fn(
    //     HWINEVENTHOOK,
    //     winapi::shared::minwindef::DWORD,
    //     winapi::shared::windef::HWND,
    //     LONG,
    //     LONG,
    //     winapi::shared::minwindef::DWORD,
    //     winapi::shared::minwindef::DWORD,
    // );

    // let win_event_callback_2 = |
    //         _hwin_event_hook: HWINEVENTHOOK,
    //         _event: winapi::shared::minwindef::DWORD,
    //         _hwnd: winapi::shared::windef::HWND,
    //         _id_object: LONG,
    //         _id_child: LONG,
    //         _id_event_thread: winapi::shared::minwindef::DWORD,
    //         _dwms_event_time: winapi::shared::minwindef::DWORD, 
    // | {
    //     unsafe{
    //         return win_event_callback(_hwin_event_hook, _event, _hwnd, _id_object, _id_child, _id_event_thread, _dwms_event_time);
    //     }

    // } as MyFn;


    

    use winapi::um::winuser::{SetWinEventHook ,EVENT_OBJECT_LOCATIONCHANGE,EVENT_SYSTEM_FOREGROUND, WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS, WINEVENT_SKIPOWNTHREAD}; 

    unsafe{
        let hook_handle = unsafe {
            SetWinEventHook(
                EVENT_OBJECT_LOCATIONCHANGE, 
                EVENT_OBJECT_LOCATIONCHANGE, 
                std::ptr::null_mut(), 
                Some(win_event_callback), 
                pid, 
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
async fn get_exec_mode() -> String {
    unsafe {
        if IS_TOOL_MODE {
            return "tool".to_string();
        } else {
            return "local".to_string();
        }
    }
}

#[tauri::command]
async fn lanuch_scrcpy(handle: tauri::AppHandle, id: String)  {
    // let local_window = tauri::WindowBuilder::new(
    //     &handle,
    //     "local",
    //     tauri::WindowUrl::App("tool.html".into())
    //   ).build();

    // watch_window_size_and_position();

   
    
    let child = Command::new("scrcpy.exe")
        .arg(format!("-s{}", id))
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


static mut IS_TOOL_MODE : bool = false;
static mut PID : u32 = 0;




fn main() {
    println!("Start");

    // print work dir
    // let work_dir = env::current_dir().unwrap();
    // println!("work dir: {:?}", work_dir);

    // get pars
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    


    // watch_window_size_and_position();

    tauri::Builder::default()
        .setup(|app| {
            let mut window = app.get_window("main").unwrap();

            match app.get_cli_matches() {
                    Ok(matches) => {
                    matches.args.iter().for_each(|(key, value)| {
                        println!("{}: {:?}", key, value);
                        unsafe {
                            if key == "spw-tool" {
                                IS_TOOL_MODE = value.value.as_bool().unwrap();
                            }
                            if key == "spw-pid" {
                                match value.value.as_str() {
                                    Some(v) => {
                                        PID = v.parse::<u32>().unwrap();
                                    }
                                    None => {}
                                }
                            }
                        }
                    });

                    unsafe {
                        println!("PID: {:?}", &PID);
                        println!("IS_TOOL_MODE: {:?}", &IS_TOOL_MODE);
                    }
                }
                Err(_) => {}
            }

            if unsafe { IS_TOOL_MODE } {
                window.set_title("SPW Tool");
                window.set_size(Size::Logical(LogicalSize {
                    width: 40.0,
                    height: 600.0,
                }));
                window.set_decorations(false);
                window.set_resizable(false);


                window.set_position(Position::Logical(LogicalPosition {
                    x: 0.0,
                    y: 0.0,
                }));
            } else {
                window.set_title("SPW");
                window.set_size(Size::Logical(LogicalSize {
                    width: 480.0,
                    height: 1049.0,
                }));
                
            }

            unsafe{
                if IS_TOOL_MODE {
                    let pid = PID;
                    if pid == 0 {
                        panic!("PID is 0");
                    }

                    // MAIN_WINDOW.lock().unwrap().replace(window);
                    MAIN_WINDOWS.push(window);
                    watch_window_size_and_position(pid);
                }
            }


            
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
            adb_devices_l,
            lanuch_scrcpy
            ])
        .run(tauri::generate_context!())
        .expect("***********************\nerror while running tauri application");

    println!("End");

    
}
