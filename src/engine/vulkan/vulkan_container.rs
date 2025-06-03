use std::{collections::HashSet, fs::File, io::Read, ops::Range, sync::Arc, vec};
use glam::vec3;
use vulkano::{self, shader};
use winit::{event_loop::{ActiveEventLoop}, window::{Window}};
use smallvec::{smallvec, SmallVec};
use std::path::Path;

use crate::engine::utils::logger::{Logger, LogLevel};
use crate::engine::structs::viewport::ViewportInfo;
use crate::engine::structs::vertex;

pub struct VulkanContainer {
    logical_device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>,
    swapchain: Arc<vulkano::swapchain::Swapchain>,
    graphics_pipeline: Arc<vulkano::pipeline::GraphicsPipeline>,
    framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>>,
    viewports: SmallVec<[vulkano::pipeline::graphics::viewport::Viewport; 2]>,
    scissors: SmallVec<[vulkano::pipeline::graphics::viewport::Scissor; 2]>,
    vertexbuffers: Vec<vulkano::buffer::Subbuffer<[vertex::Vertex]>>,
}

impl VulkanContainer {
    pub fn new(event_loop: &ActiveEventLoop, window: Arc<Window>, viewport_info: &ViewportInfo) -> Self {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating Vulkan wrapper...");

        let device_extensions = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };
        
        let instance = VulkanContainer::create_instance(event_loop);
        let surface = VulkanContainer::create_surface(&instance, window.clone());
        let (physical_device, queue_family_index) = VulkanContainer::create_physical_device(&instance, &surface, &device_extensions);
        let (logical_device, queue) = VulkanContainer::create_logical_device(physical_device.clone(), queue_family_index, &device_extensions);
        let (swapchain, images) = VulkanContainer::create_swapchain(physical_device.clone(), logical_device.clone(), window.clone(), surface.clone());
        let image_views = VulkanContainer::create_image_views(&images);
        let render_pass = VulkanContainer::create_render_pass(logical_device.clone(), images[0].format());
        let graphics_pipeline = VulkanContainer::create_graphics_pipeline(logical_device.clone(), render_pass.clone());
        let framebuffers = VulkanContainer::create_frame_buffers(render_pass.clone(), image_views.clone());

        let viewports = smallvec![vulkano::pipeline::graphics::viewport::Viewport {
            offset: [viewport_info.offset[0], viewport_info.offset[1]],
            extent: [viewport_info.extent[0], viewport_info.extent[1]],
            depth_range: 0.0..=1.0,
        }];

        let scissors = smallvec![vulkano::pipeline::graphics::viewport::Scissor {
            offset: [viewport_info.offset[0] as u32, viewport_info.offset[1] as u32],
            extent: [viewport_info.extent[0] as u32, viewport_info.extent[1] as u32],
        }];

        let vulkan_wrapper = VulkanContainer {
            logical_device,
            queue,
            swapchain,
            graphics_pipeline,
            framebuffers,
            viewports,
            scissors,
            vertexbuffers: Vec::new(),
        };

        Logger::log(LogLevel::High, "vulkan_wrapper", "Vulkan wrapper created successfully.");
        return vulkan_wrapper;
    }

    fn create_instance(event_loop: &ActiveEventLoop) -> Arc<vulkano::instance::Instance> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating Vulkan instance...");
        
        let library = vulkano::library::VulkanLibrary::new().unwrap();
        let required_extensions = vulkano::swapchain::Surface::required_extensions(&event_loop).unwrap();

        let instance = vulkano::instance::Instance::new(
            library,
            vulkano::instance::InstanceCreateInfo {
                flags: vulkano::instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            }
        ).expect("Could not create Vulkan instance");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Vulkan instance created successfully.");
        return instance
    }

    fn create_surface(instance: &Arc<vulkano::instance::Instance>, window: Arc<Window>) -> Arc<vulkano::swapchain::Surface> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating surface...");

        let surface = vulkano::swapchain::Surface::from_window(instance.clone(), window).expect("Could not create surface");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Surface created successfully.");
        return surface;
    }

    fn create_physical_device(instance: &Arc<vulkano::instance::Instance>, surface: &Arc<vulkano::swapchain::Surface>, device_extensions: &vulkano::device::DeviceExtensions) -> (Arc<vulkano::device::physical::PhysicalDevice>, u32) {
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
                        q.queue_flags.contains(vulkano::device::QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                vulkano::device::physical::PhysicalDeviceType::DiscreteGpu => 0,
                vulkano::device::physical::PhysicalDeviceType::IntegratedGpu => 1,
                vulkano::device::physical::PhysicalDeviceType::VirtualGpu => 2,
                vulkano::device::physical::PhysicalDeviceType::Cpu => 3,
                _ => 4,
            })
            .expect("no device available");

        Logger::log(LogLevel::High, "vulkan_wrapper", "Physical device created successfully.");
        return physical_device;
    }

    fn create_logical_device(physical_device: Arc<vulkano::device::physical::PhysicalDevice>, queue_family_index: u32, device_extensions: &vulkano::device::DeviceExtensions) -> (Arc<vulkano::device::Device>, Arc<vulkano::device::Queue>) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating logical device...");

        let (device, mut queues) = vulkano::device::Device::new(
            physical_device.clone(),
            vulkano::device::DeviceCreateInfo {
                queue_create_infos: vec![vulkano::device::QueueCreateInfo {
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

    fn create_swapchain(physical_device: Arc<vulkano::device::physical::PhysicalDevice>, device: Arc<vulkano::device::Device>, window: Arc<Window>, surface: Arc<vulkano::swapchain::Surface>) -> (Arc<vulkano::swapchain::Swapchain>, Vec<Arc<vulkano::image::Image>>) {
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

        let (swapchain, images) = vulkano::swapchain::Swapchain::new(
            device.clone(),
            surface.clone(),
            vulkano::swapchain::SwapchainCreateInfo {
                min_image_count: caps.min_image_count + 1,
                image_format,
                image_extent: dimensions.into(),
                image_usage: vulkano::image::ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            }
        ).unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Swapchain created successfully.");
        return (swapchain, images);
    }

    fn create_image_views(images: &Vec<Arc<vulkano::image::Image>>) -> Vec<Arc<vulkano::image::view::ImageView>> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating image views...");

        let mut image_views: Vec<Arc<vulkano::image::view::ImageView>> = Vec::new();
        for image in images {
            let create_info = vulkano::image::view::ImageViewCreateInfo {
                view_type: vulkano::image::view::ImageViewType::Dim2d,
                format: image.format(),
                component_mapping: vulkano::image::sampler::ComponentMapping {
                    r: vulkano::image::sampler::ComponentSwizzle::Identity,
                    g: vulkano::image::sampler::ComponentSwizzle::Identity,
                    b: vulkano::image::sampler::ComponentSwizzle::Identity,
                    a: vulkano::image::sampler::ComponentSwizzle::Identity,
                },
                subresource_range: vulkano::image::ImageSubresourceRange {
                    aspects: vulkano::image::ImageAspect::Color.into(),
                    mip_levels: Range {
                        start: 0,
                        end: 1,
                    },
                    array_layers: Range {
                        start: 0,
                        end: 1,
                    },
                },
                ..Default::default()
            };

            let image_view = vulkano::image::view::ImageView::new(image.clone(), create_info);
            image_views.push(image_view.unwrap());
        }

        Logger::log(LogLevel::High, "vulkan_wrapper", "Image views created successfully.");
        return image_views;
    }

    fn create_render_pass(logical_device: Arc<vulkano::device::Device>, image_format: vulkano::format::Format) -> Arc<vulkano::render_pass::RenderPass> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating renderpass...");

        let render_pass = vulkano::render_pass::RenderPass::new(
            logical_device.clone(),
            vulkano::render_pass::RenderPassCreateInfo {
                attachments: vec![
                    vulkano::render_pass::AttachmentDescription {
                        format: image_format,
                        samples: vulkano::image::SampleCount::Sample1,
                        load_op: vulkano::render_pass::AttachmentLoadOp::Clear,
                        store_op: vulkano::render_pass::AttachmentStoreOp::Store,
                        stencil_load_op: Some(vulkano::render_pass::AttachmentLoadOp::DontCare),
                        stencil_store_op: Some(vulkano::render_pass::AttachmentStoreOp::DontCare),
                        initial_layout: vulkano::image::ImageLayout::Undefined,
                        final_layout: vulkano::image::ImageLayout::PresentSrc, // present after rendering
                        ..Default::default()
                    }
                ],
                subpasses: vec![
                    vulkano::render_pass::SubpassDescription {
                        color_attachments: vec![
                            Some(vulkano::render_pass::AttachmentReference {
                                attachment: 0,
                                layout: vulkano::image::ImageLayout::ColorAttachmentOptimal,
                                ..Default::default()
                            })
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
        ).unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Created renderpass.");
        return render_pass;
    }

    fn create_graphics_pipeline(logical_device: Arc<vulkano::device::Device>, render_pass: Arc<vulkano::render_pass::RenderPass>) -> Arc<vulkano::pipeline::GraphicsPipeline> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating graphics pipeline...");

        let vs_path = Path::new(env!("OUT_DIR")).join("shader.vert.spv");
        let fs_path = Path::new(env!("OUT_DIR")).join("shader.frag.spv");

        let vs = VulkanContainer::load_shader(logical_device.clone(), vs_path);
        let fs = VulkanContainer::load_shader(logical_device.clone(), fs_path);

        let stages = smallvec![
            vulkano::pipeline::PipelineShaderStageCreateInfo::new(vs.entry_point("main").unwrap()),
            vulkano::pipeline::PipelineShaderStageCreateInfo::new(fs.entry_point("main").unwrap()),
        ];

        let pipeline_layout = vulkano::pipeline::PipelineLayout::new(
            logical_device.clone(),
            vulkano::pipeline::layout::PipelineLayoutCreateInfo::default(),
        ).unwrap();

        let mut pipeline_info = vulkano::pipeline::graphics::GraphicsPipelineCreateInfo::layout(pipeline_layout.clone());

        pipeline_info.stages = stages;

        pipeline_info.vertex_input_state = Some(
            vulkano::pipeline::graphics::vertex_input::VertexDefinition::definition(
                &<vertex::Vertex as vulkano::pipeline::graphics::vertex_input::Vertex>::per_vertex(),
                &vs.entry_point("main").unwrap()
            ).unwrap()
        );
        
        pipeline_info.input_assembly_state = Some(vulkano::pipeline::graphics::input_assembly::InputAssemblyState::default());

        pipeline_info.dynamic_state = HashSet::from_iter([
            vulkano::pipeline::DynamicState::ViewportWithCount,
            vulkano::pipeline::DynamicState::ScissorWithCount,
        ]);
        
        pipeline_info.viewport_state = Some(vulkano::pipeline::graphics::viewport::ViewportState {
            viewports: smallvec![],
            scissors: smallvec![],
            ..Default::default()
        });
        pipeline_info.rasterization_state = Some(vulkano::pipeline::graphics::rasterization::RasterizationState::default());

        pipeline_info.multisample_state = Some(vulkano::pipeline::graphics::multisample::MultisampleState::default());

        pipeline_info.color_blend_state = Some(vulkano::pipeline::graphics::color_blend::ColorBlendState {
            attachments: vec![
                vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState {
                    blend: None,           // no blending
                    color_write_mask: vulkano::pipeline::graphics::color_blend::ColorComponents::all(),
                    ..Default::default()
                }
            ],
            ..Default::default()
        });

        pipeline_info.subpass = Some(vulkano::render_pass::Subpass::from(render_pass.clone(), 0).unwrap().into());

        pipeline_info.layout = pipeline_layout.clone();
        
        let pipeline = vulkano::pipeline::GraphicsPipeline::new(
            logical_device.clone(),
            None,
            pipeline_info
        ).unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Graphics pipeline created successfully.");
        return pipeline;
    }

    fn create_frame_buffers(render_pass: Arc<vulkano::render_pass::RenderPass>, image_views: Vec<Arc<vulkano::image::view::ImageView>>) -> Vec<Arc<vulkano::render_pass::Framebuffer>> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating frame buffer...");

        let framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>> = image_views.iter().map(|image_view| {
            vulkano::render_pass::Framebuffer::new(
                render_pass.clone(),
                vulkano::render_pass::FramebufferCreateInfo {
                    attachments: vec![image_view.clone()],
                    ..Default::default()
                }
            ).unwrap()
        }).collect();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Created frame buffer...");
        return framebuffers;
    }
    
    fn load_shader(device: Arc<vulkano::device::Device>, path: impl AsRef<std::path::Path>) -> Arc<vulkano::shader::ShaderModule> {
        let mut file = File::open(path).expect("Failed to open shader file");
        let mut bytes = vec![];
        
        file.read_to_end(&mut bytes).expect("Failed to read shader file");
    
        let words = shader::spirv::bytes_to_words(&bytes);
        
        unsafe {
            vulkano::shader::ShaderModule::new(
                device,
                vulkano::shader::ShaderModuleCreateInfo::new(&words.unwrap())).expect("Failed to create shader module")
        }
    }
    
    pub fn create_vertex_buffer(&mut self, vertices: Vec<vertex::Vertex>) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating vertex buffer...");
        
        let memory_allocator = Arc::new(vulkano::memory::allocator::StandardMemoryAllocator::new_default(self.logical_device.clone()));
        let vertex_buffer = vulkano::buffer::Buffer::from_iter(
            memory_allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                usage: vulkano::buffer::BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo {
                memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_DEVICE | vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertices.iter().cloned()
        ).expect("Failed to create vertex buffer");

        self.vertexbuffers.push(vertex_buffer);
        Logger::log(LogLevel::High, "vulkan_wrapper", "Vertex buffer created successfully.");
    }

    pub fn delete_vertex_buffer(&mut self, index: usize) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Deleting vertex buffer...");

        if index >= self.vertexbuffers.len() {
            Logger::log(LogLevel::High, "vulkan_wrapper", "Index out of bounds for vertex buffer deletion.");
            return;
        }

        self.vertexbuffers.remove(index);

        Logger::log(LogLevel::High, "vulkan_wrapper", "Vertex buffer deleted successfully.");
    }

    fn create_command_buffer(&self, image_index: usize) -> Arc<vulkano::command_buffer::PrimaryAutoCommandBuffer> {
        let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            Arc::new(vulkano::command_buffer::allocator::StandardCommandBufferAllocator::new(self.logical_device.clone(), Default::default())),
            self.logical_device.active_queue_family_indices()[0],
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit
        ).unwrap();

        builder.begin_render_pass(
                vulkano::command_buffer::RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())], // background color
                    ..vulkano::command_buffer::RenderPassBeginInfo::framebuffer(self.framebuffers[image_index].clone())
                },
                vulkano::command_buffer::SubpassBeginInfo {
                    contents: vulkano::command_buffer::SubpassContents::Inline,
                    ..Default::default()
                },
            ).unwrap();
        builder.bind_pipeline_graphics(self.graphics_pipeline.clone()).unwrap();
        builder.set_viewport_with_count(self.viewports.clone()).unwrap();
        builder.set_scissor_with_count(self.scissors.clone()).unwrap();

        for vertex_buffer in self.vertexbuffers.iter() {
            builder.bind_vertex_buffers(0, vertex_buffer.clone()).unwrap();
            unsafe { builder.draw(vertex_buffer.len().try_into().unwrap(), 1, 0, 0).unwrap(); };
        }

        builder.end_render_pass(vulkano::command_buffer::SubpassEndInfo::default()).unwrap();
        let command_buffer = builder.build().unwrap();
        
        return command_buffer;
    }

    pub fn draw_frame(&self) {
        let (image_index, _, acquire_future) = vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();
        let command_buffer = self.create_command_buffer(image_index.try_into().unwrap());
        let future = vulkano::sync::GpuFuture::then_signal_fence_and_flush(
            vulkano::sync::GpuFuture::then_swapchain_present(
                vulkano::sync::GpuFuture::then_execute(acquire_future, self.queue.clone(),
                command_buffer).unwrap(),
                self.queue.clone(),
                vulkano::swapchain::SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(),
                image_index
            )
        ));

        match future {
            Ok(fut) => { fut.wait(None).unwrap(); }
            Err(e) => {
                eprintln!("Failed to flush frame: {:?}", e);
            }
        }
    }

    pub fn resize_viewport(&mut self, viewport_info: &ViewportInfo) {
        Logger::log(LogLevel::Medium, "vulkan_wrapper", "Resizing viewport...");

        self.viewports[0] = vulkano::pipeline::graphics::viewport::Viewport {
            offset: [viewport_info.offset[0], viewport_info.offset[1]],
            extent: [viewport_info.extent[0], viewport_info.extent[1]],
            depth_range: 0.0..=1.0,
        };

        self.scissors[0] = vulkano::pipeline::graphics::viewport::Scissor {
            offset: [viewport_info.offset[0] as u32, viewport_info.offset[1] as u32],
            extent: [viewport_info.extent[0] as u32, viewport_info.extent[1] as u32],
        };

        Logger::log(LogLevel::Medium, "vulkan_wrapper", "Viewport resized successfully.");
    }
}