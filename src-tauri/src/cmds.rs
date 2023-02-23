use std::os::windows::process::CommandExt;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

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

pub fn run_scrcpy(pars: &Vec<String>) -> Option<(u32, usize)> {
    // noconsole
    let child = Command::new("scrcpy.exe")
        // .stdout(Stdio::null())
        // .stderr(Stdio::null())
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
