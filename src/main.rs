use engine::{utils::logger::{LogLevel, Logger}, vulkan_wrapper::VulkanWrapper, window_wrapper::WindowWrapper};

mod engine;

fn main() {
    run();
}

fn run() {
    Logger::log(LogLevel::Low, "main", "Starting application...");

    let window_wrapper = WindowWrapper::new();
    let vulkan_wrapper = VulkanWrapper::new();

    let mut app = engine::app::App{
        vulkan_wrapper: Some(vulkan_wrapper),
        ..Default::default()
    };

    Logger::log(LogLevel::Low, "main", "Entering application...");
    let _ = window_wrapper.event_loop.run_app(&mut app);
}