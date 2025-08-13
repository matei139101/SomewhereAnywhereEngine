use crate::engine::{
    components::{
        entity_component::{
            entities::{
                base_entities::CubeEntity,
                entity::{Entity, HasModel},
            },
            entity_events::EntityEvent,
        },
        vulkan_component::vulkan_events::{ObjectCreateInfo, VulkanEvent},
    },
    event_bus::event_bus::EventBus,
    repositories::entity_repository::EntityRepository,
};
use std::sync::{Arc, Mutex};

pub struct EntityComponent {
    event_bus_ptr: Arc<Mutex<EventBus>>,
    entity_repository: EntityRepository,
}

impl EntityComponent {
    pub fn new(event_bus_ptr: Arc<Mutex<EventBus>>) -> Arc<Mutex<Self>> {
        let entity_component = Arc::new(Mutex::new(EntityComponent {
            entity_repository: EntityRepository::new(),
            event_bus_ptr: event_bus_ptr.clone(),
        }));

        EntityComponent::subscribe_to_entity_events(&entity_component, event_bus_ptr);

        entity_component
    }

    fn handle_event(&mut self, event: &EntityEvent) {
        match event {
            EntityEvent::EntityCreationEvent(entity_creation_info) => {
                let entity = CubeEntity::new(entity_creation_info.transform.clone());
                let vertices = entity.get_model().get_model().to_vec();
                let transform = entity.get_transform().clone();
                let entity_id = self.entity_repository.add_entity(Box::new(entity));

                //[TO-DO]: Error check the unwrap and try to get rid of some clones where possible
                let event_info = ObjectCreateInfo {
                    object_id: entity_id.clone(),
                    vertices,
                    object_transform: transform,
                    texture_path: "src/engine/vulkan/base_resources/default_texture.png"
                        .to_string(),
                };

                let event = VulkanEvent::CreateObjectEvent(event_info);
                self.event_bus_ptr.lock().unwrap().publish(&event);
            }
            EntityEvent::EntityDeletionEvent(entity_deletion_info) => {
                self.entity_repository
                    .remove_entity(&entity_deletion_info.entity_id);
                let event = VulkanEvent::DeleteObjectEvent(entity_deletion_info.entity_id.clone());
                self.event_bus_ptr.lock().unwrap().publish(&event);
            }
        }
    }

    fn subscribe_to_entity_events(
        self_ptr: &Arc<Mutex<Self>>,
        event_bus_ptr: Arc<Mutex<EventBus>>,
    ) {
        let weak_self = Arc::downgrade(self_ptr);

        event_bus_ptr.lock().unwrap().subscribe({
            move |event: &EntityEvent| {
                if let Some(strong_self) = weak_self.upgrade() {
                    if let Ok(mut entity_component) = strong_self.lock() {
                        entity_component.handle_event(event);
                    }
                }
            }
        });
    }
}
