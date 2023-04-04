use tauri::LogicalSize;
use tauri::Size;

#[cfg(not(debug_assertions))]
pub fn init_main_window(app: &tauri::AppHandle) {
    tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
        .center()
        .visible(false)
        .title("Scrcpy Partner")
        .build()
        .unwrap();
}

#[cfg(debug_assertions)]
pub fn init_main_window(app: &tauri::AppHandle) {
    tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
        .center()
        // .visible(false)
        .title("Scrcpy Partner")
        .build()
        .unwrap();
}

#[cfg(not(debug_assertions))]
pub fn init_tool_window(app: &tauri::AppHandle) {
    let tool_window =
        tauri::WindowBuilder::new(app, "tool", tauri::WindowUrl::App("tool.html".into()))
            .visible(false)
            .decorations(false)
            .resizable(false)
            .skip_taskbar(true)
            .position(-99999.0, -99999.0)
            .build()
            .unwrap();

    tool_window
        .set_size(Size::Logical(LogicalSize {
            width: 48.0,
            height: 650.0,
        }))
        .unwrap();
}

#[cfg(debug_assertions)]
pub fn init_tool_window(app: &tauri::AppHandle) -> tauri::Window {
    let tool_window =
        tauri::WindowBuilder::new(app, "tool", tauri::WindowUrl::App("tool.html".into()))
            // .visible(false)
            .decorations(false)
            .resizable(false)
            .position(-99999.0, -99999.0)
            .skip_taskbar(true)
            .build()
            .unwrap();

    tool_window
        .set_size(Size::Logical(LogicalSize {
            width: 50.0,
            height: 50.0,
        }))
        .unwrap();

    return tool_window;

    // println!("tool_window_loaded++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    // tool_window.emit("tool_window_loaded", ()).unwrap();
}

pub fn init_record_window(app: &tauri::AppHandle) {
    let tool_window =
        tauri::WindowBuilder::new(app, "record", tauri::WindowUrl::App("record.html".into()))
            // .visible(false)
            // .decorations(false)
            .resizable(false)
            .position(-99999.0, -99999.0)
            .skip_taskbar(true)
            // .always_on_top(true)
            .transparent(true)
            .build()
            .unwrap();

    tool_window
        .set_size(Size::Logical(LogicalSize {
            width: 1.0,
            height: 1.0,
        }))
        .unwrap();

    // https://github.com/tauri-apps/tauri/issues/4881
    tool_window.set_decorations(false).unwrap();
}
