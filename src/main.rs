use engine::utils::logger::{LogLevel, Logger};
use winit::event_loop::EventLoop;

mod engine;

fn main() {
    run();
}

fn run() {
    Logger::log(LogLevel::Low, "main", "Starting application...");

    let event_loop = make_event_loop();

    let mut app = engine::app::App{
        ..Default::default()
    };


    Logger::log(LogLevel::Low, "main", "Entering application...");
    let _ = event_loop.run_app(&mut app);
}

fn make_event_loop() -> EventLoop<()> {
    Logger::log(LogLevel::High, "window_wrapper", "Creating eventloop...");

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    Logger::log(LogLevel::High, "window_wrapper", "Eventloop created successfully.");
    return event_loop;
}