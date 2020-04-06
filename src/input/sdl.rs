use std::collections::*;
use std::f32::consts::*;

use sdl2::*;
use sdl2::event::*;
use sdl2::keyboard::*;

use crate::input::*;
use crate::misc::vec::*;

struct SDLContext {
    events: EventPump,
}

pub struct SDLInput {
    keys: Keys,
    win_dim: Vec2<u32>,
    movement: Vec2<f32>,
    context: SDLContext
}

impl Input for SDLInput {
    fn input(&mut self) -> InputState {
        for event in self.context.events.poll_iter() {
            match event {
                Event::Quit {..} => return InputState::Stopping,
                Event::Window { win_event: e, .. } => {
                    match e {
                        WindowEvent::Resized(w, h) => self.win_dim = Vec2::new(w as u32, h as u32),
                        _ => {}
                    }
                },
                Event::KeyDown { keycode: Some(k), .. } => self.keys.press(k),
                Event::KeyUp { keycode: Some(k), .. } => self.keys.release(k),
                Event::MouseButtonUp { x, y, .. } => println!("Click at ({}, {})", x, y),
                _ => {}
            }
        }

        self.movement = match (self.keys.pressed(Keycode::W), self.keys.pressed(Keycode::S), self.keys.pressed(Keycode::D), self.keys.pressed(Keycode::A)) {
            (true, true, true, true) => Vec2::new(0.0, 0.0),
            (true, true, true, false) => Vec2::new(1.0, 0.0),
            (true, true, false, true) => Vec2::new(-1.0, 0.0),
            (true, true, false, false) => Vec2::new(0.0, 0.0),
            (true, false, true, true) => Vec2::new(0.0, 1.0),
            (true, false, true, false) => Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (true, false, false, true) => Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (true, false, false, false) => Vec2::new(0.0, 1.0),
            (false, true, true, true) => Vec2::new(0.0, -1.0),
            (false, true, true, false) => Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            (false, true, false, true) => Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            (false, true, false, false) => Vec2::new(0.0, -1.0),
            (false, false, true, true) => Vec2::new(0.0, 0.0),
            (false, false, true, false) => Vec2::new(1.0, 0.0),
            (false, false, false, true) => Vec2::new(-1.0, 0.0),
            (false, false, false, false) => Vec2::new(0.0, 0.0)
        };

        InputState::Running
    }

    fn get_win_dim(&self) -> Vec2<u32> {
        self.win_dim
    }

    fn get_movement(&self) -> Vec2<f32> {
        self.movement
    }
}

impl SDLInput {
    pub fn init(sdl_context: &Sdl, win_dim: Vec2<u32>) -> SDLInput {
        let events = sdl_context.event_pump().unwrap();

        let context = SDLContext {
            events: events
        };

        SDLInput {
            keys: Keys::new(),
            movement: Vec2::new(0.0, 0.0),
            win_dim: win_dim,
            context: context
        }
    }
}

struct Keys {
    keys: HashMap<Keycode, bool>
}

impl Keys {
    fn new() -> Self {
        Keys {
            keys: HashMap::new()
        }
    }

    fn press(&mut self, key: Keycode) {
        self.keys.insert(key, true);
    }

    fn release(&mut self, key: Keycode) {
        self.keys.insert(key, false);
    }

    fn pressed(&self, key: Keycode) -> bool {
        match self.keys.contains_key(&key) {
            true => self.keys[&key],
            false => false
        }
    }
}
