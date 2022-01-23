mod entities;
mod utils;

use entities::{
    asteroid::Asteroid,
    base_entity::{Controllable, Entity, Renderable},
    ship::Ship,
};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{
    collections::HashSet,
    thread::sleep,
    time::{Duration, Instant},
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const NS_PER_FRAME: u64 = 1_001_000_000 / 60;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Ships", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut render_target = texture_creator
        .create_texture_target(texture_creator.default_pixel_format(), 256, 224)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let time_step = Duration::from_nanos(NS_PER_FRAME);

    let mut ship = Ship::new(&texture_creator);
    let mut asteroid = Asteroid::new(&texture_creator);

    'running: loop {
        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let keys: HashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        ship.handle_input(keys);

        ship.update();
        asteroid.update();

        canvas
            .with_texture_canvas(&mut render_target, |canvas| {
                canvas.set_draw_color(Color::RGB(10, 15, 30));
                canvas.clear();
                ship.render(canvas);
                asteroid.render(canvas);
            })
            .unwrap();
        canvas.copy(&render_target, None, None).unwrap();
        canvas.present();
        sleep(time_step.saturating_sub(start.elapsed()));
    }
}
