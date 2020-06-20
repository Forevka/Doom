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
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1], 
            vec![1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
        ];

        return Map::new_from_grid(size, grid)
    }

    
    pub fn new_from_grid(size: u8, grid: Vec<Vec<u8>>) -> Map
    {
        return Map {size: size, grid: grid}
    }

    pub fn get_point(map: &Map, x: f32, y: f32) -> u8 {
        let row = map.grid.get(x.floor() as usize);
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
                                return *v
                            },
                            None => return 1
                        }
                    }
                    return 1
                },
                None => return 1
            }
        }
        return 1;
    }

    pub fn can_move_to(map: &Map, x: f32, y: f32) -> bool {
        return Map::get_point(map, x, y) == 0
    }


    pub fn render(map: &Map, gfx: &mut quicksilver::graphics::Graphics) -> () {
        for i in 0..map.grid.len() {
            for j in 0..map.grid[0].len() {
                gfx.fill_rect(
                    &Rectangle::new(Vector::new(i as f32 * 12.0, j as f32 * 12.0), Vector::new(12.0, 12.0)),
                    match map.grid[i][j] {
                        0 => Color::WHITE,
                        1 => Color::BLACK,
                        _ => Color::WHITE,
                    },
                );
            }
        }
    }
}