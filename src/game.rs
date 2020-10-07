pub mod controller;

use std::net::ToSocketAddrs;

use specs::*;

use crate::sound::{SoundRequestQueue, SoundRequest, MusicID, LayerID};
use crate::sound::imp::SoundImp;
use crate::render::{UIEventQueue, UIEvent, ConstraintCom, PositionConstraint, SizeConstraint, ButtonUICom, TextUICom, SpriteCom, StartVerticalGroupCom, EndGroupCom, TextFieldUICom};
use crate::net::{NetworkRequestQueue, NetworkRequest, NetMasterTransformCom};
use crate::physics::{TransformCom, PhysicsRes};
use crate::misc::{StateRes, AppState, Vector};
use crate::game::controller::ControllerCom;
use crate::misc::persist::{PersistRequestQueue, PersistRequest};

pub struct DebugUISys {
    layer: LayerID,
    music: MusicID,
    file: String
}

impl<'a> System<'a> for DebugUISys {
    type SystemData = (Read<'a, UIEventQueue>,
        Write<'a, SoundRequestQueue>,
        Write<'a, NetworkRequestQueue>,
        Write<'a, PersistRequestQueue>,
        Write<'a, StateRes>);

    fn run(&mut self, data: Self::SystemData) {
        let (ui_events, mut sound_requests, mut net_requests, mut persist_requests, mut state) = data;

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
                    "load" => persist_requests.push(PersistRequest::LoadStage(format!("assets/placeholder/stages/{}.mst", self.file))),
                    "save" => persist_requests.push(PersistRequest::SaveStage(format!("assets/placeholder/stages/{}.mst", self.file))),
                    _ => ()
                }
                UIEvent::TextChanged { id, text } => match id.as_str() {
                    "file" => self.file = text.clone(),
                    _ => ()
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(0), SizeConstraint::Pixels(200), SizeConstraint::Proportion(1.0)))
            .with(StartVerticalGroupCom::new()).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("r", "g", "connect"))
            .with(TextUICom::new("Connect", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("g", "b", "host"))
            .with(TextUICom::new("Host", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("b", "r", "debug"))
            .with(TextUICom::new("Debug", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("r", "g", "sound"))
            .with(TextUICom::new("Sound", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("g", "b", "quit"))
            .with(TextUICom::new("Quit", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("b", "r", "load"))
            .with(TextUICom::new("Load", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)))
            .with(ButtonUICom::new("r", "g", "save"))
            .with(TextUICom::new("Save", "caveat")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(200), SizeConstraint::Pixels(50)))
            .with(TextFieldUICom::new("g", "save", "caveat", "file")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::StartPixelOffset(0), PositionConstraint::StartPixelOffset(0), SizeConstraint::Pixels(0), SizeConstraint::Pixels(0)))
            .with(EndGroupCom::new()).build();
    }
}

impl DebugUISys {
    pub fn new(sound: &mut SoundImp) -> Self {
        Self {
            layer: 0,
            music: sound.load_music(&["assets/placeholder/music/you-are-my-hope.ogg", "assets/placeholder/music/windward.ogg", "assets/placeholder/music/baby-bird.ogg", "assets/placeholder/music/loves-vagrant.ogg"]),
            file: "save".to_string()
        }
    }
}

pub fn build_world(world: &mut World, physics: &mut PhysicsRes) {
    let rb = physics.create_rigid_body();
    let col = physics.create_collider_rectangle(Vector::new(1.9, 1.9), Vector::new(0.05, 0.05), &rb);
    world.create_entity().with(SpriteCom::new("wizard", Vector::new(2.0, 2.0)))
        .with(TransformCom::new(Vector::new(0.0, 1.0)))
        .with(ControllerCom::new())
	    .with(NetMasterTransformCom::new())
        .with(rb)
        .with(col).build();
}
