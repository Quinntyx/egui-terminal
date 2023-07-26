// Special thanks to Speak2Erase, again; This file is almost entirely copied from her code. Go
// check out Luminol on Github, it's a cool project :) 

use crate::error::conversion::TermConversionError;

pub trait IntoEgui<T> {
    fn into_egui(self) -> T;
}

impl IntoEgui<egui::Color32> for wezterm_term::color::SrgbaTuple {
    fn into_egui(self) -> egui::Color32 {
        let (r, g, b, a) = self.to_srgb_u8();
        egui::Color32::from_rgba_unmultiplied(r, g, b, a)
    }
}

pub trait IntoWez<T> {
    fn into_wez(self) -> T;
}

pub trait TryIntoWez<T>
where
    Self: Sized,
{
    fn try_into_wez(self) -> Result<T, TermConversionError>;
}

impl TryIntoWez<wezterm_term::KeyCode> for egui::Key {
    fn try_into_wez(self) -> Result<wezterm_term::KeyCode, TermConversionError> {
        Ok(match self {
            egui::Key::ArrowDown => wezterm_term::KeyCode::DownArrow,
            egui::Key::ArrowLeft => wezterm_term::KeyCode::LeftArrow,
            egui::Key::ArrowRight => wezterm_term::KeyCode::RightArrow,
            egui::Key::ArrowUp => wezterm_term::KeyCode::UpArrow,
            egui::Key::Escape => wezterm_term::KeyCode::Escape,
            egui::Key::Tab => wezterm_term::KeyCode::Tab,
            egui::Key::Backspace => wezterm_term::KeyCode::Backspace,
            egui::Key::Enter => wezterm_term::KeyCode::Enter,
            egui::Key::Insert => wezterm_term::KeyCode::Insert,
            egui::Key::Delete => wezterm_term::KeyCode::Delete,
            egui::Key::Home => wezterm_term::KeyCode::Home,
            egui::Key::End => wezterm_term::KeyCode::End,
            egui::Key::PageUp => wezterm_term::KeyCode::PageUp,
            egui::Key::PageDown => wezterm_term::KeyCode::PageDown,
            egui::Key::Num0 => wezterm_term::KeyCode::Numpad0,
            egui::Key::Num1 => wezterm_term::KeyCode::Numpad1,
            egui::Key::Num2 => wezterm_term::KeyCode::Numpad2,
            egui::Key::Num3 => wezterm_term::KeyCode::Numpad3,
            egui::Key::Num4 => wezterm_term::KeyCode::Numpad4,
            egui::Key::Num5 => wezterm_term::KeyCode::Numpad5,
            egui::Key::Num6 => wezterm_term::KeyCode::Numpad6,
            egui::Key::Num7 => wezterm_term::KeyCode::Numpad7,
            egui::Key::Num8 => wezterm_term::KeyCode::Numpad8,
            egui::Key::Num9 => wezterm_term::KeyCode::Numpad9,
            egui::Key::F1 => wezterm_term::KeyCode::Function(1),
            egui::Key::F2 => wezterm_term::KeyCode::Function(2),
            egui::Key::F3 => wezterm_term::KeyCode::Function(3),
            egui::Key::F4 => wezterm_term::KeyCode::Function(4),
            egui::Key::F5 => wezterm_term::KeyCode::Function(5),
            egui::Key::F6 => wezterm_term::KeyCode::Function(6),
            egui::Key::F7 => wezterm_term::KeyCode::Function(7),
            egui::Key::F8 => wezterm_term::KeyCode::Function(8),
            egui::Key::F9 => wezterm_term::KeyCode::Function(9),
            egui::Key::F10 => wezterm_term::KeyCode::Function(10),
            egui::Key::F11 => wezterm_term::KeyCode::Function(11),
            egui::Key::F12 => wezterm_term::KeyCode::Function(12),
            egui::Key::F13 => wezterm_term::KeyCode::Function(13),
            egui::Key::F14 => wezterm_term::KeyCode::Function(14),
            egui::Key::F15 => wezterm_term::KeyCode::Function(15),
            egui::Key::F16 => wezterm_term::KeyCode::Function(16),
            egui::Key::F17 => wezterm_term::KeyCode::Function(17),
            egui::Key::F18 => wezterm_term::KeyCode::Function(18),
            egui::Key::F19 => wezterm_term::KeyCode::Function(19),
            egui::Key::F20 => wezterm_term::KeyCode::Function(20),
            _ => return Err(TermConversionError),
        })
    }
}

impl IntoWez<wezterm_term::KeyModifiers> for egui::Modifiers {
    fn into_wez(self) -> wezterm_term::KeyModifiers {
        let mut keymod = wezterm_term::KeyModifiers::NONE;
        keymod.set(wezterm_term::KeyModifiers::ALT, self.alt);
        keymod.set(wezterm_term::KeyModifiers::CTRL, self.ctrl);
        keymod.set(wezterm_term::KeyModifiers::SHIFT, self.shift);
        //keymod.set(wezterm_term::KeyModifiers::SUPER, self.command);

        keymod
    }
}

impl IntoWez<wezterm_term::MouseButton> for egui::PointerButton {
    fn into_wez(self) -> wezterm_term::MouseButton {
        match self {
            egui::PointerButton::Primary => wezterm_term::MouseButton::Left,
            egui::PointerButton::Secondary => wezterm_term::MouseButton::Right,
            egui::PointerButton::Middle => wezterm_term::MouseButton::Middle,
            _ => wezterm_term::MouseButton::None,
        }
    }
}
