use std::process::Command;
use std::time::Duration;

#[cfg(not(debug_assertions))]
use std::os::windows::process::CommandExt;

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

#[cfg(not(debug_assertions))]
static mut STATIC_PIPE_NAME: Option<String> = None;
#[cfg(not(debug_assertions))]
fn generate_pipe_name() -> String {
    use uuid::Uuid;
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
                #[cfg(debug_assertions)]
                {
                    let pipe_name = format!("\\\\.\\pipe\\{}", "spw-mini-prisma");
                    // FIXME: make it more safe
                    tokio::time::sleep(Duration::from_millis(1000)).await;

                    PRISMA = Some(pipe_name);
                }
                #[cfg(not(debug_assertions))]
                {
                    let db_url = get_db_url();
                    let current_dir = std::env::current_dir().unwrap();
                    let exe_path;

                    exe_path = "./mini-prisma.exe";

                    let mut pipe_name = generate_pipe_name();

                    // // TODO: add some error handling
                    let child = Command::new(exe_path)
                        .arg(db_url)
                        .arg(pipe_name.clone())
                        .creation_flags(0x08000000)
                        .spawn()
                        .unwrap();

                    // FIXME: make it more safe
                    tokio::time::sleep(Duration::from_millis(1000)).await;

                    PRISMA = Some(pipe_name);
                }
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

use winapi::shared::windef::RECT;
pub fn save_size_and_position(rect: RECT, is_borderless: bool, config_id: String) {
    dbg!("save window size and position");
    println!("config id: {:?}", config_id);
    println!(
        "rect: yt:{} xl:{} xr:{}",
        &rect.top, &rect.left, &rect.right
    );

    fn get_prisma_json(config_id: String, key: String, value: String) -> String {
        serde_json::json!({
            "where": {
                "deviceConfigId_key": {
                    "deviceConfigId": config_id,
                    "key": key
                }
            },
            "update":{
                "value": serde_json::json!(value).to_string()
            },
            "create": {
                "deviceConfigId": config_id,
                "key": key,
                "value": serde_json::json!(value).to_string()
            }
        })
        .to_string()
    }

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            let new_top = rect.top;
            let mut new_left = rect.left;
            let mut new_height = rect.bottom - rect.top;
            let mut new_width = rect.right - rect.left;

            // FIXME: windows borader size magic number
            if !is_borderless {
                new_left = new_left + 8;
                new_height = new_height - 8;
                new_width = new_width - 16;
            }

            call_prisma(
                "deviceConfigValue".to_string(),
                "upsert".to_string(),
                get_prisma_json(
                    config_id.clone(),
                    "--window-x".to_string(),
                    serde_json::json!(new_left).to_string(),
                ),
            )
            .await
            .unwrap();

            call_prisma(
                "deviceConfigValue".to_string(),
                "upsert".to_string(),
                get_prisma_json(
                    config_id.clone(),
                    "--window-y".to_string(),
                    serde_json::json!(new_top).to_string(),
                ),
            )
            .await
            .unwrap();

            if new_width > 0 {
                call_prisma(
                    "deviceConfigValue".to_string(),
                    "upsert".to_string(),
                    get_prisma_json(
                        config_id.clone(),
                        "--window-width".to_string(),
                        serde_json::json!(new_width).to_string(),
                    ),
                )
                .await
                .unwrap();
            }
            if new_height > 0 {
                call_prisma(
                    "deviceConfigValue".to_string(),
                    "upsert".to_string(),
                    get_prisma_json(
                        config_id.clone(),
                        "--window-height".to_string(),
                        serde_json::json!(new_height).to_string(),
                    ),
                )
                .await
                .unwrap();
            }

            std::process::exit(0);
        });
}
