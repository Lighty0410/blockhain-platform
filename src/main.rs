mod platform;
use platform::{model::PeerInfo, node::Node};

use anyhow::Result;
use libp2p::{floodsub, identity, PeerId};

fn main() -> Result<()> {
    let key_pair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(key_pair.public());

    let peer_info = PeerInfo::new(key_pair, peer_id);

    let topic = floodsub::Topic::new("chat");

    let mut node = Node::new(peer_info, &topic.clone())?;

    node.run(topic)?;

    Ok(())
}
