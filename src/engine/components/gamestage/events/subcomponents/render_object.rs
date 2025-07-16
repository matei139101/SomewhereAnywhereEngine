use std::sync::{Arc, Mutex};

use crate::engine::components::gamestage::events::subcomponents::event::{Event, EventStatus};
use crate::engine::utils::logger::{LogLevel, Logger};
use crate::engine::vulkan::structs::vertex::Vertex;
use crate::engine::vulkan::vulkan_container::{VulkanContainer};

pub struct RenderObject {
    status: EventStatus,
    object: Vec<Vertex>,
    vulkan_container: Arc<Mutex<VulkanContainer>>
}

impl RenderObject {
    pub fn new(object: Vec<Vertex>, vulkan_container: Arc<Mutex<VulkanContainer>>) -> Self {
        return RenderObject {
            status: EventStatus::Pending,
            object: object,
            vulkan_container,
        }
    }

    fn process(&mut self) {
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event starting process...");
        self.status = EventStatus::Processing;
        
        self.vulkan_container.lock().unwrap().create_vertex_buffer(std::mem::take(&mut self.object));

        self.status = EventStatus::Done;
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event finished process.");
    }
}

impl Event for RenderObject {
    fn execute(&mut self) {
        self.process();
    }

    fn get_status(&self) -> &EventStatus {
        return &self.status;
    }
}