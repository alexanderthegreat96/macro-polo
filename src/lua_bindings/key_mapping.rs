use enigo::Key;

// simply maps the string from the user
// to a Key type
// it is also platofmr aware
pub fn key_from_str(name: &str) -> Option<Key> {
    use Key::*;

    let lower = name.to_ascii_lowercase();
    match lower.as_str() {
        "alt" | "option" => Some(Alt),
        "backspace" => Some(Backspace),
        "capslock" => Some(CapsLock),
        "control" | "ctrl" | "lcontrol" => Some(Control),
        "shift" | "lshift" => Some(Shift),
        "rshift" => Some(RShift),
        "rcontrol" => Some(RControl),
        "space" => Some(Space),
        "tab" => Some(Tab),
        "enter" | "return" => Some(Return),
        "escape" | "esc" => Some(Escape),
        "meta" | "command" | "super" | "windows" => Some(Meta),
        "up" | "uparrow" => Some(UpArrow),
        "down" | "downarrow" => Some(DownArrow),
        "left" | "leftarrow" => Some(LeftArrow),
        "right" | "rightarrow" => Some(RightArrow),
        "volumeup" => Some(VolumeUp),
        "volumedown" => Some(VolumeDown),
        "volumemute" => Some(VolumeMute),
        #[cfg(all(unix, not(target_os = "macos")))]
        "micmute" => Some(MicMute),

        // F1-F35
        s if s.starts_with('f')
            && s[1..]
                .parse::<u8>()
                .ok()
                .filter(|n| (1..=35).contains(n))
                .is_some() =>
        {
            match s {
                "f1" => Some(F1),
                "f2" => Some(F2),
                "f3" => Some(F3),
                "f4" => Some(F4),
                "f5" => Some(F5),
                "f6" => Some(F6),
                "f7" => Some(F7),
                "f8" => Some(F8),
                "f9" => Some(F9),
                "f10" => Some(F10),
                "f11" => Some(F11),
                "f12" => Some(F12),
                "f13" => Some(F13),
                "f14" => Some(F14),
                "f15" => Some(F15),
                "f16" => Some(F16),
                "f17" => Some(F17),
                "f18" => Some(F18),
                "f19" => Some(F19),
                "f20" => Some(F20),
                #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
                "f21" => Some(F21),
                #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
                "f22" => Some(F22),
                #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
                "f23" => Some(F23),
                #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
                "f24" => Some(F24),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f25" => Some(F25),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f26" => Some(F26),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f27" => Some(F27),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f28" => Some(F28),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f29" => Some(F29),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f30" => Some(F30),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f31" => Some(F31),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f32" => Some(F32),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f33" => Some(F33),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f34" => Some(F34),
                #[cfg(all(unix, not(target_os = "macos")))]
                "f35" => Some(F35),
                _ => None,
            }
        }

        // A-Z, 0-9 (platform-specific)
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            if c.is_ascii_alphanumeric() {
                #[cfg(target_os = "windows")]
                {
                    return Some(match c {
                        'a' => Key::A,
                        'b' => Key::B,
                        'c' => Key::C,
                        'd' => Key::D,
                        'e' => Key::E,
                        'f' => Key::F,
                        'g' => Key::G,
                        'h' => Key::H,
                        'i' => Key::I,
                        'j' => Key::J,
                        'k' => Key::K,
                        'l' => Key::L,
                        'm' => Key::M,
                        'n' => Key::N,
                        'o' => Key::O,
                        'p' => Key::P,
                        'q' => Key::Q,
                        'r' => Key::R,
                        's' => Key::S,
                        't' => Key::T,
                        'u' => Key::U,
                        'v' => Key::V,
                        'w' => Key::W,
                        'x' => Key::X,
                        'y' => Key::Y,
                        'z' => Key::Z,
                        '0' => Key::Num0,
                        '1' => Key::Num1,
                        '2' => Key::Num2,
                        '3' => Key::Num3,
                        '4' => Key::Num4,
                        '5' => Key::Num5,
                        '6' => Key::Num6,
                        '7' => Key::Num7,
                        '8' => Key::Num8,
                        '9' => Key::Num9,
                        _ => return Some(Unicode(c)),
                    });
                }
                // fallback for Unix/macOS
                return Some(Unicode(c));
            } else {
                return Some(Unicode(c));
            }
        }

        // Support literal unicode like "char:é"
        s if s.starts_with("char:") => s[5..].chars().next().map(Unicode),

        // Support numeric raw values like "code:42"
        s if s.starts_with("code:") => s[5..].parse().ok().map(Other),

        _ => None,
    }
}
