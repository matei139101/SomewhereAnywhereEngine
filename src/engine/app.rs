use std::{default, sync::{Arc, Mutex}};
use crate::engine::components::entities::{entity::Entity, subcomponents::player_entity::PlayerEntity};
use glam::{vec3, Vec3};
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, MouseScrollDelta, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}};
use crate::engine::{components::{entities::{entity_manager::{self, EntityManager}, subcomponents::player_entity::PlayerEntityCreateInfo}, events::{event_manager::EventManager, subcomponents::render_object::RenderObject}, gamestage::gamestage::GameStage}, utils::{logger::{LogLevel, Logger}, structs::transform::Transform}, vulkan::{structs::viewport::ViewportInfo, vulkan_container::VulkanContainer}};
use crate::engine::vulkan::structs::vertex::Vertex;

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
    pub vulkan_container: Option<Arc<Mutex<VulkanContainer>>>,
    pub gamestage: Option<GameStage>,
}

impl ApplicationHandler for App {
    //[TO-DO]: This needs to be cleaned up and have dev stuff removed from it.
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

        let mut event_manager = EventManager::new(self.vulkan_container.as_ref().unwrap().clone());
        event_manager.add_event(RenderObject::new(cube));

        let mut entity_manager = EntityManager::new();
        let player_transform = Transform::new(
            vec3(-1.0, -1.0, -5.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        );
        entity_manager.create_entity(Box::new(PlayerEntityCreateInfo::new(player_transform)));
        self.gamestage = Some(GameStage::new(entity_manager, event_manager));

        //[TO:DO]: Locking the mouse for now. Needs to be thought over if it's meant to be here or elsewhere.
        self.window.as_mut().unwrap().set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        //[TO-DO]: temp for dev testing.
        let gamestage = self.gamestage.as_mut().unwrap();
        let player_entity = &mut gamestage.entity_manager.get_player_entities()[0];
        
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let camera_transform: &Transform = player_entity.get_transform();
                self.vulkan_container.as_ref().unwrap().lock().unwrap().draw_frame(camera_transform.get_position(), camera_transform.get_rotation());
                self.gamestage.as_mut().unwrap().update();
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
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyD) => { player_entity.move_right(0.03); },
                    PhysicalKey::Code(KeyCode::KeyA) => { player_entity.move_right(-0.03); },
                    PhysicalKey::Code(KeyCode::KeyW) => { player_entity.move_forward(0.03); },
                    PhysicalKey::Code(KeyCode::KeyS) => { player_entity.move_forward(-0.03); },
                    PhysicalKey::Code(KeyCode::Space) => { player_entity.move_up(0.03); },
                    PhysicalKey::Code(KeyCode::ShiftLeft) => { player_entity.move_up(-0.03); },
                    _ => {}
                }
            },
            _ => (),
        }
    }

    //[TO-DO]: For camera turning, will need to be cleaned up later.
    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let sensitivity = 0.001;
                let mut new_camera_transform: Transform = self.gamestage.as_mut().unwrap().entity_manager.get_player_entities()[0].get_transform().clone();
                new_camera_transform.rotation.y += delta.0 as f32 * sensitivity;
                new_camera_transform.rotation.x += delta.1 as f32 * -sensitivity;
                new_camera_transform.rotation.x = new_camera_transform.get_rotation().x.clamp(-1.5, 1.5);
                    
                self.gamestage.as_mut().unwrap().entity_manager.modify_entity_transform(0, new_camera_transform);
            },
            _ => {}
        }
    }
}