use invader::InvaderBuilder;

mod game;
use game::{GameManagerSys, ControllerSys, DebugUISys};

fn main() {
    let engine = InvaderBuilder::new()
        .add_system(GameManagerSys::new())
        .add_system(ControllerSys::new())
        .add_system(DebugUISys::new())
        .set_stage("assets/placeholder/stages/stage.mst")
        .add_sprite("wizard", "assets/placeholder/sprites/wizard.png")
        .add_sprite("tree", "assets/placeholder/sprites/tree.png")
        .add_sprite("bolt0", "assets/placeholder/sprites/bolt0.png")
        .add_sprite("bolt1", "assets/placeholder/sprites/bolt1.png")
        .add_sprite("bolt2", "assets/placeholder/sprites/bolt2.png")
        .add_sprite("bolt3", "assets/placeholder/sprites/bolt3.png")
        .add_sprite("r", "assets/placeholder/sprites/32x32-w-r.png")
        .add_sprite("g", "assets/placeholder/sprites/32x32-w-g.png")
        .add_sprite("b", "assets/placeholder/sprites/32x32-w-b.png")
        .add_sprite("blank", "assets/placeholder/sprites/blank.png")
        .add_font("caveat", "assets/placeholder/fonts/caveat.ttf", 64, 10, 10, 10)
        .add_font("nemoy", "assets/placeholder/fonts/nemoy.otf", 64, 200, 128, 255)
        .add_font("patrickhand", "assets/placeholder/fonts/patrickhand.ttf", 64, 255, 255, 255)
        .build();
    engine.run();
}
