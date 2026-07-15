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

    // El relleno (fill.rs) calcula el borde de cada fila con intersecciones
    // en punto flotante, mientras que esta línea usa Bresenham sobre
    // coordenadas ya redondeadas a enteros. Son dos aproximaciones
    // independientes de la MISMA arista, así que en ciertas filas/columnas
    // -sobre todo en diagonales de pendiente baja, las "descendentes"-
    // pueden no coincidir exactamente en qué píxel es el límite, dejando
    // que el relleno se asome 1px más allá de donde quedó el trazo de la
    // línea (fondo visible). Para cubrir ese posible desfase de 1px sin
    // importar la dirección de la arista, en vez de engrosar solo hacia un
    // lado, pintamos un bloque de 3x3 alrededor de cada punto del trazo.
    loop {
        for ddy in -1..=1 {
            for ddx in -1..=1 {
                framebuffer.set_pixel(x0 + ddx, y0 + ddy, color);
            }
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