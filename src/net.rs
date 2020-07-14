pub mod client;
pub mod packet;

use std::net::{IpAddr, Ipv4Addr};
use vitrellogy_macro::DefaultConstructor;
use specs::{Component, NullStorage, System, ReadStorage, WriteStorage, Join, Write, Entities, WriteExpect};

use crate::misc::TransformCom;
use crate::net::client::NetworkClient;
use crate::net::packet::{Packet, TransformPacket};
use crate::render::ui::UIEventRes;

#[derive(DefaultConstructor)]
pub struct NetworkRes {
    pub dirty: bool,
    pub open: bool,
    pub hosting: bool,
    pub remote: (u8, u8, u8, u8),
    pub client: NetworkClient
}

#[derive(DefaultConstructor)]
pub struct NetworkSyncSys;

impl<'a> System<'a> for NetworkSyncSys {
    type SystemData = (Entities<'a>,
        WriteExpect<'a, NetworkRes>,
        Write<'a, UIEventRes>,
        ReadStorage<'a, NetMasterTransformCom>,
        ReadStorage<'a, NetSlaveTransformCom>,
        WriteStorage<'a, TransformCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut entities, mut net, mut ui_events, master_transform_flags, slave_transform_flags, mut transforms) = data;

        for i in 0..ui_events.0.len() {
            match ui_events.0.get(i).unwrap().element_name.as_str() {
                "net_connect" => {
                    println!("Connecting!");
                    net.hosting = false;
                    net.dirty = true;
                    ui_events.0.remove(i);
                },
                _ => ()
            }
        }

        let data = net.client.receive();
        if data.len() > 0 {
            for packet in data {
                match packet {
                    Packet::Transform(TransformPacket { transform: t }) => {
                        for (_slave_transform, transform) in (&slave_transform_flags, &mut transforms).join() {
                            transform.pos = t.pos;
                        }
                    },
                    _ => ()
                }
            }
        }

        for (_master_transform, transform) in (&master_transform_flags, &mut transforms).join() {
            net.client.broadcast(Packet::Transform(TransformPacket::new(transform.clone()))).unwrap();
        }

        if net.dirty {
            if net.open {
                net.client.open();
                if !net.hosting {
                    let remote = net.remote;
                    if net.client.connect(IpAddr::V4(Ipv4Addr::new(remote.0, remote.1, remote.2, remote.3))).is_err() {
                        println!("Could not connect to remote client");
                    }
                }
            } else {
                net.client.close();
            }
            net.dirty = false;
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct NetMasterTransformCom;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct NetSlaveTransformCom;

