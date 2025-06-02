#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable, vulkano::pipeline::graphics::vertex_input::Vertex)]
pub struct Vertex {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    color: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}