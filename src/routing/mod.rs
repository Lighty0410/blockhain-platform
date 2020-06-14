use crate::controller::Controller;
use crate::platform::event::WithBehaviour;
use crate::platform::model::{Event, PeerInfo};
use anyhow::Result;
use libp2p::{floodsub::Topic, swarm::NetworkBehaviour, Swarm};

use libp2p::{floodsub::Floodsub, mdns::Mdns};

pub struct Handler<T: NetworkBehaviour> {
    ctrl: Controller,

    swarm: Swarm<T>,
}

impl Handler<WithBehaviour> {
    pub fn new(peer_info: PeerInfo, topic: &Topic, ctrl: Controller) -> Result<Self> {
        let transport = libp2p::build_development_transport(peer_info.key)?;
        let mdns = Mdns::new()?;

        let mut behaviour = WithBehaviour {
            floodsub: Floodsub::new(peer_info.peer_id.clone()),
            mdns,
            ignored_member: false,
        };

        behaviour.floodsub.subscribe(topic.clone()); // TODO: do it after the registration event.

        let mut swarm = Swarm::new(transport, behaviour, peer_info.peer_id);

        Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;

        Ok(Handler { ctrl, swarm })
    }

    pub fn proceed_event(&mut self, event: Event, topic: Topic) -> Result<()> {
        match event.event_type.as_str() {
            "/register" => {
                self.ctrl.register(event.register)?;
                println!("all ok");
                self.swarm.floodsub.publish(topic, "ok".as_bytes());
            }
            _ => {}
        }

        Ok(())
    }
}
