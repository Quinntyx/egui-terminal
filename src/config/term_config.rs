use std::sync::Arc;

use egui::{Color32, Ui};

use wezterm_term::color::ColorPalette;

use crate::into::IntoWez;

#[derive(Debug, Default)]
pub struct Style {
    bg_color: Option<Color32>,
    fg_color: Option<Color32>,
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
            Some(c) => c,
            None => ui.style().visuals.text_color(),
        };

        let bg = match self.bg_color {
            Some(c) => c, 
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
