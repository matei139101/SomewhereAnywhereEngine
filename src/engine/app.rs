use glam::vec3;
use std::sync::{Arc, Mutex};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::engine::{
    components::vulkan_component::{self, VulkanComponent},
    event_bus::{event_bus::EventBus, events::VulkanEvents::VulkanDrawEvent},
    utils::logger::{LogLevel, Logger},
    vulkan::{
        structs::viewport::ViewportInfo,
        vulkan_container::{self, VulkanContainer},
    },
};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub viewport_info: Option<ViewportInfo>,
    pub event_bus: Option<Arc<Mutex<EventBus>>>,
    pub vulkan_component: Option<Arc<Mutex<VulkanComponent>>>,
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
            [
                self.window.as_ref().unwrap().inner_size().width as f32,
                self.window.as_ref().unwrap().inner_size().height as f32,
            ],
        ));

        let vulkan_container = VulkanContainer::new(
            event_loop,
            self.window.as_ref().unwrap().clone(),
            self.viewport_info.as_ref().unwrap(),
        );
        let event_bus = Arc::new(Mutex::new(EventBus::new()));
        self.event_bus = Some(event_bus);
        let vulkan_component =
            VulkanComponent::new(vulkan_container, self.event_bus.as_ref().unwrap().clone());
        self.vulkan_component = Some(vulkan_component);

        //[TO:DO]: Locking the mouse for now. Needs to be thought over if it's meant to be here or elsewhere.
        self.window
            .as_mut()
            .unwrap()
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.event_bus
                    .as_mut()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .publish(&VulkanDrawEvent {
                        viewport_location: vec3(0.0, 0.0, 0.0),
                        viewport_rotation: vec3(0.0, 0.0, 0.0),
                    });
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                Logger::log(
                    LogLevel::Medium,
                    "app",
                    &format!("Window resized to: {}x{}", size.width, size.height),
                );
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {}
            _ => (),
        }
    }

    //[TO-DO]: For camera turning, will need to be cleaned up later.
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta: _ } => {}
            DeviceEvent::MouseWheel { delta: _ } => {}
            _ => {}
        }
    }
}
