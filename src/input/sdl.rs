use sdl2::*;
use sdl2::event::*;
use sdl2::keyboard::*;

use crate::input::{Input, MouseRes};
use crate::input::key::{KeysRes, Key};
use crate::misc::{AppStateRes, AppState};
use crate::misc::vec::Vec2;
use crate::render::CameraRes;

struct SDLContext {
    events: EventPump,
}

pub struct SDLInput {
    context: SDLContext
}

impl Input for SDLInput {
    fn input(&mut self, state: &mut AppStateRes, camera: &mut CameraRes, keys: &mut KeysRes, mouse: &mut MouseRes) {
        mouse.0 = None;
        for event in self.context.events.poll_iter() {
            match event {
                Event::Quit {..} => state.0 = AppState::Stopping,
                Event::Window { win_event: e, .. } => {
                    match e {
                        WindowEvent::Resized(w, h) => camera.screen = Vec2::new(w as u32, h as u32),
                        _ => {}
                    }
                },
                Event::KeyDown { keycode: Some(k), .. } => keys.press(SDLInput::sdl_to_key(k)),
                Event::KeyUp { keycode: Some(k), .. } => keys.release(SDLInput::sdl_to_key(k)),
                Event::MouseButtonUp { x, y, .. } => mouse.0 = Some(Vec2::new(x as u32, camera.screen.y - y as u32)),
                _ => {}
            }
        }
    }
}

impl SDLInput {
    pub fn init(sdl_context: &Sdl) -> SDLInput {
        let events = sdl_context.event_pump().unwrap();

        let context = SDLContext {
            events: events
        };

        SDLInput {
            context: context
        }
    }

    fn sdl_to_key(k: Keycode) -> Key {
        match k {
            Keycode::A => Key::A,
            Keycode::B => Key::B,
            Keycode::C => Key::C,
            Keycode::D => Key::D,
            Keycode::E => Key::E,
            Keycode::F => Key::F,
            Keycode::G => Key::G,
            Keycode::H => Key::H,
            Keycode::I => Key::I,
            Keycode::J => Key::J,
            Keycode::K => Key::K,
            Keycode::L => Key::L,
            Keycode::M => Key::M,
            Keycode::N => Key::N,
            Keycode::O => Key::O,
            Keycode::P => Key::P,
            Keycode::Q => Key::Q,
            Keycode::R => Key::R,
            Keycode::S => Key::S,
            Keycode::T => Key::T,
            Keycode::U => Key::U,
            Keycode::V => Key::V,
            Keycode::W => Key::W,
            Keycode::X => Key::X,
            Keycode::Y => Key::Y,
            Keycode::Z => Key::Z,
            Keycode::Space => Key::Space,
            _ => Key::Unknown
        }
    }
}
