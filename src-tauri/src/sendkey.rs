use std::thread::sleep;
use std::time::Duration;

use winapi::um::winuser::PostMessageW;
use winapi::shared::windef::HWND;
pub fn sendkey(
  hwnd_usize: usize, 
  key_code: usize, 
  scan_code: usize, 
  extend_key_flag: usize,
  is_alt: bool,
  is_shift: bool,
){
  let hwnd = hwnd_usize as HWND;

  unsafe{
    if is_alt {
      PostMessageW(hwnd, 0x0104, 0x12, 0x00380001);
    } 
    if is_shift {
      PostMessageW(hwnd, 0x0104, 0xA0, 0x202a0001);
    }
    
    PostMessageW(hwnd, 0x0104, key_code, (0x20000001 + (scan_code<<16) + (extend_key_flag<<24)) as isize);
    sleep(Duration::from_millis(100));
    PostMessageW(hwnd, 0x0105, key_code, (0xe0000001 + (scan_code<<16) + (extend_key_flag<<24)) as isize);
    
    if is_shift {
      PostMessageW(hwnd, 0x0105, 0xA0, 0xe02a0001);
    }
    if is_alt {
      PostMessageW(hwnd, 0x0105, 0x12, 0xc0380001); 
    }
    
  }

}

