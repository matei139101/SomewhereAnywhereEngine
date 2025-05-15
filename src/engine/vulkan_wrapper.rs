use std::sync::Arc;

use crate::engine::utils::logger::{Logger, LogLevel};

use vulkano::{self as vk, device::{DeviceCreateInfo}, swapchain::SwapchainCreateInfo};
use winit::{event_loop::{ActiveEventLoop}, window::{Window}};

pub struct VulkanWrapper {
    instance: Arc<vk::instance::Instance>,
    surface: Arc<vk::swapchain::Surface>,
    physical_device: Arc<vk::device::physical::PhysicalDevice>,
    logical_device: Arc<vk::device::Device>,
    queue: Arc<vk::device::Queue>,
    swapchain: Arc<vk::swapchain::Swapchain>,
    images: Vec<Arc<vk::image::Image>>,
}

impl VulkanWrapper {
    pub fn new(event_loop: &ActiveEventLoop, window: Arc<Window>) -> Self {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating Vulkan wrapper...");

        let device_extensions = vk::device::DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };
        
        let instance = VulkanWrapper::create_instance(event_loop);
        let surface = VulkanWrapper::create_surface(&instance, window.clone());
        let (physical_device, queue_family_index) = VulkanWrapper::create_physical_device(&instance, &surface, &device_extensions);
        let (logical_device, queue) = VulkanWrapper::create_logical_device(physical_device.clone(), queue_family_index, &device_extensions);
        let (swapchain, images) = VulkanWrapper::create_swapchain(physical_device.clone(), logical_device.clone(), window.clone(), surface.clone());

        let vulkan_wrapper = VulkanWrapper {
            instance: instance,
            surface: surface,
            physical_device,
            logical_device,
            queue,
            swapchain: swapchain,
            images: images,
        };

        Logger::log(LogLevel::High, "vulkan_wrapper", "Vulkan wrapper created successfully.");
        return vulkan_wrapper;
    }

    fn create_instance(event_loop: &ActiveEventLoop) -> Arc<vk::instance::Instance> {
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
        return instance
    }

    pub fn create_surface(instance: &Arc<vk::instance::Instance>, window: Arc<Window>) -> Arc<vk::swapchain::Surface> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating surface...");

        let surface = vk::swapchain::Surface::from_window(instance.clone(), window).expect("Could not create surface");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Surface created successfully.");
        return surface;
    }

    fn create_physical_device(instance: &Arc<vk::instance::Instance>, surface: &Arc<vk::swapchain::Surface>, device_extensions: &vk::device::DeviceExtensions) -> (Arc<vk::device::physical::PhysicalDevice>, u32) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating physical device...");

        let physical_devices = instance
            .enumerate_physical_devices()
            .expect("Could not enumerate physical devices");

        Logger::log(LogLevel::Dev, "vulkan_wrapper", &format!("Found {} physical devices.", physical_devices.len()));

        //This is a lot of black magic to filter the physical devices to find which support the needed extensions and queue families. And then also score them by type.
        let physical_device = physical_devices
            .filter(|p| p.supported_extensions().contains(device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.contains(vk::device::QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                vk::device::physical::PhysicalDeviceType::DiscreteGpu => 0,
                vk::device::physical::PhysicalDeviceType::IntegratedGpu => 1,
                vk::device::physical::PhysicalDeviceType::VirtualGpu => 2,
                vk::device::physical::PhysicalDeviceType::Cpu => 3,
                _ => 4,
            })
            .expect("no device available");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Physical device created successfully.");
        return physical_device;
    }

    fn create_logical_device(physical_device: Arc<vk::device::physical::PhysicalDevice>, queue_family_index: u32, device_extensions: &vk::device::DeviceExtensions) -> (Arc<vk::device::Device>, Arc<vk::device::Queue>) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating logical device...");

        let (device, mut queues) = vk::device::Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![vk::device::QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: *device_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create device");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Logical device created successfully.");
        return (device, queues.next().unwrap());
    }

    pub fn create_swapchain(physical_device: Arc<vk::device::physical::PhysicalDevice>, device: Arc<vk::device::Device>, window: Arc<Window>, surface: Arc<vk::swapchain::Surface>) -> (Arc<vk::swapchain::Swapchain>, Vec<Arc<vk::image::Image>>) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating swapchain...");

        let caps = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("Could not get surface capabilities");

        let dimensions = window.inner_size();
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        let (swapchain, images) = vk::swapchain::Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count + 1,
                image_format,
                image_extent: dimensions.into(),
                image_usage: vk::image::ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            }
        ).unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Swapchain created successfully.");
        return (swapchain, images);
    }
}