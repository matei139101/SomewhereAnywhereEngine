use std::{ops::ControlFlow, sync::{Arc, Mutex}};
use glam::vec3;
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}};

use crate::engine::{components::{command_bus::command_bus::{CommandBus, CommandType}, entities::{entity::{Entity, EntityCreateInfo}, entity_manager::EntityManager, subcomponents::player_entity}, events::{event_manager::EventManager, subcomponents::render_object::RenderObject}, gamestage::gamestage::GameStage, input_manager::input_manager::InputManager}, utils::{logger::{LogLevel, Logger}, structs::transform::Transform}, vulkan::{structs::{vertex::Vertex, viewport::ViewportInfo}, vulkan_container::VulkanContainer}};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
    pub vulkan_container: Option<Arc<Mutex<VulkanContainer>>>,
    pub gamestage: Option<GameStage>,
    pub command_bus: Option<CommandBus>,
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

        let mut event_manager = EventManager::new(self.vulkan_container.as_mut().unwrap().clone());
        event_manager.add_event(Box::new(RenderObject::new(cube, self.vulkan_container.as_mut().unwrap().clone())));

        let mut entity_manager = EntityManager::new();
        let player_transform = Transform::new(
            vec3(-1.0, -1.0, -5.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        );
        
        entity_manager.create_entity(EntityCreateInfo::PlayerEntity(player_transform));
        
        let keys = vec![PhysicalKey::Code(KeyCode::KeyW), PhysicalKey::Code(KeyCode::KeyA), PhysicalKey::Code(KeyCode::KeyS), PhysicalKey::Code(KeyCode::KeyD), PhysicalKey::Code(KeyCode::ControlLeft), PhysicalKey::Code(KeyCode::Space)];
        let input_manager = InputManager::new(keys, vec!["mouse".to_string()], 0);
        self.gamestage = Some(GameStage::new(0, vec3(1.0, 1.0, -5.0)));
        self.command_bus = Some(CommandBus::new(event_manager, entity_manager, input_manager));

        //[TO:DO]: Locking the mouse for now. Needs to be thought over if it's meant to be here or elsewhere.
        self.window.as_mut().unwrap().set_cursor_grab(winit::window::CursorGrabMode::Locked).unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.command_bus.as_mut().unwrap().update_managers();

                //[TO-DO]: Clean this whole block up. Probably just not smart enough to realise why rust forces me to do this black magic just to read a value.
                let command_bus = self.command_bus.as_ref().unwrap();
                let entity_manager = command_bus.get_entity_manager();
                let player_entity = entity_manager.get_player_entity_ref(0);
                let player_entity = player_entity.lock().unwrap();
                let camera_transform = player_entity.get_transform();
                self.vulkan_container.as_ref().unwrap().lock().unwrap().draw_frame(camera_transform.get_position(), camera_transform.get_rotation());
                self.gamestage.as_mut().unwrap().update();
            },
            WindowEvent::Resized(size) => {
                Logger::log(LogLevel::Medium, "app", &format!("Window resized to: {}x{}", size.width, size.height));
                
                if let Some(viewport_info) = self.viewport_info.as_mut() {
                    viewport_info.set_extent([size.width as f32, size.height as f32]);

                    self.vulkan_container.as_ref().unwrap().lock().unwrap().resize_viewport(viewport_info);
                }
            },
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::KeyQ) {
                    event_loop.exit();
                }
                let key_state = match event.state {
                    winit::event::ElementState::Pressed => true,
                    winit::event::ElementState::Released => false,
                };

                self.command_bus.as_mut().unwrap().send_command(CommandType::KeyStateChange(event.physical_key, key_state));
            },
            _ => (),
        }
    }

    //[TO-DO]: For camera turning, will need to be cleaned up later.
    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.command_bus.as_mut().unwrap().send_command(CommandType::AxisStateChange("mouse".to_string(), delta));
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}