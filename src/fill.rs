use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

/// Rellena uno o varios contornos usando el algoritmo de scanline (regla even-odd).

/// Al recibir varios contornos en un solo llamado (por ejemplo, un polígono
/// exterior y un polígono "agujero"), las intersecciones de todos los
/// contornos se combinan y se ordenan por scanline, de forma que la regla
/// even-odd deja automáticamente sin pintar la zona del agujero.
pub fn fill_polygons(framebuffer: &mut Framebuffer, contours: &[&[Vector2]], fill_color: Color) {
    if contours.is_empty() {
        return;
    }

    // Bounding box vertical de todos los contornos.
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for contour in contours {
        for p in contour.iter() {
            if p.y < min_y {
                min_y = p.y;
            }
            if p.y > max_y {
                max_y = p.y;
            }
        }
    }

    if min_y > max_y {
        return;
    }

    let y_start = min_y.floor() as i32;
    let y_end = max_y.ceil() as i32;

    for y in y_start..=y_end {
        let scan_y = y as f32 + 0.5; // muestreo en el centro del pixel

        let mut intersections: Vec<f32> = Vec::new();

        for contour in contours {
            let n = contour.len();
            if n < 2 {
                continue;
            }

            for i in 0..n {
                let p1 = contour[i];
                let p2 = contour[(i + 1) % n];

                let (y1, y2) = (p1.y, p2.y);

                // Ignorar aristas horizontales (no aportan cruces útiles).
                if (y1 - y2).abs() < f32::EPSILON {
                    continue;
                }

                // ¿La scanline cruza esta arista?
                if (scan_y >= y1 && scan_y < y2) || (scan_y >= y2 && scan_y < y1) {
                    let t = (scan_y - y1) / (y2 - y1);
                    let x = p1.x + t * (p2.x - p1.x);
                    intersections.push(x);
                }
            }
        }

        if intersections.len() < 2 {
            continue;
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Pintar por pares de intersecciones (regla even-odd).
        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i].round() as i32;
            let x_end = intersections[i + 1].round() as i32;

            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y, fill_color);
            }

            i += 2;
        }
    }
}

/// Azúcar sintáctica para rellenar un único contorno (sin agujeros).
pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &[Vector2], fill_color: Color) {
    fill_polygons(framebuffer, std::slice::from_ref(&points), fill_color);
}
