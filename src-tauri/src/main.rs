#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use std::os::windows::process::CommandExt;
use std::process::Command;

mod cmds;
mod sendkey;
mod tauri_funcs;
mod wins;

use cmds::kill_process;
use tauri_funcs::{init_main_window, init_record_window, init_tool_window};
use wins::get_hwnd_by_pid;
static mut RECORD_WINDOW: Option<tauri::Window> = None;

// IS_RECORD_PANEL_WITH_MOTION_ROCORD
#[tauri::command]
fn set_record_panel_with_motion_record(record_panel_with_motion_record: bool) {
    unsafe {
        WATCHER
            .app_state
            .as_mut()
            .unwrap()
            .set_is_record_panel_with_motion_record(record_panel_with_motion_record);
        WATCHER.fresh_loc_and_size();
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

#[tauri::command]
async fn get_config_id() -> String {
    unsafe { WATCHER.app_state.as_ref().unwrap().get_config_id() }
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

static mut IS_AUTO_SAVE_LOCATION_AND_SIZE: bool = false;
static mut IS_WINDOW_BORDERLESS: bool = false;
static mut CONFIG_ID: String = String::new();
// static mut C: Option<FnMut(usize, usize, usize, usize)> = None;

// remember to call `.manage(MyState::default())`
use watcher::WATCHER;

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
    // state: tauri::State<'_, MyState>,
    // why Result<(), ()>
    // https://github.com/tauri-apps/tauri/discussions/4317
) -> Result<(), ()> {
    if is_tool_mode {
        unsafe {
            PID = pid as u32;
            HWND = hwnd;

            assert!(HWND != 0, "Failed to get hwnd");
            assert!(PID != 0, "Failed to get pid");

            println!("Mode: tool");

            let win = init_tool_window(&app);

            let mut app_state = AppState::new();
            app_state.set_tool_window(win);
            app_state.set_pid_and_hwnd(pid, hwnd);
            app_state.set_save_info(
                is_auto_save_location_and_size,
                is_window_borderless,
                config_id.clone(),
            );

            WATCHER.set_app_state(app_state);

            println!("*** tool init done ***")
        }
    } else {
        println!("Mode: main");
        init_main_window(&app);
    }

    Ok(())
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
            // RECORD_WINDOW = None;

            WATCHER.app_state.as_mut().unwrap().unset_record_window();
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
            let win = init_record_window(&app);
            WATCHER.app_state.as_mut().unwrap().set_record_window(win);
        }
    }
}

mod watcher;
use watcher::AppState;

fn main() {
    tauri::Builder::default()
        .on_page_load(|window, _payload| {
            if window.label() == "tool" {
                unsafe {
                    WATCHER.start();
                    WATCHER.fresh_loc_and_size();
                };
                println!("*** tool window loaded ***");
            } else if window.label() == "record" {
                unsafe {
                    WATCHER.fresh_loc_and_size();
                }
                println!("*** record window loaded ***");
            } else {
                println!("*** non tool window loaded ***");
            }
        })
        .invoke_handler(tauri::generate_handler![
            sendkey,
            lanuch_self,
            init,
            call_prisma,
            get_env_args,
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
