pub mod sdl;

use crate::misc::vec::*;

#[derive(Debug, Clone, Copy)]
pub enum InputState {
    Running,
    Stopping
}

pub trait Input {
    fn input(&mut self) -> InputState;
    fn get_win_dim(&self) -> Vec2<u32>;
    fn get_movement(&self) -> Vec2<f32>;
}
