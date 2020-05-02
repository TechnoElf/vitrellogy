pub mod colliders;
pub mod controller;
pub mod forces;

use specs::{NullStorage, VecStorage, Component, ReadStorage, WriteStorage, Read, Entities, System, Join, Entity};
use vitrellogy_macro::DefaultConstructor;

use crate::misc::vec::Vec2;
use crate::physics::colliders::{ColliderAABBCom, aabb_intersection};

#[derive(Component, Debug, Clone, DefaultConstructor)]
#[storage(VecStorage)]
pub struct TransformCom {
    pub pos: Vec2<f32>,
    pub vel: Vec2<f32>
}

impl TransformCom {
    pub fn new_pos(pos: Vec2<f32>) -> Self {
        Self {
            pos: pos,
            vel: Vec2::new(0.0, 0.0)
        }
    }

    pub fn new_null() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0)
        }
    }
}

#[derive(Default, Debug, DefaultConstructor)]
pub struct DeltaTimeRes(pub f32);

pub struct PhysicsSys {
    update: Vec<(Entity, Vec2<f32>, bool, bool)>
}

impl<'a> System<'a> for PhysicsSys {
    type SystemData = (Entities<'a>,
        Read<'a, DeltaTimeRes>,
        WriteStorage<'a, TransformCom>,
        ReadStorage<'a, ColliderAABBCom>,
        ReadStorage<'a, DynamicCom>,
        ReadStorage<'a, KinematicCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, delta_time, mut transforms, colliders_aabb, dynamic_flags, kinematic_flags) = data;
        let delta_time = delta_time.0;

        self.update.clear();

        for (entity, collider_aabb, transform, _dynamic) in (&entities, &colliders_aabb, &transforms, &dynamic_flags).join() {
            let new_pos = transform.pos + (transform.vel * delta_time);
            let new_pos_x = transform.pos + (Vec2::new(transform.vel.x, 0.0) * delta_time);
            let new_pos_y = transform.pos + (Vec2::new(0.0, transform.vel.y) * delta_time);

            let mut intersecting = (false, false);
            for (other_entity, other_collider_aabb, other_transform) in (&entities, &colliders_aabb, &transforms).join() {
                if entity != other_entity && aabb_intersection(new_pos, collider_aabb.dim, other_transform.pos, other_collider_aabb.dim) {
                    intersecting.0 |= aabb_intersection(new_pos_x, collider_aabb.dim, other_transform.pos, other_collider_aabb.dim);
                    intersecting.1 |= aabb_intersection(new_pos_y, collider_aabb.dim, other_transform.pos, other_collider_aabb.dim);
                }
            }

            match intersecting {
                (false, false) => self.update.push((entity, new_pos, false, false)),
                (false, true) => self.update.push((entity, new_pos_x, false, true)),
                (true, false) => self.update.push((entity, new_pos_y, true, false)),
                (true, true) => ()
            }
        }

        for (entity, new_pos, int_x, int_y) in &self.update {
            let mut transform = transforms.entry(*entity).unwrap().or_insert(TransformCom::new_null());
            transform.pos = *new_pos;
            if *int_x {
                transform.vel.x = 0.0;
            }
            if *int_y {
                transform.vel.y = 0.0;
            }
        }

        for (transform, _kinematic) in (&mut transforms, &kinematic_flags).join() {
            transform.pos += transform.vel * delta_time;
        }
    }
}

impl PhysicsSys {
    pub fn new() -> Self {
        Self {
            update: Vec::new()
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct DynamicCom;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct KinematicCom;
