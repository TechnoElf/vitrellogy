use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use specs::prelude::*;

mod misc;
use misc::{AppState, AppStateRes};
use misc::vec::Vec2;

mod render;
use render::{Renderer, SpriteCom, SpriteRenderSys, CameraRes, CameraCom, CameraSys, TextRenderSys, TextCom};
use render::sdl::SDLRenderer;
use render::ui::{UISys, TextUICom, ButtonUICom};

mod input;
use input::{InputSys, MouseRes};
use input::key::KeysRes;
use input::sdl::SDLInput;

mod physics;
use physics::{TransformCom, DeltaTimeRes, PhysicsSys, DynamicCom};
use physics::colliders::ColliderAABBCom;
use physics::controller::{ControllerCom, ControllerSys};
use physics::forces::{ForcesSys, GravityCom, DragCom};

mod net;
use net::{SyncTransformCom, NetworkSyncSys, NetworkRes};
use net::client::NetworkClient;

fn main() {
    // Initialise systems
    let sdl_context = sdl2::init().unwrap();
    let shared_renderer = Rc::new(RefCell::new(SDLRenderer::init(&sdl_context, Vec2::new(800, 600))));
    let sprite_renderer = SpriteRenderSys::new(Rc::clone(&shared_renderer));
    let text_renderer = TextRenderSys::new(Rc::clone(&shared_renderer));
    let ui_renderer = UISys::new(Rc::clone(&shared_renderer));

    let input = InputSys::new(SDLInput::init(&sdl_context));

    let shared_network = Arc::new(Mutex::new(NetworkClient::new().unwrap()));
    let network_sync = NetworkSyncSys::new(Arc::clone(&shared_network));

    shared_renderer.borrow_mut().add_sprite("wizard", "assets/placeholder/sprites/wizard.png");
    shared_renderer.borrow_mut().add_sprite("tree", "assets/placeholder/sprites/tree.png");
    shared_renderer.borrow_mut().add_sprite("r", "assets/placeholder/sprites/32x32-w-r.png");
    shared_renderer.borrow_mut().add_sprite("g", "assets/placeholder/sprites/32x32-w-g.png");
    shared_renderer.borrow_mut().add_sprite("b", "assets/placeholder/sprites/32x32-w-b.png");

    shared_renderer.borrow_mut().add_font("caveat", "assets/placeholder/fonts/caveat.ttf", 64, 10, 10, 10);
    shared_renderer.borrow_mut().add_font("nemoy", "assets/placeholder/fonts/nemoy.otf", 64, 200, 128, 255);
    shared_renderer.borrow_mut().add_font("patrickhand", "assets/placeholder/fonts/patrickhand.ttf", 64, 255, 255, 255);

    let controller = ControllerSys::new();
    let camera = CameraSys::new();
    let physics = PhysicsSys::new();
    let forces = ForcesSys::new();


    // Combine all systems into a single dispatcher and set up the game world
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(input)
        .with(network_sync, "network_sync", &[])
        .with(controller, "controller", &[])
        .with(forces, "forces", &[])
        .with(physics, "physics", &["controller", "forces", "network_sync"])
        .with(camera, "camera", &["physics"])
        .with_thread_local(sprite_renderer)
        .with_thread_local(text_renderer)
        .with_thread_local(ui_renderer).build();

    let mut world = World::new();
    dispatcher.setup(&mut world);

    // Add resources
    world.insert(AppStateRes::new(AppState::Running));
    world.insert(DeltaTimeRes::new(0.0));
    world.insert(KeysRes::new());
    world.insert(MouseRes::new(None));
    world.insert(CameraRes::new(Vec2::new(0.0, 0.0), 1.0, Vec2::new(800, 600)));
    world.insert(NetworkRes::new(true, true, false, (127, 0, 0, 1)));

    // Create entites
    world.create_entity().with(SpriteCom::new("tree", Vec2::new(4.0, 4.0)))
        .with(TransformCom::new_pos(Vec2::new(2.0, 0.0))).build();
    for pos in 0..20 {
        world.create_entity().with(SpriteCom::new("b", Vec2::new(1.0, 1.0)))
            .with(TransformCom::new_pos(Vec2::new(pos as f32, -1.0)))
            .with(ColliderAABBCom::new(Vec2::new(1.0, 1.0))).build();
    }

    world.create_entity().with(TextCom::new("Sphinx of black quartz, judge my vow", "caveat", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 5.0))).build();
    world.create_entity().with(TextCom::new("Vitrellogy", "nemoy", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 6.0)))
        .with(DynamicCom::new())
        .with(GravityCom::new())
        .with(ColliderAABBCom::new(Vec2::new(5.0, 1.0))).build();
    world.create_entity().with(TextCom::new("Vitrellogy", "patrickhand", Vec2::new(1.0, 1.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 7.0))).build();

    world.create_entity().with(ButtonUICom::new("g", "r", Vec2::new(200, 50), "net_connect"))
        .with(TransformCom::new_pos(Vec2::new(0.0, 0.0))).build();

    world.create_entity().with(TextUICom::new("Connect", "caveat", Vec2::new(200, 50)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 0.0))).build();

    world.create_entity().with(SpriteCom::new("wizard", Vec2::new(2.0, 2.0)))
        .with(TransformCom::new_pos(Vec2::new(0.0, 1.0)))
        .with(ControllerCom::new())
        .with(CameraCom::new())
        .with(ColliderAABBCom::new(Vec2::new(2.0, 2.0)))
        .with(DynamicCom::new())
        .with(GravityCom::new())
        .with(DragCom::new())
	    .with(SyncTransformCom::new()).build();


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
