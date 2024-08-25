use std::collections::HashMap;
use std::env::args;

use eframe::{egui, CreationContext};
use egui::Color32;
use egui::FontId;
use egui::Stroke;
use egui_terminal::render::CursorType;
use egui_terminal::prelude::*;

use ecolor::HexColor;


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

    pub fn setup<E> (_cc: &CreationContext) -> Result<Box<dyn eframe::App>, E> {
        Ok(Box::new(App::new()))
    }
}

impl eframe::App for App {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        {
            let s1 = self.terminals.get_mut("root").unwrap().style_mut();
            s1.cursor_trail = true;
            s1.cursor_trail_color = Some(HexColor::Hex8(Color32::LIGHT_BLUE.gamma_multiply(0.5)));
            s1.default_focus_cursor = CursorType::OpenBlock(HexColor::Hex8(Color32::RED));
            s1.default_unfocus_cursor = CursorType::None;
            s1.cursor_stroke = Stroke::new(1., Color32::WHITE);
            s1.font = FontId::monospace(6.);
        }
        {
            let s2 = self.terminals.get_mut("root2").unwrap().style_mut();
            s2.cursor_trail = true;
            s2.cursor_trail_color = Some(HexColor::Hex8(Color32::RED.gamma_multiply(0.5)));
            s2.default_focus_cursor = CursorType::Beam(HexColor::Hex8(Color32::RED));
            s2.default_unfocus_cursor = CursorType::Block(HexColor::Hex8(Color32::GREEN));
            s2.cursor_stroke = Stroke::new(2., Color32::YELLOW);
            s2.font = FontId::proportional(12.);
        }
        {
            let s3 = self.terminals.get_mut("root3").unwrap().style_mut();
            s3.cursor_trail = true;
            s3.cursor_trail_color = None;
            s3.default_focus_cursor = CursorType::Beam(HexColor::Hex8(Color32::WHITE));
            s3.default_unfocus_cursor = CursorType::None;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Terminals Closed: {:?}, {:?}, {:?}", 
                self.terminals.get_mut("root").unwrap().exit_status(),
                self.terminals.get_mut("root2").unwrap().exit_status(),
                self.terminals.get_mut("root3").unwrap().exit_status(),
            ));

            ui.label(format!(
                "Terminal Titles: {}, {}, {}", 
                self.terminals.get_mut("root").unwrap().title("test"),
                self.terminals.get_mut("root2").unwrap().title("test"),
                self.terminals.get_mut("root3").unwrap().title("test"),
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
