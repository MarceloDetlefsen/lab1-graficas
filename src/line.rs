use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;
    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    let color = framebuffer.current_color;

    // Una línea de 1px de grosor es frágil: si la arista es horizontal o
    // vertical, el borde vive en una sola fila/columna y puede desaparecer
    // casi por completo cuando la imagen se reescala o comprime (por
    // ejemplo, al subirla a git, en una miniatura, o en el visor que use
    // quien la revise). Las líneas diagonales no sufren esto porque ya
    // ocupan varias filas/columnas por su naturaleza. Para evitarlo,
    // engrosamos el trazo 1px extra en la dirección perpendicular al avance
    // dominante de la línea.
    let thicken_vertical = dx >= dy; // línea más horizontal -> engrosar en y
    let thicken_horizontal = dy >= dx; // línea más vertical -> engrosar en x

    loop {
        framebuffer.set_pixel(x0, y0, color);
        if thicken_vertical {
            framebuffer.set_pixel(x0, y0 + 1, color);
        }
        if thicken_horizontal {
            framebuffer.set_pixel(x0 + 1, y0, color);
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }

        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_poligon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 2 {
        return;
    }

    for pair in points.windows(2) {
        line(framebuffer, pair[0], pair[1]);
    }

    line(framebuffer, points[points.len() - 1], points[0]);
}