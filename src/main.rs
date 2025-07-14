mod framebuffer;
mod line;
mod polygon_fill;

use framebuffer::Framebuffer;
use line::Vector2;
use polygon_fill::{Polygon, draw_filled_polygon};
use image::Rgb;

fn main() {
    let width = 800;
    let height = 600;
    let mut fb = Framebuffer::new(width, height);

    // Colores
    let white = Rgb([255, 255, 255]);
    let yellow = Rgb([255, 255, 0]);

    // Definir polígono 1
    let polygon1_vertices = vec![
        Vector2 { x: 165, y: 380 },
        Vector2 { x: 185, y: 360 },
        Vector2 { x: 180, y: 330 },
        Vector2 { x: 207, y: 345 },
        Vector2 { x: 233, y: 330 },
        Vector2 { x: 230, y: 360 },
        Vector2 { x: 250, y: 380 },
        Vector2 { x: 220, y: 385 },
        Vector2 { x: 205, y: 410 },
        Vector2 { x: 193, y: 383 },
    ];

    // Crear objeto Polygon
    let polygon1 = Polygon::new(polygon1_vertices, yellow, white);

    // Dibujar polígono 1
    draw_filled_polygon(&mut fb, &polygon1, None);

    fb.save("out.bmp");
    println!("Polígono 1 (amarillo) guardado como out.bmp");
}