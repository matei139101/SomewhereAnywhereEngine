use std::sync::{Arc, Mutex};

use crate::engine::vulkan::vulkan_container::VulkanContainer;

pub trait Event {
    fn execute(&mut self, vulkan_container: Arc<Mutex<VulkanContainer>>);
    fn get_status(&self) -> &EventStatus;
}

#[derive(PartialEq)]
pub enum EventStatus {
    Pending,
    Processing,
    Done,
    Failed,
}