pub mod sdl;

use crate::misc::vec::*;

pub trait Renderer {
    fn render(&mut self);
    fn camera(&mut self, pos: Vec2<f32>, zoom: f32, screen: Vec2<u32>);
    fn queue(&mut self, name: &str, pos: Vec2<f32>, dim: Vec2<f32>);
    fn add_sprite(&mut self, name: &str, file: &str);
}
