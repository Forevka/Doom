
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};
use std::rc::Rc;
use std::cell::RefCell;

mod player;
mod map;
mod camera;

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

    let mapRef: Rc<RefCell<map::Map>> = Rc::new(RefCell::new(map));

    let mut player = player::Player::new(100.0, 100.0, -90.0);

    let mut playerRef: Rc<RefCell<player::Player>> = Rc::new(RefCell::new(player));

    //let mut camera = camera::Camera::new(Rc::clone(&playerRef), Rc::clone(&mapRef));

    //println!("{:?}", camera);

    let mut prevMouse: Vector = Vector {
        x: 0.0,
        y: 0.0,
    };

    const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };

    loop {
        while let Some(_) = input.next_event().await {}

        const SPEED: f32 = 0.6;

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

        if input.key_down(Key::Escape) {
            return Ok(());
        }

        let mouse = input.mouse().location();
        //println!("{}", mouse);
        let mut mouse_x_delta = prevMouse.x - mouse.x;
        prevMouse = mouse;
        if mouse_x_delta != 0.0
        {
        println!("{}", mouse_x_delta);
        if mouse_x_delta > 1.0 {mouse_x_delta = 1.0}
        if mouse_x_delta < -1.0 {mouse_x_delta = -1.0}
        player::Player::rotate(&mut playerRef.borrow_mut(), -mouse_x_delta.sin() / 20.0);
        }
        
        

        player::Player::rotate(&mut playerRef.borrow_mut(), rotating);

        if vector_x != 0.0
        {
            player::Player::move_(&mut playerRef.borrow_mut(), vector_x, &mapRef.borrow());
        }

        gfx.clear(Color::WHITE);

        map::Map::render(&mapRef.borrow(), &mut gfx);
        player::Player::draw(&playerRef.borrow(), &mut gfx);
        camera::Camera::render(&playerRef.borrow(), &mapRef.borrow(), &mut gfx);

        for i in 0..720 {
            let x = 1.0 * (i as f32);
            //println!("{}",x);
            let camera_x = 2f32 * x / 720.0 - 1.0;
            //println!("{}", cameraX);
            let ray_dir_x = playerRef.borrow().direction.cos() + 0.0 * camera_x;
            let ray_dir_y = playerRef.borrow().direction.sin() + 0.66 * camera_x;
            //println!("{}, {}", ray_dir_x, ray_dir_y);
            let mut map_x = (playerRef.borrow().x) as i32;
            let mut map_y = (playerRef.borrow().y) as i32;

            let mut side_dist_x: f32;
            let mut side_dist_y: f32;

            let step_x: i32;
            let step_y: i32;

            let delta_dist_x = if ray_dir_y == 0.0 {0.0} else { if ray_dir_x == 0.0 { 1.0 } else { (1.0 / ray_dir_x).abs()} };
            let delta_dist_y = if ray_dir_x == 0.0 {0.0} else { if ray_dir_y == 0.0 { 1.0 } else { (1.0 / ray_dir_y).abs()} };

            //println!("{}, {}", delta_dist_x, delta_dist_y);
            let mut hit: i32 = 0;
            let mut side: i32 = 0;

            let mut perp_wall_dist: f32 = 0.0;

            if ray_dir_x < 0.0
            {   
                step_x = -1;
                side_dist_x = ((playerRef.borrow().x) - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = (map_x as f32 + 1.0 - (playerRef.borrow().x )) * delta_dist_x;
            }

            if ray_dir_y < 0.0
            {
                step_y= -1;
                side_dist_y = ((playerRef.borrow().y ) - map_y as f32) * delta_dist_y;
            }else {
                step_y = 1;
                side_dist_y = (map_y as f32 + 1.0 - (playerRef.borrow().y )) * delta_dist_y;
            }

            while hit == 0
            {
                if side_dist_x < side_dist_y
                {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                if !map::Map::can_move_to(&mapRef.borrow(), map_x as f32 / 12.0, map_y as f32 / 12.0) {
                    hit = 1;
                }
            }

            
            if side == 0 {
                perp_wall_dist = (map_x as f32 - (playerRef.borrow().x) + (1.0 - step_x as f32) / 2.0) / ray_dir_x;
            } else {
                perp_wall_dist = (map_y as f32 - (playerRef.borrow().y) + (1.0 - step_y as f32) / 2.0) / ray_dir_y;
            }

            let line_height = 420.0 / perp_wall_dist;

            let mut draw_start = -line_height / 2.0 + 420.0 / 2.0;

            if draw_start < 0.0 { draw_start = 0.0};

            let mut draw_end = line_height / 2.0 + 420.0 / 2.0;

            if draw_end >= 420.0 {draw_end = 420.0 - 1.0};
            //println!("{}, {}, {}", x);

            let mut color: Color = GRAY;
            if side == 1
            {
                color = Color::BLACK;
            }

            gfx.fill_rect(
                &Rectangle::new(Vector::new(300.0 + x, draw_start), Vector::new(1.0, line_height)),
                color
            );
        }
        //gfx.fill_rect(rect: &Rectangle, color: Color)
        //let x = Rc::get_mut(&mut playerRef).unwrap();
        //x.draw(&mut gfx);
        //playerRef.draw(&mut gfx);
        //camera.render(&mut gfx);

        gfx.present(&window)?;
    }
}