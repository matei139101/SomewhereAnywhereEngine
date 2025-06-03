use std::sync::Arc;

use glam::vec3;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};
use crate::engine::{components::events::{EventManager, RenderObject}, vulkan::{structs::vertex::Vertex, vulkan_container::VulkanContainer}};
use crate::engine::utils::logger::{Logger, LogLevel};
use crate::engine::vulkan::structs::viewport::ViewportInfo;

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
    pub vulkan_container: Option<VulkanContainer>,
    pub event_manager: Option<EventManager>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        Logger::log(LogLevel::Medium, "app", "Resumed application...");

        let window_attributes = Window::default_attributes();

        self.window = Some(event_loop.create_window(window_attributes).unwrap().into());
        self.window.as_ref().unwrap().request_redraw();

        self.viewport_info = Some(ViewportInfo::new(
            [0.0, 0.0],
            [self.window.as_ref().unwrap().inner_size().width as f32, self.window.as_ref().unwrap().inner_size().height as f32]
        ));

        self.vulkan_container = Some(VulkanContainer::new(event_loop, self.window.clone().unwrap(), self.viewport_info.as_ref().unwrap()));

        let vertices1 = vec![
            Vertex::new(vec3(-0.5, -0.5, 0.0), [1.0, 0.0, 0.0] ),
            Vertex::new(vec3(-0.5, 0.5, 0.0), [0.0, 1.0, 0.0] ),
            Vertex::new(vec3(0.5, -0.5, 0.0), [0.0, 0.0, 1.0] ),
        ];

        let vertices2 = vec![
            Vertex::new(vec3(0.5, 0.5, 0.0), [1.0, 0.0, 0.0] ),
            Vertex::new(vec3(0.5, -0.5, 0.0), [0.0, 0.0, 1.0] ),
            Vertex::new(vec3(-0.5, 0.5, 0.0), [0.0, 1.0, 0.0] ),
        ];

        self.vulkan_container.as_mut().unwrap().create_vertex_buffer(vertices1.clone());
        self.vulkan_container.as_mut().unwrap().create_vertex_buffer(vertices2);
        self.vulkan_container.as_mut().unwrap().delete_vertex_buffer(5);

        self.event_manager = Some(EventManager::new());
        self.event_manager.as_mut().unwrap().add_event(RenderObject::new(vertices1));

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.vulkan_container.as_mut().expect("No vulkan wrapper found").draw_frame();
                self.event_manager.as_mut().unwrap().actualize();
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(size) => {
                Logger::log(LogLevel::Medium, "app", &format!("Window resized to: {}x{}", size.width, size.height));
                
                if let Some(viewport_info) = self.viewport_info.as_mut() {
                    viewport_info.set_extent([size.width as f32, size.height as f32]);

                    self.vulkan_container.as_mut().expect("No vulkan wrapper found").resize_viewport(viewport_info);
                }
            },
            _ => (),
        }
    }
}