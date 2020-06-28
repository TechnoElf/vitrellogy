use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use specs::{System, Read, Write, ReadStorage, Join, Component, DenseVecStorage};

use crate::render::{Renderer, CameraRes};
use crate::physics::TransformCom;
use crate::misc::vec::Vec2;
use crate::input::MouseRes;

pub struct UISys<T: Renderer> {
    pub renderer: Rc<RefCell<T>>
}

impl<'a, T: Renderer> System<'a> for UISys<T> {
    type SystemData = (Read<'a, CameraRes>,
        Read<'a, MouseRes>,
        Write<'a, UIEventRes>,
        ReadStorage<'a, TextUICom>,
        ReadStorage<'a, ButtonUICom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (camera, mouse, mut events, text_labels, buttons, transforms) = data;

        for (button, transform) in (&buttons, &transforms).join() {
            let pos = transform.pos.convert();
            let dim = button.dim;
            self.renderer.borrow_mut().render_ss(&button.sprite, pos, dim, camera.screen);
            match mouse.0 {
                Some(Vec2 { x, y }) if pos.x < x && x < pos.x + dim.x && pos.y < y && y < pos.y + dim.y => events.0.push_back(UIEvent::new(&button.element_name, 1)),
                Some(_) | None => ()
            }
        }

        for (text_label, transform) in (&text_labels, &transforms).join() {
            self.renderer.borrow_mut().write_ss(&text_label.text, &text_label.font, transform.pos.convert(), text_label.dim, camera.screen);
        }

        self.renderer.borrow_mut().post();
    }
}

impl<T: Renderer> UISys<T> {
    pub fn new(renderer: Rc<RefCell<T>>) -> Self {
        Self {
            renderer: renderer
        }
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
    pub dim: Vec2<u32>
}

impl TextUICom {
    pub fn new(text: &str, font: &str, dim: Vec2<u32>) -> Self {
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
    pub dim: Vec2<u32>,
    pub element_name: String
}

impl ButtonUICom {
    pub fn new(sprite: &str, sprite_pressed: &str, dim: Vec2<u32>, element_name: &str) -> Self {
        Self {
            sprite: sprite.to_string(),
            sprite_pressed: sprite_pressed.to_string(),
            dim: dim,
            element_name: element_name.to_string()
        }
    }
}
