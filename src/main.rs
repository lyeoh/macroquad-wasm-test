use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

struct Shape {
    size: f32,
    x: f32,
    y: f32,
    v_x: f32,
    v_y: f32,
    color: Color,
}

#[macroquad::main("My macroquad game")]
async fn main() {
    const V_MAX: f32 = 400.0;
    const MOVE_ACCEL: f32 = 50.0;
    const FRIC_ACCEL: f32 = 15.0;

    rand::srand(miniquad::date::now() as u64); // seed rng
    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        v_x: 0.0,
        v_y: 0.0,
        color: YELLOW,
    };

    let mut frame_counter = 0;
    let mut frametime_counter = 0.0;
    let mut fps_str = String::new();
    loop {
        clear_background(DARKGREEN);

        let delta_time = get_frame_time();

        // handle and display average FPS
        frame_counter += 1;
        frametime_counter += delta_time;
        if frametime_counter > 1.0 {
            let avg_frametime = frametime_counter / frame_counter as f32;
            fps_str = format!("{:.0}", 1.0 / avg_frametime);
            frame_counter = 0;
            frametime_counter = 0.0;
        }
        let text_dimensions = measure_text(&fps_str, None, 50, 1.0);
        draw_text(
            &fps_str,
            screen_width() - text_dimensions.width,
            text_dimensions.offset_y,
            50.0,
            BLACK,
        );

        // handle circle velocity
        if !(is_key_down(KeyCode::Right) ^ is_key_down(KeyCode::Left)) {
            if circle.v_x.abs() < 1e-3 {
                circle.v_x = 0.0;
            } else {
                circle.v_x -= circle.v_x.signum() * FRIC_ACCEL;
            }
        } else if is_key_down(KeyCode::Right) {
            circle.v_x += MOVE_ACCEL;
        } else if is_key_down(KeyCode::Left) {
            circle.v_x -= MOVE_ACCEL;
        }
        if !(is_key_down(KeyCode::Down) ^ is_key_down(KeyCode::Up)) {
            if circle.v_y.abs() < 1e-3 {
                circle.v_y = 0.0;
            } else {
                circle.v_y -= circle.v_y.signum() * FRIC_ACCEL;
            }
        } else if is_key_down(KeyCode::Down) {
            circle.v_y += MOVE_ACCEL;
        } else if is_key_down(KeyCode::Up) {
            circle.v_y -= MOVE_ACCEL;
        }

        // ensure circle speed stays within bounds
        circle.v_x = clamp(circle.v_x, -V_MAX, V_MAX);
        circle.v_y = clamp(circle.v_y, -V_MAX, V_MAX);

        // handle circle movement
        circle.x += circle.v_x * delta_time;
        circle.y += circle.v_y * delta_time;

        // ensure circle stays within screen
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        // handle square spawning
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size: size,
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                v_x: 0.0,
                v_y: rand::gen_range(50.0, 150.0),
                color: *[PURPLE, DARKPURPLE, RED, PINK, MAROON, VIOLET]
                    .choose()
                    .unwrap(),
            });
        }

        // handle squares' movement
        for square in &mut squares {
            square.y += square.v_y * delta_time;
        }

        // remove squares which have moved past bottom of screen
        squares.retain(|square| square.y < screen_height() + square.size);

        // draw everything
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }
        draw_circle(circle.x, circle.y, 16.0, YELLOW);

        next_frame().await
    }
    // println!("Hello, world!");
}
