use std::sync::Arc;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use super::{utils::logger::{LogLevel, Logger}, vulkan_wrapper::VulkanWrapper};

#[derive(Default)]
pub struct App {
    pub vulkan_wrapper: Option<VulkanWrapper>,
    pub window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        Logger::log(LogLevel::Medium, "app", "Resumed application...");
        
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap().into());
        self.window.as_ref().unwrap().request_redraw();

        let vulkan_wrapper = match &mut self.vulkan_wrapper {
            Some(vulkan_wrapper) => vulkan_wrapper,
            None => panic!("Vulkan instance is none!"),
        };

        vulkan_wrapper.create_instance(event_loop);
        vulkan_wrapper.create_device();
        vulkan_wrapper.create_surface(self.window.as_ref().unwrap().clone());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}