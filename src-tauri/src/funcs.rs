use crate::cmds::get_adb_devices;

pub fn is_device_valid_args(args: Vec<String>) -> bool {
    // let args: Vec<String> = env::args().collect();

    let devices_ids = get_adb_devices();

    is_device_valid_args_by_devices(args, devices_ids)
}

fn is_device_valid_args_by_devices(args: Vec<String>, devices_ids: Vec<String>) -> bool {
    if devices_ids.len() == 0 {
        return false;
    }

    let mut have_device_arg_flag = false;

    dbg!(args.clone());

    for (i, arg) in args.iter().enumerate() {
        for device_id in &devices_ids {
            if arg == &format!("--serial={}", device_id)
                || arg == &format!("-s{}", device_id)
                || arg == &format!("--serial {}", device_id)
                || arg == &format!("-s {}", device_id)
                || arg.starts_with("--tcpip")
                || (arg == "-s" && args.len() > i + 1 && &args[i + 1] == device_id)
                || (arg == "--serial" && args.len() > i + 1 && &args[i + 1] == device_id)
            {
                have_device_arg_flag = true;
                break;
            }
        }
    }
    have_device_arg_flag
}

#[test]
fn test_is_device_valid_args_by_devices() {
    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "-s1234567890".to_string()],
            vec!["1234567890".to_string(), "1234567891".to_string()]
        ),
        true
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "--serial=1234567890".to_string()],
            vec!["1234567891".to_string()]
        ),
        false
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "--serial=1234567890".to_string()],
            vec!["1234567890".to_string(), "1234567891".to_string()]
        ),
        true
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "--serial 1234567890".to_string()],
            vec!["1234567890".to_string(), "1234567891".to_string()]
        ),
        true
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "-s 1234567890".to_string()],
            vec!["1234567890".to_string(), "1234567891".to_string()]
        ),
        true
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "--serial 1234567893".to_string()],
            vec!["1234567890".to_string(), "1234567891".to_string()]
        ),
        false
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "-s 1234567893".to_string()],
            vec!["1234567890".to_string(), "1234567891".to_string()]
        ),
        false
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec![
                "scrcpy.exe".to_string(),
                "-s".to_string(),
                "123".to_string()
            ],
            vec!["1234567890".to_string(), "123".to_string()]
        ),
        true
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec![
                "scrcpy.exe".to_string(),
                "--serial".to_string(),
                "123".to_string()
            ],
            vec!["1234567890".to_string(), "123".to_string()]
        ),
        true
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec![
                "scrcpy.exe".to_string(),
                "--tcpip".to_string(),
                "123".to_string()
            ],
            vec!["1234567890".to_string(), "123".to_string()]
        ),
        true
    );

    assert_eq!(
        is_device_valid_args_by_devices(
            vec!["scrcpy.exe".to_string(), "--tcpip=123".to_string()],
            vec![]
        ),
        true
    );
}
