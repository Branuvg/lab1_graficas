mod framebuffer;
mod line;
mod polygon;
mod polygon_fill;

use raylib::prelude::*;
use framebuffer::Framebuffer;
//use line::line;
use polygon::draw_polygon;
use polygon_fill::fill_polygon;

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

    //Poligono con array con polygon.rs
    /*let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    let polygon1_points: Vec<Vector2> = polygon1
        .into_iter()
        .map(|(x, y)| Vector2::new(x as f32, y as f32))
        .collect();

        frame_buffer.set_current_color(Color::YELLOW);
        fill_polygon(&mut frame_buffer, &polygon1_points);
        frame_buffer.set_current_color(Color::WHITE);
        draw_polygon(&mut frame_buffer, &polygon1_points);

        let polygon2 = vec![
            (321, 335), (288, 286), (339, 251), (374, 302)
        ];
    
        let polygon2_points: Vec<Vector2> = polygon2
            .into_iter()
            .map(|(x, y)| Vector2::new(x as f32, y as f32))
            .collect();
    
        frame_buffer.set_current_color(Color::BLUE);
        fill_polygon(&mut frame_buffer, &polygon2_points);
        frame_buffer.set_current_color(Color::WHITE);
        draw_polygon(&mut frame_buffer, &polygon2_points);

        let polygon3 = vec![
            (377, 249), (411, 197), (436, 249)
        ];
    
        let polygon3_points: Vec<Vector2> = polygon3
            .into_iter()
            .map(|(x, y)| Vector2::new(x as f32, y as f32))
            .collect();
    
        frame_buffer.set_current_color(Color::RED);
        fill_polygon(&mut frame_buffer, &polygon3_points);
        frame_buffer.set_current_color(Color::WHITE);
        draw_polygon(&mut frame_buffer, &polygon3_points);*/
    
    let polygon4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    let polygon4_points: Vec<Vector2> = polygon4
        .into_iter()
        .map(|(x, y)| Vector2::new(x as f32, y as f32))
        .collect();

    frame_buffer.set_current_color(Color::GREEN);
    fill_polygon(&mut frame_buffer, &polygon4_points);
    frame_buffer.set_current_color(Color::WHITE);
    draw_polygon(&mut frame_buffer, &polygon4_points);

    let polygon5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    let polygon5_points: Vec<Vector2> = polygon5
        .into_iter()
        .map(|(x, y)| Vector2::new(x as f32, y as f32))
        .collect();

    frame_buffer.set_current_color(Color::PURPLE);
    fill_polygon(&mut frame_buffer, &polygon5_points);
    frame_buffer.set_current_color(Color::WHITE);
    draw_polygon(&mut frame_buffer, &polygon5_points);

    let output_file = "polygon4.bmp";
    frame_buffer.render_to_file(output_file);
}