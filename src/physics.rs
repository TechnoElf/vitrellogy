use specs::{prelude::*, Component};

use crate::misc::vec::Vec2;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TransformCom {
    pub pos: Vec2<f32>
}

impl TransformCom {
    pub fn new(pos: Vec2<f32>) -> Self {
        Self {
            pos: pos
        }
    }
}

#[derive(Default, Debug)]
pub struct DeltaTimeRes(pub f32);

impl DeltaTimeRes {
    pub fn new(delta_time: f32) -> Self {
        Self(delta_time)
    }
}
