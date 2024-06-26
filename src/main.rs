mod entities;
mod graphics;

use entities::{
    asteroid::Asteroid,
    base_entity::{Controllable, Entity, Renderable},
    ship::Ship,
};
use graphics::{canvas::Canvas, color::Color};
use std::{
    collections::HashSet,
    num::NonZeroU32,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const LOGICAL_WIDTH: u32 = 256;
const LOGICAL_HEIGHT: u32 = 224;
const NS_PER_FRAME: u64 = 1_001_000_000 / 60;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let min_size = LogicalSize::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);
        Rc::new(
            WindowBuilder::new()
                .with_title("Ships")
                .with_inner_size(size)
                .with_min_inner_size(min_size)
                .build(&event_loop)
                .unwrap(),
        )
    };
    let mut canvas = Canvas::new(window.clone(), LOGICAL_WIDTH, LOGICAL_HEIGHT);
    canvas.set_color(Color::from_rgba(10, 15, 30, 0xff));

    let time_step = Duration::from_nanos(NS_PER_FRAME);

    let mut ship = Ship::new();
    let mut asteroid = Asteroid::new();

    let mut keys: HashSet<Key> = HashSet::new();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| {
            let start = Instant::now();

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    ship.handle_input(&keys);

                    ship.update();
                    asteroid.update();

                    let size = window.inner_size();
                    canvas.resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );

                    canvas.clear();
                    ship.render(&mut canvas);
                    asteroid.render(&mut canvas);
                    canvas.present();

                    sleep(time_step.saturating_sub(start.elapsed()));
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
                } if window_id == window.id() => elwt.exit(),
                Event::WindowEvent {
                    window_id,
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key, state, ..
                                },
                            ..
                        },
                } if window_id == window.id() => match state {
                    ElementState::Pressed => {
                        keys.insert(logical_key);
                    }
                    ElementState::Released => {
                        keys.remove(&logical_key);
                    }
                },
                Event::AboutToWait => window.request_redraw(),
                _ => {}
            }
        })
        .unwrap();
}
