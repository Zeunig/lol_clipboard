use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::{KEYEVENTF_UNICODE, VIRTUAL_KEY, KEYEVENTF_KEYUP, INPUT, INPUT_KEYBOARD, INPUT_0, KEYBDINPUT, SendInput};

pub fn key_sequence(sequence: &str) {
    let mut buffer = [0; 2];
    for c in sequence.chars() {
        let result = c.encode_utf16(&mut buffer);
        if result.len() == 1 {
            //unicode_key_click(result[0]);

            keyclick_event(VIRTUAL_KEY(0),result[0])
        } else {
            for utf16_surrogate in result {
                keyclick_event(VIRTUAL_KEY(0),*utf16_surrogate);
                //unicode_key_down(*utf16_surrogate);
            }
        }
        //println!("CHAR : {}, BUFFER : {:?}",c,buffer);
    }
}

fn keyclick_event(vk: VIRTUAL_KEY, scan: u16) {
    let input = [INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    },
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
    ];
    unsafe {
        SendInput(
            &input,
            size_of::<INPUT>()
                .try_into()
                .expect("Could not convert the size of INPUT to i32"),
        )
    };
}
