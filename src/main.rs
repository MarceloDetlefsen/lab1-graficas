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

    // ----------------------------------------------------------------
    // Polígono 2 - relleno naranja, línea blanca
    // ----------------------------------------------------------------
    let polygon2: [Vector2; 4] = [
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];

    // ----------------------------------------------------------------
    // Polígono 3 (triángulo) - relleno verde oliva, línea blanca
    // ----------------------------------------------------------------
    let polygon3: [Vector2; 3] = [
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];

    // ----------------------------------------------------------------
    // Polígono 4 - relleno arena, línea blanca (con agujero: polígono 5)
    // ----------------------------------------------------------------
    let polygon4: [Vector2; 18] = [
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0),
    ];

    // ----------------------------------------------------------------
    // Polígono 5 - agujero dentro del polígono 4, no se pinta
    // ----------------------------------------------------------------
    let polygon5: [Vector2; 4] = [
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];

    // --- Relleno (scanline con regla even-odd) ---
    fill_polygon(&mut framebuffer, polygon1.as_slice(), Color::GOLD);

    fill_polygon(&mut framebuffer, polygon2.as_slice(), Color::ORANGERED);

    fill_polygon(&mut framebuffer, polygon3.as_slice(), Color::OLIVE);

    // El polígono 4 y su agujero (polígono 5) se rellenan juntos para que la regla even-odd deje el agujero sin pintar.
    let contours = [polygon4.as_slice(), polygon5.as_slice()];
    fill_polygons(&mut framebuffer, contours.as_slice(), Color::SANDYBROWN);

    // --- Líneas / bordes ---
    framebuffer.set_current_color(Color::WHITE);
    draw_poligon(&mut framebuffer, polygon1.as_slice());
    draw_poligon(&mut framebuffer, polygon2.as_slice());
    draw_poligon(&mut framebuffer, polygon3.as_slice());
    draw_poligon(&mut framebuffer, polygon4.as_slice());
    draw_poligon(&mut framebuffer, polygon5.as_slice());

    let output_file = "out.bmp";
    framebuffer.render_to_file(output_file);
}
