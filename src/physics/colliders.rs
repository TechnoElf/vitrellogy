use specs::{DenseVecStorage, Component};
use vitrellogy_macro::DefaultConstructor;

use crate::misc::vec::Vec2;

#[derive(Component, Debug, DefaultConstructor)]
#[storage(DenseVecStorage)]
pub struct ColliderAABBCom {
    pub dim: Vec2<f32>
}

pub fn aabb_intersection(pos_a: Vec2<f32>, dim_a: Vec2<f32>, pos_b: Vec2<f32>, dim_b: Vec2<f32>) -> bool {
    let pos_a = pos_a + (dim_a / 2.0);
    let pos_b = pos_b + (dim_b / 2.0);
    let delta_pos = (pos_b - pos_a).abs();
    let total_dim = (dim_a / 2.0) + (dim_b / 2.0);

    delta_pos.x <= total_dim.x && delta_pos.y <= total_dim.y
}
