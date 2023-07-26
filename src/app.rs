use eframe::{egui, CreationContext};

use egui_terminal::{Terminal, TermHandler};

use egui::vec2;

pub struct App {
    term: TermHandler,
}
impl App {
    pub fn new () -> Self {
        Self {
            term: TermHandler::new_from_str("zsh"),
        }
    }

    pub fn setup (_cc: &CreationContext) -> Box<dyn eframe::App> {
        Box::new(App::new())
    }
}

impl eframe::App for App {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Terminal::new(&mut self.term, vec2(1600., 1000.)))
        });
    }
}
