use libp2p::identity::Keypair;
use libp2p::PeerId;

pub struct PeerInfo {
    pub key: Keypair,
    pub peer_id: PeerId,
}

impl PeerInfo {
    pub fn new(key: Keypair, peer_id: PeerId) -> Self {
        PeerInfo { key, peer_id }
    }
}
