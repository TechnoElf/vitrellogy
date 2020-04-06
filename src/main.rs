use std::rc::Rc;
use std::cell::RefCell;
use std::time::Instant;

mod misc;
use misc::world::*;
use misc::vec::*;

mod render;
use render::*;
use render::sdl::*;

mod input;
use input::*;
use input::sdl::*;

mod component;
use component::c_background::*;
use component::c_player::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let renderer = Rc::new(RefCell::new(SDLRenderer::init(&sdl_context, Vec2::new(800, 600))));
    let input = Rc::new(RefCell::new(SDLInput::init(&sdl_context, Vec2::new(800, 600))));
    let mut world = World::new(renderer.clone(), input.clone());

    renderer.borrow_mut().add_sprite("wizard", "assets/placeholder/sprites/wizard.png");
    renderer.borrow_mut().add_sprite("tree", "assets/placeholder/sprites/tree.png");
    renderer.borrow_mut().add_sprite("r", "assets/placeholder/sprites/32x32-w-r.png");
    renderer.borrow_mut().add_sprite("g", "assets/placeholder/sprites/32x32-w-g.png");
    renderer.borrow_mut().add_sprite("b", "assets/placeholder/sprites/32x32-w-b.png");

    world.add_e_player(EPlayer::new("wizard", Vec2::new(2.0, 2.0), Vec2::new(0.0, 0.0)));
    world.add_e_background(EBackground::new("tree", Vec2::new(2.0, 0.0), Vec2::new(4.0, 4.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(0.0, -1.0), Vec2::new(1.0, 1.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(2.0, -1.0), Vec2::new(1.0, 1.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(3.0, -1.0), Vec2::new(1.0, 1.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(4.0, -1.0), Vec2::new(1.0, 1.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(5.0, -1.0), Vec2::new(1.0, 1.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(6.0, -1.0), Vec2::new(1.0, 1.0)));
    world.add_e_background(EBackground::new("b", Vec2::new(7.0, -1.0), Vec2::new(1.0, 1.0)));

    world.set_state("game_state", "in_game");

    let target_frame_rate = 60.0;
    let target_frame_time = 1.0 / target_frame_rate;

    let mut time = Instant::now();
    let mut delta_time;

    loop {
        renderer.borrow_mut().render();

        delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        while delta_time < target_frame_time {
            delta_time = (time.elapsed().as_micros() as f64 / 1000000.0) as f32;
        }
        time = Instant::now();

        if delta_time > target_frame_time + 0.001 {
            println!("dt={}", delta_time);
        }

        match input.borrow_mut().input() {
            InputState::Running => {},
            InputState::Stopping => break
        }

        world.process(delta_time)
    }
}
