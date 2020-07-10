pub mod key;
pub mod sdl;

use nalgebra::Vector2;

use specs::prelude::*;

use vitrellogy_macro::DefaultConstructor;
use crate::input::key::KeysRes;
use crate::misc::AppStateRes;
use crate::render::CameraRes;
use crate::input::sdl::SDLInputImpl;

#[derive(DefaultConstructor)]
pub struct InputSys;

impl<'a> System<'a> for InputSys {
    type SystemData = (WriteExpect<'a, InputRes>,
        Write<'a, AppStateRes>,
        Write<'a, CameraRes>,
        Write<'a, KeysRes>,
        Write<'a, MouseRes>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut input, mut state, mut camera, mut keys, mut mouse) = data;
        input.input(&mut state, &mut camera, &mut keys, &mut mouse);
    }
}

pub struct InputRes {
    pub input: SDLInputImpl
}

// Do NOT, and I repeat, DO NOT attempt to send -
unsafe impl Send for InputRes {}
// - OR SYNC!
unsafe impl Sync for InputRes {}

impl InputRes {
    pub fn input(&mut self, state: &mut AppStateRes, camera: &mut CameraRes, keys: &mut KeysRes, mouse: &mut MouseRes) {
        self.input.input(state, camera, keys, mouse);
    }
}

#[derive(Default, Debug, DefaultConstructor)]
pub struct MouseRes(pub Option<Vector2<u32>>);
