pub mod controller;
pub mod key;

pub mod sdl;

use specs::prelude::*;

use crate::input::key::KeysRes;
use crate::misc::AppStateRes;
use crate::render::CameraRes;

pub trait Input {
    fn input(&mut self, state: &mut AppStateRes, camera: &mut CameraRes, keys: &mut KeysRes);
}

pub struct InputSys<T: Input> {
    pub input: T
}

impl<'a, T: Input> System<'a> for InputSys<T> {
    type SystemData = (Write<'a, AppStateRes>,
        Write<'a, CameraRes>,
        Write<'a, KeysRes>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut state, mut camera, mut keys) = data;
        self.input.input(&mut state, &mut camera, &mut keys);
    }
}

impl<T: Input> InputSys<T> {
    pub fn new(input: T) -> Self {
        Self {
            input: input
        }
    }
}
