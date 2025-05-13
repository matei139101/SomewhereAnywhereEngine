use std::sync::Arc;

pub struct VulkanWrapper {
    vk_instance: Arc<vulkano::instance::Instance>,
}

impl VulkanWrapper {
    pub fn new(glfw: &glfw::Glfw) -> Self {
        
        VulkanWrapper {
            vk_instance: VulkanWrapper::create_instance(glfw),
        }
    }

    fn create_instance(glfw: &glfw::Glfw) -> Arc<vulkano::instance::Instance> {
        println!("Creating Vulkan instance...");

        let vulkano_library = vulkano::library::VulkanLibrary::new().expect("Failed to create Vulkan library");
        let required_extensions: Vec<String> = glfw.get_required_instance_extensions().unwrap();

        let instance_info = vulkano::instance::InstanceCreateInfo {
            application_name: Some("SomewhereAnywhereEngine".to_string()),
            application_version: vulkano::Version {major: 0, minor: 1, patch: 0},
        
            enabled_extensions: required_extensions.iter().map(|s| s.as_str()).collect(),
            ..Default::default()
        };

        let instance = vulkano::instance::Instance::new(vulkano_library, instance_info).expect("Failed to create Vulkan instance");

        return instance
    }
}