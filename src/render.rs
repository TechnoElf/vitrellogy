pub mod sdl;

use specs::{prelude::*, Component};
use vitrellogy_macro::DefaultConstructor;

use crate::misc::vec::Vec2;
use crate::physics::TransformCom;

pub trait Renderer {
    fn render(&mut self, sprite_name: &str, sprite_pos: Vec2<f32>, sprite_dim: Vec2<f32>, cam_pos: Vec2<f32>, cam_zoom: f32, cam_screen: Vec2<u32>);
    fn pre(&mut self);
    fn post(&mut self);
    fn add_sprite(&mut self, name: &str, file: &str);
}

pub struct RenderSys<T: Renderer> {
    pub renderer: T
}

impl<'a, T: Renderer> System<'a> for RenderSys<T> {
    type SystemData = (Read<'a, CameraRes>,
        ReadStorage<'a, SpriteCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, sprites, transforms) = data;

        self.renderer.pre();

        for (sprite, transform) in (&sprites, &transforms).join() {
            self.renderer.render(&sprite.name, transform.pos, sprite.dim, camera.pos, camera.zoom, camera.screen);
        }

        self.renderer.post();
    }
}

impl<T: Renderer> RenderSys<T> {
    pub fn new(renderer: T) -> Self {
        Self {
            renderer: renderer
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteCom {
    pub name: String,
    pub dim: Vec2<f32>
}

impl SpriteCom {
    pub fn new(name: &str, dim: Vec2<f32>) -> Self {
        Self {
            name: name.to_string(),
            dim: dim
        }
    }
}

#[derive(Default, Debug, DefaultConstructor)]
pub struct CameraRes {
    pub pos: Vec2<f32>,
    pub zoom: f32,
    pub screen: Vec2<u32>
}

#[derive(Component, Debug, DefaultConstructor)]
#[storage(VecStorage)]
pub struct CameraCom;

#[derive(DefaultConstructor)]
pub struct CameraSys;

impl<'a> System<'a> for CameraSys {
    type SystemData = (Write<'a, CameraRes>,
        ReadStorage<'a, CameraCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, coms, transforms) = data;

        for (_com, transform) in (&coms, &transforms).join() {
            camera.pos = transform.pos;
        }
    }
}
