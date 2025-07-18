use std::{collections::{BTreeMap, HashMap, HashSet}, fs::File, io::Read, ops::Range, sync::Arc, vec};
use glam::{Mat4, Vec3};
use vulkano::{self, descriptor_set::{layout::{self, DescriptorSetLayoutBinding}}, device::{physical::PhysicalDevice}, format::{ClearValue}, image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage}, instance::Instance, memory::{allocator::{AllocationCreateInfo, MemoryTypeFilter}}, pipeline::{graphics::{depth_stencil::{DepthState, DepthStencilState}}, layout::PushConstantRange, Pipeline}, render_pass::{RenderPass}, shader::{self, ShaderStages}, swapchain::{Surface, Swapchain, SwapchainCreateInfo}};
use winit::{event_loop::{ActiveEventLoop}, window::{Window}};
use smallvec::{smallvec, SmallVec};
use std::path::Path;

use crate::engine::{utils::{logger::{LogLevel, Logger}, structs::transform::Transform}, vulkan::structs::{push_constants::PushConstants, vulkan_object::VulkanObject}};
use crate::engine::vulkan::structs::viewport::ViewportInfo;
use crate::engine::vulkan::structs::vertex;

pub struct VulkanContainer {
    instance: Arc<Instance>,
    surface: Arc<Surface>,
    physical_device: Arc<PhysicalDevice>,
    logical_device: Arc<vulkano::device::Device>,
    window: Arc<Window>,
    queue: Arc<vulkano::device::Queue>,
    swapchain: Arc<vulkano::swapchain::Swapchain>,
    images: Vec<Arc<Image>>,
    image_views: Vec<Arc<ImageView>>,
    render_pass: Arc<RenderPass>,
    memory_allocator: Arc<vulkano::memory::allocator::StandardMemoryAllocator>,
    graphics_pipeline: Arc<vulkano::pipeline::GraphicsPipeline>,
    framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>>,
    viewports: SmallVec<[vulkano::pipeline::graphics::viewport::Viewport; 2]>,
    scissors: SmallVec<[vulkano::pipeline::graphics::viewport::Scissor; 2]>,
    vertexbuffers: HashMap<usize, VulkanObject>,
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
        let render_pass = VulkanContainer::create_render_pass(logical_device.clone(), swapchain.clone());
        let memory_allocator = VulkanContainer::create_memory_allocator(logical_device.clone());
        let graphics_pipeline = VulkanContainer::create_graphics_pipeline(logical_device.clone(), render_pass.clone());
        let framebuffers = VulkanContainer::create_frame_buffers(render_pass.clone(), image_views.clone(), memory_allocator.clone());

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
            instance,
            surface,
            physical_device,
            logical_device,
            window,
            queue,
            swapchain,
            images,
            image_views,
            render_pass,
            memory_allocator,
            graphics_pipeline,
            framebuffers,
            viewports,
            scissors,
            vertexbuffers: HashMap::new(),
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

        let (swapchain, images) = vulkano::swapchain::Swapchain::new(
            device.clone(),
            surface.clone(),
            VulkanContainer::prepare_swapchain_create_info(physical_device, surface, window)
        ).unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Swapchain created successfully.");
        return (swapchain, images);
    }

    fn prepare_swapchain_create_info(physical_device: Arc<PhysicalDevice>, surface: Arc<Surface>, window: Arc<Window>) -> vulkano::swapchain::SwapchainCreateInfo {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Preparing swapchain createinfo...");

        let caps = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("Could not get surface capabilities");

        let dimensions = window.inner_size();
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        let swapchain_create_info = SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1,
            image_format,
            image_extent: dimensions.into(),
            image_usage: vulkano::image::ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            present_mode: vulkano::swapchain::PresentMode::Fifo,
            ..Default::default()
        };

        Logger::log(LogLevel::High, "vulkan_wrapper", "Swapchain createinfo prepared successfully.");
        return swapchain_create_info;
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

    fn create_render_pass(logical_device: Arc<vulkano::device::Device>, swapchain: Arc<Swapchain>) -> Arc<vulkano::render_pass::RenderPass> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating renderpass...");

        let render_pass = vulkano::single_pass_renderpass!(
            logical_device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
                depth: {
                    format: vulkano::format::Format::D16_UNORM,
                    samples: 1,
                    load_op: Clear,
                    store_op: DontCare,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth},
            }
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

        // Beginning of me not knowing what is happening...
        let mut descriptor_set_layout_binding = DescriptorSetLayoutBinding::descriptor_type(layout::DescriptorType::UniformBuffer);
        descriptor_set_layout_binding.stages = ShaderStages::VERTEX;
        let bindings = BTreeMap::from([(
            0,
            descriptor_set_layout_binding
        )]);


        let descriptor_set_layout = layout::DescriptorSetLayout::new(
            logical_device.clone(),
            layout::DescriptorSetLayoutCreateInfo {
                bindings,
                ..Default::default()
            },
        );

        let pipeline_layout = vulkano::pipeline::PipelineLayout::new(
            logical_device.clone(),
            vulkano::pipeline::layout::PipelineLayoutCreateInfo {
                set_layouts: vec![descriptor_set_layout.unwrap()],
                push_constant_ranges: vec![PushConstantRange {
                    stages: ShaderStages::VERTEX,
                    offset: 0,
                    size: std::mem::size_of::<PushConstants>() as u32,
                }],
                ..Default::default()
            },
        );
        // End of me not knowing what is happing... (Sorta)

        let mut pipeline_info = vulkano::pipeline::graphics::GraphicsPipelineCreateInfo::layout(pipeline_layout.unwrap().clone());

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

        let depth_sencil_state = DepthStencilState { depth: Some(DepthState::simple()), ..Default::default()};
        pipeline_info.depth_stencil_state = Some(depth_sencil_state);
        
        let pipeline = vulkano::pipeline::GraphicsPipeline::new(
            logical_device.clone(),
            None,
            pipeline_info
        ).unwrap();

        Logger::log(LogLevel::High, "vulkan_wrapper", "Graphics pipeline created successfully.");
        return pipeline;
    }

    fn create_frame_buffers(render_pass: Arc<vulkano::render_pass::RenderPass>, image_views: Vec<Arc<vulkano::image::view::ImageView>>, memory_allocator: Arc<vulkano::memory::allocator::StandardMemoryAllocator>) -> Vec<Arc<vulkano::render_pass::Framebuffer>> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating frame buffer...");

        let mut framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>> = vec!();
        for image_view in image_views.iter() {
            let depth_image_create_info = ImageCreateInfo { 
                image_type: ImageType::Dim2d,
                format: render_pass.attachments()[1].format, 
                extent: [image_view.image().extent()[0], image_view.image().extent()[1], 1],
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                ..Default::default()
            };
            let depth_image_allocation_info = AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            };
            let depth_image = Image::new(memory_allocator.clone(), depth_image_create_info, depth_image_allocation_info).unwrap();
            let depth_view = ImageView::new_default(depth_image).unwrap();

            framebuffers.push(
                vulkano::render_pass::Framebuffer::new(
                    render_pass.clone(),
                    vulkano::render_pass::FramebufferCreateInfo {
                        attachments: vec![image_view.clone(), depth_view.clone()],
                        ..Default::default()
                    }
                ).unwrap()
            );
        }

        Logger::log(LogLevel::High, "vulkan_wrapper", "Created frame buffer...");
        return framebuffers;
    }
    
    fn create_memory_allocator(logical_device: Arc<vulkano::device::Device>) -> Arc<vulkano::memory::allocator::StandardMemoryAllocator> {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating memory allocator...");

        let memory_allocator = Arc::new(vulkano::memory::allocator::StandardMemoryAllocator::new_default(logical_device.clone()));

        Logger::log(LogLevel::High, "vulkan_wrapper", "Created memory allocator.");
        return memory_allocator;
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
    
    pub fn create_vulkan_object(&mut self, id: usize, vertices: Vec<vertex::Vertex>, object_transform: Transform) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Creating vertex buffer...");
        
        let vertex_buffer = vulkano::buffer::Buffer::from_iter(
            self.memory_allocator.clone(),
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

        let vulkan_object = VulkanObject::new(vertex_buffer, object_transform);
        self.vertexbuffers.insert(id, vulkan_object);
        Logger::log(LogLevel::High, "vulkan_wrapper", "Vertex buffer created successfully.");
    }

    pub fn delete_vulkan_object(&mut self, index: usize) {
        Logger::log(LogLevel::High, "vulkan_wrapper", "Deleting vertex buffer...");

        if index >= self.vertexbuffers.len() {
            Logger::log(LogLevel::High, "vulkan_wrapper", "Index out of bounds for vertex buffer deletion.");
            return;
        }

        self.vertexbuffers.remove(&index);

        Logger::log(LogLevel::High, "vulkan_wrapper", "Vertex buffer deleted successfully.");
    }

    fn create_command_buffer(&self, image_index: usize, view_projection: Mat4) -> Arc<vulkano::command_buffer::PrimaryAutoCommandBuffer> {
        // Not needed as of now?
        /*
        let ubo = UniformBufferObject {
            mvp: VulkanContainer::make_mvp(self.viewports[0].extent[0] as f32 / self.viewports[0].extent[1] as f32).to_cols_array_2d(),
        };

        let uniform_buffer = Buffer::from_data(
            self.memory_allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            ubo
        ).expect("Failed to create uniform buffer");

        let descriptor_set_allocator: Arc<dyn DescriptorSetAllocator> = Arc::new(StandardDescriptorSetAllocator::new(self.logical_device.clone(), Default::default()));
        let pipeline_layout = self.graphics_pipeline.layout();
        let descriptor_set_layout = pipeline_layout.set_layouts().get(0).unwrap();
        let descriptor_set = DescriptorSet::new(
            descriptor_set_allocator,
            descriptor_set_layout.clone(),
            [WriteDescriptorSet::buffer(0, uniform_buffer)], // 0 is the binding
            [],
        ).unwrap();
        */

        let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            // [TO-DO]: Making a new allocator each call should be illegal.
            Arc::new(vulkano::command_buffer::allocator::StandardCommandBufferAllocator::new(self.logical_device.clone(), Default::default())),
            self.logical_device.active_queue_family_indices()[0],
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit
        ).unwrap();

        builder.begin_render_pass(
                vulkano::command_buffer::RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into()), Some(ClearValue::Depth(1.0))], // background color
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
        
        // for the ubo that isn't needed yet
        //builder.bind_descriptor_sets(vulkano::pipeline::PipelineBindPoint::Graphics, pipeline_layout.clone(), 0, descriptor_set).expect("Failed to bind uniform buffer");

        for vulkan_object in self.vertexbuffers.iter() {
            let model = Mat4::from_translation(vulkan_object.1.get_transform().position);
            let mvp = view_projection * model;
            let push_constants = PushConstants::new(mvp);


            let vertex_buffer = vulkan_object.1.get_buffer().clone();
            builder.bind_vertex_buffers(0, vertex_buffer.clone()).unwrap();
            builder.push_constants(self.graphics_pipeline.layout().clone(), 0, push_constants).unwrap();
            unsafe { builder.draw(vertex_buffer.len().try_into().unwrap(), 1, 0, 0).unwrap(); };
        }

        builder.end_render_pass(vulkano::command_buffer::SubpassEndInfo::default()).unwrap();
        let command_buffer = builder.build().unwrap();
        
        return command_buffer;
    }

    pub fn draw_frame(&self, camera_location: Vec3, camera_rotation: Vec3) {
        let (image_index, _, acquire_future) = vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();
        let view_projection = VulkanContainer::make_view_projection(self.viewports[0].extent[0] as f32 / self.viewports[0].extent[1] as f32, camera_location, camera_rotation);
        
        let command_buffer = self.create_command_buffer(image_index.try_into().unwrap(), view_projection);
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

        self.surface = VulkanContainer::create_surface(&self.instance.clone(), self.window.clone());
        (self.swapchain, self.images) = self.swapchain.recreate(VulkanContainer::prepare_swapchain_create_info(self.physical_device.clone(), self.surface.clone(), self.window.clone())).unwrap();
        self.image_views = VulkanContainer::create_image_views(&self.images.clone());
        self.render_pass = VulkanContainer::create_render_pass(self.logical_device.clone(), self.swapchain.clone());
        self.framebuffers = VulkanContainer::create_frame_buffers(self.render_pass.clone(), self.image_views.clone(), self.memory_allocator.clone());

        Logger::log(LogLevel::Medium, "vulkan_wrapper", "Viewport resized successfully.");
    }

    fn make_view_projection(aspect_ratio: f32, camera_location: Vec3, camera_rotation: Vec3) -> Mat4 {
        let rotation_x = Mat4::from_rotation_x(camera_rotation.x);
        let rotation_y = Mat4::from_rotation_y(camera_rotation.y);
        let rotation_z = Mat4::from_rotation_z(camera_rotation.z);
        let rotation = rotation_x * rotation_y * rotation_z;

        let translation = Mat4::from_translation(camera_location);
        let view = rotation * translation;
        let proj = Mat4::perspective_rh_gl(45.0_f32.to_radians(), aspect_ratio, 0.1, 10.0);
        let view_projection = proj * view;

        return view_projection;
    }
}