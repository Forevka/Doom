// Example 8: Input
// Respond to user keyboard and mouse input onscreen
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

mod player;
mod map;

fn main() {
    run(
        Settings {
            title: "Doom 1234",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mut map = map::Map::new(64);
    let mut player = player::Player::new(100.0, 100.0, 30.0);

    loop {
        while let Some(_) = input.next_event().await {}
        // Check the state of the keys, and move the square accordingly
        const SPEED: f32 = 2.0;

        let mut vector_x: f32 = 0.0;
        let mut rotating: f32 = 0.0;

        if input.key_down(Key::A) {
            rotating -= SPEED / 20.0;
        }

        if input.key_down(Key::D) {
            rotating += SPEED / 20.0;
        }

        if input.key_down(Key::W) {
            vector_x += SPEED;
        }

        if input.key_down(Key::S) {
            vector_x -= SPEED;
        }

        player.rotate(rotating);

        if vector_x != 0.0
        {
            player.move_(vector_x, &mut map);
        }

        gfx.clear(Color::WHITE);
        // Paint a blue square at the given position
        for i in 0..map.grid.len() {
            for j in 0..map.grid[0].len() {
                gfx.fill_rect(
                    &Rectangle::new(Vector::new(i as f32 * 32.0, j as f32 * 32.0), Vector::new(32.0, 32.0)),
                    match map.grid[i][j] {
                        0 => Color::WHITE,
                        1 => Color::BLACK,
                        _ => Color::WHITE,
                    },
                );
            }
        }
        player.draw(&mut gfx);

        gfx.present(&window)?;
    }
}