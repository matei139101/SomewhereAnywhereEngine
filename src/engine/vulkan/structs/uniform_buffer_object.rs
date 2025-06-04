#[repr(C)]
#[derive(Default, Copy, Debug, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformBufferObject {
    pub mvp: [[f32; 4]; 4],
}