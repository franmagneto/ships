mod entities;
mod graphics;

use entities::{
    asteroid::Asteroid,
    base_entity::{Entity, Renderable},
    ship::Ship,
};
use graphics::{canvas::Canvas, color::Color};
use std::{num::NonZeroU32, rc::Rc, time::Duration};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const NS_PER_FRAME: u64 = 1_001_000_000 / 60;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        Rc::new(
            WindowBuilder::new()
                .with_title("Ships")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap(),
        )
    };
    let mut canvas = Canvas::new(
        window.clone(),
        NonZeroU32::new(256).unwrap(),
        NonZeroU32::new(224).unwrap(),
    );

    let time_step = Duration::from_nanos(NS_PER_FRAME);

    let mut ship = Ship::new();
    let mut asteroid = Asteroid::new();

    event_loop.set_control_flow(ControlFlow::wait_duration(time_step));

    event_loop
        .run(move |event, elwt| {
            if let Event::AboutToWait = event {
                canvas.set_color(Color::from_rgba(10, 15, 30, 0xff));
                canvas.clear();
                ship.render(&mut canvas);
                asteroid.render(&mut canvas);
                canvas.present();
            }

            if input.update(&event) {
                if input.key_pressed_logical(Key::Named(NamedKey::Escape))
                    || input.close_requested()
                {
                    elwt.exit();
                }

                if input.key_held_logical(Key::Named(NamedKey::ArrowUp)) {
                    ship.go_up();
                }

                if input.key_held_logical(Key::Named(NamedKey::ArrowDown)) {
                    ship.go_down();
                }

                ship.update();
                asteroid.update();
            }
        })
        .unwrap();
}
