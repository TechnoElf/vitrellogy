use std::time::Instant;

use invader::ecs::*;
use invader::misc::*;
use invader::misc::persist::*;
use invader::render::*;
use invader::render::sdl::*;
use invader::physics::*;
use invader::net::*;
use invader::net::imp::*;
use invader::input::*;
use invader::input::sdl::*;
use invader::sound::*;
use invader::sound::imp::*;

mod game;
use game::{DebugUISys, ControllerSys, GameManagerSys};

const TARGET_FRAME_RATE: f32 = 60.0;
const TARGET_FRAME_TIME: f32 = 1.0 / TARGET_FRAME_RATE;

fn main() {
    // Initialise resources
    let sdl_context = sdl2::init().unwrap();
    let mut render = SDLRenderImpl::init(&sdl_context, Vector::new(800.0, 600.0).convert());
    let input = SDLInputImpl::init(&sdl_context);
    let physics = PhysicsRes::new();
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
    render.add_sprite("blank", "assets/placeholder/sprites/blank.png");
    render.add_font("caveat", "assets/placeholder/fonts/caveat.ttf", 64, 10, 10, 10);
    render.add_font("nemoy", "assets/placeholder/fonts/nemoy.otf", 64, 200, 128, 255);
    render.add_font("patrickhand", "assets/placeholder/fonts/patrickhand.ttf", 64, 255, 255, 255);

    // Initialise systems and set up the game world
    let debug_ui_sys = DebugUISys::new(&mut sound);
    let render_sys = RenderSys::new(render);
    let input_sys = InputSys::new(input);
    let network_sys = NetworkSyncSys::new(net);
    let controller_sys = ControllerSys::new();
    let physics_sys = PhysicsSys::new();
    let sound_sys = SoundSys::new(sound);
    let persist_sys = PersistSys::new();
    let game_sys = GameManagerSys::new();
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(persist_sys, "perist", &[])
        .with(network_sys, "network_sync", &[])
        .with(controller_sys, "controller", &[])
        .with(physics_sys, "physics", &["controller", "network_sync"])
        .with(debug_ui_sys, "debug_ui", &[])
        .with(game_sys, "game_manager", &[])
        .with_thread_local(input_sys)
        .with_thread_local(sound_sys)
        .with_thread_local(render_sys).build();

    let mut world = World::new();
    invader::render::register(&mut world);
    invader::misc::register(&mut world);
    dispatcher.setup(&mut world);

    // Add resources
    world.insert(physics);

    let mut time = Instant::now();
    let mut delta_time;

    world.write_resource::<StateRes>().insert("app", AppState::Running);
    world.write_resource::<PersistRequestQueue>().push(PersistRequest::LoadStage("assets/placeholder/stages/stage.mst".to_string()));

    while world.read_resource::<StateRes>().get::<AppState>("app").unwrap() == &AppState::Running {
        // Calculate the time since the last frame and wait to lock to 60fps, if necessary
        delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        while delta_time < TARGET_FRAME_TIME {
            delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        }
        time = Instant::now();

        if delta_time > TARGET_FRAME_TIME * 2.0 {
            //println!("dt={}", delta_time);
            delta_time = 0.0;
        }

        world.write_resource::<PhysicsRes>().delta_time = delta_time;

        // Run the game
        dispatcher.dispatch(&mut world);
        world.maintain();
    }
}
