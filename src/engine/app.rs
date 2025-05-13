extern crate glfw;

use glfw::{Action, Context, GlfwReceiver, Key, PWindow, WindowEvent};

pub struct App {
    windowWrapper: WindowWrapper
}

impl App {
    pub fn new() -> Self {
        App {
            windowWrapper: WindowWrapper::new()
        }
    }

    pub fn run(&mut self) {
        println!("Running the app...");

        App::init_window();
        self.main_loop();
        self.clean_up();
    }

    fn init_window() -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        println!("Initializing window...");

        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw.create_window(500, 500, "Some Title", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");
        
        window.make_current();
        window.set_key_polling(true);

        return (window, events);
    }

    fn init_vulkan() {
        println!("Initializing Vulkan...");
    }

    fn main_loop(&mut self) {
        println!("Entering main loop...");

        while !self.windowWrapper.window.should_close() {
            // Swap front and back buffers
            self.windowWrapper.window.swap_buffers();

            // Poll for and process events
            self.windowWrapper.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.windowWrapper.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.windowWrapper.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }
    
    fn clean_up(&self) {
        println!("Cleaning up resources...");
    }
}

struct WindowWrapper {
    glfw: glfw::Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl WindowWrapper {
    fn new() -> Self {
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
        let (mut window, events) = glfw.create_window(500, 500, "Some Title", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");
        
        window.make_current();
        window.set_key_polling(true);

        return (window, events);
    }
}