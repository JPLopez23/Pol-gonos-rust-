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
    let blue = Rgb([0, 0, 255]);

    // Definir polígono 2
    let polygon2_vertices = vec![
        Vector2 { x: 321, y: 335 },
        Vector2 { x: 288, y: 286 },
        Vector2 { x: 339, y: 251 },
        Vector2 { x: 374, y: 302 },
    ];

    // Crear objeto Polygon
    let polygon2 = Polygon::new(polygon2_vertices, blue, white);

    // Dibujar polígono 2
    draw_filled_polygon(&mut fb, &polygon2, None);

    fb.save("out.bmp");
    println!("Polígono 2 (azul) guardado como out.bmp");
}