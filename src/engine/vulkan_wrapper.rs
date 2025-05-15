use std::sync::Arc;

use crate::engine::utils::logger::{Logger, LogLevel};

use vulkano::{self as vk, device::{DeviceCreateInfo}};
use winit::{event_loop::{ActiveEventLoop}, window::{Window}};

pub struct VulkanWrapper {
    instance: Option<Arc<vk::instance::Instance>>,
    device: Option<Arc<vk::device::Device>>,
    queue: Option<Arc<vk::device::Queue>>,
    surface: Option<Arc<vk::swapchain::Surface>>,
}

impl VulkanWrapper {
    pub fn new() -> Self {
        VulkanWrapper {
            instance: None,
            device: None,
            queue: None,
            surface: None,
        }
    }

    pub fn create_instance(&mut self, event_loop: &ActiveEventLoop) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating Vulkan instance...");
        
        let library = vk::library::VulkanLibrary::new().unwrap();
        let required_extensions = vk::swapchain::Surface::required_extensions(&event_loop).unwrap();

        let instance = vk::instance::Instance::new(
            library,
            vk::instance::InstanceCreateInfo {
                flags: vk::instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            }
        ).expect("Could not create Vulkan instance");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Vulkan instance created successfully.");
        self.instance = Some(instance.clone());
    }

    pub fn create_device(&mut self) {
        // Check if the instance is created and crash if not
        let instance = match &self.instance {
            Some(instance) => instance,
            None => panic!("Vulkan instance is none!"),
        };

        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating device...");

        let physical_device = instance
            .enumerate_physical_devices()
            .expect("Could not enumerate physical devices")
            .next()
            .expect("No physical devices available");

        for family in physical_device.queue_family_properties() {
            Logger::log(LogLevel::Dev, "vulkan_wrapper", &format!("Found a queue family with {:?} queue(s) with flags: {:?}", family.queue_count, family.queue_flags));
        }

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties.queue_flags.contains(vk::device::QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family") as u32;
        
        Logger::log(LogLevel::Dev, "vulkan_wrapper", &format!("Found a graphical queue family at index {}", queue_family_index));

        let (device, mut queues) = vk::device::Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![vk::device::QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            }
        ).expect("Could not create device");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Device created successfully.");

        self.device = Some(device);
        self.queue = Some(queues.next().unwrap());
    }

    pub fn create_surface(&mut self, window: Arc<Window>) {
        // Check if the instance is created and crash if not
        let instance = match &self.instance {
            Some(instance) => instance,
            None => panic!("Vulkan instance is none!"),
        };

        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating surface...");

        let surface = vk::swapchain::Surface::from_window(instance.clone(), window)
            .expect("Could not create surface");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Surface created successfully.");

        self.surface = Some(surface);
    }
}