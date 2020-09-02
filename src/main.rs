use std::time::Instant;
use specs::prelude::*;

use nalgebra::Vector2;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};

#[macro_use]
mod misc;
use misc::{AppState, StateRes};

mod render;
use render::{SpriteCom, RenderSys, CameraRes, TextCom, UISys, TextUICom, ButtonUICom, Constraints, PositionConstraint, SizeConstraint};
use render::sdl::SDLRenderImpl;

mod input;
use input::{InputSys, InputRes};
use input::sdl::SDLInputImpl;

mod physics;
use physics::{PhysicsSys, PhysicsRes, TransformCom};

mod net;
use net::{NetMasterTransformCom, NetworkSyncSys, NetworkRes};

mod sound;
use sound::{SoundRes, SoundSys};

mod game;
use game::controller::{ControllerCom, ControllerSys};

fn main() {
    // Initialise resources
    let sdl_context = sdl2::init().unwrap();
    let mut render = SDLRenderImpl::init(&sdl_context, Vector2::new(800, 600));
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
    let net = NetworkRes::new();
    let mut sound = SoundRes::new();
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

    let bg_music = sound.load_music(&["assets/placeholder/music/you-are-my-hope.ogg", "assets/placeholder/music/windward.ogg", "assets/placeholder/music/baby-bird.ogg", "assets/placeholder/music/loves-vagrant.ogg"]);

    // Initialise systems and set up the game world
    let render_sys = RenderSys::new(render);
    let ui_sys = UISys::new();
    let input_sys = InputSys::new();
    let network_sys = NetworkSyncSys::new();
    let controller_sys = ControllerSys::new();
    let physics_sys = PhysicsSys::new();
    let sound_sys = SoundSys::new(0, bg_music);
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(network_sys, "network_sync", &[])
        .with(controller_sys, "controller", &[])
        .with(physics_sys, "physics", &["controller", "network_sync"])
        .with(ui_sys, "ui", &[])
        .with_thread_local(input_sys)
        .with_thread_local(sound_sys)
        .with_thread_local(render_sys).build();

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
    for pos in 0..18 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(0.0, pos as f32))).build();
    }
    for pos in 0..18 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(19.0, pos as f32))).build();
    }
    for pos in 0..20 {
        world.create_entity().with(SpriteCom::new("b", Vector2::new(1.0, 1.0)))
            .with(TransformCom::new(Vector2::new(pos as f32, 18.0))).build();
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
    let col = physics.create_collider_rectangle(Vector2::new(1.0, 18.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(0.0, 0.0)))
        .with(rb)
        .with(col).build();
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(1.0, 18.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(19.0, 0.0)))
        .with(rb)
        .with(col).build();
    let rb = physics.create_rigid_body_static();
    let col = physics.create_collider_rectangle(Vector2::new(20.0, 1.0), &rb);
    world.create_entity().with(TransformCom::new(Vector2::new(0.0, 18.0)))
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

    world.create_entity().with(ButtonUICom::new(Constraints::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(10), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)), "r", "g", "net_connect")).build();
    world.create_entity().with(TextUICom::new("Connect", "caveat", Vector2::new(120, 50)))
        .with(TransformCom::new(Vector2::new(10.0, 10.0))).build();

    world.create_entity().with(ButtonUICom::new(Constraints::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(70), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)), "g", "b", "net_host")).build();
    world.create_entity().with(TextUICom::new("Host", "caveat", Vector2::new(120, 50)))
        .with(TransformCom::new(Vector2::new(10.0, 70.0))).build();

    world.create_entity().with(ButtonUICom::new(Constraints::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(130), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)), "b", "r", "debug")).build();
    world.create_entity().with(TextUICom::new("Debug", "caveat", Vector2::new(120, 50)))
        .with(TransformCom::new(Vector2::new(10.0, 130.0))).build();

    world.create_entity().with(ButtonUICom::new(Constraints::new(PositionConstraint::StartPixelOffset(10), PositionConstraint::StartPixelOffset(190), SizeConstraint::Pixels(120), SizeConstraint::Pixels(50)), "r", "g", "sound")).build();
    world.create_entity().with(TextUICom::new("Sound", "caveat", Vector2::new(120, 50)))
        .with(TransformCom::new(Vector2::new(10.0, 190.0))).build();

    let rb = physics.create_rigid_body();
    let col = physics.create_collider_rectangle(Vector2::new(2.0, 2.0), &rb);
    world.create_entity().with(SpriteCom::new("wizard", Vector2::new(2.0, 2.0)))
        .with(TransformCom::new(Vector2::new(0.0, 1.0)))
        .with(ControllerCom::new())
	    .with(NetMasterTransformCom::new())
        .with(rb)
        .with(col).build();

    // Add resources
    world.insert(CameraRes::new(Vector2::new(0.0, 0.0), 1.0, Vector2::new(800, 600)));
    world.insert(net);
    world.insert(input);
    world.insert(physics);
    world.insert(sound);
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
            //println!("dt={}", delta_time);
            delta_time = 0.0;
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
