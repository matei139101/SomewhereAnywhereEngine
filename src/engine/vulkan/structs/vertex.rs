use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, vulkano::pipeline::graphics::vertex_input::Vertex)]
pub struct Vertex {
    #[format(R32G32B32_SFLOAT)]
    position: Vec3,
    #[format(R32G32B32_SFLOAT)]
    color: Vec3,
    #[format(R32G32_SFLOAT)]
    tex_coord: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec3, tex_coord: Vec2) -> Self {
        Self { position, color , tex_coord}
    }
}