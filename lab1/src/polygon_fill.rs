use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

// Función auxiliar para calcular los puntos de intersección entre una línea horizontal y los lados del polígono
fn find_intersections(points: &[Vector2], y: f32) -> Vec<f32> {
    let mut intersections = Vec::new();
    let n = points.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let p1 = points[i];
        let p2 = points[j];
        
        // Solo considerar lados que cruzan la línea y
        if (p1.y > y && p2.y <= y) || (p1.y <= y && p2.y > y) {
            // Calcular la intersección x
            let x = p1.x + (y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
            intersections.push(x);
        }
    }
    
    // Ordenar las intersecciones
    intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    intersections
}

// Función principal para rellenar el polígono
pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 3 {
        return; // Necesitamos al menos 3 puntos para un polígono relleno
    }
    
    // Encontrar los límites verticales del polígono
    let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min).floor() as i32;
    let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;
    
    // Procesar cada línea horizontal dentro del rango vertical
    for y in min_y..=max_y {
        let y_f32 = y as f32;
        let intersections = find_intersections(points, y_f32);
        
        // Dibujar segmentos entre pares de intersecciones
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 >= intersections.len() {
                break;
            }
            
            let x_start = intersections[i].floor() as i32;
            let x_end = intersections[i + 1].ceil() as i32;
            
            // Dibujar la línea horizontal entre los puntos de intersección
            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }
        }
    }
}