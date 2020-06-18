use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

#[derive(Debug)]
pub struct Camera {

}

impl Camera {
    pub fn render(player: &crate::player::Player, map: &crate::map::Map, gfx: &mut quicksilver::graphics::Graphics) -> ()
    {
        let resolution: i32 = 320;
        for column in 0..resolution {
            let x = column as f32 / resolution as f32 - 0.5;
            let angle = x.atan2(0.8);
            //let ray = 
        }
    }
}