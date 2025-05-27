use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use super::{utils::logger::{LogLevel, Logger}, vulkan_wrapper::{VulkanWrapper}};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub vulkan_wrapper: Option<VulkanWrapper>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        Logger::log(LogLevel::Medium, "app", "Resumed application...");
        
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap().into());
        self.window.as_ref().unwrap().request_redraw();

        self.vulkan_wrapper = Some(VulkanWrapper::new(event_loop, self.window.clone().unwrap()));

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
            }
            _ => (),
        }
    }
}