use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};
use std::rc::Rc;

#[derive(Debug)]
pub struct Player {
    x: f32,
    y: f32,
    direction: f32,
    fov: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, dir: f32) -> Player
    {
        return Player{x: x, y:y, direction: dir, fov: 120.0};
    }

    pub fn rotate(player: &mut Player, angle: f32) -> ()
    {
        player.direction = (
            player.direction + angle + 
            std::f32::consts::PI * 2f32
        ) % (std::f32::consts::PI * 2f32);

        //return newPlayer;
    }

    pub fn move_(player: &mut Player, x: f32, map: &crate::map::Map) -> ()
    {
        let hitbox_x = 0.0;//self.direction.cos() * 3.5;
        let hitbox_y = 0.0;//self.direction.sin() * 3.5;

        let dx = player.direction.cos() * x;
        let dy = player.direction.sin() * x;

        if crate::map::Map::can_move_to(&map, (player.x + dx + hitbox_x) / 32.0, (player.y + hitbox_y) / 32.0) 
        {
            player.x += dx;
        }

        if crate::map::Map::can_move_to(&map, (player.x + hitbox_x) / 32.0, (player.y + dy + hitbox_y) / 32.0) 
        {
            player.y += dy;
        }
    }

    pub fn coordinates(&mut self) -> (f32, f32) {
        return (self.x as f32, self.y as f32)
    }

    pub fn draw(player: &Player, gfx: &mut quicksilver::graphics::Graphics) -> () {
        let coords = Vector::new(player.x, player.y);
        gfx.fill_circle(&Circle::new(coords, 8.0), Color::RED);

        let dx = player.direction.cos() * 20.0;
        let dy = player.direction.sin() * 20.0;

        let coords = Vector::new(player.x + dx, player.y + dy);


        gfx.fill_circle(&Circle::new(coords, 3.0), Color::GREEN);
    }
}
