
use std::{process::exit, time::Duration, ffi::CString};
use mki::Keyboard;
use winapi::{um::{wincon::{GetConsoleWindow, SetConsoleTitleA}, winuser::{OpenClipboard, CloseClipboard, GetForegroundWindow, GetWindowTextA, CF_UNICODETEXT}, winbase::GlobalSize, winnt::LPCSTR,}, ctypes::c_void, shared::windef::HWND__};
pub mod input;
#[allow(dead_code)]
#[derive(Debug)]
struct ClipboardData {
    pointer: *mut c_void,
    size: usize
}

unsafe fn get_clipboard_data(hwnd: *mut HWND__) -> ClipboardData {
    let _ = OpenClipboard(hwnd);
    std::thread::sleep(Duration::from_millis(50));
    let clipboard = winapi::um::winuser::GetClipboardData(CF_UNICODETEXT);
    let clipboard_size = GlobalSize(clipboard);
    let _ = CloseClipboard();
    ClipboardData { pointer: clipboard, size: clipboard_size }
}

unsafe fn get_process_name() -> String {
    let h = GetForegroundWindow();
    let hh = h.cast();
    let mut buffer: [i8; 32] = [0;32];    
    let _ = GetWindowTextA(hh, buffer.as_mut_ptr(), 32);
    std::thread::sleep(Duration::from_millis(50));
    let window_title = std::str::from_utf8_unchecked(std::mem::transmute(&buffer as &[i8])).trim();
    window_title.to_string()
}

fn main() {
    let titlebar = CString::new("LoL clipboard || https://zeunig.hu || https://github.com/Zeunig/LoL-clipboard").unwrap();
    let titlebar = titlebar.as_ptr() as LPCSTR;
    unsafe { SetConsoleTitleA(titlebar) };
    println!(r#"----------------------------------------------------------
            CLIPBOARD FOR LEAGUE OF LEGENDS
                   MADE BY: ZEUNIG
                  GITHUB.COM/ZEUNIG
                      ZEUNIG.HU
           CTRL+V TO PASTE (UP TO 250 CHARACTER)
    CTRL+SHIFT+V TO PASTE TO ALL CHAT (NO CHARACTER LIMIT)
              CTRL+ALT TO EXIT APPLICATION
----------------------------------------------------------"#);
    mki::register_hotkey(&[Keyboard::LeftControl, Keyboard::V],
    move || unsafe {
        if get_process_name().contains("League of Legends (TM) Client") {
            let hwnd = GetConsoleWindow();
            let clipboard = get_clipboard_data(hwnd);
            let slice = std::slice::from_raw_parts(clipboard.pointer as *const u16, clipboard.size+32).to_vec();
            let strslice = char::decode_utf16(slice)
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect::<String>();
            let strslice = strslice.split("\0").collect::<Vec<&str>>()[0];
            crate::input::key_sequence(strslice.trim()); 
            std::thread::sleep(Duration::from_millis(50)); // if he spams that fucking ctrl v too much it will crash my program

        }else {
            std::thread::sleep(Duration::from_millis(150)); // if he spams that fucking ctrl v too much it will crash my program
        }
    }
    );
    mki::register_hotkey(&[Keyboard::LeftControl, Keyboard::LeftAlt],
    || {
        exit(0);
    }
    );
    std::thread::sleep(Duration::from_secs(u64::MAX));
}
