use invader::InvaderBuilder;

mod game;
use game::{GameManagerSys, ControllerSys, DebugUISys};

fn main() {
    let engine = InvaderBuilder::new()
        .add_system(GameManagerSys::new())
        .add_system(ControllerSys::new())
        .add_system(DebugUISys::new())
        .set_stage("assets/placeholder/stages/stage.mst")
        .add_sprite_sheet("assets/placeholder/stages/sprite_sheet.mss")
        .build();
    engine.run();
}
