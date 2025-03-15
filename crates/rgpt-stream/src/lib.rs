use std::collections::HashMap;

use bytes::Bytes;
use futures::channel::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct StreamRegistry {
    pub handle_map: HashMap<i32, AttachHandle>,
}

impl StreamRegistry {
    /// Creates a new `StreamRegistry` with an empty collection of handles.
    ///
    /// This method initializes a fresh registry without any registered handle mappings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgpt_stream::StreamRegistry;
    ///
    /// let registry = StreamRegistry::new();
    /// // The registry starts without any attached handles.
    /// ```
    pub fn new() -> Self {
        StreamRegistry {
            handle_map: HashMap::new(),
        }
    }

    /// Registers an `AttachHandle` with the given unique identifier.
    ///
    /// Inserts the provided handle into the registry if no handle is already associated with the
    /// specified `id`. If a handle is already registered under that `id`, the function panics with
    /// the message "should not be duplicate IDs".
    ///
    /// # Panics
    ///
    /// Panics if an `AttachHandle` is already registered under the given `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rgpt_stream::{StreamRegistry, AttachHandle};
    /// use tokio::sync::{oneshot, mpsc};
    /// use bytes::Bytes;
    ///
    /// let mut registry = StreamRegistry::new();
    /// let (attacher, _receiver) = oneshot::channel::<mpsc::UnboundedSender<Bytes>>();
    /// let handle = AttachHandle::new(attacher);
    /// registry.register(1, handle);
    /// ```
    pub fn register(&mut self, id: i32, handle: AttachHandle) {
        if self.handle_map.get(&id).is_none() {
            self.handle_map
                .insert(id, handle)
                .ok_or(())
                .expect_err("unreachable")
        }
        panic!("should not be duplicate IDs");
    }

    /// Attempts to attach to the stream handle for the given ID.
    ///
    /// This function removes the `AttachHandle` associated with the provided `id` from the registry.
    /// If a handle is found, it calls its `attach()` method, which creates a new unbounded channel,
    /// sends the channel's sender through a oneshot channel, and returns the corresponding receiver.
    ///
    /// # Returns
    ///
    /// Returns `Some(mpsc::UnboundedReceiver<Bytes>)` if a handle was found and attached, or `None`
    /// if no handle is registered with the specified `id`.
    ///
    /// # Panics
    ///
    /// Panics if the receiver end of the oneshot channel was dropped before the sender could be transmitted.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::{oneshot, mpsc};
    /// use bytes::Bytes;
    /// use crate::{StreamRegistry, AttachHandle};
    ///
    /// // Create a new registry.
    /// let mut registry = StreamRegistry::new();
    ///
    /// // Prepare an attach handle.
    /// let (attacher, _unused_receiver) = oneshot::channel::<mpsc::UnboundedSender<Bytes>>();
    /// let handle = AttachHandle::new(attacher);
    ///
    /// // Register the handle with a unique identifier.
    /// registry.register(1, handle);
    ///
    /// // Attempt to attach to the handle.
    /// if let Some(receiver) = registry.try_attach(1) {
    ///     // Successfully attached; use `receiver` for further communication.
    /// } else {
    ///     // No handle associated with the given ID.
    /// }
    /// ```
    pub fn try_attach(&mut self, id: i32) -> Option<mpsc::UnboundedReceiver<Bytes>> {
        self.handle_map.remove(&id).map(|handle| handle.attach())
    }
}

#[derive(Debug)]
pub struct AttachHandle {
    pub attacher: oneshot::Sender<mpsc::UnboundedSender<Bytes>>,
}

impl AttachHandle {
    /// Constructs a new `AttachHandle` by wrapping the specified oneshot sender.
    ///
    /// The provided attacher is used to transmit an unbounded sender during the attachment process.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::{oneshot, mpsc};
    /// use bytes::Bytes;
    ///
    /// // Create a oneshot channel for sending a mpsc unbounded sender.
    /// let (attacher, _receiver) = oneshot::channel::<mpsc::UnboundedSender<Bytes>>();
    /// let handle = AttachHandle::new(attacher);
    /// ```
    pub fn new(attacher: oneshot::Sender<mpsc::UnboundedSender<Bytes>>) -> Self {
        AttachHandle { attacher }
    }

    /// Consumes this `AttachHandle`, establishing a new byte stream channel.
    /// 
    /// This method creates an unbounded channel and sends its sender through the internal oneshot channel.
    /// It panics with the message "recv end was dropped first" if the corresponding oneshot receiver has already been dropped.
    /// The returned receiver can then be used to receive streamed bytes.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tokio::sync::oneshot;
    /// use futures::channel::mpsc::UnboundedReceiver;
    /// use bytes::Bytes;
    /// use my_crate::AttachHandle; // Adjust the module path as needed
    /// 
    /// // Create a oneshot channel for sending an unbounded sender.
    /// let (attacher, _oneshot_receiver) = oneshot::channel();
    /// let handle = AttachHandle::new(attacher);
    /// let rx: UnboundedReceiver<Bytes> = handle.attach();
    /// // `rx` can now be used to receive bytes from the stream.
    /// ```
    pub fn attach(self) -> mpsc::UnboundedReceiver<Bytes> {
        let (tx, rx) = mpsc::unbounded();
        self.attacher.send(tx).expect("recv end was dropped first");
        rx
    }
}
