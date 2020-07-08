pub mod ui;

pub mod sdl;

use std::rc::Rc;
use std::cell::RefCell;

use nalgebra::Vector2;

use specs::{prelude::*, Component};

use vitrellogy_macro::DefaultConstructor;
use crate::misc::TransformCom;

pub trait Renderer {
    fn render(&mut self, sprite_name: &str, sprite_pos: Vector2<f32>, sprite_dim: Vector2<f32>, cam_pos: Vector2<f32>, cam_zoom: f32, cam_screen: Vector2<u32>);
    fn write(&mut self, text: &str, font: &str, text_pos: Vector2<f32>, text_dim: Vector2<f32>, cam_pos: Vector2<f32>, cam_zoom: f32, cam_screen: Vector2<u32>);
    fn render_ss(&mut self, sprite_name: &str, sprite_pos: Vector2<u32>, sprite_dim: Vector2<u32>, cam_screen: Vector2<u32>);
    fn write_ss(&mut self, text: &str, font: &str, text_pos: Vector2<u32>, text_dim: Vector2<u32>, cam_screen: Vector2<u32>);
    fn pre(&mut self);
    fn post(&mut self);
    fn add_sprite(&mut self, name: &str, file: &str);
    fn add_font(&mut self, name: &str, file: &str, size: u16, red: u8, green: u8, blue: u8);
}

pub struct SpriteRenderSys<T: Renderer> {
    pub renderer: Rc<RefCell<T>>
}

impl<'a, T: Renderer> System<'a> for SpriteRenderSys<T> {
    type SystemData = (Read<'a, CameraRes>,
        ReadStorage<'a, SpriteCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, sprites, transforms) = data;

        self.renderer.borrow_mut().pre();

        for (sprite, transform) in (&sprites, &transforms).join() {
            self.renderer.borrow_mut().render(&sprite.name, transform.pos, sprite.dim, camera.pos, camera.zoom, camera.screen);
        }
    }
}

impl<T: Renderer> SpriteRenderSys<T> {
    pub fn new(renderer: Rc<RefCell<T>>) -> Self {
        Self {
            renderer: renderer
        }
    }
}

pub struct TextRenderSys<T: Renderer> {
    pub renderer: Rc<RefCell<T>>
}

impl<'a, T: Renderer> System<'a> for TextRenderSys<T> {
    type SystemData = (Read<'a, CameraRes>,
        ReadStorage<'a, TextCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, text_fields, transforms) = data;

        for (text_field, transform) in (&text_fields, &transforms).join() {
            self.renderer.borrow_mut().write(&text_field.text, &text_field.font, transform.pos, text_field.dim, camera.pos, camera.zoom, camera.screen);
        }
    }
}

impl<T: Renderer> TextRenderSys<T> {
    pub fn new(renderer: Rc<RefCell<T>>) -> Self {
        Self {
            renderer: renderer
        }
    }
}

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

#[derive(Component, Debug, DefaultConstructor)]
#[storage(HashMapStorage)]
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
