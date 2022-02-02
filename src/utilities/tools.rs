use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use winit::event::VirtualKeyCode;

/// Helper function to convert [c_char; SIZE] to string
pub fn vk_to_string(raw_string_array: &[c_char]) -> String {
    // Implementation 1
    //    let end = '\0' as u8;
    //
    //    let mut content: Vec<u8> = vec![];
    //
    //    for ch in raw_string_array.iter() {
    //        let ch = (*ch) as u8;
    //
    //        if ch != end {
    //            content.push(ch);
    //        } else {
    //            break
    //        }
    //    }
    //
    //    String::from_utf8(content)
    //        .expect("Failed to convert vulkan raw string")

    // Implementation 2
    let raw_string = unsafe {
        let pointer = raw_string_array.as_ptr();
        CStr::from_ptr(pointer)
    };

    raw_string
        .to_str()
        .expect("Failed to convert vulkan raw string.")
        .to_owned()
}

pub fn read_shader_code(shader_path: &Path) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;

    let spv_file =
        File::open(shader_path).expect(&format!("Failed to find spv file at {:?}", shader_path));
    let bytes_code: Vec<u8> = spv_file.bytes().filter_map(|byte| byte.ok()).collect();

    bytes_code
}

pub fn keycode_from_i8(value: u8) -> Option<VirtualKeyCode> {
    match value {
        1 => Some(VirtualKeyCode::Key1),
        2 => Some(VirtualKeyCode::Key2),
        3 => Some(VirtualKeyCode::Key3),
        4 => Some(VirtualKeyCode::Key4),
        5 => Some(VirtualKeyCode::Key5),
        6 => Some(VirtualKeyCode::Key6),
        7 => Some(VirtualKeyCode::Key7),
        8 => Some(VirtualKeyCode::Key8),
        9 => Some(VirtualKeyCode::Key9),
        10 => Some(VirtualKeyCode::Key0),
        11 => Some(VirtualKeyCode::A),
        12 => Some(VirtualKeyCode::B),
        13 => Some(VirtualKeyCode::C),
        14 => Some(VirtualKeyCode::D),
        15 => Some(VirtualKeyCode::E),
        16 => Some(VirtualKeyCode::F),
        17 => Some(VirtualKeyCode::G),
        18 => Some(VirtualKeyCode::H),
        19 => Some(VirtualKeyCode::I),
        20 => Some(VirtualKeyCode::J),
        21 => Some(VirtualKeyCode::K),
        22 => Some(VirtualKeyCode::L),
        23=> Some(VirtualKeyCode::M),
        24=> Some(VirtualKeyCode::N),
        25 => Some(VirtualKeyCode::O),
        26 => Some(VirtualKeyCode::P),
        27 => Some(VirtualKeyCode::Q),
        28 => Some(VirtualKeyCode::R),
        29 => Some(VirtualKeyCode::S),
        30 => Some(VirtualKeyCode::T),
        31 => Some(VirtualKeyCode::U),
        32 => Some(VirtualKeyCode::V),
        33 => Some(VirtualKeyCode::W),
        34 => Some(VirtualKeyCode::X),
        35 => Some(VirtualKeyCode::Y),
        36 => Some(VirtualKeyCode::Z),
        37 => Some(VirtualKeyCode::Escape),
        38 => Some(VirtualKeyCode::F1),
        39 => Some(VirtualKeyCode::F2),
        40 => Some(VirtualKeyCode::F3),
        41 => Some(VirtualKeyCode::F4),
        42 => Some(VirtualKeyCode::F5),
        43 => Some(VirtualKeyCode::F6),
        44 => Some(VirtualKeyCode::F7),
        45 => Some(VirtualKeyCode::F8),
        46 => Some(VirtualKeyCode::F9),
        47 => Some(VirtualKeyCode::F10),
        48 => Some(VirtualKeyCode::F11),
        49 => Some(VirtualKeyCode::F12),
        50 => Some(VirtualKeyCode::F13),
        51 => Some(VirtualKeyCode::F14),
        52 => Some(VirtualKeyCode::F15),
        53 => Some(VirtualKeyCode::Snapshot),
        54 => Some(VirtualKeyCode::Scroll),
        55 => Some(VirtualKeyCode::Pause),
        56 => Some(VirtualKeyCode::Insert),
        57 => Some(VirtualKeyCode::Home),
        58 => Some(VirtualKeyCode::Delete),
        59 => Some(VirtualKeyCode::End),
        60 => Some(VirtualKeyCode::PageDown),
        61 => Some(VirtualKeyCode::PageUp),
        62 => Some(VirtualKeyCode::Left),
        63 => Some(VirtualKeyCode::Up),
        64 => Some(VirtualKeyCode::Right),
        65 => Some(VirtualKeyCode::Down),
        66 => Some(VirtualKeyCode::Back),
        67 => Some(VirtualKeyCode::Return),
        68 => Some(VirtualKeyCode::Space),
        69 => Some(VirtualKeyCode::Numlock),
        70 => Some(VirtualKeyCode::Numpad0),
        71 => Some(VirtualKeyCode::Numpad1),
        72 => Some(VirtualKeyCode::Numpad2),
        73 => Some(VirtualKeyCode::Numpad3),
        74 => Some(VirtualKeyCode::Numpad4),
        75 => Some(VirtualKeyCode::Numpad5),
        76 => Some(VirtualKeyCode::Numpad6),
        77 => Some(VirtualKeyCode::Numpad7),
        78 => Some(VirtualKeyCode::Numpad8),
        79 => Some(VirtualKeyCode::Numpad9),
        80 => Some(VirtualKeyCode::AbntC1),
        81 => Some(VirtualKeyCode::AbntC2),
        82 => Some(VirtualKeyCode::Add),
        83 => Some(VirtualKeyCode::Apostrophe),
        84 => Some(VirtualKeyCode::Apps),
        85 => Some(VirtualKeyCode::At),
        86 => Some(VirtualKeyCode::Ax),
        87 => Some(VirtualKeyCode::Backslash),
        88 => Some(VirtualKeyCode::Calculator),
        89 => Some(VirtualKeyCode::Capital),
        90 => Some(VirtualKeyCode::Colon),
        91 => Some(VirtualKeyCode::Comma),
        92 => Some(VirtualKeyCode::Convert),
        93 => Some(VirtualKeyCode::Decimal),
        94 => Some(VirtualKeyCode::Divide),
        95 => Some(VirtualKeyCode::Equals),
        96 => Some(VirtualKeyCode::Grave),
        97 => Some(VirtualKeyCode::Kana),
        98 => Some(VirtualKeyCode::Kanji),
        99 => Some(VirtualKeyCode::LAlt),
        100 => Some(VirtualKeyCode::LBracket),
        101 => Some(VirtualKeyCode::LControl),
        //102 => Some(VirtualKeyCode::L),
        103 => Some(VirtualKeyCode::LShift),
        104 => Some(VirtualKeyCode::LWin),
        105 => Some(VirtualKeyCode::Mail),
        106 => Some(VirtualKeyCode::MediaSelect),
        107 => Some(VirtualKeyCode::MediaStop),
        108 => Some(VirtualKeyCode::Minus),
        109 => Some(VirtualKeyCode::Multiply),
        110 => Some(VirtualKeyCode::Mute),
        111 => Some(VirtualKeyCode::MyComputer),
        112 => Some(VirtualKeyCode::NavigateForward),
        113 => Some(VirtualKeyCode::NavigateBackward),
        114 => Some(VirtualKeyCode::NextTrack),
        115 => Some(VirtualKeyCode::NoConvert),
        116 => Some(VirtualKeyCode::NumpadComma),
        117 => Some(VirtualKeyCode::NumpadEnter),
        118 => Some(VirtualKeyCode::NumpadEquals),
        119 => Some(VirtualKeyCode::OEM102),
        120 => Some(VirtualKeyCode::Period),
        121 => Some(VirtualKeyCode::PlayPause),
        122 => Some(VirtualKeyCode::Power),
        123 => Some(VirtualKeyCode::PrevTrack),
        124 => Some(VirtualKeyCode::RAlt),
        125 => Some(VirtualKeyCode::RBracket),
        126 => Some(VirtualKeyCode::RControl),
        //127 => Some(VirtualKeyCode::RMenu),
        128 => Some(VirtualKeyCode::RShift),
        129 => Some(VirtualKeyCode::RWin),
        130 => Some(VirtualKeyCode::Semicolon),
        131 => Some(VirtualKeyCode::Slash),
        132 => Some(VirtualKeyCode::Sleep),
        133 => Some(VirtualKeyCode::Stop),
        134 => Some(VirtualKeyCode::Subtract),
        135 => Some(VirtualKeyCode::Sysrq),
        136 => Some(VirtualKeyCode::Tab),
        137 => Some(VirtualKeyCode::Underline),
        138 => Some(VirtualKeyCode::Unlabeled),
        139 => Some(VirtualKeyCode::VolumeDown),
        140 => Some(VirtualKeyCode::VolumeUp),
        141 => Some(VirtualKeyCode::Wake),
        142 => Some(VirtualKeyCode::WebBack),
        143 => Some(VirtualKeyCode::WebFavorites),
        144 => Some(VirtualKeyCode::WebForward),
        145 => Some(VirtualKeyCode::WebHome),
        146 => Some(VirtualKeyCode::WebRefresh),
        147 => Some(VirtualKeyCode::WebSearch),
        148 => Some(VirtualKeyCode::WebStop),
        149 => Some(VirtualKeyCode::Yen),
        _ => None
    }

}