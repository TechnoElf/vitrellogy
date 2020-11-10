use invader::InvaderBuilder;
use invader::misc::persist::SpriteSheet;

mod game;
use game::{GameManagerSys, ControllerSys, DebugUISys};

fn main() {
    let mut ss = SpriteSheet::from_file("assets/placeholder/stages/sprite_sheet.mss").unwrap();
    ss.add_font("ui", "assets/placeholder/fonts/ui.ttf", 64, 64, 62, 65);
    ss.add_font("ui_fg", "assets/placeholder/fonts/ui.ttf", 64, 252, 252, 250);
    ss.into_file("assets/placeholder/stages/sprite_sheet.mss");

    let engine = InvaderBuilder::new()
        .add_system(GameManagerSys::new())
        .add_system(ControllerSys::new())
        .add_system(DebugUISys::new())
        .set_stage("assets/placeholder/stages/stage.mst")
        .add_sprite_sheet("assets/placeholder/stages/sprite_sheet.mss")
        .build();
    engine.run();
}
