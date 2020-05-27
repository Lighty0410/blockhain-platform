use super::model;
use anyhow::Result;
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, Swarm,
};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,

    #[behaviour(ignore)]
    #[allow(dead_code)]
    pub ignored_member: bool,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for Behaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = event {
            println!("message: {:?}", String::from_utf8_lossy(&message.data))
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }

            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}
