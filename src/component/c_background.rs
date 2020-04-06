use std::rc::Rc;
use std::cell::RefCell;

use crate::misc::vec::*;
use crate::render::*;

pub struct PBackground<'a> {
    renderer: Rc<RefCell<dyn Renderer + 'a>>
}

impl<'a> PBackground<'a> {
    pub fn new(renderer: Rc<RefCell<dyn Renderer + 'a>>) -> Self {
        PBackground {
            renderer: renderer
        }
    }

    pub fn process(&mut self, entity: &EBackground) {
        self.renderer.borrow_mut().queue(&entity.name, entity.pos, entity.dim);
    }
}

pub struct EBackground {
    name: String,
    pos: Vec2<f32>,
    dim: Vec2<f32>
}

impl EBackground {
    pub fn new(name: &str, pos: Vec2<f32>, dim: Vec2<f32>) -> Self {
        EBackground {
            name: name.to_string(),
            pos: pos,
            dim: dim
        }
    }
}
