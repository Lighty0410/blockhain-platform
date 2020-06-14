pub mod event;
pub mod model;

use crate::platform::event::WithBehaviour;
use crate::routing::Handler;
use crate::tcp::server::Server;

use anyhow::{Error, Result};
use async_std::task;
use futures::{
    future,
    task::{Context, Poll},
};

use crate::controller::Controller;
use crate::platform::model::{Event, PeerInfo};
use libp2p::{floodsub::Topic, identity, swarm::NetworkBehaviour, PeerId};

pub struct Node<T: NetworkBehaviour> {
    handler: Handler<T>,
    server: Server,
}

impl Node<WithBehaviour> {
    pub fn new(topic: &Topic) -> Result<Self> {
        let key_pair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(key_pair.public());
        let ctrl = Controller::new(peer_id.clone());

        let peer_info = PeerInfo::new(key_pair, peer_id);
        let handler = Handler::new(peer_info, topic, ctrl)?; // TODO: take the topic from the request. Don't create it manually.

        let server = Server::new()?;

        Ok(Node { handler, server })
    }

    pub fn run(&mut self, topic: Topic) -> Result<()> {
        loop {
            match self.server.parse_event::<model::Event>(&mut Vec::new()) {
                Ok(event) => {
                    if let Err(e) = self.handler.proceed_event(event, topic.clone()) {
                        println!("{:?}", e);
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }
}
