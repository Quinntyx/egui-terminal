use egui::{Ui, Vec2, Response};

pub use crate::term::TermHandler;
pub use crate::Terminal;

pub trait TerminalSpawner {
    fn terminal (&mut self, term: &mut TermHandler) -> Response;
    fn terminal_sized (&mut self, term: &mut TermHandler, size: Vec2) -> Response;
}

impl TerminalSpawner for Ui {
    fn terminal (&mut self, term: &mut TermHandler) -> Response {
        self.add(Terminal::new(term))
    }

    fn terminal_sized (&mut self, term: &mut TermHandler, size: Vec2) -> Response {
        self.add(Terminal::new(term).with_size(size))
    }
}
