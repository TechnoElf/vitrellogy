use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::render::*;
use crate::input::*;

use crate::component::c_background::*;
use crate::component::c_player::*;

pub struct World<'a> {
    state: WorldState,

    p_background: PBackground<'a>,
    p_player: PPlayer<'a>,

    e_background: Vec<EBackground>,
    e_player: Vec<EPlayer>
}

impl<'a> World<'a> {
    pub fn new(renderer: Rc<RefCell<dyn Renderer + 'a>>, input: Rc<RefCell<dyn Input + 'a>>) -> Self {
        World {
            state: WorldState::new(),

            p_background: PBackground::new(renderer.clone()),
            p_player: PPlayer::new(renderer.clone(), input.clone()),

            e_background: Vec::new(),
            e_player: Vec::new()
        }
    }

    pub fn set_state(&mut self, key: &str, val: &str) {
        self.state.set(key, val);
    }

    pub fn process(&mut self, delta_time: f32) {
        for entity in &self.e_background {
            self.p_background.process(entity);
        }

        for entity in &mut self.e_player {
            self.p_player.process(entity, delta_time, &mut self.state);
        }
    }

    pub fn add_e_background(&mut self, entity: EBackground) {
        self.e_background.push(entity);
    }

    pub fn add_e_player(&mut self, entity: EPlayer) {
        self.e_player.push(entity);
    }
}

pub struct WorldState {
    state: HashMap<String, String>
}

impl WorldState {
    fn new() -> Self {
        Self {
            state: HashMap::new()
        }
    }

    pub fn set(&mut self, key: &str, val: &str) {
        self.state.insert(key.to_string(), val.to_string());
    }

    pub fn get(&self, key: &str) -> &str {
        match self.state.contains_key(key) {
            true => &self.state[key],
            false => &""
        }
    }
}
