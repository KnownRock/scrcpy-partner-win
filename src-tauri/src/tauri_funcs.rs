use tauri::LogicalSize;
use tauri::Size;

pub fn init_main_window(app: &tauri::AppHandle) {
    tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
        .center()
        .visible(false)
        .title("Scrcpy Partner")
        .build()
        .unwrap();
}

pub fn init_tool_window(app: &tauri::AppHandle) {
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