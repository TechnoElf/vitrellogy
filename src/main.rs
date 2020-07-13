use std::time::Instant;
use std::sync::{Arc, Mutex};
use specs::prelude::*;

use nalgebra::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

mod misc;
use misc::{AppState, TransformCom, StateRes};

mod render;
use render::{RenderRes, SpriteCom, SpriteRenderSys, CameraRes, CameraCom, CameraSys, TextRenderSys, TextCom};
use render::sdl::SDLRenderImpl;
use render::ui::{UISys, TextUICom, ButtonUICom};

mod input;
use input::{InputSys, MouseRes, InputRes};
use input::key::KeysRes;
use input::sdl::SDLInputImpl;

mod physics;
use physics::{PhysicsSys, PhysicsRes};
use physics::controller::{ControllerCom, ControllerSys};

mod net;
use net::{SyncTransformCom, NetworkSyncSys, NetworkRes};
use net::client::NetworkClient;

fn main() {
    // Initialise resources
    let sdl_context = sdl2::init().unwrap();
    let mut render = RenderRes {
        renderer: SDLRenderImpl::init(&sdl_context, Vector2::new(800, 600))
    };
    let input = InputRes {
        input: SDLInputImpl::init(&sdl_context)
    };
    let mut physics = PhysicsRes {
        delta_time: 0.0,
        m_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
        g_world: DefaultGeometricalWorld::new(),
        bodies: DefaultBodySet::new(),
        colliders: DefaultColliderSet::new(),
        constraints: DefaultJointConstraintSet::new(),
        forces: DefaultForceGeneratorSet::new()
    };
    let mut state = StateRes::new();

    render.add_sprite("wizard", "assets/placeholder/sprites/wizard.png");
    render.add_sprite("tree", "assets/placeholder/sprites/tree.png");
    render.add_sprite("r", "assets/placeholder/sprites/32x32-w-r.png");
    render.add_sprite("g", "assets/placeholder/sprites/32x32-w-g.png");
    render.add_sprite("b", "assets/placeholder/sprites/32x32-w-b.png");
    render.add_font("caveat", "assets/placeholder/fonts/caveat.ttf", 64, 10, 10, 10);
    render.add_font("nemoy", "assets/placeholder/fonts/nemoy.otf", 64, 200, 128, 255);
    render.add_font("patrickhand", "assets/placeholder/fonts/patrickhand.ttf", 64, 255, 255, 255);

    state.insert("app", AppState::Running);

    // Initialise systems and set up the game world
    let sprite_renderer = SpriteRenderSys::new();
    let text_renderer = TextRenderSys::new();
    let ui_renderer = UISys::new();
    let input_sys = InputSys::new();
    let shared_network = Arc::new(Mutex::new(NetworkClient::new().unwrap()));
    let network_sync = NetworkSyncSys::new(Arc::clone(&shared_network));
    let controller_sys = ControllerSys::new();
    let camera_sys = CameraSys::new();
    let physics_sys = PhysicsSys::new();
    
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(input_sys)
        .with(network_sync, "network_sync", &[])
        .with(controller_sys, "controller", &[])
        .with(physics_sys, "physics", &["controller", "network_sync"])
        .with(camera_sys, "camera", &["physics"])
        .with_thread_local(sprite_renderer)
        .with_thread_local(text_renderer)
        .with_thread_local(ui_renderer).build();

    let mut world = World::new();
    dispatcher.setup(&mut world);
    
    // Create entites
    world.create_entity().with(SpriteCom::new("tree", Vector2::new(4.0, 4.0)))
        .with(TransformCom::new(Vector2::new(2.0, 0.0))).build();

    for pos in 0..20 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, -1.0))).build();
    }
    for pos in 10..15 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, 0.0))).build();
    }
    for pos in 0..5 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(0.0, pos as f32))).build();
    }
    for pos in 0..5 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(19.0, pos as f32))).build();
    }

    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(20.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(0.0, -1.0)))
        .with(rb)
        .with(col).build();
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(5.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(10.0, 0.0)))
        .with(rb)
        .with(col).build();
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(1.0, 5.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(0.0, 0.0)))
        .with(rb)
        .with(col).build();
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(1.0, 5.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(19.0, 0.0)))
        .with(rb)
        .with(col).build();

    world.create_entity().with(TextCom::new("Sphinx of black quartz, judge my vow", "caveat", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(7.0, 3.0))).build();
    let rb = physics.create_rigid_body();
    let col = physics.create_collider_rectangle(Vector2::new(5.0, 1.0), &rb);
    world.create_entity().with(TextCom::new("Vitrellogy", "nemoy", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(7.0, 4.0)))
        .with(rb)
        .with(col).build();
    world.create_entity().with(TextCom::new("Vitrellogy", "patrickhand", Vector2::new(1.0, 1.0)))
        .with(TransformCom::new(Vector2::new(7.0, 5.0))).build();

    world.create_entity().with(ButtonUICom::new("g", "r", Vector2::new(200, 50), "net_connect"))
        .with(TransformCom::new(Vector2::new(0.0, 0.0))).build();

    world.create_entity().with(TextUICom::new("Connect", "caveat", Vector2::new(200, 50)))
        .with(TransformCom::new(Vector2::new(0.0, 0.0))).build();

    let rb = physics.create_rigid_body();
    let col = physics.create_collider_rectangle(Vector2::new(2.0, 2.0), &rb);
    world.create_entity().with(SpriteCom::new("wizard", Vector2::new(2.0, 2.0)))
        .with(TransformCom::new(Vector2::new(0.0, 1.0)))
        .with(ControllerCom::new())
        .with(CameraCom::new(Vector2::new(1.0, 1.0)))
	    .with(SyncTransformCom::new())
        .with(rb)
        .with(col).build();

    // Add resources
    world.insert(KeysRes::new());
    world.insert(MouseRes::new(None));
    world.insert(CameraRes::new(Vector2::new(0.0, 0.0), 1.0, Vector2::new(800, 600)));
    world.insert(NetworkRes::new(true, true, false, (127, 0, 0, 1)));
    world.insert(render);
    world.insert(input);
    world.insert(physics);
    world.insert(state);

    let target_frame_rate = 60.0;
    let target_frame_time = 1.0 / target_frame_rate;

    let mut time = Instant::now();
    let mut delta_time;

    loop {
        // Calculate the time since the last frame and wait to lock to 60fps, if necessary
        delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        while delta_time < target_frame_time {
            delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        }
        time = Instant::now();

        if delta_time > target_frame_time + 0.001 {
            println!("dt={}", delta_time);
        }

        // Update the delta time resource in a block so rust doesn't complain about multiple borrows
        {
            let mut physics = world.write_resource::<PhysicsRes>();
            physics.delta_time = delta_time;
        }

        // Check if the game has been quit
        match world.read_resource::<StateRes>().get("app").unwrap() {
            AppState::Stopping => break,
            _ => {}
        }

        // Run the game
        dispatcher.dispatch(&mut world);
        world.maintain();
    }
}
