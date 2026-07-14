mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::draw_poligon;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::new(50, 50, 100, 255));

    framebuffer.clear();

    framebuffer.set_current_color(Color::GREEN);
    let polygon_points: [Vector2; 5] = [
        Vector2::new(100.0, 80.0),
        Vector2::new(500.0, 120.0),
        Vector2::new(700.0, 320.0),
        Vector2::new(420.0, 520.0),
        Vector2::new(-60.0, 300.0),
    ];
    draw_poligon(&mut framebuffer, &polygon_points[..]);

    let output_file = "polygons.png";

    framebuffer.render_to_file(output_file);
}
