use glam::Mat4;

#[repr(C)]
#[derive(Default, Copy, Debug, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PushConstants {
    mvp: Mat4,
}

impl PushConstants {
    pub fn new(mvp: Mat4) -> Self {
        return PushConstants { mvp };
    }
}