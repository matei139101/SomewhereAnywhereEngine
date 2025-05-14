use std::sync::Arc;

use crate::engine::utils::logger::{Logger, LogLevel};

use vulkano::{self as vk, device::{physical, DeviceCreateInfo}};

pub struct VulkanWrapper {
    vk_instance: Arc<vk::instance::Instance>,
    device: Arc<vk::device::Device>,
    queue: Arc<vk::device::Queue>,
}

impl VulkanWrapper {
    pub fn new(glfw: &glfw::Glfw) -> Self {
        let vk_instance = VulkanWrapper::create_instance(glfw);
        let (device, queue) = VulkanWrapper::create_device(&vk_instance);
        //let vk_surface = VulkanWrapper::create_surface();

        VulkanWrapper {
            vk_instance,
            device,
            queue,
        }
    }

    fn create_instance(glfw: &glfw::Glfw) -> Arc<vk::instance::Instance> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating Vulkan instance...");
        
        let vk_library = vk::library::VulkanLibrary::new().unwrap();
        let required_extensions: Vec<String> = glfw.get_required_instance_extensions().unwrap();

        let instance_info = vk::instance::InstanceCreateInfo {
            application_name: Some("SomewhereAnywhereEngine".to_string()),
            application_version: vk::Version {major: 0, minor: 1, patch: 0},
        
            enabled_extensions: required_extensions.iter().map(|s| s.as_str()).collect(),
            ..Default::default()
        };

        let instance = vk::instance::Instance::new(vk_library, instance_info).unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Vulkan instance created successfully.");
        return instance
    }

    fn create_device(vk_instance: &Arc<vk::instance::Instance>) -> ( Arc<vk::device::Device>, Arc<vk::device::Queue> ) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating device...");

        let physical_device = vk_instance
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

        return (device, queues.next().unwrap());
    }

    fn create_surface() {
        
    }
}