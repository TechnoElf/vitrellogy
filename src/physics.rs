use specs::{prelude::*, Component};
use vitrellogy_macro::DefaultConstructor;

use crate::misc::vec::Vec2;

#[derive(Component, Debug, DefaultConstructor)]
#[storage(VecStorage)]
pub struct TransformCom {
    pub pos: Vec2<f32>
}

#[derive(Default, Debug, DefaultConstructor)]
pub struct DeltaTimeRes(pub f32);
