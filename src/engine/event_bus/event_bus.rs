use std::{
    any::{type_name_of_val, Any, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
type Callback = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventBus {
    observers: RwLock<HashMap<TypeId, Vec<Callback>>>,
}

impl EventBus {
    pub fn new() -> Arc<EventBus> {
        Arc::new(EventBus {
            observers: RwLock::new(HashMap::new()),
        })
    }

    pub fn observe<E: 'static + Send + Sync>(&self, callback: Callback) {
        let mut observers = self.observers.write().unwrap();

        observers
            .entry(TypeId::of::<E>())
            .or_default()
            .push(callback);
    }

    pub fn emit(&self, event: Box<dyn Any + Send + Sync>) {
        let observers = self.observers.read().unwrap();

        if let Some(callbacks) = observers.get(&(*event).type_id()) {
            for callback in callbacks {
                callback(event.as_ref());
            }
        }
    }

    pub async fn run(
        self_ptr: Arc<EventBus>,
        async_receiver: UnboundedReceiver<Box<dyn Any + Send + Sync>>,
    ) {
        let mut stream = UnboundedReceiverStream::new(async_receiver);

        while let Some(event) = stream.next().await {
            let bus = self_ptr.clone();
            tokio::spawn(async move {
                bus.clone().emit(event);
            });
        }
    }
}
