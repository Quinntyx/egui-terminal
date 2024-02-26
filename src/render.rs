use egui::{pos2, vec2, Color32, Mesh, Painter, Pos2, Rect, Stroke, Vec2};
use ecolor::HexColor;
use egui::epaint::Vertex;
use termwiz::surface::CursorVisibility;
use wezterm_term::CursorPosition;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

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

pub fn quad_trail (cursor: Rect, trail: Rect, color: Color32) -> Mesh {
    let mut m = Mesh::default();

    if cursor.top() > trail.top() {
        m.add_quad_simple(
            cursor.left_top(),
            cursor.right_top(),
            trail.left_top(),
            trail.right_top(),
            color
        );
    }

    if cursor.right() < trail.right() {
        m.add_quad_simple(
            cursor.right_top(),
            cursor.right_bottom(),
            trail.right_top(),
            trail.right_bottom(),
            color
        );
    }

    if cursor.bottom() < trail.bottom() {
        m.add_quad_simple(
            cursor.right_bottom(),
            cursor.left_bottom(),
            trail.right_bottom(),
            trail.left_bottom(),
            color
        );
    }

    if cursor.left() > trail.left() {
        m.add_quad_simple(
            cursor.left_top(),
            cursor.left_bottom(),
            trail.left_top(),
            trail.left_bottom(),
            color
        );
    }

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

#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CursorType {
    Block(HexColor),
    SolidBlock(HexColor),
    Beam(HexColor),
    OpenBlock(HexColor),
    #[default]
    None
}

pub struct CursorRenderer {
    cursor_trail_source: Rect,
    pub draw_trail: bool,
    cursor_rect: Rect,
    pub cursor_type: CursorType,
    visible: bool,
    widget_offset: Vec2,
    stable_time_factor: f64,
    pub trail_color: Option<Color32>,
    pub cursor_stroke: Stroke,
}

impl CursorRenderer {
    pub fn new () -> Self {
        Self {
            cursor_trail_source: Rect::from_points(&[pos2(0., 0.)]),
            draw_trail: true,
            cursor_rect: Rect::from_points(&[pos2(0., 0.)]),
            cursor_type: CursorType::Block(HexColor::Hex8(Color32::TRANSPARENT)),
            visible: true,
            widget_offset: vec2(0., 0.),
            stable_time_factor: 0.,
            trail_color: None,
            cursor_stroke: Stroke::NONE,
        }
    }

    pub fn set_offset (&mut self, offset: Vec2) {
        self.widget_offset = offset;
    }

    pub fn update_cursor_state (&mut self, cur: CursorPosition, text_width: f32, text_height: f32) {
        self.visible = cur.visibility == CursorVisibility::Visible;

        let cursor_rect = match &self.cursor_type {
            &CursorType::Block(_) | &CursorType::SolidBlock(_) | &CursorType::OpenBlock(_) | &CursorType::None => Rect::from_min_size(  
                egui::pos2(
                    (cur.x) as f32 * text_width + self.widget_offset.x+ 1.,
                    (cur.y) as f32 * text_height + self.widget_offset.y
                ),
                egui::vec2(text_width - 2., text_height),
            ),
            &CursorType::Beam(_) => Rect::from_min_size(
                egui::pos2(
                    (cur.x) as f32 * text_width + self.widget_offset.x+ 1.,
                    (cur.y) as f32 * text_height + self.widget_offset.y
                ),
                egui::vec2(text_width - 4., text_height),
            ),
        };

        self.cursor_rect = cursor_rect;
    }

    fn get_trail_color (&self) -> Color32 {
        self.trail_color.unwrap_or_else(|| {
            self.get_color().gamma_multiply(0.5)
        })
    }

    fn get_color (&self) -> Color32 {
        let (alpha, color) = match &self.cursor_type {
            &CursorType::Block(c) => (((self.stable_time_factor / std::f64::consts::FRAC_PI_2 * 13.).sin() + 1.) / 2., c),
            &CursorType::SolidBlock(c) => (1., c),
            &CursorType::OpenBlock(c) => (0., c),
            &CursorType::Beam(c) => (1., c),
            &CursorType::None => (0., HexColor::Hex8(Color32::TRANSPARENT)),
        };
        
        color.color().gamma_multiply(alpha as f32)
    }

    pub fn draw_cursor (&mut self, painter: Painter, delta_time: f32) {
        self.stable_time_factor += delta_time as f64;
        if matches!(self.cursor_type, CursorType::None) { return; }
        if !self.visible { return; }

        painter.rect(
            self.cursor_rect,
            egui::Rounding::same(1.),
            self.get_color(),
            self.cursor_stroke,
        );
    
        if self.draw_trail {
            painter.add(quad_trail(
                self.cursor_rect,
                self.cursor_trail_source,
                self.get_trail_color(),
            ));
        }
    }

    pub fn update_cursor_trail (&mut self, f: f32) {
        self.cursor_trail_source = self.cursor_trail_source.lerp_towards(&self.cursor_rect, f);
    }
}
