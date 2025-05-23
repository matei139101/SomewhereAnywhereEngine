use std::{collections::btree_map::Entry, default, ops::Range, sync::Arc, vec};

use crate::engine::utils::logger::{Logger, LogLevel};
use winit::{event_loop::{ActiveEventLoop}, window::{Window}};
use smallvec::smallvec;

pub struct VulkanWrapper {
    instance: Arc<vulkano::instance::Instance>,
    surface: Arc<vulkano::swapchain::Surface>,
    physical_device: Arc<vulkano::device::physical::PhysicalDevice>,
    logical_device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>,
    swapchain: Arc<vulkano::swapchain::Swapchain>,
    images: Vec<Arc<vulkano::image::Image>>,
    image_views: Vec<Arc<vulkano::image::view::ImageView>>,
    render_pass: Arc<vulkano::render_pass::RenderPass>,
    graphics_pipeline: Arc<vulkano::pipeline::GraphicsPipeline>,
    framebuffer: Arc<vulkano::render_pass::Framebuffer>,
    command_buffer: Arc<vulkano::command_buffer::PrimaryAutoCommandBuffer>,
}

impl VulkanWrapper {
    pub fn new(event_loop: &ActiveEventLoop, window: Arc<Window>) -> Self {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating Vulkan wrapper...");

        let device_extensions = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };
        
        let instance = VulkanWrapper::create_instance(event_loop);
        let surface = VulkanWrapper::create_surface(&instance, window.clone());
        let (physical_device, queue_family_index) = VulkanWrapper::create_physical_device(&instance, &surface, &device_extensions);
        let (logical_device, queue) = VulkanWrapper::create_logical_device(physical_device.clone(), queue_family_index, &device_extensions);
        let (swapchain, images) = VulkanWrapper::create_swapchain(physical_device.clone(), logical_device.clone(), window.clone(), surface.clone());
        let image_views = VulkanWrapper::create_image_views(&images);
        let render_pass = VulkanWrapper::create_render_pass(logical_device.clone(), images[0].format());
        let graphics_pipeline = VulkanWrapper::create_graphics_pipeline(logical_device.clone(), render_pass.clone());
        let framebuffer = VulkanWrapper::create_frame_buffer(render_pass.clone(), image_views.clone());
        let command_buffer = VulkanWrapper::create_command_buffer(logical_device.clone(), graphics_pipeline.clone(), framebuffer.clone());

        let vulkan_wrapper = VulkanWrapper {
            instance: instance,
            surface: surface,
            physical_device,
            logical_device,
            queue,
            swapchain,
            images,
            image_views,
            render_pass,
            graphics_pipeline,
            framebuffer,
            command_buffer
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

    // Magic number hell!!! WHAT THE F*** IS EVEN HAPPENIIIIING
    fn create_graphics_pipeline(logical_device: Arc<vulkano::device::Device>, render_pass: Arc<vulkano::render_pass::RenderPass>) -> Arc<vulkano::pipeline::GraphicsPipeline> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating graphics pipeline...");

        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                path: "src/shaders/shader.vert",
            }
        }
        
        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                path: "src/shaders/shader.frag",
            }
        }

        let vs = vs::load(logical_device.clone()).unwrap();
        let fs = fs::load(logical_device.clone()).unwrap();

        let stages = smallvec![
            vulkano::pipeline::PipelineShaderStageCreateInfo::new(vs.entry_point("main").unwrap()),
            vulkano::pipeline::PipelineShaderStageCreateInfo::new(fs.entry_point("main").unwrap()),
        ];

        let viewport = vulkano::pipeline::graphics::viewport::Viewport {
            offset: [0.0, 0.0],
            extent: [600.0, 800.0],
            depth_range: 0.0..=1.0,
        };

        let pipeline_layout = vulkano::pipeline::PipelineLayout::new(
            logical_device.clone(),
            vulkano::pipeline::layout::PipelineLayoutCreateInfo::default(),
        ).unwrap();

        let mut pipeline_info = vulkano::pipeline::graphics::GraphicsPipelineCreateInfo::layout(pipeline_layout.clone());
        pipeline_info.stages = stages;
        pipeline_info.vertex_input_state = Some(vulkano::pipeline::graphics::vertex_input::VertexInputState::new());
        pipeline_info.input_assembly_state = Some(vulkano::pipeline::graphics::input_assembly::InputAssemblyState::default());
        pipeline_info.viewport_state = Some(vulkano::pipeline::graphics::viewport::ViewportState {
            viewports: smallvec![viewport.clone()],
            scissors: smallvec![vulkano::pipeline::graphics::viewport::Scissor::default()], // or define scissor to match viewport.extent
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

    fn create_frame_buffer(render_pass: Arc<vulkano::render_pass::RenderPass>, image_views: Vec<Arc<vulkano::image::view::ImageView>>) -> Arc<vulkano::render_pass::Framebuffer> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating frame buffer...");

        let framebuffer = vulkano::render_pass::Framebuffer::new(
            render_pass.clone(),
            vulkano::render_pass::FramebufferCreateInfo {
                attachments: vec![image_views[0].clone()],
                ..Default::default()
            },
        )
        .unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Created frame buffer...");
        return framebuffer;
    }
    
    fn create_command_buffer(device: Arc<vulkano::device::Device>, pipeline: Arc<vulkano::pipeline::GraphicsPipeline>, framebuffer: Arc<vulkano::render_pass::Framebuffer>) -> Arc<vulkano::command_buffer::PrimaryAutoCommandBuffer> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating command buffer...");

        let command_buffer_allocator = Arc::new(
            vulkano::command_buffer::allocator::StandardCommandBufferAllocator::new(device.clone(), Default::default())
        );

        let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            device.active_queue_family_indices()[0],
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit
        ).unwrap();

        builder.begin_render_pass(
                vulkano::command_buffer::RenderPassBeginInfo {
                    clear_values: vec![Some([0.1, 0.2, 0.3, 1.0].into())], // background color
                    ..vulkano::command_buffer::RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                vulkano::command_buffer::SubpassBeginInfo {
                    contents: vulkano::command_buffer::SubpassContents::Inline,
                    ..Default::default()
                },
            ).unwrap();
        builder.bind_pipeline_graphics(pipeline.clone()).unwrap();
        unsafe { builder.draw(3, 1, 0, 0).unwrap(); };
        builder.end_render_pass(vulkano::command_buffer::SubpassEndInfo::default()).unwrap();

        let command_buffer = builder.build().unwrap();
        Logger::log(LogLevel::High, "vulkan_wrapper", "Created command buffer...");
        return command_buffer;
    }

    pub fn draw_frame(&self) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Drawing frame...");

        let (image_index, _, acquire_future) = vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();

        let framebuffer = VulkanWrapper::create_frame_buffer(
            self.render_pass.clone(),
            vec![self.image_views[image_index as usize].clone()],
        );
    
        let command_buffer = VulkanWrapper::create_command_buffer(
            self.logical_device.clone(),
            self.graphics_pipeline.clone(),
            framebuffer,
        );

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

        Logger::log(LogLevel::High, "vulkan_wrapper", "Drew frame...");
    }
}