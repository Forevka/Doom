#[derive(Debug)]
pub struct Map {
    pub size: u8,
    pub grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(size: u8) -> Map {
        let grid: Vec<Vec<u8>> = vec![
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
}