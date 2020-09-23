use std::fs::File;
use std::io::{Write, Read};

use serde::{Serialize, Deserialize, ser::Serializer, ser::SerializeSeq};

use nphysics2d::object::*;
use ncollide2d::shape::*;
use nalgebra::geometry::*;

use specs::*;
use specs::saveload::*;

use crate::physics::{TransformCom, RigidBodyCom, ColliderCom, PhysicsRes};
use crate::render::SpriteCom;
use crate::misc::Vector;

pub struct StageMarkerType;
pub type StageMarker = SimpleMarker<StageMarkerType>;
pub type StageMarkerAllocator = SimpleMarkerAllocator<StageMarkerType>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StageEntity {
    transform: Option<TransformCom>,
    sprite: Option<SpriteCom>,
    body: Option<PersistentRigidBody>,
    collider: Option<PersistentCollider>
}

pub fn load_stage(file: &str, world: &mut World) {
    let mut file = File::open(file).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let (entities, mut markers, mut allocator, mut physics, mut transforms, mut sprites, mut bodies, mut colliders): (Entities, WriteStorage<StageMarker>, specs::Write<StageMarkerAllocator>, specs::Write<PhysicsRes>, WriteStorage<TransformCom>, WriteStorage<SpriteCom>, WriteStorage<RigidBodyCom>, WriteStorage<ColliderCom>) = world.system_data();

    let elements: Vec<StageEntity> = ron::from_str(&data).unwrap();
    for element in elements.iter() {
        let entity = entities.create();

        let mut rb: Option<DefaultBodyHandle> = None;

        if let Some(transform) = &element.transform {
            transforms.insert(entity, transform.clone()).unwrap();
        }
        if let Some(sprite) = &element.sprite {
            sprites.insert(entity, sprite.clone()).unwrap();
        }
        if let Some(body) = &element.body {
            let com = physics.register_rigid_body(body.clone().into());
            rb = Some(com.0);
            bodies.insert(entity, com).unwrap();
        }
        if let Some(collider) = &element.collider {
            if let Some(rb) = rb {
                colliders.insert(entity, physics.register_collider(collider.clone().into_collider(&rb))).unwrap();
            }
        }

        allocator.mark(entity, &mut markers);
    }
}

pub fn save_stage(file: &str, world: &mut World) {
    let mut data: Vec<u8> = Vec::new();
    let mut ser = ron::Serializer::new(&mut data, None, false).unwrap();
    let (markers, physics, transforms, sprites, bodies, colliders): (ReadStorage<StageMarker>, specs::Read<PhysicsRes>, ReadStorage<TransformCom>, ReadStorage<SpriteCom>, ReadStorage<RigidBodyCom>, ReadStorage<ColliderCom>) = world.system_data();

    let mut seq = ser.serialize_seq(None).unwrap();
    for (_marker, transform, sprite, body, collider) in (&markers, (&transforms).maybe(), (&sprites).maybe(), (&bodies).maybe(), (&colliders).maybe()).join() {
        seq.serialize_element(&StageEntity {
            transform: transform.map(|c| c.clone()),
            sprite: sprite.map(|c| c.clone()),
            body: body.map(|c| physics.read_rigid_body(c).unwrap().into()),
            collider: collider.map(|c| physics.read_collider(c).unwrap().into())
        }).unwrap();
    }
    seq.end().unwrap();

    let data = String::from_utf8(data).unwrap();
    let mut file = File::create(file).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentRigidBody {
    #[serde(with = "BodyStatusDef")]
    status: BodyStatus
}

impl From<&RigidBody<f32>> for PersistentRigidBody {
    fn from(rb: &RigidBody<f32>) -> Self {
        Self {
            status: rb.status()
        }
    }
}

impl Into<RigidBody<f32>> for PersistentRigidBody {
    fn into(self) -> RigidBody<f32> {
        RigidBodyDesc::new().status(self.status.into()).build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentCollider {
    shape: ShapeDef,
    position: IsometryDef
}

impl From<&Collider<f32, DefaultBodyHandle>> for PersistentCollider {
    fn from(c: &Collider<f32, DefaultBodyHandle>) -> Self {
        Self {
            shape: c.shape_handle().into(),
            position: c.position_wrt_body().into()
        }
    }
}

impl PersistentCollider {
    fn into_collider(self, rb: &DefaultBodyHandle) -> Collider<f32, DefaultBodyHandle> {
        ColliderDesc::new(self.shape.into()).position(self.position.into()).build(BodyPartHandle(rb.clone(), 0))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "nphysics2d::object::BodyStatus")]
enum BodyStatusDef {
    Disabled,
    Static,
    Dynamic,
    Kinematic
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ShapeDef {
    Cuboid(CuboidDef)
}

impl From<&ShapeHandle<f32>> for ShapeDef {
    fn from(s: &ShapeHandle<f32>) -> Self {
        if s.is_shape::<Cuboid<f32>>() {
            ShapeDef::Cuboid(s.as_shape::<Cuboid<f32>>().unwrap().into())
        } else {
            panic!("attempted to serialize unknown shape");
        }
    }
}

impl Into<ShapeHandle<f32>> for ShapeDef {
    fn into(self) -> ShapeHandle<f32> {
        match self {
            ShapeDef::Cuboid(s) => ShapeHandle::new::<Cuboid<f32>>(s.into())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CuboidDef {
    half_extents: Vector
}

impl From<&Cuboid<f32>> for CuboidDef {
    fn from(c: &Cuboid<f32>) -> Self {
        Self {
            half_extents: c.half_extents.into()
        }
    }
}

impl Into<Cuboid<f32>> for CuboidDef {
    fn into(self) -> Cuboid<f32> {
        Cuboid::new(*self.half_extents)
    }
}

type Isometryf = nphysics2d::math::Isometry<f32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IsometryDef {
    translation: Vector,
    rotation: f32
}

impl From<Isometryf> for IsometryDef {
    fn from(i: Isometryf) -> Self {
        Self {
            translation: i.translation.vector.into(),
            rotation: i.rotation.angle()
        }
    }
}

impl Into<Isometryf> for IsometryDef {
    fn into(self) -> Isometryf {
        Isometryf::from_parts((*self.translation).into(), UnitComplex::from_angle(self.rotation))
    }
}
