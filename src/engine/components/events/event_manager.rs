use std::sync::{Arc, Mutex};

use crate::engine::components::events::subcomponents::event::{Event, EventStatus};
use crate::engine::utils::logger::{LogLevel, Logger};
use crate::engine::vulkan::vulkan_container::VulkanContainer;

pub struct EventManager {
    vulkan_container: Arc<Mutex<VulkanContainer>>,
    events: Vec<Box<dyn Event>>
}

impl EventManager {
    pub fn new(vulkan_container: Arc<Mutex<VulkanContainer>>) -> Self {
        return EventManager {
            vulkan_container,
            events: vec![],
        }
    }

    pub fn process_frame(&mut self) {
        for event in self.events.iter_mut() {
            event.execute(self.vulkan_container.clone());

            if event.get_status() == &EventStatus::Failed {
                Logger::log(LogLevel::High, "EventManager", "An event failed to process...");
            }
        }

        self.events.retain(|e| e.get_status() != &EventStatus::Done);
    }

    pub fn add_event<T: Event + 'static>(&mut self, event: T) {
        self.events.push(Box::new(event));
    }
}