use std::collections::HashMap;
use std::env::args;

use eframe::{egui, CreationContext};
use egui_terminal::{Terminal, TermHandler};
use egui::vec2;

pub struct App {
    terminals: HashMap<String, TermHandler>
}
impl App {
    pub fn new () -> Self {
        let mut map = HashMap::new();
        let mut args = args();

        args.next();
        let cmd = match args.next() {
            Some(c) => c,
            None => String::from("bash"),
        };

        map.insert(String::from("root"), TermHandler::new_from_str(&cmd));
        map.insert(String::from("root2"), TermHandler::new_from_str(&cmd));
        map.insert(String::from("root3"), TermHandler::new_from_str(&cmd));

        Self {
            terminals: map
        }
    }

    pub fn setup (_cc: &CreationContext) -> Box<dyn eframe::App> {
        Box::new(App::new())
    }
}

impl eframe::App for App {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for (idx, (_id, term)) in self.terminals.iter_mut().enumerate() {
                ui.add(
                    Terminal::new(term)
                        .with_size(
                            vec2(
                                1400. + 200. * idx as f32,
                                300. + 100. * idx as f32
                            )
                        )
                );
            }
        });
    }
}
