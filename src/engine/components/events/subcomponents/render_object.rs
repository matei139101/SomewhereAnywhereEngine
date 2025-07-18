use crate::engine::components::command_bus::command_bus::CommandType;
use crate::engine::components::events::subcomponents::event::{Event, EventStatus};
use crate::engine::utils::logger::{LogLevel, Logger};
use crate::engine::utils::structs::transform::Transform;
use crate::engine::vulkan::structs::vertex::Vertex;

pub struct RenderObject {
    status: EventStatus,
    object_id: usize,
    object: Vec<Vertex>,
    object_transform: Transform
}

impl RenderObject {
    pub fn new(object_id: usize, object: Vec<Vertex>, object_transform: Transform) -> Self {
        return RenderObject {
            status: EventStatus::Pending,
            object_id,
            object: object,
            object_transform,
        }
    }

    fn process(&mut self) -> CommandType {
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event starting process...");
        self.status = EventStatus::Processing;
        
        let command = CommandType::CreateVulkanObject(self.object_id, std::mem::take(&mut self.object), self.object_transform.clone());

        self.status = EventStatus::Done;
        Logger::log(LogLevel::Medium, "Event-RenderObject", "Event finished process.");
        return command;
    }
}

impl Event for RenderObject {
    fn execute(&mut self) -> Option<CommandType> {
        return Some(self.process());
    }

    fn get_status(&self) -> &EventStatus {
        return &self.status;
    }
}  