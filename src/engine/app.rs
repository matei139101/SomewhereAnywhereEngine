use glfw::{Action, Context, Key};
use crate::engine::{vulkan_wrapper::VulkanWrapper, window_wrapper::WindowWrapper};
use crate::engine::utils::logger::{Logger, LogLevel};

pub struct App {
    window_wrapper: WindowWrapper,
    vulkan_wrapper: VulkanWrapper,
}

impl App {
    pub fn new() -> Self {
        let window_wrapper = WindowWrapper::new();
        let vulkan_wrapper = VulkanWrapper::new(&window_wrapper.glfw);

        App {
            window_wrapper,
            vulkan_wrapper,
        }
    }

    pub fn run(&mut self) {
        self.main_loop();
        self.clean_up();
    }

    fn main_loop(&mut self) {
        Logger::log(LogLevel::High, "app","Starting main loop...");

        while !self.window_wrapper.window.should_close() {
            self.window_wrapper.window.swap_buffers();

            self.window_wrapper.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.window_wrapper.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window_wrapper.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }
    
    fn clean_up(&self) {
        // println!("Cleaning up resources...");
    }
}