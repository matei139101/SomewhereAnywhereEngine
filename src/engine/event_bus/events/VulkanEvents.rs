use glam::Vec3;

use crate::engine::vulkan::structs::viewport::ViewportInfo;

pub struct VulkanDrawEvent {
    pub viewport_location: Vec3,
    pub viewport_rotation: Vec3,
}

pub struct VulkanViewportResizeEvent {
    pub viewport_information: ViewportInfo,
}
