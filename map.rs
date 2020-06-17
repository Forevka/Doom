use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};


#[derive(Debug)]
pub struct Map {
    pub size: u8,
    pub grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(size: u8) -> Map {
        let mut grid: Vec<Vec<u8>> = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
        ];

        return Map::new_from_grid(size, grid)
    }

    
    pub fn new_from_grid(size: u8, grid: Vec<Vec<u8>>) -> Map
    {
        return Map {size: size, grid: grid}
    }

    pub fn can_move_to(&mut self, x: f32, y: f32) -> bool 
    {
        let row = self.grid.get(x.floor() as usize);
        if row.is_some()
        {
            match row
            {
                Some(p) => {
                    let val = p.get(y.floor() as usize);
                    if val.is_some()
                    {
                        match val
                        {
                            Some(v) => {
                                if v == &0u8 {return true}
                            },
                            None => return false
                        }
                    }
                    return false
                },
                None => return false
            }
        }
        return false;
    }

    pub fn render(&mut self, gfx: &mut quicksilver::graphics::Graphics) -> () {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                gfx.fill_rect(
                    &Rectangle::new(Vector::new(i as f32 * 32.0, j as f32 * 32.0), Vector::new(32.0, 32.0)),
                    match self.grid[i][j] {
                        0 => Color::WHITE,
                        1 => Color::BLACK,
                        _ => Color::WHITE,
                    },
                );
            }
        }
    }
}