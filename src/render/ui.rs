use nalgebra::Vector2;

use specs::{System, Read, Write, ReadStorage, Join, Component, DenseVecStorage};

use vitrellogy_macro::DefaultConstructor;
use crate::physics::TransformCom;
use crate::input::MouseRes;
use crate::misc::Convertable;

#[derive(DefaultConstructor)]
pub struct UISys;

impl<'a> System<'a> for UISys {
    type SystemData = (Write<'a, UIEventQueue>,
        Read<'a, MouseRes>,
        ReadStorage<'a, ButtonUICom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut events, mouse, buttons, transforms) = data;

        events.clear();
        for (button, transform) in (&buttons, &transforms).join() {
            let pos: Vector2<u32> = transform.pos.convert();
            let dim = button.dim;
            match mouse.0 {
                Some(m) if pos.x < m.x && m.x < pos.x + dim.x && pos.y < m.y && m.y < pos.y + dim.y => events.push(UIEvent::ButtonPressed { id: button.element_name.clone() }),
                Some(_) | None => ()
            }
        }
    }
}

event_queue! {
    UIEventQueue: pub enum UIEvent {
        ButtonPressed { id: String }
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
