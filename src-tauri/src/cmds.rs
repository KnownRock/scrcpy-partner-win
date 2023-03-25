use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{os::windows::process::CommandExt, process::Child};
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

use crate::wins::get_hwnd_by_pid;
// use async_process::Stdio;
pub fn get_adb_devices_raw() -> String {
    let output = std::process::Command::new("adb")
        .arg("devices")
        .arg("-l")
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();

    output
}

#[test]
fn test_get_adb_devices_raw() {
    let output = get_adb_devices_raw();
    assert!(output.len() > 0);
}

pub fn get_adb_devices() -> Vec<String> {
    let output = get_adb_devices_raw();

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

#[test]
fn test_get_adb_devices() {
    let devices = get_adb_devices();
    assert!(devices.len() > 0 || devices.len() == 0);
}

pub fn get_all_pids() -> Vec<u32> {
    let mut pids = Vec::new();
    let system = System::new_all();
    for proc in system.processes() {
        let pid = proc.0;
        let pid_string = pid.to_string();
        let pid_u32 = pid_string.parse::<u32>().unwrap();
        pids.push(pid_u32);
    }
    pids
}

#[test]
fn test_get_all_pids() {
    let pids = get_all_pids();
    assert!(pids.len() > 0);
}

pub fn is_process_alive(pid: u32) -> bool {
    let pids = get_all_pids();
    pids.contains(&pid)
}

#[test]
fn test_is_process_alive() {
    let pids = get_all_pids();
    let pid = pids[0];
    let alive = is_process_alive(pid);
    assert!(alive);
}

fn connect_tcpip(device: &str) {
    let devices = get_adb_devices();
    if devices.contains(&device.to_string()) {
        return;
    }

    let output = std::process::Command::new("adb")
        .arg("connect")
        .arg(format!("{}", device))
        .creation_flags(0x08000000)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("connect_tcpip: {}", output);
}

#[test]
fn test_connect_tcpip() {
    connect_tcpip("127.0.0.1:5555");
}

fn get_tcpip_device(pars: &Vec<String>) -> Option<&str> {
    for (i, par) in pars.iter().enumerate() {
        if par == "--tcpip" {
            if i + 1 < pars.len() {
                return Some(&pars[i + 1]);
            }
        } else if par.starts_with("--tcpip=") {
            return Some(&par[8..]);
        } else if par.starts_with("--tcpip ") {
            return Some(&par[8..]);
        }
    }
    None
}

#[test]
fn test_get_tcpip_device() {
    let pars = vec![
        "--window-x",
        "0",
        "--window-y",
        "31",
        "--window-width",
        "480",
        "--window-height",
        "1049",
        "--max-size",
        "1080",
        "--turn-screen-off",
        "--max-fps=30",
        "--tcpip=127.0.0.1:5555",
        "--display-buffer=100",
    ];

    let pars_strings = pars.iter().map(|s| s.to_string()).collect();

    let device = get_tcpip_device(&pars_strings);
    assert_eq!(device, Some("10.8.0.8:8888"));
}

fn filter_tcpip_arg_from_args(pars: &Vec<String>) -> Vec<String> {
    let mut new_pars = Vec::new();
    for (i, par) in pars.iter().enumerate() {
        if par == "--tcpip" {
            continue;
        } else if par.starts_with("--tcpip=") {
            continue;
        } else if par.starts_with("--tcpip ") {
            continue;
        } else {
            if i >= 1 {
                if pars[i - 1] == "--tcpip" {
                    continue;
                }
            }

            new_pars.push(par.clone());
        }
    }
    new_pars
}

#[test]
fn test_filter_tcpip_arg_from_args() {
    let pars = vec![
        "--window-x",
        "0",
        "--window-y",
        "31",
        "--window-width",
        "480",
        "--window-height",
        "1049",
        "--max-size",
        "1080",
        "--turn-screen-off",
        "--max-fps=30",
        "--tcpip=127.0.0.1:5555",
        "--display-buffer=100",
    ];

    let pars_strings = pars.iter().map(|s| s.to_string()).collect();

    let new_pars = filter_tcpip_arg_from_args(&pars_strings);
    assert_eq!(new_pars.len(), 13);
}

// async fn get_pars_by_config_id(config_id: String) {
//     let config = call_prisma(
//         "deviceConfig".to_string(),
//         "findUnique".to_string(),
//         serde_json::json!({
//             "where": {
//                 "id": config_id,
//             },
//         })
//         .to_string(),
//     )
//     .await
//     .unwrap();

//     println!("config: {:?}", config);

//     let
// }

// #[tokio::test]
// async fn test_get_pars_by_config_id() {
//     get_pars_by_config_id("ckq0q0q0q0000".to_string()).await;
// }

fn get_add_serial_arg_by_tcpip_device(device: &str, pars: &Vec<String>) -> Vec<String> {
    let mut pars = pars.clone();
    let serial = device.to_string();
    pars.push(format!("--serial={}", serial));
    pars
}

pub fn run_scrcpy(pars: &Vec<String>) -> Option<(u32, usize)> {
    // FIXME: make more safe and robust and check args
    let mut pars = pars.clone();
    match get_tcpip_device(&pars) {
        Some(device) => {
            connect_tcpip(device);
            pars = get_add_serial_arg_by_tcpip_device(device, &pars);
            pars = filter_tcpip_arg_from_args(&pars);
        }
        None => {}
    }

    dbg!("The scrcpy args: {:?}", &pars);

    #[cfg(not(debug_assertions))]
    let child = Command::new("scrcpy.exe")
        .args(pars)
        .creation_flags(0x08000000)
        .spawn()
        .unwrap();
    #[cfg(debug_assertions)]
    let child = Command::new("scrcpy.exe").args(pars).spawn().unwrap();

    let pid = child.id();

    // MEMO: can not use tcpip arg, only can use serial arg and adb connect manually
    let is_scrcpy_process_alive = is_process_alive(pid);
    if !is_scrcpy_process_alive {
        return None;
    }

    let hwnd_usize;
    loop {
        sleep(Duration::from_millis(100));

        if is_process_alive(pid) == false {
            return None;
        }

        let hwnd = get_hwnd_by_pid(pid);
        println!("hwnd: {:?}", hwnd);

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

#[test]
fn test_run_scrcpy() {
    let devices = get_adb_devices();

    if devices.len() == 0 {
        return;
    }

    let pars = vec![
        "-s".to_string(),
        devices[0].clone(),
        "-m".to_string(),
        "240".to_string(),
        "-b".to_string(),
        "2M".to_string(),
    ];

    let result = run_scrcpy(&pars);
    assert!(result.is_some());

    match result {
        Some((pid, hwnd)) => {
            // println!("pid: {}, hwnd: {}", pid, hwnd);
            if pid != 0 && hwnd != 0 {
                kill_process(pid);
            }
        }
        None => {}
    }
}

pub fn kill_process(pid: u32) {
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

#[cfg(debug_assertions)]
fn get_db_url() -> String {
    let current_dir = std::env::current_dir().unwrap();
    let db_path = current_dir.join("../prisma").join("main.db");
    let db_url = format!("file:{}", db_path.to_str().unwrap());
    // println!("db_url: {}", db_url);
    db_url
}
#[cfg(not(debug_assertions))]
fn get_db_url() -> String {
    let current_dir = std::env::current_dir().unwrap();
    let db_path = current_dir.join("main.db");
    let db_url = format!("file:{}", db_path.to_str().unwrap());
    db_url
}

use tokio::net::windows::named_pipe;
static mut PRISMA: Option<String> = None;
use std::error::Error;
use std::io;
use tokio::io::{AsyncReadExt, Interest};
use uuid::Uuid;

static mut STATIC_PIPE_NAME: Option<String> = None;
fn generate_pipe_name() -> String {
    unsafe {
        match &mut STATIC_PIPE_NAME {
            Some(_) => {}
            None => {
                let uuid = Uuid::new_v4();
                let pipe_name = format!("\\\\.\\pipe\\{}", uuid);
                STATIC_PIPE_NAME = Some(pipe_name);
            }
        }
    }

    unsafe { STATIC_PIPE_NAME.clone().unwrap() }
}

// FIXME: kill prisma process after main process is killed in spec conditions
pub async fn call_prisma(
    table: String,
    func: String,
    arg_json: String,
) -> Result<String, Box<dyn Error>> {
    unsafe {
        match &mut PRISMA {
            Some(_) => {}
            None => {
                let db_url = get_db_url();

                let current_dir = std::env::current_dir().unwrap();
                dbg!(current_dir);

                let exe_path;
                #[cfg(debug_assertions)]
                {
                    exe_path = "./target/release/mini-prisma.exe";
                }
                #[cfg(not(debug_assertions))]
                {
                    exe_path = "./mini-prisma.exe";
                }

                let mut pipe_name = generate_pipe_name();
                #[cfg(debug_assertions)]
                {
                    pipe_name = format!("\\\\.\\pipe\\{}", "spw-mini-prisma");
                }

                #[cfg(not(debug_assertions))]
                {
                    // // TODO: add some error handling
                    let child = Command::new(exe_path)
                        .arg(db_url)
                        .arg(pipe_name.clone())
                        .creation_flags(0x08000000)
                        .spawn()
                        .unwrap();
                }

                // FIXME: make it more safe
                tokio::time::sleep(Duration::from_millis(1000)).await;

                PRISMA = Some(pipe_name);
            }
        }
    };

    unsafe {
        let pipe_name = PRISMA.as_ref().unwrap();

        let mut client = named_pipe::ClientOptions::new().open(pipe_name).unwrap();

        loop {
            let ready = client
                .ready(Interest::READABLE | Interest::WRITABLE)
                .await?;

            if ready.is_writable() {
                // write a json string to the pipe
                let json = serde_json::json!({
                    "table": table,
                    "func": func,
                    "arg_json": arg_json
                });

                let text = json.to_string();

                match client.try_write(text.as_bytes()) {
                    Ok(n) => {
                        dbg!("write {} bytes", &n);
                        // dbg!("text: {}", &text);
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(_) => {
                        continue;
                    }
                }

                break;
            }
        }
        #[cfg(debug_assertions)]
        {
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
        #[cfg(not(debug_assertions))]
        {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        let mut full_data = vec![0; 1024 * 1024];
        let mut ptr = 0;

        // FIXME: may unicode problem
        loop {
            let ready = client
                .ready(Interest::READABLE | Interest::WRITABLE)
                .await?;

            if ready.is_readable() {
                dbg!("readable");

                let mut data = Vec::<u8>::new();
                client.read_to_end(&mut data).await?;

                let mut text = String::from_utf8(data).unwrap();
                text = text.trim_end_matches(char::from(0)).to_string();
                // println!("text: {:?}", &text);

                return Ok(text);
            }
        }
    }

    Ok("{\"error\": \"no data\"}".to_string())
}

#[tokio::test]
async fn test_call_prisma() {
    // print exec path
    let path = std::env::current_exe().unwrap();
    println!("path: {:?}", path);

    // print working dir
    let path = std::env::current_dir().unwrap();
    println!("path: {:?}", path);

    let table = "test".to_string();
    let func = "deleteMany".to_string();
    let arg_json = "".to_string();
    let output = call_prisma(table.clone(), func, arg_json).await.unwrap();
    println!("output: \"{}\"", output);
    let object = serde_json::from_str::<serde_json::Value>(&output).unwrap();
    assert!(
        object["data"]["count"].as_u64().unwrap() == 0
            || object["data"]["count"].as_u64().unwrap() >= 1
    );

    let func = "findMany".to_string();
    let arg_json = "".to_string();
    let output = call_prisma(table.clone(), func, arg_json).await.unwrap();
    println!("output: \"{}\"", output);
    // data structure {data:[content]}

    let object = serde_json::from_str::<serde_json::Value>(&output).unwrap();
    assert!(object["data"].as_array().unwrap().len() == 0);

    let func = "create".to_string();
    let arg_json = r#"{"data":{"name":"test1"}}"#.to_string();
    let output = call_prisma(table.clone(), func, arg_json).await.unwrap();
    println!("output: \"{}\"", output);
    let object = serde_json::from_str::<serde_json::Value>(&output).unwrap();
    assert!(object["data"]["name"].as_str().unwrap() == "test1");

    let func = "findMany".to_string();
    let arg_json = "".to_string();
    let output = call_prisma(table.clone(), func, arg_json).await.unwrap();
    println!("output: \"{}\"", output);
    let object = serde_json::from_str::<serde_json::Value>(&output).unwrap();
    assert!(object["data"].as_array().unwrap().len() == 1);

    let func = "error".to_string();
    let arg_json = "".to_string();
    let output = call_prisma(table.clone(), func, arg_json).await.unwrap();
    println!("output: \"{}\"", output);
    let object = serde_json::from_str::<serde_json::Value>(&output).unwrap();
    assert!(object["error"].as_str().unwrap().len() > 0);
}
