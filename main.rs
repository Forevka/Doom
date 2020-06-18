
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

    let mut player = player::Player::new(100.0, 100.0, 30.0);

    let mut playerRef: Rc<RefCell<player::Player>> = Rc::new(RefCell::new(player));

    //let mut camera = camera::Camera::new(Rc::clone(&playerRef), Rc::clone(&mapRef));

    //println!("{:?}", camera);

    loop {
        while let Some(_) = input.next_event().await {}

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
        

        player::Player::rotate(&mut playerRef.borrow_mut(), rotating);

        if vector_x != 0.0
        {
            player::Player::move_(&mut playerRef.borrow_mut(), vector_x, &mapRef.borrow());
        }

        gfx.clear(Color::WHITE);

        map::Map::render(&mapRef.borrow(), &mut gfx);
        player::Player::draw(&playerRef.borrow(), &mut gfx);
        camera::Camera::render(&playerRef.borrow(), &mapRef.borrow(), &mut gfx);

        //let x = Rc::get_mut(&mut playerRef).unwrap();
        //x.draw(&mut gfx);
        //playerRef.draw(&mut gfx);
        //camera.render(&mut gfx);

        gfx.present(&window)?;
    }
}