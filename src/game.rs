pub mod controller;

use std::net::ToSocketAddrs;

use nalgebra::Vector2;

use specs::*;

use crate::sound::{SoundRequestQueue, SoundRequest, MusicID, LayerID};
use crate::sound::imp::SoundImp;
use crate::render::{UIEventQueue, UIEvent, ConstraintCom, PositionConstraint, SizeConstraint, ButtonUICom, TextUICom, SpriteCom, TextCom, StartVerticalGroupCom, EndGroupCom};
use crate::net::{NetworkRequestQueue, NetworkRequest, NetMasterTransformCom};
use crate::physics::{TransformCom, PhysicsRes};
use crate::misc::{StateRes, AppState};
use crate::game::controller::ControllerCom;

pub struct DebugUISys {
    layer: LayerID,
    music: MusicID
}

impl<'a> System<'a> for DebugUISys {
    type SystemData = (Read<'a, UIEventQueue>,
        Write<'a, SoundRequestQueue>,
        Write<'a, NetworkRequestQueue>,
        Write<'a, StateRes>);

    fn run(&mut self, data: Self::SystemData) {
        let (ui_events, mut sound_requests, mut net_requests, mut state) = data;

        for event in ui_events.iter() {
            match event {
                UIEvent::ButtonPressed { id } => match id.as_str() {
                    "sound" => {
                        sound_requests.push(SoundRequest::ChangeMusic(self.music, self.layer));
                        self.layer += 1;
                        if self.layer > 3 {
                            self.layer = 0;
                        }
                    },
                    "connect" => {
                        net_requests.push(NetworkRequest::Open);
                        net_requests.push(NetworkRequest::Connect(("apollo.undertheprinter.com", 0).to_socket_addrs().unwrap().next().unwrap()));
                    },
                    "host" => net_requests.push(NetworkRequest::Open),
                    "debug" => net_requests.push(NetworkRequest::Debug),
                    "quit" => state.insert("app", AppState::Stopping),
                    _ => ()
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::StartPixelOffset(0), PositionConstraint::StartPixelOffset(0), SizeConstraint::Pixels(140), SizeConstraint::Proportion(1.0)))
            .with(StartVerticalGroupCom::new()).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Center, PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("r", "g", "connect"))
            .with(TextUICom::new("Connect", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Center, PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("g", "b", "host"))
            .with(TextUICom::new("Host", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Center, PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("b", "r", "debug"))
            .with(TextUICom::new("Debug", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Center, PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("r", "g", "sound"))
            .with(TextUICom::new("Sound", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Center, PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("g", "b", "quit"))
            .with(TextUICom::new("Quit", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::StartPixelOffset(0), PositionConstraint::StartPixelOffset(0), SizeConstraint::Pixels(0), SizeConstraint::Pixels(0)))
            .with(EndGroupCom::new()).build();
    }
}

impl DebugUISys {
    pub fn new(sound: &mut SoundImp) -> Self {
        Self {
            layer: 0,
            music: sound.load_music(&["assets/placeholder/music/you-are-my-hope.ogg", "assets/placeholder/music/windward.ogg", "assets/placeholder/music/baby-bird.ogg", "assets/placeholder/music/loves-vagrant.ogg"])
        }
    }
}

pub fn build_world(world: &mut World, physics: &mut PhysicsRes) {
    for pos in 0..20 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, -1.0))).build();
    }
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(20.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(0.0, -1.0)))
        .with(rb)
        .with(col).build();

    for pos in 10..15 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, 3.0))).build();
    }
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(5.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(10.0, 3.0)))
        .with(rb)
        .with(col).build();

    for pos in 4..7 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, 3.0))).build();
    }
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(3.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(4.0, 3.0)))
        .with(rb)
        .with(col).build();

    for pos in 0..18 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(0.0, pos as f32))).build();
    }
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(1.0, 18.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(0.0, 0.0)))
        .with(rb)
        .with(col).build();

    for pos in 0..18 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(19.0, pos as f32))).build();
    }
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(1.0, 18.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(19.0, 0.0)))
        .with(rb)
        .with(col).build();

    for pos in 0..20 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, 18.0))).build();
    }
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(20.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(0.0, 18.0)))
        .with(rb)
        .with(col).build();

    for pos in 3..8 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, 14.0))).build();
    }
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(5.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(3.0, 14.0)))
        .with(rb)
        .with(col).build();

    world.create_entity().with(TextCom::new("Sphinx of black quartz, judge my vow", "caveat", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(1.0, 15.0))).build();
    let rb = physics.create_rigid_body();
    let col = physics.create_collider_rectangle(Vector2::new(5.0, 1.0), &rb);
    world.create_entity().with(TextCom::new("Vitrellogy", "nemoy", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(7.0, 6.0)))
        .with(rb)
        .with(col).build();
    world.create_entity().with(TextCom::new("Vitrellogy", "patrickhand", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(1.0, 0.0))).build();

    world.create_entity().with(SpriteCom::new("tree", Vector2::new(4.0, 4.0)))
        .with(TransformCom::new(Vector2::new(4.0, 4.0))).build();
    world.create_entity().with(SpriteCom::new("tree", Vector2::new(4.0, 4.0)))
        .with(TransformCom::new(Vector2::new(11.0, 4.0))).build();

    world.create_entity().with(SpriteCom::new("bolt0", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(2.0, 5.0))).build();
    world.create_entity().with(SpriteCom::new("bolt1", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(3.0, 5.0))).build();
    world.create_entity().with(SpriteCom::new("bolt2", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(3.0, 6.0))).build();
    world.create_entity().with(SpriteCom::new("bolt3", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(2.0, 6.0))).build();

    let rb = physics.create_rigid_body();
    let col = physics.create_collider_rectangle(Vector2::new(1.9, 1.9), &rb);
    world.create_entity().with(SpriteCom::new("wizard", Vector2::new(2.0, 2.0)))
        .with(TransformCom::new(Vector2::new(0.0, 1.0)))
        .with(ControllerCom::new())
	    .with(NetMasterTransformCom::new())
        .with(rb)
        .with(col).build();
}