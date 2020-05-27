use super::{model::PeerInfo, node_behaviour::Behaviour};
use anyhow::Result;
use libp2p::{floodsub::Floodsub, floodsub::Topic, mdns::Mdns, swarm::NetworkBehaviour, Swarm};

pub struct Node<T: NetworkBehaviour> {
    pub swarm: Swarm<T>,
}

impl Node<Behaviour> {
    pub fn new(peer_info: PeerInfo, topic: &Topic) -> Result<Self> {
        let transport = libp2p::build_development_transport(peer_info.key)?;
        let mdns = Mdns::new()?;

        let mut b = Behaviour {
            floodsub: Floodsub::new(peer_info.peer_id.clone()),
            mdns,
            ignored_member: false,
        };
        b.floodsub.subscribe(topic.clone());

        let swarm = Swarm::new(transport, b, peer_info.peer_id);
        Ok(Node { swarm })
    }
}
