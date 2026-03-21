use macroquad::prelude::*;

#[macroquad::main("Flyway")]
async fn main() {
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
