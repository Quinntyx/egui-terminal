// Special thanks to Speak2Erase for the code used as reference for this implementation (and for
// some code taken) :)
//
// You can find her on Github, she does good work

mod into;
pub mod error;
pub mod config;
pub mod term;


use egui::{Widget, Ui, Response, Vec2};

pub use crate::config::definitions::TermResult;
pub use crate::term::TermHandler;


pub struct Terminal<'a> {
    terminal: &'a mut TermHandler,
    size: Vec2,
}

impl Widget for Terminal<'_> {
    fn ui (self, ui: &mut Ui) -> Response {
        self.terminal.draw(ui, self.size).expect("terminal should not error")
    }
}

impl<'a> Terminal<'a> {
    pub fn new (terminal: &'a mut TermHandler, size: Vec2) -> Self {
        Self {
            terminal,
            size,
        }
    }
}

