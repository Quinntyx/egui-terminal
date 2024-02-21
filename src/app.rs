use std::collections::HashMap;
use std::env::args;

use eframe::{egui, CreationContext};
use egui_terminal::prelude::*;


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
            None => String::from("zsh"),
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
            ui.label(format!(
                "Terminals Closed: {}, {}, {}", 
                self.terminals.get("root").unwrap().is_closed(),
                self.terminals.get("root2").unwrap().is_closed(),
                self.terminals.get("root3").unwrap().is_closed(),
            ));

            let ht = ui.available_height() / 3.;
            for (_idx, (_id, term)) in self.terminals.iter_mut().enumerate() {
                ui.terminal_sized(
                    term,
                    egui::vec2(
                        ui.available_width(),
                        ht,
                    )
                );
            }
        });
    }
}
