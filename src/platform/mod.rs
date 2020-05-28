pub mod model;
pub mod node;
pub mod node_behaviour;

use crate::platform::node::Node;
use crate::platform::node_behaviour::Behaviour;

use anyhow::Result;
use async_std::{io, task};
use futures::{
    future,
    prelude::*,
    task::{Context, Poll},
    AsyncBufReadExt,
};
use libp2p::{floodsub::Topic, Swarm};

impl Node<Behaviour> {
    pub fn run(&mut self, topic: Topic) -> Result<()> {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        Swarm::listen_on(&mut self.swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;

        let mut listening = false;

        task::block_on(future::poll_fn(move |cx: &mut Context| {
            loop {
                match stdin.try_poll_next_unpin(cx)? {
                    Poll::Ready(Some(line)) => {
                        self.swarm.floodsub.publish(topic.clone(), line.as_bytes())
                    }
                    Poll::Ready(None) => panic!("stdin closed"),
                    Poll::Pending => break,
                }
            }
            loop {
                match self.swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => println!("{:?}", event),
                    Poll::Ready(None) => return Poll::Ready(Ok(())),
                    Poll::Pending => {
                        if !listening {
                            for addr in Swarm::listeners(&self.swarm) {
                                println!("listening on {:?}", addr);
                                listening = true
                            }
                        }
                        break;
                    }
                }
            }

            Poll::Pending
        }))
    }
}
