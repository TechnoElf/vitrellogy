use std::rc::Rc;
use std::cell::RefCell;

use crate::misc::world::*;
use crate::misc::vec::*;
use crate::render::*;
use crate::input::*;

pub struct PPlayer<'a> {
    renderer: Rc<RefCell<dyn Renderer + 'a>>,
    input: Rc<RefCell<dyn Input + 'a>>
}

impl<'a> PPlayer<'a> {
    pub fn new(renderer: Rc<RefCell<dyn Renderer + 'a>>, input: Rc<RefCell<dyn Input + 'a>>) -> Self {
        Self {
            renderer: renderer,
            input: input
        }
    }

    pub fn process(&mut self, entity: &mut EPlayer, delta_time: f32, state: &mut WorldState) {
        if state.get("game_state") == "in_game" {
            if entity.local {
                self.renderer.borrow_mut().camera(entity.pos + (entity.dim / 2.0), 1.0, self.input.borrow_mut().get_win_dim());
            }

            entity.vel += self.input.borrow_mut().get_movement();
            entity.vel = entity.vel * 0.9;

            entity.pos += entity.vel * delta_time;

            self.renderer.borrow_mut().queue(&entity.sprite, entity.pos, entity.dim);
        }
    }
}

pub struct EPlayer {
    local: bool,
    pos: Vec2<f32>,
    vel: Vec2<f32>,
    sprite: String,
    dim: Vec2<f32>
}

impl EPlayer {
    pub fn new(sprite: &str, dim: Vec2<f32>, pos: Vec2<f32>) -> Self {
        Self {
            local: true,
            pos: pos,
            vel: Vec2::new(0.0, 0.0),
            sprite: sprite.to_string(),
            dim: dim
        }
    }
}
