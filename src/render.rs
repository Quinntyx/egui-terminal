use egui::{Color32, Mesh, Pos2, Rect};
use egui::epaint::Vertex;

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