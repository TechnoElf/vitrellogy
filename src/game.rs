use std::net::ToSocketAddrs;
use std::cmp::Ordering;

use invader::macros::*;
use invader::ecs::*;
use invader::misc::*;
use invader::misc::persist::*;
use invader::render::*;
use invader::physics::*;
use invader::net::*;
use invader::input::*;
use invader::input::key::*;

#[derive(DefaultConstructor)]
pub struct GameManagerSys;

impl<'a> System<'a> for GameManagerSys {
    type SystemData = (Entities<'a>,
        WriteResource<'a, StateRes>,
        ReadResource<'a, UIEventQueue>,
        WriteResource<'a, PhysicsRes>,
        WriteResource<'a, CameraRes>,
        WriteStorage<'a, TransformCom>,
        WriteStorage<'a, SpriteCom>,
        WriteStorage<'a, PlayerMarker>,
        WriteStorage<'a, NetMasterTransformCom>,
        WriteStorage<'a, RigidBodyCom>,
        WriteStorage<'a, ColliderCom>,
        WriteStorage<'a, TitleUIMarker>,
        WriteStorage<'a, ConstraintCom>,
        WriteStorage<'a, ButtonUICom>,
        WriteStorage<'a, TextUICom>);

    fn run(&mut self, (entities, mut state, ui_events, mut physics, mut camera, mut transforms, mut sprites, mut player_markers, mut net_master_transform_markers, mut bodies, mut colliders, mut title_ui_markers, mut ui_constraints, mut ui_buttons, mut ui_text_labels): Self::SystemData) {
        for event in ui_events.iter() {
            match event {
                UIEvent::ButtonPressed { id } => match id.as_str() {
                    "title_start" => {
                        let entity = entities.create();

                        sprites.insert(entity, SpriteCom::new("wizard", Vector::new(2.0, 2.0))).unwrap();
                        transforms.insert(entity, TransformCom::new(Vector::new(1.0, 1.0))).unwrap();
                        player_markers.insert(entity, PlayerMarker).unwrap();
	                    net_master_transform_markers.insert(entity, NetMasterTransformCom::new()).unwrap();
                        let rb = physics.create_rigid_body();
                        let col = physics.create_collider_rectangle(Vector::new(1.9, 1.9), Vector::new(0.05, 0.05), &rb);
                        bodies.insert(entity, rb).unwrap();
                        colliders.insert(entity, col).unwrap();

                        for (entity, _marker) in (&entities, &title_ui_markers).join() {
                            ui_constraints.remove(entity);
                            ui_buttons.remove(entity);
                            ui_text_labels.remove(entity);
                        }
                        title_ui_markers.clear();

                        state.insert("game", GameState::Hub);
                        state.insert("title_anim", AnimationState::AB(0.0));
                    },
                    _ => ()
                },
                _ => ()
            }
        }

        match state.get::<AnimationState>("title_anim").unwrap().clone() {
            AnimationState::A => {
                camera.zoom = 0.5;
                camera.pos = Vector::new(2.0, 1.0);
            },
            AnimationState::AB(t) => {
                camera.zoom = 0.5 + 0.5 * t;
                if t >= 1.0 {
                    state.insert("title_anim", AnimationState::B);
                } else {
                    state.insert("title_anim", AnimationState::AB(t + physics.delta_time));
                }
            },
            AnimationState::B => camera.zoom = 1.0,
            AnimationState::BA(t) => {
                camera.zoom = 1.0 - 0.5 * t;
                if t >= 1.0 {
                    state.insert("title_anim", AnimationState::A);
                } else {
                    state.insert("title_anim", AnimationState::BA(t + physics.delta_time));
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        world.write_resource::<StateRes>().insert("game", GameState::Title);
        world.write_resource::<StateRes>().insert("title_anim", AnimationState::A);
/*
        world.create_entity().with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::Start, SizeConstraint::Fill, SizeConstraint::Fill))
            .with(StartVerticalGroupCom::new("blank")).build();

        world.create_entity()
            .with(ConstraintCom::new(PositionConstraint::Center, PositionConstraint::StartPixelOffset(100), SizeConstraint::Pixels(450), SizeConstraint::Pixels(100)))
            .with(TextUICom::new("Vitrellogy", "nemoy"))
            .with(TitleUIMarker).build();

        world.create_entity().with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::Start, SizeConstraint::Fill, SizeConstraint::Fill))
            .with(EndGroupCom::new()).build();

        world.create_entity().with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::Start, SizeConstraint::Fill, SizeConstraint::Fill))
            .with(StartVerticalGroupCom::new("blank")).build();
*/
        world.create_entity().with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::Start, SizeConstraint::Fill, SizeConstraint::Fill))
            .with(ButtonUICom::new("blank", "blank", "title_start"))
            .with(TitleUIMarker).build();
/*
        world.create_entity().with(ConstraintCom::new(PositionConstraint::Start, PositionConstraint::Start, SizeConstraint::Fill, SizeConstraint::Fill))
            .with(EndGroupCom::new()).build();
*/
    }
}

pub struct DebugUISys {
    active: bool,
    file: String
}

impl<'a> System<'a> for DebugUISys {
    type SystemData = (Entities<'a>,
        ReadResource<'a, UIEventQueue>,
        WriteResource<'a, NetworkRequestQueue>,
        WriteResource<'a, PersistRequestQueue>,
        WriteResource<'a, StateRes>,
        ReadResource<'a, InputEventQueue>,
        WriteStorage<'a, DebugUIMarker>,
        WriteStorage<'a, ConstraintCom>,
        WriteStorage<'a, StartVerticalGroupCom>,
        WriteStorage<'a, EndGroupCom>,
        WriteStorage<'a, ButtonUICom>,
        WriteStorage<'a, TextUICom>,
        WriteStorage<'a, TextFieldUICom>);

    fn run(&mut self, (entities, ui_events, mut net_requests, mut persist_requests, mut state, input_events, mut debug_ui_markers, mut constraints, mut v_group_start_markers, mut group_end_markers, mut ui_buttons, mut ui_text_labels, mut ui_text_fields): Self::SystemData) {
        for event in ui_events.iter() {
            match event {
                UIEvent::ButtonPressed { id } => match id.as_str() {
                    "sound" => (),
                    "connect" => {
                        net_requests.push(NetworkRequest::Open);
                        net_requests.push(NetworkRequest::Connect(("127.0.0.1", 0).to_socket_addrs().unwrap().next().unwrap()));
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

        for event in input_events.iter() {
            match event {
                InputEvent::KeyDown(Key::Backtick) => if self.active {
                    for (entity, _marker) in (&entities, &debug_ui_markers).join() {
                        constraints.remove(entity);
                        v_group_start_markers.remove(entity);
                        group_end_markers.remove(entity);
                        ui_buttons.remove(entity);
                        ui_text_labels.remove(entity);
                        ui_text_fields.remove(entity);
                    }
                    debug_ui_markers.clear();

                    self.active = false;
                } else {
                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::Start, PositionConstraint::Start, SizeConstraint::Pixels(240), SizeConstraint::Proportion(1.0))).unwrap();
                    v_group_start_markers.insert(e, StartVerticalGroupCom::new("bg")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_buttons.insert(e, ButtonUICom::new("fg", "bg", "connect")).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Connect", "ui")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_buttons.insert(e, ButtonUICom::new("fg", "bg", "host")).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Host", "ui")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_buttons.insert(e, ButtonUICom::new("fg", "bg", "debug")).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Debug", "ui")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_buttons.insert(e, ButtonUICom::new("fg", "bg", "sound")).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Sound", "ui")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_buttons.insert(e, ButtonUICom::new("fg", "bg", "quit")).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Quit", "ui")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Save/Load", "ui_fg")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_text_fields.insert(e, TextFieldUICom::new("fg", "save", "ui", "file")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_buttons.insert(e, ButtonUICom::new("fg", "bg", "save")).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Save", "ui")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::NegativePixels(20), SizeConstraint::Pixels(50))).unwrap();
                    ui_buttons.insert(e, ButtonUICom::new("fg", "bg", "load")).unwrap();
                    ui_text_labels.insert(e, TextUICom::new("Load", "ui")).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    let e = entities.create();
                    constraints.insert(e, ConstraintCom::new(PositionConstraint::Start, PositionConstraint::Start, SizeConstraint::Fill, SizeConstraint::Fill)).unwrap();
                    group_end_markers.insert(e, EndGroupCom).unwrap();
                    debug_ui_markers.insert(e, DebugUIMarker).unwrap();

                    self.active = true;
                },
                _ => ()
            }
        }
    }
}

impl DebugUISys {
    pub fn new() -> Self {
        Self {
            active: false,
            file: "save".to_string()
        }
    }
}

#[derive(DefaultConstructor)]
pub struct ControllerSys;

impl<'a> System<'a> for ControllerSys {
    type SystemData = (ReadResource<'a, KeysRes>,
        WriteResource<'a, PhysicsRes>,
        WriteResource<'a, CameraRes>,
        ReadStorage<'a, PlayerMarker>,
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
                Some(rb) => rb.apply_force(0, &Force::linear(*Vector::new(horizontal, vertical)), ForceType::Impulse, true),
                None => ()
            }
        }

        for (_controller, transform) in (&controllers, &transforms).join() {
            let centre = *transform.pos + *Vector::new(1.0, 1.0);
            
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
pub struct PlayerMarker;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct TitleUIMarker;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct DebugUIMarker;

#[derive(Debug, Clone)]
pub enum GameState {
    Title,
    Hub
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AnimationState {
    A,
    AB(f32),
    B,
    BA(f32)
}
