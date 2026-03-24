use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("Flyway")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    // let mut x = screen_width() / 2.0;
    // let mut y = screen_height() / 2.0;

    let mut meteorites:Vec<Shape> = vec![];

    let mut number_block = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    number_block.x = clamp(number_block.x, 0.0, screen_width());
    number_block.y = clamp(number_block.y, 0.0, screen_height());
    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Right) {
            number_block.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            number_block.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            number_block.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            number_block.y -= MOVEMENT_SPEED * delta_time;
        }

        number_block.x = clamp(number_block.x, 0.0, screen_width());
        number_block.y = clamp(number_block.y, 0.0, screen_height());

        // Add enemy squares
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            meteorites.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }

        // Update square positions
        for meteor in &mut meteorites {
            meteor.y += meteor.speed * delta_time;
        }

        // Remove invisible squares
        meteorites.retain(|square| square.y < screen_height() + square.size);

        draw_circle(number_block.x, number_block.y, 16.0, YELLOW);

        for square in &meteorites {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        next_frame().await
    }
}
