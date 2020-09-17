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
use render::RenderSys;
use render::sdl::SDLRenderImpl;

mod input;
use input::{InputSys, InputRes};
use input::sdl::SDLInputImpl;

mod physics;
use physics::{PhysicsSys, PhysicsRes};

mod net;
use net:: NetworkSyncSys;
use net::imp::NetworkImp;

mod sound;
use sound::SoundSys;
use sound::imp::SoundImp;

mod game;
use game::{DebugUISys, build_world};
use game::controller::ControllerSys;

const TARGET_FRAME_RATE: f32 = 60.0;
const TARGET_FRAME_TIME: f32 = 1.0 / TARGET_FRAME_RATE;

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
    let net = NetworkImp::new();
    let mut sound = SoundImp::new();

    render.add_sprite("wizard", "assets/placeholder/sprites/wizard.png");
    render.add_sprite("tree", "assets/placeholder/sprites/tree.png");
    render.add_sprite("bolt0", "assets/placeholder/sprites/bolt0.png");
    render.add_sprite("bolt1", "assets/placeholder/sprites/bolt1.png");
    render.add_sprite("bolt2", "assets/placeholder/sprites/bolt2.png");
    render.add_sprite("bolt3", "assets/placeholder/sprites/bolt3.png");
    render.add_sprite("r", "assets/placeholder/sprites/32x32-w-r.png");
    render.add_sprite("g", "assets/placeholder/sprites/32x32-w-g.png");
    render.add_sprite("b", "assets/placeholder/sprites/32x32-w-b.png");
    render.add_font("caveat", "assets/placeholder/fonts/caveat.ttf", 64, 10, 10, 10);
    render.add_font("nemoy", "assets/placeholder/fonts/nemoy.otf", 64, 200, 128, 255);
    render.add_font("patrickhand", "assets/placeholder/fonts/patrickhand.ttf", 64, 255, 255, 255);

    // Initialise systems and set up the game world
    let debug_ui_sys = DebugUISys::new(&mut sound);
    let render_sys = RenderSys::new(render);
    let input_sys = InputSys::new();
    let network_sys = NetworkSyncSys::new(net);
    let controller_sys = ControllerSys::new();
    let physics_sys = PhysicsSys::new();
    let sound_sys = SoundSys::new(sound);
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(network_sys, "network_sync", &[])
        .with(controller_sys, "controller", &[])
        .with(physics_sys, "physics", &["controller", "network_sync"])
        .with(debug_ui_sys, "debug_ui", &[])
        .with_thread_local(input_sys)
        .with_thread_local(sound_sys)
        .with_thread_local(render_sys).build();

    let mut world = World::new();
    render::register(&mut world);
    dispatcher.setup(&mut world);

    // Create entites
    build_world(&mut world, &mut physics);
 
    // Add resources
    world.insert(input);
    world.insert(physics);

    let mut time = Instant::now();
    let mut delta_time;

    world.write_resource::<StateRes>().insert("app", AppState::Running);

    while world.read_resource::<StateRes>().get::<AppState>("app").unwrap() == &AppState::Running {
        // Calculate the time since the last frame and wait to lock to 60fps, if necessary
        delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        while delta_time < TARGET_FRAME_TIME {
            delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        }
        time = Instant::now();

        if delta_time > TARGET_FRAME_TIME + 0.001 {
            //println!("dt={}", delta_time);
            delta_time = 0.0;
        }

        world.write_resource::<PhysicsRes>().delta_time = delta_time;

        // Run the game
        dispatcher.dispatch(&mut world);
        world.maintain();
    }
}
