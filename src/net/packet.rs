use std::convert::TryInto;
use vitrellogy_macro::DefaultConstructor;

use crate::net::client::ClientID;
use crate::physics::TransformCom;
use crate::misc::vec::Vec2;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Packet {
    Empty,
    ConRequest,
    ConAcknowledge(ConAcknowledgePacket),
    Transform(TransformPacket)
}

impl Packet {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();

        match self {
            Packet::Empty => packet.push(0),
            Packet::ConRequest => {
                packet.push(1);
            },
            Packet::ConAcknowledge(p) => {
                packet.push(2);
                packet.extend_from_slice(&p.new_local_id.0.to_le_bytes());
            }
            Packet::Transform(p) => {
                packet.push(3);
                packet.extend_from_slice(&p.transform.pos.x.to_le_bytes());
                packet.extend_from_slice(&p.transform.pos.y.to_le_bytes());
                packet.extend_from_slice(&p.transform.vel.x.to_le_bytes());
                packet.extend_from_slice(&p.transform.vel.y.to_le_bytes());
            }
        }

        packet
    }

    pub fn from_bytes(packet: Vec<u8>) -> Self {
        match packet[0] {
            1 => Packet::ConRequest,
            2 => Packet::ConAcknowledge(ConAcknowledgePacket::new(
                ClientID::new(u32::from_le_bytes(packet[1..=4].try_into().unwrap()))
            )),
            3 => Packet::Transform(TransformPacket::new(
                TransformCom::new(
                    Vec2::new(
                        f32::from_le_bytes(packet[1..=4].try_into().unwrap()),
                        f32::from_le_bytes(packet[5..=8].try_into().unwrap())
                    ),
                    Vec2::new(
                        f32::from_le_bytes(packet[9..=12].try_into().unwrap()),
                        f32::from_le_bytes(packet[13..=16].try_into().unwrap())
                    )
                )
            )),
            _ => Packet::Empty
        }
    }
}

#[derive(Copy, Clone, Debug, DefaultConstructor)]
pub struct ConAcknowledgePacket {
    pub new_local_id: ClientID
}

#[derive(Clone, Debug, DefaultConstructor)]
pub struct TransformPacket {
    pub transform: TransformCom
}
