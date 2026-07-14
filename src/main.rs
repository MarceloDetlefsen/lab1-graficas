mod framebuffer;
mod line;
mod fill;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::draw_poligon;
use fill::{fill_polygon, fill_polygons};

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);

    framebuffer.clear();

    // ----------------------------------------------------------------
    // Polígono 1 (estrella) - relleno dorado, línea blanca
    // ----------------------------------------------------------------
    let polygon1: [Vector2; 10] = [
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    // --- Relleno (scanline con regla even-odd) ---
    fill_polygon(&mut framebuffer, polygon1.as_slice(), Color::GOLD);

    // --- Líneas / bordes ---
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, polygon1.as_slice());

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
}
