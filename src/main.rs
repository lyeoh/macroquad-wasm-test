use macroquad::prelude::*;

#[macroquad::main("My macroquad game")]
async fn main() {
    let dt = 1.0;
    let v_max = 3.0;
    let dv_driv = 0.5;
    let dv_fric = 0.1;
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut v_x: f32 = 0.0;
    let mut v_y: f32 = 0.0;
    loop {
        clear_background(DARKGREEN);
        // let v = 2.0;
        // if is_key_down(KeyCode::Right) {
        //     x += v;
        // }
        // if is_key_down(KeyCode::Left) {
        //     x -= v;
        // }
        // if is_key_down(KeyCode::Down) {
        //     y += v;
        // }
        // if is_key_down(KeyCode::Up) {
        //     y -= v;
        // }

        if !(is_key_down(KeyCode::Right) ^ is_key_down(KeyCode::Left)) {
            if v_x.abs() < 1e-3 {
                v_x = 0.0;
            } else {
                v_x -= v_x.signum() * dv_fric;
            }
        } else if is_key_down(KeyCode::Right) {
            v_x += dv_driv;
        } else if is_key_down(KeyCode::Left) {
            v_x -= dv_driv;
        }

        if !(is_key_down(KeyCode::Down) ^ is_key_down(KeyCode::Up)) {
            if v_y.abs() < 1e-3 {
                v_y = 0.0;
            } else {
                v_y -= v_y.signum() * dv_fric;
            }
        } else if is_key_down(KeyCode::Down) {
            v_y += dv_driv;
        } else if is_key_down(KeyCode::Up) {
            v_y -= dv_driv;
        }

        if v_x.abs() > v_max {
            v_x = v_x.signum() * v_max
        }
        if v_y.abs() > v_max {
            v_y = v_y.signum() * v_max
        }

        x += v_x * dt;
        y += v_y * dt;

        draw_circle(x, y, 16.0, YELLOW);
        next_frame().await
    }
    // println!("Hello, world!");
}
