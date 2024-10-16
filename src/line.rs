// line.rs

use crate::framebuffer::Framebuffer;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Posiciones iniciales y finales
    let x0 = a.position.x as isize;
    let y0 = a.position.y as isize;
    let x1 = b.position.x as isize;
    let y1 = b.position.y as isize;

    // Diferencias absolutas
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    // Incrementos
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Diferencias acumuladas
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        // Agregar un fragmento en la posición actual con el color interpolado
        let t = ((x - x0) as f32 / (x1 - x0).max(1) as f32).abs();
        let color = a.color.lerp(&b.color, t);
        fragments.push(Fragment::new(x as f32, y as f32, color, 1.0));

        // Si llegamos al punto final, terminamos
        if x == x1 && y == y1 {
            break;
        }

        // Actualizar el error acumulado y las coordenadas
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    fragments
}

pub fn triangle() -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));
}