use std::time::Instant;

use specs::prelude::*;

mod misc;
use misc::{AppState, AppStateRes};
use misc::vec::Vec2;

mod render;
use render::{Renderer, SpriteCom, RenderSys, CameraRes, CameraCom, CameraSys};
use render::sdl::SDLRenderer;

mod input;
use input::InputSys;
use input::key::KeysRes;
use input::controller::{ControllerCom, ControllerSys};
use input::sdl::SDLInput;

mod physics;
use physics::{TransformCom, DeltaTimeRes};

fn main() {
    // Register components
    let mut world = World::new();
    world.register::<TransformCom>();
    world.register::<SpriteCom>();
    world.register::<ControllerCom>();
    world.register::<CameraCom>();

    // Initialise systems
    let sdl_context = sdl2::init().unwrap();
    let mut renderer = RenderSys::new(SDLRenderer::init(&sdl_context, Vec2::new(800, 600)));
    let input = InputSys::new(SDLInput::init(&sdl_context));

    renderer.renderer.add_sprite("wizard", "assets/placeholder/sprites/wizard.png");
    renderer.renderer.add_sprite("tree", "assets/placeholder/sprites/tree.png");
    renderer.renderer.add_sprite("r", "assets/placeholder/sprites/32x32-w-r.png");
    renderer.renderer.add_sprite("g", "assets/placeholder/sprites/32x32-w-g.png");
    renderer.renderer.add_sprite("b", "assets/placeholder/sprites/32x32-w-b.png");

    let controller = ControllerSys::new();
    let camera = CameraSys::new();

    // Combine all systems into a single dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(controller, "controller", &[])
        .with(camera, "camera", &["controller"])
        .with_thread_local(input)
        .with_thread_local(renderer).build();

    // Add resources
    world.insert(AppStateRes::new(AppState::Running));
    world.insert(DeltaTimeRes::new(0.0));
    world.insert(KeysRes::new());
    world.insert(CameraRes::new(Vec2::new(0.0, 0.0), 1.0, Vec2::new(800, 600)));

    // Create entites
    world.create_entity().with(SpriteCom::new("tree", Vec2::new(4.0, 4.0)))
        .with(TransformCom::new(Vec2::new(2.0, 0.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(0.0, -1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(1.0, -1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(2.0, -1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(3.0, -1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(4.0, -1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(5.0, -1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(6.0, -1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new(Vec2::new(7.0, -1.0))).build();

    world.create_entity().with(SpriteCom::new("wizard", Vec2::new(2.0, 2.0)))
        .with(TransformCom::new(Vec2::new(0.0, 0.0)))
        .with(ControllerCom::new())
        .with(CameraCom::new()).build();

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
            let mut delta_time_res = world.write_resource::<DeltaTimeRes>();
            *delta_time_res = DeltaTimeRes::new(delta_time);
        }

        // Check if the game has been quit
        match world.read_resource::<AppStateRes>().0 {
            AppState::Stopping => break,
            _ => {}
        }

        // Run the game
        dispatcher.dispatch(&mut world);
        world.maintain();
    }
}
