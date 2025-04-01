use std::collections::HashMap;

use bytes::Bytes;
use futures::channel::mpsc;
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Debug)]
pub struct StreamRegistry {
    pub handle_map: HashMap<Uuid, AttachHandle>,
}

impl Default for StreamRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamRegistry {
    pub fn new() -> Self {
        StreamRegistry {
            handle_map: HashMap::new(),
        }
    }

    pub fn register(&mut self, uuid: Uuid, handle: AttachHandle) -> Option<Uuid> {
        if self.handle_map.contains_key(&uuid) {
            return None; // ID already exists, registration failed
        }
        self.handle_map.insert(uuid, handle);
        Some(uuid)
    }

    pub fn try_attach(&mut self, id: Uuid) -> Option<mpsc::UnboundedReceiver<Bytes>> {
        self.handle_map.remove(&id).map(|handle| handle.attach())
    }
}

#[derive(Debug)]
pub struct AttachHandle {
    pub attacher: oneshot::Sender<mpsc::UnboundedSender<Bytes>>,
}

impl AttachHandle {
    pub fn new(attacher: oneshot::Sender<mpsc::UnboundedSender<Bytes>>) -> Self {
        AttachHandle { attacher }
    }

    pub fn attach(self) -> mpsc::UnboundedReceiver<Bytes> {
        let (tx, rx) = mpsc::unbounded();
        self.attacher.send(tx).expect("recv end was dropped first");
        rx
    }
}
