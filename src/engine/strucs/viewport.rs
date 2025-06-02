pub struct ViewportInfo {
    pub offset: [f32; 2],
    pub extent: [f32; 2]
}

impl ViewportInfo {
    pub fn new(offset: [f32; 2], extent: [f32; 2]) -> Self {
        Self { offset, extent }
    }

    pub fn set_offset(&mut self, offset: [f32; 2]) {
        self.offset = offset;
    }

    pub fn set_extent(&mut self, extent: [f32; 2]) {
        self.extent = extent;
    }
}