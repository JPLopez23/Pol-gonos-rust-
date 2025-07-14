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
    let blue = Rgb([0, 0, 255]);
    let red = Rgb([255, 0, 0]);
    let green = Rgb([0, 255, 0]);

    // Definir todos los polígonos
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

    let polygon2_vertices = vec![
        Vector2 { x: 321, y: 335 },
        Vector2 { x: 288, y: 286 },
        Vector2 { x: 339, y: 251 },
        Vector2 { x: 374, y: 302 },
    ];

    let polygon3_vertices = vec![
        Vector2 { x: 377, y: 249 },
        Vector2 { x: 411, y: 197 },
        Vector2 { x: 436, y: 249 },
    ];

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

    let polygon5_vertices = vec![
        Vector2 { x: 682, y: 175 },
        Vector2 { x: 708, y: 120 },
        Vector2 { x: 735, y: 148 },
        Vector2 { x: 739, y: 170 },
    ];

    // Crear objetos Polygon
    let polygon1 = Polygon::new(polygon1_vertices, yellow, white);
    let polygon2 = Polygon::new(polygon2_vertices, blue, white);
    let polygon3 = Polygon::new(polygon3_vertices, red, white);
    let polygon4 = Polygon::new(polygon4_vertices, green, white);
    let polygon5 = Polygon::new(polygon5_vertices, white, white); // Para el agujero

    // Dibujar todos los polígonos
    draw_filled_polygon(&mut fb, &polygon1, None);
    draw_filled_polygon(&mut fb, &polygon2, None);
    draw_filled_polygon(&mut fb, &polygon3, None);
    draw_filled_polygon(&mut fb, &polygon4, Some(&polygon5)); // Polígono 4 con agujero 5

    fb.save("out.bmp");
    println!("Imagen guardada como out.bmp");
}