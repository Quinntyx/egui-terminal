use egui::{Mesh, Pos2, Color32};
use egui::epaint::Vertex;

fn pos2_to_vertex (a: Pos2) -> Vertex {
    let mut v = Vertex::default();
    v.pos = a;
    v.color = Color32::from_rgba_unmultiplied(255, 255, 255, 10);
    v
}

pub fn triangle (a: Pos2, b: Pos2, c: Pos2) -> Mesh {
    let mut vertices = vec!(a, b, c).iter_mut()
        .map(|v| pos2_to_vertex(*v))
        .collect::<Vec<_>>();

    let mut mesh = Mesh::default();
    mesh.vertices.append(&mut vertices);

    mesh.add_triangle(0, 1, 2);

    mesh
}

pub fn quad (a: Pos2, b: Pos2, c: Pos2, d: Pos2) -> Mesh {
    let mut vertices = vec!(a, b, c, d).iter_mut()
        .map(|v| pos2_to_vertex(*v))
        .collect::<Vec<_>>();

    let mut mesh = Mesh::default();
    mesh.vertices.append(&mut vertices);

    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(1, 2, 3);

    mesh
}