use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type BoxedHandler = Box<dyn Fn(&dyn Any) + Send + Sync>;

pub struct EventBus {
    handlers: HashMap<TypeId, Vec<BoxedHandler>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe<E, F>(&mut self, handler: F)
    where
        E: Send + Sync + 'static,
        F: Fn(&E) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<E>();
        let wrapped_handler: BoxedHandler = Box::new(move |event_any| {
            if let Some(event) = event_any.downcast_ref::<E>() {
                handler(event);
            }
        });

        self.handlers
            .entry(type_id)
            .or_default()
            .push(wrapped_handler);
    }

    pub fn publish<E: Send + Sync + 'static>(&self, event: &E) {
        let type_id = TypeId::of::<E>();

        if let Some(handlers) = self.handlers.get(&type_id) {
            for handler in handlers {
                handler(event as &dyn Any)
            }
        }
    }
}
