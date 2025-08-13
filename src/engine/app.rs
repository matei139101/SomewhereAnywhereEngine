use glam::vec3;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};
use tokio::{
    runtime::Runtime,
    sync::{mpsc::UnboundedSender, oneshot},
};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::engine::{
    components::vulkan_component::vulkan_events::{CreateVulkanInstanceEvent, VulkanDrawEvent},
    vulkan::{structs::viewport::ViewportInfo, vulkan_container::VulkanContainer},
};

pub struct App {
    window: Option<Arc<Window>>,
    viewport_info: Option<ViewportInfo>,
    runtime: Runtime,
    async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
}

impl App {
    pub fn new(
        runtime: Runtime,
        async_sender: UnboundedSender<Box<dyn Any + Send + Sync>>,
    ) -> Self {
        App {
            window: Default::default(),
            viewport_info: Default::default(),
            runtime,
            async_sender,
        }
    }
}

impl ApplicationHandler for App {
    //[TO-DO]: This needs to be cleaned up and have dev stuff removed from it.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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

        let vulkan_container = Arc::new(Mutex::new(VulkanContainer::new(
            event_loop,
            self.window.as_ref().unwrap().clone(),
            self.viewport_info.as_ref().unwrap(),
        )));

        let message = CreateVulkanInstanceEvent {
            vulkan_container: vulkan_container.clone(),
        };

        let _ = self.async_sender.send(Box::new(message));

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
                let (confirmation_sender, confirmation_receiver) = oneshot::channel::<()>();
                let message = Box::new(VulkanDrawEvent {
                    viewport_location: vec3(0.0, 0.0, 0.0),
                    viewport_rotation: vec3(0.0, 0.0, 0.0),
                    confirmation_sender: Arc::new(Mutex::new(Some(confirmation_sender))),
                });

                let _ = self.async_sender.send(message);
                let _ = self.runtime.block_on(confirmation_receiver);

                self.window.as_ref().unwrap().request_redraw();
            }

            WindowEvent::Resized(_size) => {
                println!("Window Resized");
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.physical_key == PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ) {
                    event_loop.exit();
                }
            }
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
