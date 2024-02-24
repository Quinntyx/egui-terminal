// Special thanks to Speak2Erase for the code used as reference for this implementation (and for
// some code taken) :)
//
// You can find her on Github, she does good work; This project is based on luminol-term, from the
// Luminol project, at https://github.com/Astrabit-ST/Luminol. Go check it out!

use std::fmt::Debug;
use std::io::prelude::*;
use std::sync::Arc;
use std::ops::Range;
use std::ffi::OsString;

pub use portable_pty::CommandBuilder;
pub use termwiz::Error;

use crossbeam_channel::{unbounded, Receiver};
use wezterm_term::{Terminal as WezTerm, TerminalConfiguration, TerminalSize};
use termwiz::cellcluster::CellCluster;
use portable_pty::PtySize;

use egui::{Color32, Event, FontId, InputState, Modifiers, Response, TextFormat, Ui, Vec2};

use crate::into::*;
use crate::config::definitions::TermResult;
use crate::config::term_config::{Config, Style};
use crate::render::{CursorRenderer, CursorType};

pub struct TermHandler {
    terminal: WezTerm,
    reader: Receiver<Vec<termwiz::escape::Action>>,
    style: Style,
    wez_config: Arc<Config>,
    consume_tab: bool,
    consume_escape: bool,
    was_focused: bool,
    cursor_renderer: CursorRenderer,

    child: Box<dyn portable_pty::Child + Send + Sync>,
    pair: portable_pty::PtyPair,
    text_width: f32,
    text_height: f32,
    size: TerminalSize,
    system: sysinfo::System,
}

impl Debug for TermHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TermHandler")
    }
}

impl Drop for TermHandler {
    fn drop(&mut self) {
        self.kill();
    }
}

impl TermHandler {
    pub fn new (command: CommandBuilder) -> Self {
        Self::try_new(command).expect("should be able to create terminal")
    }

    pub fn new_from_str (command: &str) -> Self {
        Self::try_new_from_str(command).expect("should be able to create terminal")
    }

    pub fn try_new_from_str (command: &str) -> Result<Self, termwiz::Error> {
        Self::try_new(CommandBuilder::new(command))
    }

    pub fn try_new (command: CommandBuilder) -> Result<Self, termwiz::Error> {
        let pty_system = portable_pty::native_pty_system();
        let pair = pty_system.openpty(portable_pty::PtySize::default())?;
        let child = pair.slave.spawn_command(command.clone())?;

        let reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        let style = Style::default();

        let wez_config = style.default_wez_config();

        let terminal = WezTerm::new(
            TerminalSize::default(),
            wez_config.clone(),
            command.get_argv().join(OsString::from(" ").as_os_str()).into_string().expect("should be able to convert command to String").as_ref(),
            "1.0",
            writer,
        );

        let (sender, reciever) = unbounded();
        std::thread::spawn(move || {
            let mut buf = [0; 2usize.pow(10)];
            let mut reader = std::io::BufReader::new(reader);
            let mut parser = termwiz::escape::parser::Parser::new();

            loop {
                let Ok(len) = reader.read(&mut buf) else {
                    return
                };
                let actions = parser.parse_as_vec(&buf[0..len]);
                let Ok(_) = sender.send(actions) else {
                    return
                };
            }
        });
        
        Ok(Self {
            terminal,
            style,
            wez_config,
            consume_escape: true,
            consume_tab: true,
            was_focused: false,
            // cursor_trail_rect: Rect::from_points(&[pos2(0., 0.)]),
            cursor_renderer: CursorRenderer::new(),
            reader: reciever,
            child,
            pair,
            text_width: 0.0,
            text_height: 0.0,
            size: TerminalSize::default(),
            system: sysinfo::System::new(),
        })
    }

    pub fn title (&mut self, title: &str) -> String {
        if self.exit_status().is_some() { return String::from("") }
        if let Some(pid) = self.child.process_id() {
            let pid = sysinfo::Pid::from_u32(pid);
            if dbg!(self.system.refresh_process(pid)) {
                self.system.process(pid)
                    .map(|p| p.name())
                    .unwrap_or(title)
            } else {
                title
            }
        } else {
            title
        }.to_owned()
    }

    pub fn id (&self) -> egui::Id {
        if let Some(id) = self.child.process_id() {
            egui::Id::new(id)
        } else {
            egui::Id::new(self.terminal.get_title())
        }
    }

    fn format_to_egui (&self, cluster: &CellCluster) -> TextFormat {
        let palette = self.terminal.get_config().color_palette();

        let fg_color = palette.resolve_fg(cluster.attrs.foreground()).into_egui();
        let bg_color = palette.resolve_bg(cluster.attrs.background()).into_egui();
        let underline = if !matches!(
            cluster.attrs.underline(),
            wezterm_term::Underline::None
        ) {
            egui::Stroke::new(
                1.0,
                palette
                    .resolve_fg(cluster.attrs.underline_color())
                    .into_egui(),
            )
        } else {
            egui::Stroke::NONE
        };
        let strikethrough = if cluster.attrs.strikethrough() {
            egui::Stroke::new(
                1.0,
                palette.resolve_fg(cluster.attrs.foreground()).into_egui(),
            )
        } else {
            egui::Stroke::NONE
        };
        
        egui::TextFormat {
            font_id: egui::FontId::monospace(12.0),
            color: fg_color,
            background: bg_color,
            italics: cluster.attrs.italic(),
            underline,
            strikethrough,
            ..Default::default()
        }
    }

    fn event_pointer_move (&mut self, e: &Event, response: &Response, modifiers: Modifiers) -> TermResult {
        let Event::PointerMoved(pos) = e else { unreachable!() };
        let relative_pos = *pos - response.rect.min;
        let char_x = (relative_pos.x / 12.0) as usize;
        let char_y = (relative_pos.y / 12.0) as i64;
        self.terminal.mouse_event(wezterm_term::MouseEvent {
            kind: wezterm_term::MouseEventKind::Move,
            x: char_x,
            y: char_y,
            x_pixel_offset: 0,
            y_pixel_offset: 0,
            button: wezterm_term::MouseButton::None,
            modifiers: modifiers.into_wez(),
        })?;

        Ok(())
    }

    fn event_pointer_button (&mut self, e: &Event, response: &Response) -> TermResult {
        let Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers,
        } = e else { unreachable!() };

        let relative_pos = *pos - response.rect.min;
        let char_x = (relative_pos.x / self.text_width) as usize;
        let char_y = (relative_pos.y / self.text_height) as i64;
        self.terminal.mouse_event(wezterm_term::MouseEvent {
            kind: if *pressed {
                wezterm_term::MouseEventKind::Press
            } else {
                wezterm_term::MouseEventKind::Release
            },
            x: char_x,
            y: char_y,
            x_pixel_offset: 0,
            y_pixel_offset: 0,
            button: button.into_wez(),
            modifiers: modifiers.into_wez(),
        })?;

        Ok(())
    }

    fn event_scroll (&mut self, e: &Event, modifiers: Modifiers, pointer_position: Vec2) -> TermResult {
        let Event::Scroll(pos) = e else { unreachable!() };
        let char_x = (pointer_position.x / self.text_width) as usize;
        let char_y = (pointer_position.y / self.text_height) as i64;
        self.terminal.mouse_event(wezterm_term::MouseEvent {
            kind: wezterm_term::MouseEventKind::Press,
            x: char_x,
            y: char_y,
            x_pixel_offset: 0,
            y_pixel_offset: 0,
            button: if pos.y.is_sign_positive() {
                wezterm_term::MouseButton::WheelUp(pos.y as usize)
            } else {
                wezterm_term::MouseButton::WheelDown(-pos.y as usize)
            },
            modifiers: modifiers.into_wez(),
        })?;

        Ok(())
    }

    fn event_key (&mut self, e: &Event) -> TermResult {
        let Event::Key {
            key,
            modifiers,
            pressed,
            ..
        } = e else { unreachable!() };

        if let Ok(key) = key.try_into_wez() {
            if *pressed {
                self.terminal.key_down(key, modifiers.into_wez())?;
            } else {
                self.terminal.key_up(key, modifiers.into_wez())?;
            }
        } else {
            // dbg!(e); // @todo figure out why this prints almost every keypress (update it's supposed to do that)
        }

        Ok(())
    }

    #[allow(unused_variables)]
    fn event_text (&mut self, e: &Event, modifiers: Modifiers) -> TermResult {
        let Event::Text(t) = e else { unreachable!() };

        if t == "exit" {
            panic!("it fucking doesnt exist anymore what do you want from me");
        }

        t.chars()
            .filter(|c| !"abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(*c))
            .try_for_each(
                |c| {
                    self.terminal.key_down(
                        wezterm_term::KeyCode::Char(c),
                        modifiers.into_wez(),
                    )
                }
            ).and_then(
                |_| {
                    t.chars().try_for_each(
                        |c| {
                            self.terminal.key_up(
                                wezterm_term::KeyCode::Char(c),
                                modifiers.into_wez(),
                            )
                        }
                    )
                }
        )?;

        Ok(())
    }

    fn relative_pointer_pos (&self, response: &Response, i: &InputState) -> Vec2 {
        i.pointer.interact_pos().unwrap() - response.rect.min
    }

    fn manage_event (&mut self, event: &Event, response: &Response, i: &InputState) -> Result<(), Error> {
        match event {
            Event::PointerMoved(_) => self.event_pointer_move(event, response, i.modifiers),
            Event::PointerButton {..} => self.event_pointer_button(event, response),
            Event::Scroll(_) => self.event_scroll(event, i.modifiers, self.relative_pointer_pos(response, i)),
            Event::Key {..} => self.event_key(event),
            Event::Text(_) => self.event_text(event, i.modifiers),
            _ => Ok(()),
        }
    }

    fn manage_inputs (&mut self, i: &InputState, response: &Response) -> Vec<Result<(), Error>> {
        i.events.iter()
            .map(|event| self.manage_event(event, response, i))
            .collect()
    }

    fn generate_rows (&mut self, ui: &mut Ui, rows: Range<usize>) -> Response {
        let palette = self.terminal.get_config().color_palette();
        let size = self.size;

        let mono_format = TextFormat {
            font_id: FontId::monospace(12.0),
            ..Default::default()
        };

        let mut job = egui::text::LayoutJob::default();

        let mut iter = self.terminal
            .screen()
            .lines_in_phys_range(rows.clone())
            .into_iter()
            .peekable();

        while let Some(line) = iter.next() {
            line.cluster(None).iter()
                .for_each(|c| {
                    job.append(
                        &c.text,
                        0.0,
                        self.format_to_egui(c)
                    )
                });
            if iter.peek().is_some() {
                job.append(
                    "\n",
                    0.0,
                    mono_format.clone(),
                );
            }
        }

        let galley = ui.fonts(|f| f.layout_job(job));
        let mut galley_rect = galley.rect;
        galley_rect.set_width(self.text_width * size.cols as f32);

        let (response, painter) = ui.allocate_painter(galley_rect.size(), egui::Sense::click_and_drag());

        if response.clicked() && !response.has_focus() {
            ui.memory_mut(|mem| mem.request_focus(response.id));
        }

        painter.rect_filled(
            galley_rect.translate(response.rect.min.to_vec2()),
            0.0,
            palette.background.into_egui(),
        );

        painter.galley(response.rect.min, galley, Color32::DEBUG_COLOR); // herepoop

        // if ui.memory(|mem| mem.has_focus(response.id)) {

        ui.output_mut(|o| o.mutable_text_under_cursor = true);
        ui.ctx().set_cursor_icon(egui::CursorIcon::Text);
        // ui.memory_mut(|mem| mem.lock_focus(response.id, true));

        if response.has_focus() {
            self.was_focused = true;
            ui.input_mut(|i| {
                self.manage_inputs(i, &response).iter()
                    .for_each(|res| {
                        let Err(e) = res else { return };
                        eprintln!("terminal input error {e:?}");
                    });
            });
        } else {
            self.was_focused = false;
        }

        response
    }

    pub(crate) fn draw (&mut self, ui: &mut egui::Ui, widget_size: egui::Vec2) -> std::io::Result<Response> {
        while let Ok(actions) = self.reader.try_recv() {
            self.terminal.perform_actions(actions);
        }

        if self.was_focused {
            self.cursor_renderer.cursor_type = CursorType::Block(Color32::WHITE);
        } else {
            self.cursor_renderer.cursor_type = CursorType::OpenBlock(self.wez_config.color_palette().background.into_egui())
        }

        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

        self.text_width = ui.fonts(|f| f.glyph_width(&egui::FontId::monospace(12.0), '?')).round();
        self.text_height = ui.text_style_height(&egui::TextStyle::Monospace);

        self.size.cols = (widget_size.x / self.text_width) as usize;
        self.size.rows = (widget_size.y / self.text_height) as usize;

        self.resize_rc();
        self.config(ui);

        self.cursor_renderer.set_offset(ui.next_widget_position().to_vec2());

        let mut esc = false;
        let mut tab = false;

        if self.was_focused {
            if self.consume_escape {
                esc = ui.input_mut(|i| i.consume_key(Modifiers::NONE, egui::Key::Escape));
            }
            if self.consume_tab {
                tab = ui.input_mut(|i| i.consume_key(Modifiers::NONE, egui::Key::Tab));
            }
        }

        {
            use wezterm_term::KeyCode::Escape;
            use wezterm_term::KeyCode::Tab;
            use wezterm_term::KeyModifiers as Mod;
            if esc {
                self.terminal.key_down(Escape, Mod::NONE).unwrap();
            } else {
                self.terminal.key_up(Escape, Mod::NONE).unwrap();
            }

            if tab {
                self.terminal.key_down(Tab, Mod::NONE).unwrap();
            } else {
                self.terminal.key_up(Tab, Mod::NONE).unwrap();
            }
        }

        let r = egui::ScrollArea::vertical()
            .max_height((self.size.rows + 1) as f32 * self.text_height)
            .stick_to_bottom(true)
            .id_source(ui.next_auto_id())
            .show_rows(
                ui,
                self.text_height,
                self.terminal.screen().scrollback_rows(),
                |ui, rows| {
                    self.generate_rows(ui, rows)
                }
            ).inner;

        self.cursor_renderer.update_cursor_rect(self.terminal.cursor_pos(), self.text_width, self.text_height);
        self.cursor_renderer.draw_cursor(ui.painter_at(r.rect), ui.input(|i| i.stable_dt.min(0.1)));
        self.cursor_renderer.update_cursor_trail(0.2);

        ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));

        Ok(r)
    }

    #[inline(never)]
    fn kill(&mut self) {
        if let Err(e) = self.child.kill() {
            eprintln!("error killing child: {e}");
        }
    }

    fn resize_rc (&mut self) {
        if self.terminal.get_size() == self.size { return; }

        self.terminal.resize(self.size);
        let r = self.pair.master.resize(PtySize {
            rows: self.size.rows as u16,
            cols: self.size.cols as u16,
            ..Default::default()
        });

        if let Err(e) = r {
            eprintln!("error resizing terminal: {e}");
        }
    }

    fn config (&mut self, ui: &Ui) {
        if *self.wez_config == *self.style.generate_wez_config(ui) { return; }
        self.wez_config = self.style.generate_wez_config(ui).clone();
        self.terminal.set_config(self.wez_config.clone());
    }

    pub fn exit_status (&mut self) -> Option<u32> {
        self.child.try_wait().expect("it shouldnt crash here (seriously if this comes up im just confused)")
            .map(|c| c.exit_code())
    }
}

