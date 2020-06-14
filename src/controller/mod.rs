use anyhow::{anyhow, Result};
use libp2p::kad::{record::store::MemoryStore, record::Key, Kademlia, Quorum, Record};
use libp2p::PeerId;

use crate::platform::model::Register;

pub struct Controller {
    db: Kademlia<MemoryStore>,
}

impl Controller {
    pub fn new(peer_id: PeerId) -> Self {
        let store = MemoryStore::new(peer_id.clone());
        let kademlia = Kademlia::new(peer_id, store);

        Controller { db: kademlia }
    }

    pub fn register(&mut self, event: Register) -> Result<Key> {
        let key = Key::new(&event.username);
        let value = event.password.as_bytes().to_vec();

        let record = Record {
            key: key.clone(),
            value,
            publisher: None,
            expires: None,
        };

        self.db
            .put_record(record, Quorum::One)
            .or_else(|e| Err(anyhow!("cannot add record to the db: {:?}", e)))?;

        Ok(key)
    }
}
