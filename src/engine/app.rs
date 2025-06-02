use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use super::{strucs::viewport::ViewportInfo, utils::logger::{LogLevel, Logger}, vulkan_wrapper::VulkanWrapper};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
    pub vulkan_wrapper: Option<VulkanWrapper>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        Logger::log(LogLevel::Medium, "app", "Resumed application...");
        
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap().into());
        self.window.as_ref().unwrap().request_redraw();

        self.viewport_info = Some(ViewportInfo::new(
            [0.0, 0.0],
            [self.window.as_ref().unwrap().inner_size().width as f32, self.window.as_ref().unwrap().inner_size().height as f32]
        ));

        self.vulkan_wrapper = Some(VulkanWrapper::new(event_loop, self.window.clone().unwrap(), self.viewport_info.as_ref().unwrap()));

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.vulkan_wrapper.as_mut().expect("No vulkan wrapper found").draw_frame();

                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(size) => {
                Logger::log(LogLevel::Medium, "app", &format!("Window resized to: {}x{}", size.width, size.height));
                
                if let Some(viewport_info) = self.viewport_info.as_mut() {
                    viewport_info.set_extent([size.width as f32, size.height as f32]);

                    self.vulkan_wrapper.as_mut().expect("No vulkan wrapper found").resize_viewport(viewport_info);
                }
            },
            _ => (),
        }
    }
}