use macroquad::prelude::*;

#[macroquad::main("My macroquad game")]
async fn main() {
    loop {
        clear_background(DARKGREEN);
        next_frame().await
    }
    // println!("Hello, world!");
}
