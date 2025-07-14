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
    let green = Rgb([0, 255, 0]);

    // Definir polígono 4
    let polygon4_vertices = vec![
        Vector2 { x: 413, y: 177 },
        Vector2 { x: 448, y: 159 },
        Vector2 { x: 502, y: 88 },
        Vector2 { x: 553, y: 53 },
        Vector2 { x: 535, y: 36 },
        Vector2 { x: 676, y: 37 },
        Vector2 { x: 660, y: 52 },
        Vector2 { x: 750, y: 145 },
        Vector2 { x: 761, y: 179 },
        Vector2 { x: 672, y: 192 },
        Vector2 { x: 659, y: 214 },
        Vector2 { x: 615, y: 214 },
        Vector2 { x: 623, y: 230 },
        Vector2 { x: 580, y: 230 },
        Vector2 { x: 597, y: 215 },
        Vector2 { x: 552, y: 214 },
        Vector2 { x: 517, y: 144 },
        Vector2 { x: 466, y: 180 },
    ];

    // Definir polígono 5 (agujero)
    let polygon5_vertices = vec![
        Vector2 { x: 682, y: 175 },
        Vector2 { x: 708, y: 120 },
        Vector2 { x: 735, y: 148 },
        Vector2 { x: 739, y: 170 },
    ];

    // Crear objetos Polygon
    let polygon4 = Polygon::new(polygon4_vertices, green, white);
    let polygon5 = Polygon::new(polygon5_vertices, white, white); // Para el agujero

    // Dibujar polígono 4 con agujero 5
    draw_filled_polygon(&mut fb, &polygon4, Some(&polygon5));

    fb.save("out.bmp");
    println!("Polígono 4 (verde) con agujero guardado como out.bmp");
}