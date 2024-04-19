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
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
    window::WindowBuilder,
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const NS_PER_FRAME: u64 = 1_001_000_000 / 60;

fn main() {
    let mut event_loop = EventLoop::new().unwrap();
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

    let mut keys: HashSet<Key> = HashSet::new();

    'main: loop {
        let start = Instant::now();

        let status = event_loop.pump_events(Some(Duration::ZERO), |event, elwt| match event {
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
        });
        if let PumpStatus::Exit(_) = status {
            break 'main;
        }
        ship.handle_input(&keys);

        ship.update();
        asteroid.update();

        canvas.set_color(Color::from_rgba(10, 15, 30, 0xff));
        canvas.clear();
        ship.render(&mut canvas);
        asteroid.render(&mut canvas);
        canvas.present();

        sleep(time_step.saturating_sub(start.elapsed()));
    }
}
