use specs::{prelude::*, Component};
use std::f32::consts::FRAC_1_SQRT_2;

use crate::misc::vec::Vec2;
use crate::physics::{TransformCom, DeltaTimeRes};
use crate::input::key::{KeysRes, Key};

pub struct ControllerSys;

impl<'a> System<'a> for ControllerSys {
    type SystemData = (Read<'a, DeltaTimeRes>,
        Read<'a, KeysRes>,
        ReadStorage<'a, ControllerCom>,
        WriteStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (delta_time, keys, controllers, mut transforms) = data;
        let delta_time = delta_time.0;

        let movement = match (keys.pressed(Key::W), keys.pressed(Key::S), keys.pressed(Key::D), keys.pressed(Key::A)) {
            (true, true, true, true) => Vec2::new(0.0, 0.0),
            (true, true, true, false) => Vec2::new(1.0, 0.0),
            (true, true, false, true) => Vec2::new(-1.0, 0.0),
            (true, true, false, false) => Vec2::new(0.0, 0.0),
            (true, false, true, true) => Vec2::new(0.0, 1.0),
            (true, false, true, false) => Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (true, false, false, true) => Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            (true, false, false, false) => Vec2::new(0.0, 1.0),
            (false, true, true, true) => Vec2::new(0.0, -1.0),
            (false, true, true, false) => Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            (false, true, false, true) => Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            (false, true, false, false) => Vec2::new(0.0, -1.0),
            (false, false, true, true) => Vec2::new(0.0, 0.0),
            (false, false, true, false) => Vec2::new(1.0, 0.0),
            (false, false, false, true) => Vec2::new(-1.0, 0.0),
            (false, false, false, false) => Vec2::new(0.0, 0.0)
        };

        for (_controller, transform) in (&controllers, &mut transforms).join() {
            transform.pos += movement * delta_time;
        }
    }
}

impl ControllerSys {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ControllerCom;

impl ControllerCom {
    pub fn new() -> Self {
        Self {}
    }
}
