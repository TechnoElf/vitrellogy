pub mod controller;

use std::net::ToSocketAddrs;

use specs::*;

use vitrellogy_macro::DefaultConstructor;
use crate::sound::{SoundRequestQueue, SoundRequest, MusicID, LayerID};
use crate::render::{UIEventQueue, UIEvent};
use crate::net::{NetworkRequestQueue, NetworkRequest};

#[derive(DefaultConstructor)]
pub struct DebugUISys {
    layer: LayerID,
    music: MusicID
}

impl<'a> System<'a> for DebugUISys {
    type SystemData = (Read<'a, UIEventQueue>,
        Write<'a, SoundRequestQueue>,
        Write<'a, NetworkRequestQueue>);

    fn run(&mut self, data: Self::SystemData) {
        let (ui_events, mut sound_requests, mut net_requests) = data;

        for event in ui_events.iter() {
            match event {
                UIEvent::ButtonPressed { id } => match id.as_str() {
                    "sound" => {
                        sound_requests.push(SoundRequest::ChangeMusic(self.music, self.layer));
                        self.layer += 1;
                        if self.layer > 3 {
                            self.layer = 0;
                        }
                    },
                    "connect" => {
                        net_requests.push(NetworkRequest::Open);
                        net_requests.push(NetworkRequest::Connect(("apollo.undertheprinter.com", 0).to_socket_addrs().unwrap().next().unwrap()));
                    },
                    "host" => {
                        net_requests.push(NetworkRequest::Open);
                    },
                    "debug" => {
                        net_requests.push(NetworkRequest::Debug);
                    },
                    _ => ()
                }
            }
        }
    }
}
