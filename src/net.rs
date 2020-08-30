pub mod packet;

use std::time::Instant;
use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use rand::random;
use vitrellogy_macro::DefaultConstructor;

use nalgebra::Vector2;

use specs::{Component, NullStorage, System, ReadStorage, WriteStorage, Join, Write, Entities, WriteExpect, LazyUpdate, Read, DenseVecStorage};
use specs::world::Builder;

use crate::render::ui::{UIEvent, UIEventQueue};
use crate::render::SpriteCom;
use crate::net::packet::*;
use crate::physics::{PhysicsRes, ColliderCom, RigidBodyCom, TransformCom};

const PORT_PREFIX: u16 = 20200;
const PACKET_LENGTH: usize = 32;
const DISCONNECT_TIMEOUT: u128 = 2000;

pub struct NetworkRes {
    socket: Option<UdpSocket>,
    id: NetID,
    host_id: NetID,
    peers: Vec<(NetID, SocketAddr, Instant)>,
    connecting: bool
}

impl NetworkRes {
    pub fn new() -> Self {
        Self {
            socket: None,
            id: NetID::new(),
            host_id: NetID::new(),
            peers: Vec::new(),
            connecting: false
        }
    }

    pub fn broadcast(&self, packet: Packet) -> Result<(), String> {
        for (_, socket, _) in &self.peers {
            self.send_packet(&packet, socket)?;
        }
        Ok(())
    }

    pub fn process(&mut self, events: &mut NetworkEventQueue) {
        events.clear();
        if self.socket.is_none() {
            return;
        }

        while let Ok((packet, origin_socket)) = self.receive_packet() {
            match packet {
                Packet::ConRequest(_) => {
                    if !self.connecting && self.peers.iter().find_map(|(_, socket, _)| if &origin_socket == socket { Some(()) } else { None }).is_none() {
                        if self.host_id.is_shared() {
                            let host_socket = self.peers.iter().find_map(|(id, socket, _)| if id == &self.host_id { Some(socket) } else { None }).unwrap();
                            self.send_packet(&Packet::ConRedirect(ConRedirectPacket::new(host_socket.clone())), &origin_socket).unwrap();
                        } else {
                            let mut assigned_id = NetID::new().init(random());
                            while assigned_id == self.id || self.peers.iter().find_map(|(id, _, _)| if id == &assigned_id { Some(()) } else { None }).is_some() {
                                assigned_id = NetID::new().init(random());
                            }
                            self.send_packet(&Packet::ConAcknowledge(ConAcknowledgePacket::new(self.id, assigned_id)), &origin_socket).unwrap();

                            for (id, socket, _) in &self.peers {
                                self.send_packet(&Packet::ConNew(ConNewPacket::new(id.clone(), socket.clone())), &origin_socket).unwrap();
                                self.send_packet(&Packet::ConNew(ConNewPacket::new(assigned_id, origin_socket)), socket).unwrap();
                            }
                            self.peers.push((assigned_id, origin_socket, Instant::now()));
                            events.push(NetworkEvent::PeerConnected(assigned_id));
                        }
                    }
                },
                Packet::ConAcknowledge(p) => {
                    if !self.host_id.is_shared() && self.connecting {
                        self.id = p.assigned_id;
                        self.host_id = p.origin_id;
                        self.peers.push((p.origin_id, origin_socket, Instant::now()));
                        events.push(NetworkEvent::PeerConnected(p.origin_id));
                    }
                },
                Packet::ConRedirect(p) => {
                    if !self.host_id.is_shared() {
                        self.send_packet(&Packet::ConRequest(ConRequestPacket::new()), &p.host_socket).unwrap();
                    }
                },
                Packet::ConNew(p) => {
                    if self.host_id.is_shared() {
                        self.peers.push((p.peer_id, p.socket, Instant::now()));
                        events.push(NetworkEvent::PeerConnected(p.peer_id));
                    }
                },
                Packet::ConDelete(p) => {
                    if self.host_id.is_shared() {
                        for i in 0..self.peers.len() {
                            if p.peer_id == self.peers.get(i).unwrap().0 {
                                self.peers.remove(i);
                                break;
                            }
                        }
                        events.push(NetworkEvent::PeerDisconnected(p.peer_id));
                    }
                },
                Packet::ConHeartbeat(p) => {
                    self.peers.iter_mut().find_map(|peer| if p.origin_id == peer.0 {
                        peer.2 = Instant::now();
                        Some(())
                    } else {
                        None
                    });
                },
                Packet::Transform(p) => {
                    events.push(NetworkEvent::PeerMoved(p.origin_id, p.transform));
                }
                _ => ()
            }
        }

        let mut i = 0; 
        while i < self.peers.len() {
            if self.peers.get(i).unwrap().2.elapsed().as_millis() >= DISCONNECT_TIMEOUT {
                if !self.host_id.is_shared() {
                    let old_id = self.peers.remove(i).0;
                    self.broadcast(Packet::ConDelete(ConDeletePacket::new(old_id))).unwrap();
                    events.push(NetworkEvent::PeerDisconnected(old_id));
                } else {
                    i += 1;
                }
            } else {
                self.send_packet(&Packet::ConHeartbeat(ConHeartbeatPacket::new(self.id)), &self.peers.get(i).unwrap().1).unwrap();
                i += 1;
            }
        }
    }

    pub fn connect(&mut self, addr: SocketAddr) -> Result<(), String> {
        if let Some(socket) = &self.socket {
            self.peers.clear();
            self.connecting = true;

            let possible_addresses: Vec<SocketAddr> = (0..=9).map(|i| SocketAddr::new(addr.ip(), PORT_PREFIX + i)).collect();
            for addr in possible_addresses {
                if addr != socket.local_addr().unwrap() {
                    self.send_packet(&Packet::ConRequest(ConRequestPacket::new()), &addr)?;
                }
            }

            Ok(())
        } else {
            Err("socket closed".to_string())
        }
    }

    pub fn send_packet(&self, p: &Packet, a: &SocketAddr) -> Result<(), String> {
        if let Some(socket) = &self.socket {
            socket.send_to(&p.into_bytes(), a).map_err(|e| format!("failed to send packet to {}: {}", a, e)).map(|_| ())
        } else {
            Err("socket closed".to_string())
        }
    }

    pub fn receive_packet(&self) -> Result<(Packet, SocketAddr), String> {
        if let Some(socket) = &self.socket {
            let mut buf = [0; PACKET_LENGTH];
            socket.recv_from(&mut buf).map_err(|e| format!("could not receive packet: {}", e)).and_then(|(length, origin_sock)| {
                match length <= PACKET_LENGTH {
                    true => Ok((length, origin_sock)),
                    false => Err("packet exceeds maximum length".to_string())
                }
            }).map(|(length, origin_sock)| {
                let mut data: Vec<u8> = Vec::with_capacity(length);
                data.extend_from_slice(&buf[0..length]);
                (Packet::from_bytes(data), origin_sock)
            })
        } else {
            Err("socket closed".to_string())
        }
    }

    pub fn open(&mut self) -> Result<(), String> {
        if self.socket.is_none() {
            self.socket = Some((0..=9).filter_map(|i| UdpSocket::bind(("0.0.0.0", PORT_PREFIX + i)).ok()).next().ok_or("no available ports")?);
            self.socket.as_mut().unwrap().set_nonblocking(true).map_err(|e| format!("could not configure socket: {}", e))?;
            self.id = NetID::new().init(random());   
            self.host_id = NetID::new();
            self.peers.clear();
            self.connecting = false;
            Ok(())
        } else {
            Err("client is already open".to_string())
        }
    }

    pub fn close(&mut self) {
        self.socket = None;
        self.id = NetID::new();   
        self.host_id = NetID::new();   
        self.peers.clear();
    }
}

event_queue! {
    NetworkEventQueue: pub enum NetworkEvent {
        PeerConnected(NetID),
        PeerDisconnected(NetID),
        PeerMoved(NetID, TransformCom)
    }
}

#[derive(DefaultConstructor)]
pub struct NetworkSyncSys;

impl<'a> System<'a> for NetworkSyncSys {
    type SystemData = (Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteExpect<'a, NetworkRes>,
        Write<'a, NetworkEventQueue>,
        Read<'a, UIEventQueue>,
        Write<'a, PhysicsRes>,
        ReadStorage<'a, NetMasterTransformCom>,
        ReadStorage<'a, NetSlaveTransformCom>,
        WriteStorage<'a, TransformCom>,
        ReadStorage<'a, RigidBodyCom>,
        ReadStorage<'a, ColliderCom>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, mut net, mut net_events, ui_events, mut physics, master_transform_flags, slave_transform_flags, mut transforms, rigid_bodies, colliders) = data;

        for event in ui_events.iter() { 
            match event {
                UIEvent::ButtonPressed { id } => match id.as_ref() {
                    "net_connect" => {
                        net.close();
                        for (entity, _, rb, col) in (&entities, &slave_transform_flags, &rigid_bodies, &colliders).join() {
                            physics.bodies.remove(rb.0);
                            physics.colliders.remove(col.0);
                            updater.remove::<SpriteCom>(entity);
                            updater.remove::<TransformCom>(entity);
                            updater.remove::<NetSlaveTransformCom>(entity);
                            updater.remove::<RigidBodyCom>(entity);
                            updater.remove::<ColliderCom>(entity);
                        }
                        net.open().expect("failed to start network client");
                        match net.connect(("apollo.undertheprinter.com", 0).to_socket_addrs().unwrap().next().unwrap()) {
                            Ok(()) => (),
                            Err(e) => {
                                println!("could not connect to remote client: {}", e);
                                net.close();
                            }
                        }
                    },
                    "net_host" => {
                        net.close();
                        for (entity, _, rb, col) in (&entities, &slave_transform_flags, &rigid_bodies, &colliders).join() {
                            physics.bodies.remove(rb.0);
                            physics.colliders.remove(col.0);
                            updater.remove::<SpriteCom>(entity);
                            updater.remove::<TransformCom>(entity);
                            updater.remove::<NetSlaveTransformCom>(entity);
                            updater.remove::<RigidBodyCom>(entity);
                            updater.remove::<ColliderCom>(entity);
                        }
                        net.open().expect("failed to start network client");
                    },
                    "debug" => {
                        println!("Data for client {}:\n  open: {}\n  hosting: {}\n  host: {}\n  socket: {}\n  peers: {:?}", net.id, net.socket.is_some(), !net.host_id.is_shared(), net.host_id, net.socket.as_ref().map(|s| format!("{:?}", s)).or(Some("".to_string())).unwrap(), net.peers);
                    },
                    _ => ()
                }
            }
        }

        net.process(&mut net_events);
        for event in net_events.iter() {
            match event {
                NetworkEvent::PeerMoved(origin_id, t) => {
                    for (slave_transform, transform) in (&slave_transform_flags, &mut transforms).join() {
                        if origin_id == &slave_transform.0 {
                            transform.pos = t.pos;
                        }
                    }
                },
                NetworkEvent::PeerConnected(origin_id) => {
                    let rb = physics.create_rigid_body_static();
                    let col = physics.create_collider_rectangle(Vector2::new(2.0, 2.0), &rb);
                    updater.create_entity(&entities).with(SpriteCom::new("wizard", Vector2::new(2.0, 2.0)))
                        .with(TransformCom::new(Vector2::new(0.0, 0.0)))
                        .with(NetSlaveTransformCom::new(origin_id.clone()))
                        .with(rb).with(col).build();
                },
                NetworkEvent::PeerDisconnected(origin_id) => {
                    for (entity, slave_transform, rb, col) in (&entities, &slave_transform_flags, &rigid_bodies, &colliders).join() {
                        if origin_id == &slave_transform.0 {
                            physics.bodies.remove(rb.0);
                            physics.colliders.remove(col.0);
                            updater.remove::<SpriteCom>(entity);
                            updater.remove::<TransformCom>(entity);
                            updater.remove::<NetSlaveTransformCom>(entity);
                            updater.remove::<RigidBodyCom>(entity);
                            updater.remove::<ColliderCom>(entity);
                        }
                    }
                }
            }
        }

        for (_master_transform, transform) in (&master_transform_flags, &mut transforms).join() {
            net.broadcast(Packet::Transform(TransformPacket::new(net.id, transform.clone()))).unwrap();
        }
    }
}

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(NullStorage)]
pub struct NetMasterTransformCom;

#[derive(Component, Debug, DefaultConstructor, Default)]
#[storage(DenseVecStorage)]
pub struct NetSlaveTransformCom(NetID);

