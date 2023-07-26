use eframe::{egui, CreationContext};

use egui::{Widget, Ui, Response, Vec2, InputState};

pub use termwiz::terminal::{Terminal, new_terminal};
pub use termwiz::caps::Capabilities;

pub struct App {
    term_content: String,
    term: Box<dyn Terminal>,
}

pub struct Term<'a, T: Terminal> {
    size: Vec2,
    buf: &'a mut T,
}

impl<'a, T: Terminal> Term<'a, T> {
    pub fn new (size: Vec2, buf: &'a mut T) -> Self {
        Self {
            size,
            buf,
        }
    }

    fn manage_inputs (&mut self, i: &InputState) {
        for event in i.events.iter() {
            let result = match event {
                
            }
        }
    }

    pub fn show (mut self, ui: &mut Ui) -> Response {
        let (rect, mut response) = ui.allocate_exact_size(self.size, egui::Sense::click_and_drag());

        if response.has_focus() {
            ui.input(|i| {
                self.manage_inputs(i)
            });
        }

        response
    }
}

impl<T: Terminal> Widget for Term<'_, T> {
    fn ui (mut self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}


impl App {
    pub fn new () -> Self {
        Self {
            term_content: String::from(""),
            term: Box::new(new_terminal(Capabilities::new_from_env().expect("should be able to create capabilities")).expect("should be able to create terminal"))
        }
    }

    pub fn setup (cc: &CreationContext) -> Box<dyn eframe::App> {
        Box::new(App::new())
    }
}

impl eframe::App for App {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.code_editor(&mut self.term_content)
        });
        
    }
}
