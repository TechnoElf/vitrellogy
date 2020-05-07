pub mod client;
pub mod packet;

use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
use vitrellogy_macro::DefaultConstructor;
use specs::{Component, NullStorage, System, ReadStorage, WriteStorage, Join, Write};

use crate::physics::TransformCom;
use crate::net::client::NetworkClient;
use crate::net::packet::{Packet, TransformPacket};

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
        ReadStorage<'a, SyncTransformCom>,
        ReadStorage<'a, RemoteTransformCom>,
        WriteStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut config, sync_transform_flags, _remote_transform_flags, mut transforms) = data;
        let mut client = self.client.lock().unwrap();

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
