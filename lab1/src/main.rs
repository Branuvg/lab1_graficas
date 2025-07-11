mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;

fn main() {
    //Seteamos el canvas
    let width = 800;
    let height = 600;
    let mut frame_buffer = Framebuffer::new(width, height, Color::RAYWHITE);

    //Seteamos el color de fondo y limpiamos
    frame_buffer.set_background_color(Color::PURPLE);
    frame_buffer.clear();

    //Dibujamos lineas de prueba con line.rs
    //frame_buffer.set_current_color(Color::GREEN);
    //line(&mut frame_buffer, Vector2::new(100.0, 100.0), Vector2::new(200.0, 200.0));

    let output_file = "out.bmp";
    frame_buffer.render_to_file(output_file);
}