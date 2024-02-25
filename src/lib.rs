mod into;
pub mod error;
pub mod config;
pub mod term;
pub mod prelude;
pub mod render;

use egui::{Widget, Ui, Response, Vec2};

pub use crate::config::definitions::TermResult;
pub use crate::term::TermHandler;
pub use crate::config::term_config::{Style, Config};


pub struct Terminal<'a> {
    terminal: &'a mut TermHandler,
    size: Option<Vec2>,
}

impl Widget for Terminal<'_> {
    fn ui (self, ui: &mut Ui) -> Response {
        let size = match self.size {
            Some(s) => s,
            None => ui.available_size(),
        };
        self.terminal.draw(ui, size).expect("terminal should not error")
    }
}

impl<'a> Terminal<'a> {
    pub fn new (terminal: &'a mut TermHandler) -> Self {
        Self {
            terminal,
            size: None,
        }
    }

    pub fn with_size (mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn exit_status (&mut self) -> Option<u32> {
        self.terminal.exit_status()
    }
}

