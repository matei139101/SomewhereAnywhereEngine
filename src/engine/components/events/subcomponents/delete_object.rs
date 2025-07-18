use std::sync::{Arc, Mutex};

use crate::engine::components::events::subcomponents::event::{Event, EventStatus};
use crate::engine::utils::logger::{LogLevel, Logger};
use crate::engine::vulkan::vulkan_container::VulkanContainer;

pub struct DeleteObject {
    status: EventStatus,
    object: usize,
    vulkan_container: Arc<Mutex<VulkanContainer>>,
}

impl DeleteObject {
    /*
    pub fn new(object: usize, vulkan_container: Arc<Mutex<VulkanContainer>>) -> Self {
        return DeleteObject {
            status: EventStatus::Pending,
            object: object,
            vulkan_container,
        }
    }
    */
    
    fn process(&mut self) {
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event starting process...");
        self.status = EventStatus::Processing;
        
        self.vulkan_container.lock().unwrap().delete_vertex_buffer(std::mem::take(&mut self.object));

        self.status = EventStatus::Done;
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event finished process.");
    }
}

impl Event for DeleteObject {
    fn execute(&mut self) {
        self.process();
    }

    fn get_status(&self) -> &EventStatus {
        return &self.status;
    }
}