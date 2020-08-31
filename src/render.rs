pub mod ui;
pub mod sdl;

use nalgebra::Vector2;

use specs::{prelude::*, Component};

use vitrellogy_macro::DefaultConstructor;
use crate::physics::TransformCom;
use crate::render::sdl::SDLRenderImpl;
use crate::render::ui::{ButtonUICom, TextUICom};
use crate::misc::Convertable;

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

        for (button, transform) in (&buttons, &transforms).join() {
            self.renderer.render_ss(&button.sprite, transform.pos.convert(), button.dim, camera.screen);
        }

        for (text_label, transform) in (&text_labels, &transforms).join() {
            self.renderer.write_ss(&text_label.text, &text_label.font, transform.pos.convert(), text_label.dim, camera.screen);
        }

        self.renderer.post();
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
