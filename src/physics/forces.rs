use specs::{NullStorage, Component, System, WriteStorage, ReadStorage, Join, Read};
use vitrellogy_macro::DefaultConstructor;

use crate::misc::vec::Vec2;
use crate::physics::{TransformCom, DeltaTimeRes};

#[derive(DefaultConstructor)]
pub struct ForcesSys;

impl<'a> System<'a> for ForcesSys {
    type SystemData = (WriteStorage<'a, TransformCom>,
        Read<'a, DeltaTimeRes>,
        ReadStorage<'a, GravityCom>,
        ReadStorage<'a, DragCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut transforms, delta_time, gravity_flags, drag_flags) = data;
        let delta_time = delta_time.0;

        for (transform, _drag) in (&mut transforms, &drag_flags).join() {
            transform.vel *= Vec2::new(0.90, 0.99);
        }

        for (transform, _gravity) in (&mut transforms, &gravity_flags).join() {
            transform.vel += Vec2::new(0.0, -10.0) * delta_time;
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct GravityCom;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct DragCom;
