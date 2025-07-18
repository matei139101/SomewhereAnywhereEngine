use crate::engine::{utils::structs::transform::Transform, vulkan::structs::vertex::Vertex};

#[derive(Debug)]
pub struct VulkanObject {
    vertex_buffer: vulkano::buffer::Subbuffer<[Vertex]>,
    object_transform: Transform,
}

impl VulkanObject {
    pub fn new(vertex_buffer: vulkano::buffer::Subbuffer<[Vertex]>, object_transform: Transform) -> Self {
        return VulkanObject { vertex_buffer, object_transform }
    }

    pub fn get_transform(&self) -> &Transform {
        return &self.object_transform;
    }

    pub fn get_buffer(&self) -> &vulkano::buffer::Subbuffer<[Vertex]> {
        return &self.vertex_buffer;
    }
}