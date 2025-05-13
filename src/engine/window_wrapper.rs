use glfw::{Context, GlfwReceiver, PWindow, WindowEvent};

use crate::engine::utils::logger::{Logger, LogLevel};

pub struct WindowWrapper {
    pub glfw: glfw::Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl WindowWrapper {
    pub fn new() -> Self {
        let mut glfw = Self::make_glfw();
        let (window, events) = Self::make_window_and_events(&mut glfw);

        WindowWrapper {
            glfw,
            window,
            events,
        }
    }

    fn make_glfw() -> glfw::Glfw {
        Logger::log(LogLevel::High, "window_wrapper", "Initializing GLFW...");

        use glfw::fail_on_errors;
        let glfw = glfw::init(fail_on_errors!()).unwrap();

        Logger::log(LogLevel::High, "window_wrapper", "GLFW initialized successfully.");
        return glfw;
    }

    fn make_window_and_events(glfw: &mut glfw::Glfw) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        Logger::log(LogLevel::High, "window_wrapper", "Creating GLFW window...");

        let (mut window, events) = glfw.create_window(1920, 1080, "Some Title", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        window.make_current();
        window.set_key_polling(true);

        Logger::log(LogLevel::High, "window_wrapper", "GLFW window created successfully.");

        return (window, events);
    }
}