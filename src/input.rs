pub mod key;
pub mod sdl;

use nalgebra::Vector2;

use specs::prelude::*;

use vitrellogy_macro::DefaultConstructor;
use crate::input::key::KeysRes;
use crate::misc::AppStateRes;
use crate::render::CameraRes;

pub trait Input {
    fn input(&mut self, state: &mut AppStateRes, camera: &mut CameraRes, keys: &mut KeysRes, mouse: &mut MouseRes);
}

pub struct InputSys<T: Input> {
    pub input: T
}

impl<'a, T: Input> System<'a> for InputSys<T> {
    type SystemData = (Write<'a, AppStateRes>,
        Write<'a, CameraRes>,
        Write<'a, KeysRes>,
        Write<'a, MouseRes>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut state, mut camera, mut keys, mut mouse) = data;
        self.input.input(&mut state, &mut camera, &mut keys, &mut mouse);
    }
}

impl<T: Input> InputSys<T> {
    pub fn new(input: T) -> Self {
        Self {
            input: input
        }
    }
}

#[derive(Default, Debug, DefaultConstructor)]
pub struct MouseRes(pub Option<Vector2<u32>>);
