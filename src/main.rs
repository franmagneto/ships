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
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes},
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const LOGICAL_WIDTH: u32 = 256;
const LOGICAL_HEIGHT: u32 = 224;
const NS_PER_FRAME: u64 = 1_001_000_000 / 60;

struct Game<'a> {
    canvas: Canvas<'a>,
    time_step: Duration,
    ship: Ship,
    asteroid: Asteroid,
}

impl<'a> Game<'a> {
    fn new(window: Arc<Window>) -> Self {
        let mut canvas = Canvas::new(window.clone(), LOGICAL_WIDTH, LOGICAL_HEIGHT);
        canvas.set_color(Color::from_rgba(10, 15, 30, 0xff));

        Self {
            canvas,
            time_step: Duration::from_nanos(NS_PER_FRAME),
            ship: Ship::new(),
            asteroid: Asteroid::new(),
        }
    }

    fn update(&mut self, keys: &HashSet<Key>) {
        self.ship.handle_input(&keys);

        self.ship.update();
        self.asteroid.update();
    }

    fn render(&mut self, start: Instant) {
        self.canvas.clear();
        self.ship.render(&mut self.canvas);
        self.asteroid.render(&mut self.canvas);
        self.canvas.present();

        sleep(self.time_step.saturating_sub(start.elapsed()));
    }
}

#[derive(Default)]
struct App<'a> {
    window: Option<Arc<Window>>,
    game: Option<Game<'a>>,
    keys: HashSet<Key>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let min_size = LogicalSize::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);

        let window_attributes = WindowAttributes::default()
            .with_title("Ships")
            .with_inner_size(size)
            .with_min_inner_size(min_size);

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));
        self.game = Some(Game::new(self.window.as_ref().unwrap().clone()));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let start = Instant::now();

        match event {
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();

                self.game.as_mut().unwrap().update(&self.keys);
                self.game.as_mut().unwrap().render(start);
            }
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(physical_size) => self
                .game
                .as_mut()
                .unwrap()
                .canvas
                .resize(physical_size.width, physical_size.height),
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key, state, ..
                },
                ..
            } => match state {
                ElementState::Pressed => {
                    self.keys.insert(logical_key);
                }
                ElementState::Released => {
                    self.keys.remove(&logical_key);
                }
            },
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
