use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

#[derive(Debug)]
pub struct Player<'a> {
    x: f32,
    y: f32,
    direction: f32,
    fov: f32,
    map: &'a crate::map::Map,
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, dir: f32, map: &'a mut crate::map::Map) -> Player<'a>
    {
        return Player{x: x, y:y, direction: dir, fov: 120.0, map: map};
    }

    pub fn rotate(&mut self, angle: f32) -> () 
    {
        self.direction = (
            self.direction + angle + 
            std::f32::consts::PI * 2f32
        ) % (std::f32::consts::PI * 2f32);
    }

    pub fn move_(&mut self, x: f32) -> ()
    {
        let hitbox_x = 0.0;//self.direction.cos() * 3.5;
        let hitbox_y = 0.0;//self.direction.sin() * 3.5;

        let dx = self.direction.cos() * x;
        let dy = self.direction.sin() * x;

        if self.map.can_move_to((self.x + dx + hitbox_x) / 32.0, (self.y + hitbox_y) / 32.0) 
        {
            self.x += dx;
        }

        if self.map.can_move_to((self.x + hitbox_x) / 32.0, (self.y + dy + hitbox_y) / 32.0) 
        {
            self.y += dy;
        }
    }

    pub fn coordinates(&mut self) -> (f32, f32) {
        return (self.x as f32, self.y as f32)
    }

    pub fn draw(&mut self, gfx: &mut quicksilver::graphics::Graphics) -> () {
        
        let mut player_coords = self.coordinates();
        let coords = Vector::new(player_coords.0, player_coords.1);
        gfx.fill_circle(&Circle::new(coords, 8.0), Color::RED);

        let dx = self.direction.cos() * 20.0;
        let dy = self.direction.sin() * 20.0;

        player_coords.0 += dx;
        player_coords.1 += dy;

        let coords = Vector::new(player_coords.0, player_coords.1);


        gfx.fill_circle(&Circle::new(coords, 3.0), Color::GREEN);
    }
}