use egui::{pos2, vec2, Color32, Mesh, Painter, Pos2, Rect, Vec2};
use egui::epaint::Vertex;
use wezterm_term::CursorPosition;

fn pos2_to_vertex (a: Pos2, c: Color32) -> Vertex {
    let mut v = Vertex::default();
    v.pos = a;
    v.color = c;
    v
}

pub fn triangle (a: Pos2, b: Pos2, c: Pos2, color: Color32) -> Mesh {
    let mut vertices = vec!(a, b, c).iter_mut()
        .map(|v| pos2_to_vertex(*v, color))
        .collect::<Vec<_>>();

    let mut mesh = Mesh::default();
    mesh.vertices.append(&mut vertices);

    mesh.add_triangle(0, 1, 2);

    mesh
}

pub fn quad (a: Pos2, b: Pos2, c: Pos2, d: Pos2, color: Color32) -> Mesh {
    let mut m = Mesh::default();

    m.append(triangle(a, b, c, color));
    m.append(triangle(b, c, d, color));

    m
}

pub fn quad_trail (a: Rect, b: Rect, color: Color32) -> Mesh {
    let mut m = Mesh::default();

    m.append(quad(
        a.left_top(),
        a.right_top(),
        b.left_top(),
        b.right_top(),
        color
    ));

    m.append(quad(
        a.right_top(),
        a.right_bottom(),
        b.right_top(),
        b.right_bottom(),
        color
    ));

    m.append(quad(
        a.right_bottom(),
        a.left_bottom(),
        b.right_bottom(),
        b.left_bottom(),
        color
    ));

    m.append(quad(
        a.left_top(),
        a.left_bottom(),
        b.left_top(),
        b.left_bottom(),
        color
    ));

    m
}

pub trait SimpleMeshBuilder {
    fn add_triangle_simple (&mut self, a: Pos2, b: Pos2, c: Pos2, color: Color32);
    fn add_quad_simple (&mut self, a: Pos2, b: Pos2, c: Pos2, d: Pos2, color: Color32);
}

impl SimpleMeshBuilder for Mesh {
    fn add_triangle_simple (&mut self, a: Pos2, b: Pos2, c: Pos2, color: Color32) {
        self.append(triangle(a, b, c, color));
    }

    fn add_quad_simple (&mut self, a: Pos2, b: Pos2, c: Pos2, d: Pos2, color: Color32) {
        self.append(quad(a, b, c, d, color));
    }
}

pub enum CursorType {
    Block(Color32),
    Beam(Color32),
    OpenBlock(Color32),
    None
}

pub struct CursorRenderer {
    cursor_trail_source: Rect,
    pub draw_trail: bool,
    cursor_rect: Rect,
    pub cursor_type: CursorType,
    widget_offset: Vec2,
    stable_time_factor: f64,
}

impl CursorRenderer {
    pub fn new () -> Self {
        Self {
            cursor_trail_source: Rect::from_points(&[pos2(0., 0.)]),
            draw_trail: true,
            cursor_rect: Rect::from_points(&[pos2(0., 0.)]),
            cursor_type: CursorType::Block(Color32::TRANSPARENT),
            widget_offset: vec2(0., 0.),
            stable_time_factor: 0.,
        }
    }

    pub fn set_offset (&mut self, offset: Vec2) {
        self.widget_offset = offset;
    }

    pub fn update_cursor_rect (&mut self, ur: CursorPosition, text_width: f32, text_height: f32) {
        let cursor_rect = match &self.cursor_type {
            &CursorType::Block(_) => Rect::from_min_size(  
                egui::pos2(
                    (ur.x) as f32 * text_width + self.widget_offset.x+ 1.,
                    (ur.y) as f32 * text_height + self.widget_offset.y
                ),
                egui::vec2(text_width - 2., text_height),
            ),
            &CursorType::Beam(_) => Rect::from_min_size(
                egui::pos2(
                    (ur.x) as f32 * text_width + self.widget_offset.x+ 1.,
                    (ur.y) as f32 * text_height + self.widget_offset.y
                ),
                egui::vec2(text_width - 4., text_height),
            ),
            &CursorType::OpenBlock(_) => Rect::from_min_size(  
                egui::pos2(
                    (ur.x) as f32 * text_width + self.widget_offset.x+ 1.,
                    (ur.y) as f32 * text_height + self.widget_offset.y
                ),
                egui::vec2(text_width - 2., text_height),
            ),
            &CursorType::None => Rect::from_min_size(  
                egui::pos2(
                    (ur.x) as f32 * text_width + self.widget_offset.x+ 1.,
                    (ur.y) as f32 * text_height + self.widget_offset.y
                ),
                egui::vec2(text_width - 2., text_height),
            ),
        };

        self.cursor_rect = cursor_rect;
    }

    fn get_color (&self) -> Color32 {
        let (alpha, color) = match &self.cursor_type {
            &CursorType::Block(c) => (((self.stable_time_factor / std::f64::consts::FRAC_PI_2 * 13.).sin() + 1.) / 2., c),
            &CursorType::OpenBlock(c) => (0., c),
            &CursorType::Beam(c) => (1., c),
            &CursorType::None => (0., Color32::TRANSPARENT),
        };
        
        color.gamma_multiply(alpha as f32)
    }

    pub fn draw_cursor (&mut self, painter: Painter, delta_time: f32) {
        self.stable_time_factor += delta_time as f64;
        if matches!(self.cursor_type, CursorType::None) { return; }

        painter.rect(
            self.cursor_rect,
            egui::Rounding::same(1.),
            self.get_color(),
            egui::Stroke::new(1., Color32::WHITE),
        );
    
        if self.draw_trail {
            painter.add(quad_trail(
                self.cursor_rect,
                self.cursor_trail_source,
                Color32::from_rgba_unmultiplied(255, 255, 255, 10),
            ));
        }
    }

    pub fn update_cursor_trail (&mut self, f: f32) {
        self.cursor_trail_source = self.cursor_trail_source.lerp_towards(&self.cursor_rect, f);
    }
}
