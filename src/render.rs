pub mod sdl;

use nalgebra::Vector2;

use specs::{prelude::*, Component, DenseVecStorage};

use vitrellogy_macro::DefaultConstructor;
use crate::physics::TransformCom;
use crate::render::sdl::SDLRenderImpl;
use crate::misc::Convertable;
use crate::input::MouseRes;

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

#[derive(DefaultConstructor)]
pub struct RenderSys {
    renderer: SDLRenderImpl<'static>
}

impl<'a> System<'a> for RenderSys {
    type SystemData = (Read<'a, CameraRes>,
        ReadStorage<'a, TransformCom>,
        ReadStorage<'a, SpriteCom>,
        ReadStorage<'a, TextCom>,
        ReadStorage<'a, ButtonUICom>,
        ReadStorage<'a, TextUICom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, transforms, sprites, texts, buttons, text_labels) = data;

        self.renderer.pre();

        for (sprite, transform) in (&sprites, &transforms).join() {
            self.renderer.render(&sprite.name, transform.pos, sprite.dim, camera.pos, camera.zoom, camera.screen);
        }

        for (text, transform) in (&texts, &transforms).join() {
            self.renderer.write(&text.text, &text.font, transform.pos, text.dim, camera.pos, camera.zoom, camera.screen);
        }

        for button in (&buttons).join() {
            let size = Vector2::new(button.constraints.x_size.as_pixels(camera.screen.x), button.constraints.y_size.as_pixels(camera.screen.y));
            let pos = Vector2::new(button.constraints.x_pos.as_pixels(size.x, camera.screen.x), button.constraints.y_pos.as_pixels(size.y, camera.screen.y));
            self.renderer.render_ss(&button.sprite, pos, size, camera.screen);
        }

        for (text_label, transform) in (&text_labels, &transforms).join() {
            self.renderer.write_ss(&text_label.text, &text_label.font, transform.pos.convert(), text_label.dim, camera.screen);
        }

        self.renderer.post();
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

event_queue! {
    UIEventQueue: pub enum UIEvent {
        ButtonPressed { id: String }
    }
}

#[derive(DefaultConstructor)]
pub struct UISys;

impl<'a> System<'a> for UISys {
    type SystemData = (Write<'a, UIEventQueue>,
        Read<'a, CameraRes>,
        Read<'a, MouseRes>,
        ReadStorage<'a, ButtonUICom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut events, camera, mouse, buttons) = data;

        events.clear();
        for button in (&buttons).join() {
            let size = Vector2::new(button.constraints.x_size.as_pixels(camera.screen.x), button.constraints.y_size.as_pixels(camera.screen.y));
            let pos = Vector2::new(button.constraints.x_pos.as_pixels(size.x, camera.screen.x), button.constraints.y_pos.as_pixels(size.y, camera.screen.y));
            match mouse.0 {
                Some(m) if pos.x < m.x && m.x < pos.x + size.x && pos.y < m.y && m.y < pos.y + size.y => events.push(UIEvent::ButtonPressed { id: button.element_name.clone() }),
                Some(_) | None => ()
            }
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TextUICom {
    pub text: String,
    pub font: String,
    pub dim: Vector2<u32>
}

impl TextUICom {
    pub fn new(text: &str, font: &str, dim: Vector2<u32>) -> Self {
        Self {
            text: text.to_string(),
            font: font.to_string(),
            dim: dim
        }
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct ButtonUICom {
    pub constraints: Constraints,
    pub sprite: String,
    pub sprite_pressed: String,
    pub element_name: String
}

impl ButtonUICom {
    pub fn new(constraints: Constraints, sprite: &str, sprite_pressed: &str, element_name: &str) -> Self {
        Self {
            constraints: constraints,
            sprite: sprite.to_string(),
            sprite_pressed: sprite_pressed.to_string(),
            element_name: element_name.to_string()
        }
    }
}

#[derive(Debug, DefaultConstructor)]
pub struct Constraints {
    x_pos: PositionConstraint,
    y_pos: PositionConstraint,
    x_size: SizeConstraint,
    y_size: SizeConstraint
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum PositionConstraint {
    Start,
    Center,
    End,
    StartPixelOffset(u32)
}

impl PositionConstraint {
    fn as_pixels(&self, size: u32, container_size: u32) -> u32 {
        match self {
            PositionConstraint::Start => 0,
            PositionConstraint::Center => (container_size / 2) - (size / 2),
            PositionConstraint::End => container_size - size,
            PositionConstraint::StartPixelOffset(offset) => offset.clone()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum SizeConstraint {
    Proportion(f32),
    Pixels(u32)
}

impl SizeConstraint {
    fn as_pixels(&self, container_size: u32) -> u32 {
        match self {
            SizeConstraint::Proportion(proportion) => (proportion * container_size as f32).floor() as u32,
            SizeConstraint::Pixels(pixels) => pixels.clone()
        }
    }
}
