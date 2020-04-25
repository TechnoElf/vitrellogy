use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use specs::prelude::*;

mod misc;
use misc::{AppState, AppStateRes};
use misc::vec::Vec2;

mod render;
use render::{Renderer, SpriteCom, SpriteRenderSys, CameraRes, CameraCom, CameraSys, TextRenderSys, TextCom};
use render::sdl::SDLRenderer;

mod input;
use input::InputSys;
use input::key::KeysRes;
use input::sdl::SDLInput;

mod physics;
use physics::{TransformCom, DeltaTimeRes, PhysicsSys, DynamicCom};
use physics::colliders::ColliderAABBCom;
use physics::controller::{ControllerCom, ControllerSys};
use physics::forces::{ForcesSys, GravityCom, DragCom};

fn main() {
    // Initialise systems
    let sdl_context = sdl2::init().unwrap();
    let shared_renderer = Rc::new(RefCell::new(SDLRenderer::init(&sdl_context, Vec2::new(800, 600))));
    let sprite_renderer = SpriteRenderSys::new(Rc::clone(&shared_renderer));
    let text_renderer = TextRenderSys::new(Rc::clone(&shared_renderer));
    let input = InputSys::new(SDLInput::init(&sdl_context));

    shared_renderer.borrow_mut().add_sprite("wizard", "assets/placeholder/sprites/wizard.png");
    shared_renderer.borrow_mut().add_sprite("tree", "assets/placeholder/sprites/tree.png");
    shared_renderer.borrow_mut().add_sprite("r", "assets/placeholder/sprites/32x32-w-r.png");
    shared_renderer.borrow_mut().add_sprite("g", "assets/placeholder/sprites/32x32-w-g.png");
    shared_renderer.borrow_mut().add_sprite("b", "assets/placeholder/sprites/32x32-w-b.png");

    shared_renderer.borrow_mut().add_font("caveat", "assets/placeholder/fonts/caveat.ttf", 32, 255, 255, 255);
    shared_renderer.borrow_mut().add_font("nemoy", "assets/placeholder/fonts/nemoy.otf", 32, 200, 128, 255);
    shared_renderer.borrow_mut().add_font("patrickhand", "assets/placeholder/fonts/patrickhand.ttf", 32, 255, 255, 255);

    let controller = ControllerSys::new();
    let camera = CameraSys::new();
    let physics = PhysicsSys::new();
    let forces = ForcesSys::new();

    // Combine all systems into a single dispatcher and set up the game world
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(input)
        .with(controller, "controller", &[])
        .with(forces, "forces", &[])
        .with(physics, "physics", &["controller", "forces"])
        .with(camera, "camera", &["physics"])
        .with_thread_local(sprite_renderer)
        .with_thread_local(text_renderer).build();

    let mut world = World::new();
    dispatcher.setup(&mut world);

    // Add resources
    world.insert(AppStateRes::new(AppState::Running));
    world.insert(DeltaTimeRes::new(0.0));
    world.insert(KeysRes::new());
    world.insert(CameraRes::new(Vec2::new(0.0, 0.0), 1.0, Vec2::new(800, 600)));

    // Create entites
    world.create_entity().with(SpriteCom::new("tree", Vec2::new(4.0, 4.0)))
        .with(TransformCom::new_pos(Vec2::new(2.0, 0.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(1.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(2.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(3.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(4.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(5.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(6.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(7.0, -1.0)))
        .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();

    world.create_entity().with(TextCom::new("Vitrellogy", "caveat", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 5.0))).build();
    world.create_entity().with(TextCom::new("Vitrellogy", "nemoy", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 6.0)))
        .with(DynamicCom::new())
        .with(GravityCom::new())
        .with(ColliderAABBCom::new(Vec2::new(5.0, 1.0))).build();
    world.create_entity().with(TextCom::new("Vitrellogy", "patrickhand", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 7.0))).build();

    world.create_entity().with(SpriteCom::new("wizard", Vec2::new(2.0, 2.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 1.0)))
        .with(ControllerCom::new())
        .with(CameraCom::new())
        .with(ColliderAABBCom::new(Vec2::new(2.0, 2.0)))
        .with(DynamicCom::new())
        .with(GravityCom::new())
        .with(DragCom::new()).build();

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
