use glam::Vec3;

use crate::engine::{utils::structs::transform::Transform, vulkan::{structs::{vertex::Vertex, viewport::ViewportInfo}, vulkan_container::VulkanContainer}};

pub struct VulkanManager {
    vulkan_container: VulkanContainer,
}

impl VulkanManager {
    pub fn new(vulkan_container: VulkanContainer) -> Self {
        return VulkanManager {
            vulkan_container
        }
    }

    pub fn request_draw(&mut self, viewport_location: Vec3, viewport_rotation: Vec3) {
        self.vulkan_container.draw_frame(viewport_location, viewport_rotation);
    }

    pub fn create_vulkan_object(&mut self, object_id: usize, vertices: Vec<Vertex>, object_transform: Transform, texture_path: &str) {
        self.vulkan_container.create_vulkan_object(object_id, vertices, object_transform, texture_path);
    }

    pub fn delete_vulkan_object(&mut self, object_id: usize) {
        self.vulkan_container.delete_vulkan_object(object_id);
    }

    pub fn resize_viewport(&mut self, viewport_info: ViewportInfo) {
        self.vulkan_container.resize_viewport(&viewport_info);
    }
}