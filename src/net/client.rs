use std::time::Instant;
use std::convert::TryInto;
use std::collections::HashMap;
use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use rand::random;
use vitrellogy_macro::DefaultConstructor;

use crate::net::packet::{Packet, ConAcknowledgePacket};

const PORT_PREFIX: u16 = 20200;
const PACKET_LENGTH: usize = 32;

pub struct NetworkClient {
    socket: UdpSocket,
    id: ClientID,
    clients: HashMap<ClientID, SocketAddr>,
    time: Instant,
    open: bool
}

impl NetworkClient {
    pub fn new() -> Result<Self, &'static str> {
        let socket = (0..=9).filter_map(|i| UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT_PREFIX + i)).ok()).next().ok_or("no available ports")?;
        socket.set_nonblocking(true).or(Err("could not configure socket"))?;

        Ok(Self {
            socket: socket,
            id: ClientID::new(random()),
            clients: HashMap::new(),
            time: Instant::now(),
            open: false
        })
    }

    pub fn broadcast(&self, packet: Packet) -> Result<(), &'static str> {
        if self.open {
            for (_id, address) in &self.clients {
                self.send_packet(&packet, address)?;
            }
        }
        Ok(())
    }

    pub fn receive(&mut self) -> Vec<Packet> {
        let mut packets: Vec<Packet> = Vec::new();

        if self.open {
            while let Some((packet, origin_id, address, time)) = self.receive_packet() {
                match packet {
                    Packet::ConRequest => {
                        let mut new_id = origin_id;
                        while new_id == self.id || self.clients.contains_key(&new_id) {
                            new_id = ClientID::new(random());
                        }
                        self.clients.insert(new_id, address);
                        self.send_packet(&Packet::ConAcknowledge(ConAcknowledgePacket::new(new_id)), &address).unwrap();
                    },
                    Packet::ConAcknowledge(p) => {
                        self.id = p.new_local_id;
                        self.clients.insert(origin_id, address);
                    },
                    _ => packets.push(packet)
                }
            }
        }

        packets
    }

    pub fn connect(&mut self, addr: IpAddr) -> Result<(), &'static str> {
        self.clients.clear();

        let possible_addresses: Vec<SocketAddr> = (0..=9).map(|i| SocketAddr::new(addr, PORT_PREFIX + i)).collect();
        for addr in possible_addresses {
            if addr != self.socket.local_addr().or(Err("no socket bound"))? {
                self.send_packet(&Packet::ConRequest, &addr)?;
            }
        }

        Ok(())
    }

    pub fn send_packet(&self, p: &Packet, a: &SocketAddr) -> Result<(), &'static str> {
        let mut buf = p.into_bytes();

        let time = self.time.elapsed().as_millis() as u32;
        buf.extend_from_slice(&time.to_le_bytes());
        buf.extend_from_slice(&self.id.0.to_le_bytes());

        self.socket.send_to(&buf, a).or(Err("fialed to send packet"))?;
        Ok(())
    }

    pub fn receive_packet(&self) -> Option<(Packet, ClientID, SocketAddr, u32)> {
        let mut buf = [0; PACKET_LENGTH];
        self.socket.recv_from(&mut buf).ok().and_then(|(l, a)| {
            if l > PACKET_LENGTH {
                return None;
            }

            let mut data: Vec<u8> = Vec::with_capacity(l);
            data.extend_from_slice(&buf[0..l]);

            let time = u32::from_le_bytes(data[(l - 8)..(l - 4)].try_into().unwrap());
            let origin_id = ClientID::new(u32::from_le_bytes(data[(l - 4)..l].try_into().unwrap()));

            Some((Packet::from_bytes(data), origin_id, a, time))
        })
    }

    pub fn open(&mut self) {
        self.open = true;
        self.time = Instant::now();
    }

    pub fn close(&mut self) {
        self.open = true;
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, DefaultConstructor)]
pub struct ClientID(pub u32);
