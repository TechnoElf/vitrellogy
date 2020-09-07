pub mod controller;

use specs::*;

use vitrellogy_macro::DefaultConstructor;
use crate::sound::{SoundRequestQueue, SoundRequest, MusicID, LayerID};
use crate::render::{UIEventQueue, UIEvent};

#[derive(DefaultConstructor)]
pub struct DebugUISys {
    layer: LayerID,
    music: MusicID
}

impl<'a> System<'a> for DebugUISys {
    type SystemData = (Read<'a, UIEventQueue>,
        Write<'a, SoundRequestQueue>);

    fn run(&mut self, data: Self::SystemData) {
        let (ui_events, mut sound_requests) = data;

        for event in ui_events.iter() {
            match event {
                UIEvent::ButtonPressed { id } => match id.as_ref() {
                    "sound" => {
                        sound_requests.push(SoundRequest::ChangeMusic(self.music, self.layer));
                        self.layer += 1;
                        if self.layer > 3 {
                            self.layer = 0;
                        }
                    },
                    _ => ()
                }
            }
        }
    }
}
