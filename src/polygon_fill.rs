use crate::framebuffer::Framebuffer;
use crate::line::{line, Vector2};
use image::Rgb;

pub struct Polygon {
    pub vertices: Vec<Vector2>,
    pub fill_color: Rgb<u8>,
    pub outline_color: Rgb<u8>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector2>, fill_color: Rgb<u8>, outline_color: Rgb<u8>) -> Self {
        Self {
            vertices,
            fill_color,
            outline_color,
        }
    }
}

pub fn draw_filled_polygon(fb: &mut Framebuffer, polygon: &Polygon, hole: Option<&Polygon>) {
    // Primero rellenar el polígono principal
    scanline_fill(fb, &polygon.vertices, polygon.fill_color);
    
    // Si hay un agujero, rellenar con el color de fondo (negro)
    if let Some(hole_polygon) = hole {
        scanline_fill(fb, &hole_polygon.vertices, Rgb([0, 0, 0]));
    }
    
    // Dibujar el contorno del polígono principal
    draw_polygon_outline(fb, &polygon.vertices, polygon.outline_color);
    
    // Dibujar el contorno del agujero si existe
    if let Some(hole_polygon) = hole {
        draw_polygon_outline(fb, &hole_polygon.vertices, hole_polygon.outline_color);
    }
}

fn draw_polygon_outline(fb: &mut Framebuffer, vertices: &[Vector2], color: Rgb<u8>) {
    for i in 0..vertices.len() {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % vertices.len()];
        line(fb, p1, p2, color);
    }
}

fn scanline_fill(fb: &mut Framebuffer, vertices: &[Vector2], color: Rgb<u8>) {
    if vertices.len() < 3 {
        return;
    }
    
    // Encontrar los límites del polígono
    let mut min_y = vertices[0].y;
    let mut max_y = vertices[0].y;
    
    for vertex in vertices {
        if vertex.y < min_y {
            min_y = vertex.y;
        }
        if vertex.y > max_y {
            max_y = vertex.y;
        }
    }
    
    // Para cada línea de escaneo
    for y in min_y..=max_y {
        let mut intersections = Vec::new();
        
        // Encontrar intersecciones con todas las aristas
        for i in 0..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[(i + 1) % vertices.len()];
            
            // Verificar si la línea de escaneo intersecta con esta arista
            if let Some(x) = line_intersection(p1, p2, y) {
                intersections.push(x);
            }
        }
        
        // Ordenar las intersecciones
        intersections.sort();
        
        // Rellenar entre pares de intersecciones
        for chunk in intersections.chunks(2) {
            if chunk.len() == 2 {
                let start_x = chunk[0];
                let end_x = chunk[1];
                
                for x in start_x..=end_x {
                    fb.set_pixel(x, y, color);
                }
            }
        }
    }
}

fn line_intersection(p1: Vector2, p2: Vector2, y: i32) -> Option<i32> {
    // Verificar si la línea horizontal y intersecta con el segmento p1-p2
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);
    
    // La línea de escaneo debe estar dentro del rango y del segmento
    if y < min_y || y > max_y {
        return None;
    }
    
    // Evitar contar intersecciones en vértices horizontales
    if p1.y == p2.y {
        return None;
    }
    
    // Calcular la intersección x
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    
    if dy == 0 {
        return None;
    }
    
    let x = p1.x + (dx * (y - p1.y)) / dy;
    Some(x)
}

// Función para verificar si un punto está dentro de un polígono (algoritmo ray casting)
pub fn point_in_polygon(point: Vector2, vertices: &[Vector2]) -> bool {
    let mut inside = false;
    let mut j = vertices.len() - 1;
    
    for i in 0..vertices.len() {
        let vi = vertices[i];
        let vj = vertices[j];
        
        if ((vi.y > point.y) != (vj.y > point.y)) &&
           (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x) {
            inside = !inside;
        }
        j = i;
    }
    
    inside
}