pub mod client;
pub mod packet;

use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use vitrellogy_macro::DefaultConstructor;
use specs::{Component, NullStorage, System, ReadStorage, WriteStorage, Join, Write};

use crate::misc::TransformCom;
use crate::net::client::NetworkClient;
use crate::net::packet::{Packet, TransformPacket};
use crate::render::ui::UIEventRes;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct SyncTransformCom;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct RemoteTransformCom;

#[derive(DefaultConstructor)]
pub struct NetworkSyncSys {
    client: Arc<Mutex<NetworkClient>>
}

impl<'a> System<'a> for NetworkSyncSys {
    type SystemData = (Write<'a, NetworkRes>,
        Write<'a, UIEventRes>,
        ReadStorage<'a, SyncTransformCom>,
        ReadStorage<'a, RemoteTransformCom>,
        WriteStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut config, mut ui_events, sync_transform_flags, _remote_transform_flags, mut transforms) = data;
        let mut client = self.client.lock().unwrap();

        for i in 0..ui_events.0.len() {
            match ui_events.0.get(i).unwrap().element_name.as_str() {
                "net_connect" => {
                    println!("Connecting!");
                    config.hosting = false;
                    config.dirty = true;
                    ui_events.0.remove(i);
                },
                _ => ()
            }
        }

        let data = client.receive();
        if data.len() > 0 {
             println!("Received: {:?}", data);
        }

        for (_sync_transform, transform) in (&sync_transform_flags, &mut transforms).join() {
            client.broadcast(Packet::Transform(TransformPacket::new(transform.clone()))).unwrap();
        }

        if config.dirty {
            if config.open {
                client.open();
                if !config.hosting {
                    if client.connect(IpAddr::V4(Ipv4Addr::new(config.remote.0, config.remote.1, config.remote.2, config.remote.3))).is_err() {
                        println!("Could not connect to remote client");
                    }
                }
            } else {
                client.close();
            }
            config.dirty = false;
        }
    }
}

#[derive(Default, Debug, DefaultConstructor)]
pub struct NetworkRes {
    pub dirty: bool,
    pub open: bool,
    pub hosting: bool,
    pub remote: (u8, u8, u8, u8)
}
