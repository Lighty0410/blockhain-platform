mod controller;
mod platform;
mod routing;
mod tcp;

use crate::platform::Node;
use anyhow::Result;
use libp2p::floodsub::Topic;

fn main() -> Result<()> {
    let topic = Topic::new("chat");

    let mut node = Node::new(&topic)?;

    node.run(topic)?;

    Ok(())
}
