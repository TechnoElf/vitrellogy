use std::cmp::Ordering;

use nalgebra::Vector2;
use nphysics2d::math::{Force, ForceType};
use nphysics2d::object::Body;

use specs::{prelude::*, Component};

use vitrellogy_macro::DefaultConstructor;
use crate::physics::{PhysicsRes, RigidBodyCom, TransformCom};
use crate::input::key::{KeysRes, Key};
use crate::render::CameraRes;

#[derive(DefaultConstructor)]
pub struct ControllerSys;

impl<'a> System<'a> for ControllerSys {
    type SystemData = (Read<'a, KeysRes>,
        Write<'a, PhysicsRes>,
        Write<'a, CameraRes>,
        ReadStorage<'a, ControllerCom>,
        ReadStorage<'a, RigidBodyCom>,
        ReadStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (keys, mut physics, mut camera, controllers, rigid_bodies, transforms) = data;

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
            match physics.write_rigid_body(rigid_body) {
                Some(rb) => rb.apply_force(0, &Force::linear(Vector2::new(horizontal, vertical)), ForceType::Impulse, true),
                None => ()
            }
        }

        for (_controller, transform) in (&controllers, &transforms).join() {
            let centre = *transform.pos + Vector2::new(1.0, 1.0);
            
            match (centre.x - camera.pos.x).partial_cmp(&0.0).unwrap() {
                Ordering::Greater if centre.x - camera.pos.x > 1.0 => camera.pos.x += centre.x - camera.pos.x - 1.0,
                Ordering::Less if camera.pos.x - centre.x > 1.0 => camera.pos.x -= camera.pos.x - centre.x - 1.0,
                _ => ()
            }

            match (centre.y - camera.pos.y).partial_cmp(&0.0).unwrap() {
                Ordering::Greater if centre.y - camera.pos.y > 1.0 => camera.pos.y += centre.y - camera.pos.y - 1.0,
                Ordering::Less if camera.pos.y - centre.y > 1.0 => camera.pos.y -= camera.pos.y - centre.y - 1.0,
                _ => ()
            }
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct ControllerCom;
