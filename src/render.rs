pub mod ui;

pub mod sdl;

use nalgebra::Vector2;

use specs::{prelude::*, Component};

use vitrellogy_macro::DefaultConstructor;
use crate::misc::TransformCom;
use crate::render::sdl::SDLRenderImpl;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SpriteCom {
    pub name: String,
    pub dim: Vector2<f32>
}

impl SpriteCom {
    pub fn new(name: &str, dim: Vector2<f32>) -> Self {
        Self {
            name: name.to_string(),
            dim: dim
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TextCom {
    pub text: String,
    pub font: String,
    pub dim: Vector2<f32>
}

impl TextCom {
    pub fn new(text: &str, font: &str, dim: Vector2<f32>) -> Self {
        Self {
            text: text.to_string(),
            font: font.to_string(),
            dim: dim
        }
    }
}

#[derive(Component, Debug, DefaultConstructor)]
#[storage(HashMapStorage)]
pub struct CameraCom {
    pub offset: Vector2<f32>
}

#[derive(DefaultConstructor)]
pub struct SpriteRenderSys;

impl<'a> System<'a> for SpriteRenderSys {
    type SystemData = (Read<'a, CameraRes>,
        WriteExpect<'a, RenderRes>,
        ReadStorage<'a, SpriteCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, mut renderer, sprites, transforms) = data;

        renderer.pre();

        for (sprite, transform) in (&sprites, &transforms).join() {
            renderer.render(&sprite.name, transform.pos, sprite.dim, camera.pos, camera.zoom, camera.screen);
        }
    }
}

#[derive(DefaultConstructor)]
pub struct TextRenderSys;

impl<'a> System<'a> for TextRenderSys {
    type SystemData = (Read<'a, CameraRes>,
        WriteExpect<'a, RenderRes>,
        ReadStorage<'a, TextCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, mut renderer, text_fields, transforms) = data;

        for (text_field, transform) in (&text_fields, &transforms).join() {
            renderer.write(&text_field.text, &text_field.font, transform.pos, text_field.dim, camera.pos, camera.zoom, camera.screen);
        }
    }
}

#[derive(DefaultConstructor)]
pub struct CameraSys;

impl<'a> System<'a> for CameraSys {
    type SystemData = (Write<'a, CameraRes>,
        ReadStorage<'a, CameraCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, coms, transforms) = data;

        for (com, transform) in (&coms, &transforms).join() {
            camera.pos = transform.pos + com.offset;
        }
    }
}

pub struct RenderRes {
    pub renderer: SDLRenderImpl<'static>
}

// Do NOT, and I repeat, DO NOT attempt to send -
unsafe impl Send for RenderRes {}
// - OR SYNC!
unsafe impl Sync for RenderRes {}

impl RenderRes {
    pub fn render(&mut self, sprite_name: &str, sprite_pos: Vector2<f32>, sprite_dim: Vector2<f32>, cam_pos: Vector2<f32>, cam_zoom: f32, cam_screen: Vector2<u32>) {
        self.renderer.render(sprite_name, sprite_pos, sprite_dim, cam_pos, cam_zoom, cam_screen);
    }

    pub fn write(&mut self, text: &str, font: &str, text_pos: Vector2<f32>, text_dim: Vector2<f32>, cam_pos: Vector2<f32>, cam_zoom: f32, cam_screen: Vector2<u32>) {
        self.renderer.write(text, font, text_pos, text_dim, cam_pos, cam_zoom, cam_screen);
    }

    pub fn render_ss(&mut self, sprite_name: &str, sprite_pos: Vector2<u32>, sprite_dim: Vector2<u32>, cam_screen: Vector2<u32>)  {
        self.renderer.render_ss(sprite_name, sprite_pos, sprite_dim, cam_screen);
    }

    pub fn write_ss(&mut self, text: &str, font: &str, text_pos: Vector2<u32>, text_dim: Vector2<u32>, cam_screen: Vector2<u32>) {
        self.renderer.write_ss(text, font, text_pos, text_dim, cam_screen);
    }

    pub fn pre(&mut self) {
        self.renderer.pre();
    }

    pub fn post(&mut self) {
        self.renderer.post();
    }

    pub fn add_sprite(&mut self, name: &str, file: &str) {
        self.renderer.add_sprite(name, file);
    }

    pub fn add_font(&mut self, name: &str, file: &str, size: u16, red: u8, green: u8, blue: u8) {
        self.renderer.add_font(name, file, size, red, green, blue);
    }

}

#[derive(Debug, DefaultConstructor)]
pub struct CameraRes {
    pub pos: Vector2<f32>,
    pub zoom: f32,
    pub screen: Vector2<u32>
}

impl Default for CameraRes {
    fn default() -> Self {
        Self {
            pos: Vector2::new(0.0, 0.0),
            zoom: 1.0,
            screen: Vector2::new(800, 600)
        }
    }
}
