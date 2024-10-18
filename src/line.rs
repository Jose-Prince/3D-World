// line.rs

use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::{Vec2, Vec3, dot};

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let start = a.transformed_position;
    let end = b.transformed_position;

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    loop {
        let z = start.z + (end.z - start.z) * (x0 - start.x as i32) as f32 / (end.x - start.x) as f32;
        fragments.push(Fragment::new(x0 as f32, y0 as f32, Color::new(0, 0, 0), z));

        if x0 == x1 && y0 == y1 { break; }

        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}

pub fn _triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Draw the three sides of the triangle
    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v3, v1));

    fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    // Iterate over each pixel in the bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32, y as f32, 0.0);

            // Calculate barycentric coordinates
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c);

            // Check if the point is inside the triangule
            if w1 >= 0.0 && w1 <= 1.0 &&
            w2 >= 0.0 && w2 <= 1.0 &&
            w3 >= 0.0 && w3 <= 1.0 {
                let normal = v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3;
                let normal = normal.normalize();

                // Calculate lighting intensity
                let intensity = dot(&normal, &light_dir).max(0.0);

                // Create a gray color and apply lighting
                let base_color = Color::new(100, 100, 100);
                let lit_color = base_color * intensity;

                let depth = a.z * w1 + b.z * w2 + c.z * w3;

                fragments.push(Fragment::new(x as f32, y as f32, lit_color, depth));
            }
        }
    }

    fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    
    let total_area = area_of_triangle(*a, *b, *c);

    let area_pbc = area_of_triangle(*p, *b, *c);
    let area_apc = area_of_triangle(*a, *p, *c);
    let area_abp = area_of_triangle(*a, *b, *p);

    let lambda1 = area_pbc / total_area;
    let lambda2 = area_apc / total_area;
    let lambda3 = area_abp / total_area;

    (lambda1, lambda2, lambda3)
}

fn area_of_triangle(a: Vec3, b: Vec3, c: Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}