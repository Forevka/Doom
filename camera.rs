use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

#[derive(Debug)]
pub struct Camera<'a> {
    player: &'a crate::player::Player<'a>,
    map: &'a crate::map::Map,
}

impl<'a> Camera<'a> {
    pub fn new(player: &'a mut crate::player::Player, map: &'a mut crate::map::Map) -> Camera<'a>
    {
        return Camera{player: player, map: map};
    }

    pub fn render(&mut self, gfx: &mut quicksilver::graphics::Graphics) -> ()
    {
        
    }
}