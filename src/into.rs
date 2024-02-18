// Special thanks to Speak2Erase, again; This file is almost entirely copied from her code. Go
// check out Luminol on Github, it's a cool project :) 

use crate::error::conversion::TermConversionError;

use wezterm_term::{KeyModifiers, MouseButton};
use wezterm_term::KeyCode as WezKey;
use wezterm_term::color::SrgbaTuple;
use egui::{Color32, Modifiers, PointerButton};
use egui::Key::{*, self};

pub trait IntoEgui<T> {
    fn into_egui(self) -> T;
}

impl IntoEgui<Color32> for SrgbaTuple {
    fn into_egui(self) -> egui::Color32 {
        let (r, g, b, a) = self.to_srgb_u8();
        Color32::from_rgba_unmultiplied(r, g, b, a)
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

impl TryIntoWez<WezKey> for Key {
    fn try_into_wez(self) -> Result<WezKey, TermConversionError> {
        Ok(match self {
            ArrowDown => WezKey::DownArrow,
            ArrowLeft => WezKey::LeftArrow,
            ArrowRight => WezKey::RightArrow,
            ArrowUp => WezKey::UpArrow,
            Escape => WezKey::Escape,
            Tab => WezKey::Tab,
            Backspace => WezKey::Backspace,
            Enter => WezKey::Enter,
            Insert => WezKey::Insert,
            Delete => WezKey::Delete,
            Home => WezKey::Home,
            End => WezKey::End,
            PageUp => WezKey::PageUp,
            PageDown => WezKey::PageDown,
            Num0 => WezKey::Numpad0,
            Num1 => WezKey::Numpad1,
            Num2 => WezKey::Numpad2,
            Num3 => WezKey::Numpad3,
            Num4 => WezKey::Numpad4,
            Num5 => WezKey::Numpad5,
            Num6 => WezKey::Numpad6,
            Num7 => WezKey::Numpad7,
            Num8 => WezKey::Numpad8,
            Num9 => WezKey::Numpad9,
            F1 => WezKey::Function(1),
            F2 => WezKey::Function(2),
            F3 => WezKey::Function(3),
            F4 => WezKey::Function(4),
            F5 => WezKey::Function(5),
            F6 => WezKey::Function(6),
            F7 => WezKey::Function(7),
            F8 => WezKey::Function(8),
            F9 => WezKey::Function(9),
            F10 => WezKey::Function(10),
            F11 => WezKey::Function(11),
            F12 => WezKey::Function(12),
            F13 => WezKey::Function(13),
            F14 => WezKey::Function(14),
            F15 => WezKey::Function(15),
            F16 => WezKey::Function(16),
            F17 => WezKey::Function(17),
            F18 => WezKey::Function(18),
            F19 => WezKey::Function(19),
            F20 => WezKey::Function(20),
            A => WezKey::Char('a'),
            B => WezKey::Char('b'),
            C => WezKey::Char('c'),
            D => WezKey::Char('d'),
            E => WezKey::Char('e'),
            F => WezKey::Char('f'),
            G => WezKey::Char('g'),
            H => WezKey::Char('h'),
            I => WezKey::Char('i'),
            J => WezKey::Char('j'),
            K => WezKey::Char('k'),
            L => WezKey::Char('l'),
            M => WezKey::Char('m'),
            N => WezKey::Char('n'),
            O => WezKey::Char('o'),
            P => WezKey::Char('p'),
            Q => WezKey::Char('q'),
            R => WezKey::Char('r'),
            S => WezKey::Char('s'),
            T => WezKey::Char('t'),
            U => WezKey::Char('u'),
            V => WezKey::Char('v'),
            W => WezKey::Char('w'),
            X => WezKey::Char('x'),
            Y => WezKey::Char('y'),
            Z => WezKey::Char('z'),
            Space => WezKey::Char(' '),
            _ => return Err(TermConversionError),
        })
    }
}

impl IntoWez<KeyModifiers> for Modifiers {
    fn into_wez(self) -> wezterm_term::KeyModifiers {
        let mut keymod = wezterm_term::KeyModifiers::NONE;
        keymod.set(wezterm_term::KeyModifiers::ALT, self.alt);
        keymod.set(wezterm_term::KeyModifiers::CTRL, self.ctrl);
        keymod.set(wezterm_term::KeyModifiers::SHIFT, self.shift);
        //keymod.set(wezterm_term::KeyModifiers::SUPER, self.command);

        keymod
    }
}

impl IntoWez<MouseButton> for PointerButton {
    fn into_wez(self) -> wezterm_term::MouseButton {
        match self {
            PointerButton::Primary => MouseButton::Left,
            PointerButton::Secondary => MouseButton::Right,
            PointerButton::Middle => MouseButton::Middle,
            _ => MouseButton::None,
        }
    }
}

impl IntoWez<wezterm_term::color::SrgbaTuple> for egui::Color32 {
    fn into_wez (self) -> wezterm_term::color::SrgbaTuple {
        let (r, g, b, a) = self.to_tuple();

        // @todo figure out whether this is right
        SrgbaTuple(r as f32 / 255., g as f32 / 255., b as f32 / 255., a as f32 / 255.) 
    }
}
