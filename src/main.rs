use macroquad::prelude::*;
use std::fs;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
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

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

#[macroquad::main("Flyway")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    
    let mut bullets: Vec<Shape> = vec![];
    let mut meteorites: Vec<Shape> = vec![];

    let mut number_block = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };

    number_block.x = clamp(number_block.x, 0.0, screen_width());
    number_block.y = clamp(number_block.y, 0.0, screen_height());

    let mut game_state = GameState::MainMenu;

    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);

    loop {
        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    meteorites.clear();
                    bullets.clear();
                    number_block.x = screen_width() / 2.0;
                    number_block.y = screen_height() / 2.0;
                    score = 0;
                    game_state = GameState::Playing;
                }
                let text = "Press space";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            },
            GameState::Playing => {
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
                // Shoot bullets
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        x: number_block.x,
                        y: number_block.y,
                        speed: number_block.speed * 2.0,
                        size: 5.0,
                        collided: false,
                    });
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
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
                        collided: false,
                    });
                }

                // Update square positions
                for meteor in &mut meteorites {
                    meteor.y += meteor.speed * delta_time;
                }
                // Update bullet positions
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                // Remove invisible squares
                meteorites.retain(|square| square.y < screen_height() + square.size);
                // Remove invisible bullets
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

                meteorites.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);
                // }

                // Check for collisions
                if meteorites.iter().any(|square| number_block.collides_with(square))
                {
                    game_state = GameState::GameOver;
                }

                // Check for bullet collisions
                for meteorite in meteorites.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(meteorite) {
                            score += meteorite.size.round() as u32;
                            high_score = high_score.max(score);
                            bullet.collided = true;
                            meteorite.collided = true;
                        }
                    }
                }

                // Draw everything
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
                }
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

                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
                let highscore_text = format!("High score: {}", high_score);
                let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
                draw_text(
                    highscore_text.as_str(),
                    screen_width() - text_dimensions.width - 10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
            },

            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            },

            GameState::GameOver => {
                if score == high_score {
                    fs::write("highscore.dat", high_score.to_string()).ok();
                }
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }
                let text = "GAME OVER!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );
            },
        }
        next_frame().await
    }
}
