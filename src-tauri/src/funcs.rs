use crate::cmds::get_adb_devices;

pub fn is_device_valid_args(args: Vec<String>) -> bool {
    // let args: Vec<String> = env::args().collect();

    let devices_ids = get_adb_devices();
    if devices_ids.len() == 0 {
        return false;
    }

    let mut have_device_arg_flag = false;

    for arg in &args {
        for device_id in &devices_ids {
            if arg == &format!("--serial {}", device_id)
              || arg == &format!("--serial={}", device_id)
              || arg == &format!("-s{}", device_id)
              || arg.starts_with("--tcpip")
              // TODO: check next -s arg
              || arg == "-s"
            {
                have_device_arg_flag = true;
                break;
            }
        }
    }
    have_device_arg_flag
}
