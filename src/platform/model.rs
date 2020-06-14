use libp2p::identity::Keypair;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

pub struct PeerInfo {
    pub key: Keypair,
    pub peer_id: PeerId,
}

impl PeerInfo {
    pub fn new(key: Keypair, peer_id: PeerId) -> Self {
        PeerInfo { key, peer_id }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,

    // pub auth: Auth,
    pub register: Register,
}
//
// #[derive(Serialize, Deserialize)]
// pub struct Auth {
//     pub key: Key,
//     pub password: String,
// }

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub username: String,
    pub password: String,
}
