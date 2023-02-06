#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, Size, LogicalSize};
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_title("Hello World!!!");

            // window.set_size(Size::Logical(LogicalSize {
            //     width: 200.0,
            //     height: 200.0,
            // }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, list_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
