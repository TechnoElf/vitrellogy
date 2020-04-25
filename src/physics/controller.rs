use specs::{prelude::*, Component};
use vitrellogy_macro::DefaultConstructor;

use crate::misc::vec::Vec2;
use crate::physics::TransformCom;
use crate::input::key::{KeysRes, Key};

#[derive(DefaultConstructor)]
pub struct ControllerSys;

impl<'a> System<'a> for ControllerSys {
    type SystemData = (Read<'a, KeysRes>,
        ReadStorage<'a, ControllerCom>,
        WriteStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (keys, controllers, mut transforms) = data;

        let horizontal = match (keys.pressed(Key::D), keys.pressed(Key::A)) {
            (true, true) => 0.0,
            (true, false) => 1.0,
            (false, true) => -1.0,
            (false, false) => 0.0,
        };

        let vertical = match keys.pressed(Key::Space) {
            true => 1.0,
            false => 0.0,
        };

        for (_controller, transform) in (&controllers, &mut transforms).join() {
            transform.vel += Vec2::new(horizontal, vertical);
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct ControllerCom;
