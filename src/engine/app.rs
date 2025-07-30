use std::{sync::{Arc}};
use glam::{vec3};
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}};

use crate::engine::{components::{command_bus::command_bus::{CommandBus, CommandType}, entities::{entity::EntityType, entity_manager::EntityManager}, gamestage::gamestage::GameStage, input_manager::input_manager::InputManager, vulkan_manager::vulkan_manager::VulkanManager}, utils::{logger::{LogLevel, Logger}, structs::transform::Transform}, vulkan::{structs::{vertex::Vertex, viewport::ViewportInfo}, vulkan_container::VulkanContainer}};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
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


        let vulkan_manager = VulkanManager::new(vulkan_container);
        let entity_manager = EntityManager::new();
        
        //[TO-DO]: Should be made into an ini, yaml or json file for settings or something.
        let keys = vec![PhysicalKey::Code(KeyCode::KeyW), PhysicalKey::Code(KeyCode::KeyA), PhysicalKey::Code(KeyCode::KeyS), PhysicalKey::Code(KeyCode::KeyD), PhysicalKey::Code(KeyCode::ControlLeft), PhysicalKey::Code(KeyCode::Space), PhysicalKey::Code(KeyCode::KeyE), PhysicalKey::Code(KeyCode::KeyQ)];
        let input_manager = InputManager::new(keys, vec!["mouse".to_string()], 0);
        let gamestage = GameStage::new(0);

        let mut command_bus = CommandBus::new(vulkan_manager, entity_manager, input_manager, gamestage);

        let player_transform = Transform::new(
            vec3(0.0, 0.0, -5.0),
            vec3(0.0, 0.0, 0.0),
        );
        command_bus.send_command(CommandType::CreateEntity(EntityType::PlayerEntity(player_transform)));

        let cube_transform1 = Transform::new(
            vec3(-2.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        );
        command_bus.send_command(CommandType::CreateEntity(EntityType::CubeEntity(cube_transform1, "src/engine/vulkan/base_resources/default_texture.png".to_string())));

        let cube_transform2 = Transform::new(
            vec3(2.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
        );
        command_bus.send_command(CommandType::CreateEntity(EntityType::CubeEntity(cube_transform2, "src/engine/vulkan/base_resources/default_texture.png".to_string())));
        command_bus.update_managers();

        self.command_bus = Some(command_bus);

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
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(size) => {
                Logger::log(LogLevel::Medium, "app", &format!("Window resized to: {}x{}", size.width, size.height));
                
                if self.viewport_info.is_some() {
                    self.viewport_info.as_mut().unwrap().set_extent([size.width as f32, size.height as f32]);

                    self.command_bus.as_mut().unwrap().send_command(CommandType::VulkanViewportResize(self.viewport_info.as_mut().unwrap().clone()));
                }
            },
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
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
}