use glfw::{Context, GlfwReceiver, PWindow, WindowEvent};

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
        use glfw::fail_on_errors;
        let glfw = glfw::init(fail_on_errors!()).unwrap();

        return glfw;
    }

    fn make_window_and_events(glfw: &mut glfw::Glfw) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        let (mut window, events) = glfw.create_window(1920, 1080, "Some Title", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        window.make_current();
        window.set_key_polling(true);

        return (window, events);
    }
}