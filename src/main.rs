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
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const NS_PER_FRAME: u64 = 1_001_000_000 / 60;

fn main() {
    let event_loop = EventLoop::new().unwrap();
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
            match event {
                Event::WindowEvent {
                    window_id,
                    event:
                        WindowEvent::KeyboardInput {
                            event: KeyEvent { logical_key, .. },
                            ..
                        },
                } if window_id == window.id() => match logical_key {
                    Key::Named(NamedKey::ArrowUp) => {
                        ship.go_up();
                    }
                    Key::Named(NamedKey::ArrowDown) => {
                        ship.go_down();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    canvas.set_color(Color::from_rgb(10, 15, 30));
                    canvas.clear();
                    ship.render(&mut canvas);
                    asteroid.render(&mut canvas);
                    canvas.present();
                }
                Event::WindowEvent {
                    window_id,
                    event:
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        },
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {}
            }

            ship.update();
            asteroid.update();
        })
        .unwrap();
}
