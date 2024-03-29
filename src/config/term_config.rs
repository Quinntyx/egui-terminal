use std::sync::Arc;

use egui::ecolor::HexColor;
use egui::{Color32, FontId, Stroke, Ui};

use wezterm_term::color::ColorPalette;

use crate::{into::IntoWez, render::CursorType};

/// please make the font monospace or everything breaks :D
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Style {
    pub bg_color: Option<HexColor>,
    pub fg_color: Option<HexColor>,
    pub cursor_trail: bool,
    pub cursor_trail_color: Option<HexColor>,
    pub font: FontId,
    pub default_focus_cursor: CursorType,
    pub default_unfocus_cursor: CursorType,
    pub cursor_stroke: Stroke,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            bg_color: None,
            fg_color: None,
            cursor_trail: true,
            cursor_trail_color: None,
            font: FontId::monospace(12.),
            default_focus_cursor: CursorType::Block(HexColor::Hex8(Color32::WHITE)),
            default_unfocus_cursor: CursorType::OpenBlock(HexColor::Hex8(Color32::WHITE)),
            cursor_stroke: Stroke::new(1., Color32::WHITE),
        }
    }
}

impl Style {
    pub(crate) fn default_wez_config (&self) -> Arc<Config> {
        let res = Config {
            bg: Color32::BLACK,
            fg: Color32::WHITE,
            title_reporting: true,
        };

        Arc::new(res)
    }

    pub(crate) fn generate_wez_config (&self, ui: &Ui) -> Arc<Config> {
        let fg = match self.fg_color {
            Some(c) => c.color(),
            None => ui.style().visuals.text_color(),
        };

        let bg = match self.bg_color {
            Some(c) => c.color(), 
            None => ui.style().visuals.window_fill,
        };

        let res = Config {
            fg,
            bg,
            title_reporting: true,
        };

        Arc::new(res)
    }
}


        

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    bg: Color32, 
    fg: Color32, 
    title_reporting: bool,
}

impl wezterm_term::TerminalConfiguration for Config {
    fn color_palette(&self) -> wezterm_term::color::ColorPalette {
        ColorPalette {
            foreground: self.fg.into_wez(),
            background: self.bg.into_wez(),
            ..Default::default()
        }
    }

    fn enable_title_reporting(&self) -> bool {
        self.title_reporting
    }
}
