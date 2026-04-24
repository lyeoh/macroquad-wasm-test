use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

// Native/logical resolution the game renders at. Everything in the game
// world — positions, sizes, velocities — is expressed in these units.
// The final blit to the screen scales this up by an integer factor.
const GAME_W: f32 = 320.0;
const GAME_H: f32 = 180.0;

struct Shape {
    size: f32,
    x: f32,
    y: f32,
    v_x: f32,
    v_y: f32,
    color: Color,
}

impl Shape {
    // fn collides_with(&self, other: &Self) -> bool {
    //     self.rect().overlaps(&other.rect())
    // }

    fn circ_collides_with(&self, other: &Self) -> bool {
        self.circ_overlaps(&other.rect())
    }

    // assumes other is a rect
    fn circ_overlaps(&self, orect: &Rect) -> bool {
        let corners = [
            (orect.x, orect.y),
            (orect.x + orect.w, orect.y),
            (orect.x, orect.y + orect.h),
            (orect.x + orect.w, orect.y + orect.h),
        ];
        for corner in corners.into_iter() {
            if (corner.0 - self.x).powi(2) + (corner.1 - self.y).powi(2) <= self.size.powi(2) {
                return true;
            }
        }
        if corners[0].0 < self.x
            && self.x < corners[1].0
            && corners[0].1 < self.y
            && self.y < corners[2].1
        {
            return true;
        }
        return false;
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

/// Compute the letterboxed destination rect for blitting the render target to
/// the window. Picks the largest integer scale that fits and centers it,
/// leaving black bars around the rest.
fn compute_blit_rect() -> Rect {
    let sw = screen_width();
    let sh = screen_height();

    // Largest integer scale that fits; at least 1 so tiny windows still show
    // something.
    let scale = (sw / GAME_W).min(sh / GAME_H).floor().max(1.0);

    let w = GAME_W * scale;
    let h = GAME_H * scale;
    let x = ((sw - w) / 2.0).floor();
    let y = ((sh - h) / 2.0).floor();

    Rect { x, y, w, h }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "My macroquad game".to_owned(),
        platform: miniquad::conf::Platform {
            webgl_version: miniquad::conf::WebGLVersion::WebGL2,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // --- Pixel-perfect render target setup ---
    let render_target = render_target(GAME_W as u32, GAME_H as u32);
    // Nearest-neighbor filtering on the render target's texture so the
    // final upscale to the screen is crisp, not blurry.
    render_target.texture.set_filter(FilterMode::Nearest);

    // Camera that draws into the render target. The viewport is the full
    // logical resolution (0, 0, GAME_W, GAME_H).
    //
    // Note on Y-axis: macroquad internally flips Y when a camera has a
    // render_target set, so drawing into the target looks "right-side up"
    // as you'd expect from screen coordinates. But the resulting *texture*
    // ends up upside-down when sampled with normal UVs, so the final blit
    // below uses `flip_y: true` to compensate. Don't manually flip the
    // camera zoom — that double-flips and breaks everything.
    let game_camera = {
        let mut cam = Camera2D::from_display_rect(Rect {
            x: 0.0,
            y: 0.0,
            w: GAME_W,
            h: GAME_H,
        });
        cam.render_target = Some(render_target.clone());
        cam
    };

    // kinematic constants
    const V_MAX: f32 = 80.0;
    const MOVE_ACCEL: f32 = 700.0;
    const FRIC_ACCEL: f32 = 160.0;

    rand::srand(miniquad::date::now() as u64); // seed rng
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 6.0,
        x: GAME_W / 2.0,
        y: GAME_H / 2.0,
        v_x: 0.0,
        v_y: 0.0,
        color: YELLOW,
    };

    let mut gameover = false;

    let mut frame_counter = 0;
    let mut frametime_counter = 0.0;
    let mut fps_str = String::new();
    loop {
        let delta_time = get_frame_time();

        if !gameover {
            // update game state
            // handle circle velocity
            let fric_delta_v_i = FRIC_ACCEL * delta_time;
            let move_delta_v_i = MOVE_ACCEL * delta_time;
            if !(is_key_down(KeyCode::Right) ^ is_key_down(KeyCode::Left)) {
                if circle.v_x.abs() < fric_delta_v_i {
                    circle.v_x = 0.0;
                } else {
                    circle.v_x -= circle.v_x.signum() * fric_delta_v_i;
                }
            } else if is_key_down(KeyCode::Right) {
                circle.v_x += move_delta_v_i;
            } else if is_key_down(KeyCode::Left) {
                circle.v_x -= move_delta_v_i;
            }
            if !(is_key_down(KeyCode::Down) ^ is_key_down(KeyCode::Up)) {
                if circle.v_y.abs() < fric_delta_v_i {
                    circle.v_y = 0.0;
                } else {
                    circle.v_y -= circle.v_y.signum() * fric_delta_v_i;
                }
            } else if is_key_down(KeyCode::Down) {
                circle.v_y += move_delta_v_i;
            } else if is_key_down(KeyCode::Up) {
                circle.v_y -= move_delta_v_i;
            }

            // ensure circle speed stays within bounds
            circle.v_x = clamp(circle.v_x, -V_MAX, V_MAX);
            circle.v_y = clamp(circle.v_y, -V_MAX, V_MAX);

            // handle circle movement
            circle.x += circle.v_x * delta_time;
            circle.y += circle.v_y * delta_time;

            // ensure circle stays within logical screen
            circle.x = clamp(circle.x, 0.0, GAME_W);
            circle.y = clamp(circle.y, 0.0, GAME_H);

            // handle square spawning
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(4.0, 16.0);
                squares.push(Shape {
                    size,
                    x: rand::gen_range(size / 2.0, GAME_W - size / 2.0),
                    y: -size,
                    v_x: 0.0,
                    v_y: rand::gen_range(10.0, 30.0),
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
            squares.retain(|square| square.y < GAME_H + square.size);
        }

        // handle collisions
        if squares
            .iter()
            .any(|square| circle.circ_collides_with(square))
        {
            gameover = true;
        }

        // handle restarting the game with <space>
        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = GAME_W / 2.0;
            circle.y = GAME_H / 2.0;
            circle.v_x = 0.0;
            circle.v_y = 0.0;
            gameover = false;
        }

        // ──────── Pass 1: draw the game into the low-res render target ─────
        set_camera(&game_camera);
        clear_background(DARKGREEN);

        // Snap positions to the logical pixel grid at draw time. Game state
        // stays in floats (for smooth physics) but the rasterizer always
        // sees integer coordinates, so shape outlines don't shimmer as
        // sub-pixel positions drift frame-to-frame.
        for square in &squares {
            draw_rectangle(
                (square.x - square.size / 2.0).round(),
                (square.y - square.size / 2.0).round(),
                square.size,
                square.size,
                square.color,
            );
        }
        draw_circle(circle.x.round(), circle.y.round(), circle.size, YELLOW);

        // ───────── Pass 2: blit the render target to the actual screen ─────────
        set_default_camera();
        clear_background(BLACK); // letterbox bars

        let blit = compute_blit_rect();
        let ui_scale = blit.w / GAME_W;

        draw_texture_ex(
            &render_target.texture,
            blit.x,
            blit.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(blit.w, blit.h)),
                // Compensate for macroquad's render-target Y orientation.
                flip_y: true,
                ..Default::default()
            },
        );

        // handle and display average FPS
        frame_counter += 1;
        frametime_counter += delta_time;
        if frametime_counter > 1.0 {
            let avg_frametime = frametime_counter / frame_counter as f32;
            fps_str = format!("{:.0}", 1.0 / avg_frametime);
            frame_counter = 0;
            frametime_counter = 0.0;
        }
        let fps_fontsize = 10.0 * ui_scale;
        let text_dims = measure_text(&fps_str, None, fps_fontsize as u16, 1.0);
        draw_text(
            &fps_str,
            blit.x + blit.w - text_dims.width - 2.0 * ui_scale,
            blit.y + text_dims.offset_y + 2.0 * ui_scale,
            fps_fontsize,
            BLACK,
        );

        if gameover {
            let text = "YOU DIED";
            let fontsize = 40.0 * ui_scale;
            let text_dims = measure_text(text, None, fontsize as u16, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dims.width / 2.0,
                screen_height() / 2.0 - text_dims.offset_y + text_dims.height / 2.0,
                fontsize,
                BLACK,
            );

            let text = "press space to try again";
            let fontsize = 10.0 * ui_scale;
            let text_dims = measure_text(text, None, fontsize as u16, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dims.width / 2.0,
                blit.y + blit.h * 3.0 / 4.0 - text_dims.offset_y + text_dims.height / 2.0,
                fontsize,
                WHITE,
            );
        }

        next_frame().await
    }
    // println!("Hello, world!");
}
