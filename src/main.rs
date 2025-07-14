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
    let red = Rgb([255, 0, 0]);

    // Definir polígono 3
    let polygon3_vertices = vec![
        Vector2 { x: 377, y: 249 },
        Vector2 { x: 411, y: 197 },
        Vector2 { x: 436, y: 249 },
    ];

    // Crear objeto Polygon
    let polygon3 = Polygon::new(polygon3_vertices, red, white);

    // Dibujar polígono 3
    draw_filled_polygon(&mut fb, &polygon3, None);

    fb.save("out.bmp");
    println!("Polígono 3 (rojo) guardado como out.bmp");
}