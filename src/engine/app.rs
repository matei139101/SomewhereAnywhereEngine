use std::sync::{Arc, Mutex};

use glam::vec3;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, keyboard::KeyCode, window::{Window, WindowId}};
use crate::engine::{components::events::{delete_object::DeleteObject, manager::EventManager, render_object::RenderObject}, utils::logger::{LogLevel, Logger}, vulkan::vulkan_container::VulkanContainer};
use crate::engine::vulkan::structs::viewport::ViewportInfo;
use crate::engine::vulkan::structs::vertex::Vertex;

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
    pub vulkan_container: Option<Arc<Mutex<VulkanContainer>>>,
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

        let vulkan_container = VulkanContainer::new(event_loop, self.window.clone().unwrap(), self.viewport_info.as_ref().unwrap());
        self.vulkan_container = Some(Arc::new(Mutex::new(vulkan_container)));

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

        self.event_manager = Some(EventManager::new(self.vulkan_container.as_ref().unwrap().clone()));
        self.event_manager.as_mut().unwrap().add_event(RenderObject::new(vertices1));
        self.event_manager.as_mut().unwrap().add_event(RenderObject::new(vertices2));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.vulkan_container.as_ref().unwrap().lock().unwrap().draw_frame();
                self.event_manager.as_mut().unwrap().actualize();
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(size) => {
                Logger::log(LogLevel::Medium, "app", &format!("Window resized to: {}x{}", size.width, size.height));
                
                if let Some(viewport_info) = self.viewport_info.as_mut() {
                    viewport_info.set_extent([size.width as f32, size.height as f32]);

                    self.vulkan_container.as_ref().unwrap().lock().unwrap().resize_viewport(viewport_info);
                }
            },
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if event.physical_key == KeyCode::KeyD {
                    self.event_manager.as_mut().unwrap().add_event(DeleteObject::new(1));
                }
            }
            _ => (),
        }
    }
}