use tokio::sync::mpsc::UnboundedSender;

use crate::engine::{
    components::{
        entity_component::{
            entities::{base_entities::CubeEntity, entity::HasModel, entity_enum::EntityType},
            entity_events::{CreateEntityEvent, DeleteEntityEvent},
        },
        vulkan_component::vulkan_events::{VulkanCreateObjectEvent, VulkanDeleteObjectEvent},
    },
    event_bus::event_bus::EventBus,
    repositories::entity_repository::EntityRepository,
    utils::structs::transform::Transform,
};
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

pub struct EntityComponent {
    entity_repository: EntityRepository,
    event_bus_ptr: Arc<EventBus>,
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
}

impl EntityComponent {
    pub fn new(
        event_bus_ptr: Arc<EventBus>,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    ) -> Arc<Mutex<Self>> {
        let entity_component = Arc::new(Mutex::new(EntityComponent {
            entity_repository: EntityRepository::new(),
            event_bus_ptr: event_bus_ptr.clone(),
            async_sender,
        }));

        EntityComponent::observe_events(entity_component.clone());

        entity_component
    }

    pub fn observe_events(self_ptr: Arc<Mutex<EntityComponent>>) {
        let bus_arc = {
            let this = self_ptr.lock().unwrap();
            this.event_bus_ptr.clone()
        };

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<CreateEntityEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<CreateEntityEvent>() {
                    if let Ok(mut temp_self) = self_ptr_clone.lock() {
                        temp_self.create_entity(event.entity_type.clone(), event.transform.clone());
                    }
                }
            }));

        let self_ptr_clone = self_ptr.clone();
        bus_arc
            .clone()
            .observe::<DeleteEntityEvent>(Box::new(move |event_any| {
                if let Some(event) = event_any.downcast_ref::<DeleteEntityEvent>() {
                    if let Ok(mut temp_self) = self_ptr_clone.lock() {
                        temp_self.delete_entity(&event.entity_id);
                    }
                }
            }));
    }

    fn create_entity(&mut self, _entity_type: EntityType, entity_transform: Transform) {
        let entity = CubeEntity::new(entity_transform.clone());
        let vertices = entity.get_model().get_model().to_vec();

        let entity_id = self.entity_repository.add_entity(Box::new(entity));

        let create_vulkan_object_event = VulkanCreateObjectEvent {
            object_id: *entity_id,
            object_transform: entity_transform,
            vertices,
            texture_path: "src/engine/vulkan/base_resources/default_texture.png".to_string(),
        };

        let _ = self.async_sender.send(Box::new(create_vulkan_object_event));
    }

    fn delete_entity(&mut self, entity_id: &usize) {
        self.entity_repository.remove_entity(entity_id);

        let vulkan_delete_event = VulkanDeleteObjectEvent {
            object_id: *entity_id,
        };

        let _ = self.async_sender.send(Box::new(vulkan_delete_event));
    }
}
