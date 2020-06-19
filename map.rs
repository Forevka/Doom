use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Map {
    pub size: u8,
    pub grid: Vec<Vec<u8>>,
}

pub struct Step {
    pub x: f32,
    pub y: f32,
    pub length: f32,
    pub height: f32,
    pub distance: f32,
    pub isInfinity: bool,
    pub shading: f32,
    pub offset: f32,
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

    pub fn can_move_to(map: &Map, x: f32, y: f32) -> bool 
    {
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

    /*pub fn step(map: &Map, rise: f32, run: f32, x: f32, y: f32, inverted: bool, angle: f32) -> Step 
    {
        let mut step = Step{x: 0.0, y: 0.0, length: 0.0, distance: 0.0, isInfinity: true, height: 0.0, offset: 0.0, shading: 0.0};
        if run == 0.0 
        { 
            return step;
        }

        let dx = if run > 0.0 { f32::floor(x + 1.0) - x } else {f32::ceil(x -1.0) - x};
        let dy = dx * (rise / run);

        step.x = if inverted { y + dy } else { x + dx };
        step.y = if inverted { x + dx } else { y + dy };
        step.length = dx * dx + dy * dy;
        step.isInfinity = false;

        return step;
    }

    pub fn inspect<'a>(map: &Map, stepOld: &'a mut Step, shift_x: f32, shift_y: f32, distance: f32, offset: f32, angle: f32) -> &'a mut Step
    {
        let sin = f32::sin(angle);
        let cos = f32::cos(angle);

        let dx = if cos < 0.0 {shift_x} else {0.0};
        let dy = if sin < 0.0 {shift_y} else {0.0};

        stepOld.height = if Map::can_move_to(map, stepOld.x - dx, stepOld.y - dy) {1.0} else {0.0};
        stepOld.distance = distance + f32::sqrt(stepOld.length);

        if shift_x == 1.0 
        {
            stepOld.shading = if cos < 0.0 {2.0} else {1.0};
        } else {
            stepOld.shading = if sin < 0.0 {2.0} else {1.0};
        }

        stepOld.offset = offset - f32::floor(offset);

        return stepOld;
    }

    pub fn ray(map: &Map, step: Step, angle: f32, range: f32) -> Vec<Step>
    {
        let mut steps: Vec<Step> = Vec::new();

        let stepRef: Rc<RefCell<Step>> = Rc::new(RefCell::new(step));

        //steps.push(stepRef.borrow());

        let sin = f32::sin(angle);
        let cos = f32::cos(angle);

        let step_x = Map::step(map, sin, cos, stepRef.borrow().x, stepRef.borrow().y, false, angle);
        let step_y = Map::step(map, cos, sin, stepRef.borrow().y, stepRef.borrow().x, true, angle);
        
        let stepxRef: Rc<RefCell<Step>> = Rc::new(RefCell::new(step_x));
        let stepyRef: Rc<RefCell<Step>> = Rc::new(RefCell::new(step_y));

        let nextStep: &mut Step;

        if stepxRef.borrow().length < stepyRef.borrow().length {
            nextStep = Map::inspect(map, &mut stepxRef.borrow(), 1.0, 0.0, stepRef.borrow().distance, stepxRef.borrow().y, angle);
        } else {
            nextStep = Map::inspect(map, &mut stepyRef.borrow(), 0.0, 1.0, stepRef.borrow().distance, stepyRef.borrow().x, angle);
        };

        let nextStepRef: Rc<RefCell<&mut Step>> = Rc::new(RefCell::new(nextStep));

        if nextStep.distance < range { steps.extend(Map::ray(map, nextStepRef.borrow()., angle, range))}

        return steps;
    }

    pub fn cast(map: &Map, user_x: f32, user_y: f32, angle: f32, range: f32)
    {
        return 
    }*/
}