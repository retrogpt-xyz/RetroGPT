use std::collections::HashMap;

use bytes::Bytes;
use futures::channel::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct StreamRegistry {
    pub handle_map: HashMap<i32, AttachHandle>,
}

impl StreamRegistry {
    pub fn new() -> Self {
        StreamRegistry {
            handle_map: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: i32, handle: AttachHandle) -> Option<()> {
        if self.handle_map.get(&id).is_none() {
            return None;
        }
        self.handle_map.insert(id, handle);
        return Some(());
    }

    pub fn try_attach(&mut self, id: i32) -> Option<mpsc::UnboundedReceiver<Bytes>> {
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
