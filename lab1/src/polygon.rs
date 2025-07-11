use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() {
        let start = points[i];
        let end = if i == points.len() - 1 {
            points[0] // Último punto se conecta al primero
        } else {
            points[i + 1]
        };
        line(framebuffer, start, end);
    }
}
