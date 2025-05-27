//! Convert the Helix `KeyCode`s into something that [`vhs`](vhs) can understand
//!
//! [vhs]: https://github.com/charmbracelet/vhs

use std::fmt::Display;

use crate::{
    helix_config,
    helix_parse_keys::{KeyCode, KeyEvent, KeyModifiers, MediaKeyCode, ModifierKeyCode},
};

impl Display for KeyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mods = format!(
            "{}{}{}{}",
            if self.modifiers.contains(KeyModifiers::SUPER) {
                "Meta+"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::SHIFT) {
                "Shift+"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::ALT) {
                "Alt+"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::CONTROL) {
                "Ctrl+"
            } else {
                ""
            },
        );
        let out = match self.code {
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::F(i) => format!("F{i}"),
            KeyCode::Char(ch) => {
                if mods.is_empty() {
                    let ch = if ch == '"' {
                        // Double-quotes escaped with backtick
                        "`\"`".to_string()
                    } else {
                        format!("\"{ch}\"")
                    };
                    format!(r#"Type {ch}"#)
                } else if let Some(mods) = helix_config::remap(mods.as_str(), ch) {
                    mods
                } else {
                    format!(r#"{mods}"{ch}""#)
                }
            }
            KeyCode::Null => "Null".to_string(),
            KeyCode::Esc => "Escape".to_string(),
            KeyCode::CapsLock => "CapsLock".to_string(),
            KeyCode::ScrollLock => "ScrollLock".to_string(),
            KeyCode::NumLock => "NumLock".to_string(),
            KeyCode::PrintScreen => "PrintScreen".to_string(),
            KeyCode::Pause => "Pause".to_string(),
            KeyCode::Menu => "Menu".to_string(),
            KeyCode::KeypadBegin => "KeypadBegin".to_string(),
            KeyCode::Media(media_key_code) => match media_key_code {
                MediaKeyCode::Play => "Play",
                MediaKeyCode::Pause => "Pause",
                MediaKeyCode::PlayPause => "PlayPause",
                MediaKeyCode::Reverse => "Reverse",
                MediaKeyCode::Stop => "Stop",
                MediaKeyCode::FastForward => "FastForward",
                MediaKeyCode::Rewind => "Rewind",
                MediaKeyCode::TrackNext => "TrackNext",
                MediaKeyCode::TrackPrevious => "TrackPrevious",
                MediaKeyCode::Record => "Record",
                MediaKeyCode::LowerVolume => "LowerVolume",
                MediaKeyCode::RaiseVolume => "RaiseVolume",
                MediaKeyCode::MuteVolume => "MuteVolume",
            }
            .to_string(),
            KeyCode::Modifier(modifier_key_code) => match modifier_key_code {
                ModifierKeyCode::LeftShift => "LeftShift",
                ModifierKeyCode::LeftControl => "LeftControl",
                ModifierKeyCode::LeftAlt => "LeftAlt",
                ModifierKeyCode::LeftSuper => "LeftSuper",
                ModifierKeyCode::LeftHyper => "LeftHyper",
                ModifierKeyCode::LeftMeta => "LeftMeta",
                ModifierKeyCode::RightShift => "RightShift",
                ModifierKeyCode::RightControl => "RightControl",
                ModifierKeyCode::RightAlt => "RightAlt",
                ModifierKeyCode::RightSuper => "RightSuper",
                ModifierKeyCode::RightHyper => "RightHyper",
                ModifierKeyCode::RightMeta => "RightMeta",
                ModifierKeyCode::IsoLevel3Shift => "IsoLevel3Shift",
                ModifierKeyCode::IsoLevel5Shift => "IsoLevel5Shift",
            }
            .to_string(),
        };

        write!(f, "{out}")?;

        Ok(())
    }
}
