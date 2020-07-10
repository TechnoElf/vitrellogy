use std::collections::VecDeque;

use nalgebra::Vector2;

use specs::{System, Read, Write, ReadStorage, Join, Component, DenseVecStorage, WriteExpect};

use vitrellogy_macro::DefaultConstructor;
use crate::render::{RenderRes, CameraRes};
use crate::misc::TransformCom;
use crate::input::MouseRes;
use crate::misc::Convertable;

#[derive(DefaultConstructor)]
pub struct UISys;

impl<'a> System<'a> for UISys {
    type SystemData = (Read<'a, CameraRes>,
        Read<'a, MouseRes>,
        Write<'a, UIEventRes>,
        WriteExpect<'a, RenderRes>,
        ReadStorage<'a, TextUICom>,
        ReadStorage<'a, ButtonUICom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, mouse, mut events, mut renderer, text_labels, buttons, transforms) = data;

        for (button, transform) in (&buttons, &transforms).join() {
            let pos = transform.pos.convert();
            let dim = button.dim;
            renderer.render_ss(&button.sprite, pos, dim, camera.screen);
            match mouse.0 {
                Some(m) if pos.x < m.x && m.x < pos.x + dim.x && pos.y < m.y && m.y < pos.y + dim.y => events.0.push_back(UIEvent::new(&button.element_name, 1)),
                Some(_) | None => ()
            }
        }

        for (text_label, transform) in (&text_labels, &transforms).join() {
            renderer.write_ss(&text_label.text, &text_label.font, transform.pos.convert(), text_label.dim, camera.screen);
        }

        renderer.post();
    }
}

#[derive(Default, Debug)]
pub struct UIEvent {
    pub element_name: String,
    pub value: u32
}

impl UIEvent {
    pub fn new(element_name: &str, value: u32) -> Self {
        Self {
            element_name: element_name.to_string(),
            value: value
        }
    }
}

#[derive(Default, Debug)]
pub struct UIEventRes (pub VecDeque<UIEvent>);

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
    pub sprite: String,
    pub sprite_pressed: String,
    pub dim: Vector2<u32>,
    pub element_name: String
}

impl ButtonUICom {
    pub fn new(sprite: &str, sprite_pressed: &str, dim: Vector2<u32>, element_name: &str) -> Self {
        Self {
            sprite: sprite.to_string(),
            sprite_pressed: sprite_pressed.to_string(),
            dim: dim,
            element_name: element_name.to_string()
        }
    }
}
