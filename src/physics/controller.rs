use nalgebra::Vector2;
use nphysics2d::math::{Force, ForceType};
use nphysics2d::object::Body;

use specs::{prelude::*, Component};

use vitrellogy_macro::DefaultConstructor;
use crate::physics::{PhysicsRes, RigidBodyCom};
use crate::input::key::{KeysRes, Key};

#[derive(DefaultConstructor)]
pub struct ControllerSys;

impl<'a> System<'a> for ControllerSys {
    type SystemData = (Read<'a, KeysRes>,
        Write<'a, PhysicsRes>,
        ReadStorage<'a, ControllerCom>,
        ReadStorage<'a, RigidBodyCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (keys, mut physics, controllers, rigid_bodies) = data;

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

        for (_controller, rigid_body) in (&controllers, &rigid_bodies).join() {
            physics.write_rigid_body(rigid_body).apply_force(0, &Force::linear(Vector2::new(horizontal, vertical)), ForceType::Impulse, true);
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct ControllerCom;
