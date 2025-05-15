use winit::{event_loop::EventLoop};

use crate::engine::utils::logger::{Logger, LogLevel};

pub struct WindowWrapper {
    pub event_loop: EventLoop<()>,
}

impl WindowWrapper {
    pub fn new() -> Self {
        WindowWrapper {
            event_loop: WindowWrapper::make_event_loop(),
        }
    }

    fn make_event_loop() -> EventLoop<()> {
        Logger::log(LogLevel::High, "window_wrapper", "Creating eventloop...");

        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        Logger::log(LogLevel::High, "window_wrapper", "Eventloop created successfully.");
        return event_loop;
    }
}