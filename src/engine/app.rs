use std::{default, sync::{Arc, Mutex}};

use glam::{vec3, Vec3};
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, MouseScrollDelta, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}};
use crate::engine::{components::events::{delete_object::DeleteObject, event_manager::EventManager, render_object::RenderObject}, utils::logger::{LogLevel, Logger}, vulkan::vulkan_container::VulkanContainer};
use crate::engine::vulkan::structs::viewport::ViewportInfo;
use crate::engine::vulkan::structs::vertex::Vertex;

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
    pub vulkan_container: Option<Arc<Mutex<VulkanContainer>>>,
    pub event_manager: Option<EventManager>,
    pub camera_location: Option<Vec3>,
    pub camera_rotation: Option<Vec3>,
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

        let cube = vec![
            // Front face (+Z)
            Vertex::new(vec3(-0.5, -0.5,  0.5), [1.0, 0.0, 0.0]), // bottom-left
            Vertex::new(vec3( 0.5,  0.5,  0.5), [0.0, 0.0, 1.0]), // top-right
            Vertex::new(vec3( 0.5, -0.5,  0.5), [0.0, 1.0, 0.0]), // bottom-right

            Vertex::new(vec3(-0.5, -0.5,  0.5), [1.0, 0.0, 0.0]), // bottom-left
            Vertex::new(vec3(-0.5,  0.5,  0.5), [1.0, 1.0, 0.0]), // top-left
            Vertex::new(vec3( 0.5,  0.5,  0.5), [0.0, 0.0, 1.0]), // top-right

            // Back face (-Z)
            Vertex::new(vec3( 0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3( 0.5,  0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(-0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),
            Vertex::new(vec3( 0.5,  0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5, -0.5), [0.0, 0.0, 1.0]),

            // Left face (-X)
            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5,  0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3(-0.5, -0.5,  0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(-0.5,  0.5,  0.5), [0.0, 0.0, 1.0]),

            // Right face (+X)
            Vertex::new(vec3(0.5, -0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(0.5,  0.5, -0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3(0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(0.5, -0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(0.5,  0.5,  0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3(0.5,  0.5, -0.5), [0.0, 0.0, 1.0]),

            // Top face (+Y)
            Vertex::new(vec3(-0.5, 0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3( 0.5, 0.5, -0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3( 0.5, 0.5,  0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, 0.5,  0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5, 0.5, -0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3( 0.5, 0.5, -0.5), [0.0, 0.0, 1.0]),

            // Bottom face (-Y)
            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3( 0.5, -0.5,  0.5), [0.0, 0.0, 1.0]),
            Vertex::new(vec3( 0.5, -0.5, -0.5), [0.0, 1.0, 0.0]),

            Vertex::new(vec3(-0.5, -0.5, -0.5), [1.0, 0.0, 0.0]),
            Vertex::new(vec3(-0.5, -0.5,  0.5), [1.0, 1.0, 0.0]),
            Vertex::new(vec3( 0.5, -0.5,  0.5), [0.0, 0.0, 1.0]),
        ];

        self.event_manager = Some(EventManager::new(self.vulkan_container.as_ref().unwrap().clone()));
        self.event_manager.as_mut().unwrap().add_event(RenderObject::new(cube));
        self.camera_location = Some(Vec3 { x: 1.0, y: 1.0, z: -2.5 });
        self.camera_rotation = Some(Vec3 { x: 0.0, y: 0.0, z: 0.0 });

        //Locking the mouse. Temp
        self.window.as_mut().unwrap().set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.vulkan_container.as_ref().unwrap().lock().unwrap().draw_frame(self.camera_location.unwrap(), self.camera_rotation.unwrap());
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
                //Just for testing as of now
                let mut new_camera_location: Vec3 = self.camera_location.unwrap();
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyD) => { new_camera_location.x = new_camera_location.x + -0.03; },
                    PhysicalKey::Code(KeyCode::KeyA) => { new_camera_location.x = new_camera_location.x + 0.03; },
                    PhysicalKey::Code(KeyCode::KeyW) => { new_camera_location.z = new_camera_location.z + 0.03; },
                    PhysicalKey::Code(KeyCode::KeyS) => { new_camera_location.z = new_camera_location.z + -0.03; },
                    PhysicalKey::Code(KeyCode::Space) => { new_camera_location.y = new_camera_location.y + 0.03; },
                    PhysicalKey::Code(KeyCode::ShiftLeft) => { new_camera_location.y = new_camera_location.y + -0.03; },
                    _ => {}
                }
                self.camera_location = Some(new_camera_location);
            },
            WindowEvent::CursorMoved { position, .. } => {
                println!("Mouse moved to: {:?}", position);
            },
            _ => (),
        }
    }

    //For camera turning, Temp
    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if let Some(mut camera_rotation) = self.camera_rotation {
                    let sensitivity = 0.001;
                    camera_rotation.y += delta.0 as f32 * sensitivity;
                    camera_rotation.x += delta.1 as f32 * -sensitivity;
                    
                    camera_rotation.x = camera_rotation.x.clamp(-1.5, 1.5);
                    
                    self.camera_rotation = Some(camera_rotation);
                }
            },
            _ => {}
        }
    }
}