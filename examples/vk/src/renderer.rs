use std::sync::Arc;

use exposed::{log::log_info, window::WindowHandle};
use raw_window_handle::*;
use vulkano::buffer::BufferContents;

use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo,
        SubpassBeginInfo, SubpassContents,
    },
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
    },
    image::{view::ImageView, Image, ImageUsage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryTypeFilter, StandardMemoryAllocator,
    },
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    swapchain::{acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo},
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};

use crate::shader::{fs, vs};

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct VVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

struct RendererSurface {
    surface: Arc<Surface>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<Image>>,
    framebuffers: Vec<Arc<Framebuffer>>,
}

pub struct Renderer {
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    queue_family_index: u32,
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    pipeline: Arc<GraphicsPipeline>,
    viewport: Viewport,
    command_buffer_allocator: StandardCommandBufferAllocator,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    vertex_buffer: Subbuffer<[VVertex]>,
    renderer_surface: Option<RendererSurface>,
    render_pass: Arc<RenderPass>,
    window: WindowHandle,
}

impl Renderer {
    pub unsafe fn new(window: WindowHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let library: Arc<VulkanLibrary> = VulkanLibrary::new().unwrap();

        let mut required_extensions = Surface::required_extensions(&W(window));

        required_extensions.ext_validation_features = true;

        let instance: Arc<Instance> = Instance::new(
            library.clone(),
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                enabled_layers: if required_extensions.ext_validation_features {
                    vec!["VK_LAYER_KHRONOS_validation".to_string()]
                } else {
                    vec![]
                },
                ..Default::default()
            },
        )
        .unwrap();

        let device_extensions = DeviceExtensions { khr_swapchain: true, ..DeviceExtensions::empty() };

        let surface: Arc<Surface> = Surface::from_window(instance.clone(), Arc::new(W(window))).unwrap();

        let (physical_device, queue_family_index): (Arc<vulkano::device::physical::PhysicalDevice>, u32) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS) && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .expect("no suitable physical device found");

        log_info!(
            "Exposed",
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type
        );

        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo { queue_family_index, ..Default::default() }],
                ..Default::default()
            },
        )
        .unwrap();

        let queue: Arc<Queue> = queues.next().unwrap();

        let (swapchain, images): (Arc<Swapchain>, Vec<Arc<Image>>) = {
            // Querying the capabilities of the surface. When we create the swapchain we can only pass
            // values that are allowed by the capabilities.
            let surface_capabilities = device.physical_device().surface_capabilities(&surface, Default::default()).unwrap();

            // Choosing the internal format that the images will have.
            let image_format = device.physical_device().surface_formats(&surface, Default::default()).unwrap()[0].0;

            // Please take a look at the docs for the meaning of the parameters we didn't mention.
            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),

                    image_format,

                    image_extent: {
                        let size = window.client_size().unwrap();
                        [size.width as _, size.height as _]
                    },

                    image_usage: ImageUsage::COLOR_ATTACHMENT,

                    composite_alpha: surface_capabilities.supported_composite_alpha.into_iter().next().unwrap(),

                    ..Default::default()
                },
            )
            .unwrap()
        };

        let memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>> =
            Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        let vertices =
            [VVertex { position: [-0.5, -0.25] }, VVertex { position: [0.0, 0.5] }, VVertex { position: [0.25, -0.1] }];
        let vertex_buffer: Subbuffer<[VVertex]> = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo { usage: BufferUsage::VERTEX_BUFFER, ..Default::default() },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertices,
        )
        .unwrap();

        let render_pass: Arc<RenderPass> = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .unwrap();

        let pipeline: Arc<GraphicsPipeline> = {
            let vs = vs::load(device.clone()).unwrap().entry_point("main").unwrap();
            let fs = fs::load(device.clone()).unwrap().entry_point("main").unwrap();

            let vertex_input_state = VVertex::per_vertex().definition(&vs.info().input_interface).unwrap();

            let stages = [PipelineShaderStageCreateInfo::new(vs), PipelineShaderStageCreateInfo::new(fs)];

            let layout = PipelineLayout::new(
                device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                    .into_pipeline_layout_create_info(device.clone())
                    .unwrap(),
            )
            .unwrap();

            let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

            GraphicsPipeline::new(
                device.clone(),
                None,
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(InputAssemblyState::default()),
                    viewport_state: Some(ViewportState::default()),
                    rasterization_state: Some(RasterizationState::default()),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.num_color_attachments(),
                        ColorBlendAttachmentState::default(),
                    )),
                    dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                    subpass: Some(subpass.into()),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                },
            )
            .unwrap()
        };

        let mut viewport = Viewport { offset: [0.0, 0.0], extent: [0.0, 0.0], depth_range: 0.0..=1.0 };

        let framebuffers: Vec<Arc<Framebuffer>> = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

        let command_buffer_allocator: StandardCommandBufferAllocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());

        let recreate_swapchain = false;

        let previous_frame_end: Option<Box<dyn GpuFuture>> = Some(sync::now(device.clone()).boxed());

        Ok(Self {
            library,
            instance,
            physical_device,
            queue_family_index,
            device,
            queue,
            memory_allocator,
            pipeline,
            viewport,
            command_buffer_allocator,
            previous_frame_end,
            vertex_buffer,
            window,
            render_pass,
            renderer_surface: Some(RendererSurface { surface, swapchain, images, framebuffers }),
        })
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        let size = self.window.client_size().unwrap();
        if size.height * size.width == 0 {
            return Ok(());
        }

        let (image_index, suboptimal, acquire_future) = match acquire_next_image(
            self.renderer_surface.as_mut().unwrap().swapchain.clone(),
            None,
        )
        .map_err(Validated::unwrap)
        {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                return Ok(());
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };

        if suboptimal {
            self.recreate_swapchains(size.width as _, size.width as _);
        }

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.01, 0.01, 0.01, 1.0].into())],

                    ..RenderPassBeginInfo::framebuffer(
                        self.renderer_surface.as_mut().unwrap().framebuffers[image_index as usize].clone(),
                    )
                },
                SubpassBeginInfo { contents: SubpassContents::Inline, ..Default::default() },
            )
            .unwrap()
            .set_viewport(0, [self.viewport.clone()].into_iter().collect())
            .unwrap()
            .bind_pipeline_graphics(self.pipeline.clone())
            .unwrap()
            .bind_vertex_buffers(0, self.vertex_buffer.clone())
            .unwrap()
            .draw(self.vertex_buffer.len() as u32, 1, 0, 0)
            .unwrap()
            .end_render_pass(Default::default())
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    self.renderer_surface.as_mut().unwrap().swapchain.clone(),
                    image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                panic!("failed to flush future: {e}");
            }
        }

        Ok(())
    }

    pub fn create_surface(&mut self, window: WindowHandle) {
        self.window = window;

        let surface: Arc<Surface> = Surface::from_window(self.instance.clone(), Arc::new(W(window))).unwrap();

        let is_surface_supported =
            self.physical_device.surface_support(self.queue_family_index as u32, &surface).unwrap_or(false);

        if !is_surface_supported {
            panic!("New surface is not supported.");
        }

        let (swapchain, images): (Arc<Swapchain>, Vec<Arc<Image>>) = {
            // Querying the capabilities of the surface. When we create the swapchain we can only pass
            // values that are allowed by the capabilities.
            let surface_capabilities = self.device.physical_device().surface_capabilities(&surface, Default::default()).unwrap();

            // Choosing the internal format that the images will have.
            let image_format = self.device.physical_device().surface_formats(&surface, Default::default()).unwrap()[0].0;

            // Please take a look at the docs for the meaning of the parameters we didn't mention.
            Swapchain::new(
                self.device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),

                    image_format,

                    image_extent: {
                        let size = window.client_size().unwrap();
                        [size.width as _, size.height as _]
                    },

                    image_usage: ImageUsage::COLOR_ATTACHMENT,

                    composite_alpha: surface_capabilities.supported_composite_alpha.into_iter().next().unwrap(),

                    ..Default::default()
                },
            )
            .unwrap()
        };

        let framebuffers: Vec<Arc<Framebuffer>> =
            window_size_dependent_setup(&images, self.render_pass.clone(), &mut self.viewport);

        self.renderer_surface = Some(RendererSurface { surface, swapchain, images, framebuffers })
    }

    pub fn destroy_surface(&mut self, window: WindowHandle) {
        self.renderer_surface = None;
    }

    pub fn recreate_swapchains(&mut self, width: u32, height: u32) {
        let renderer_surface = self.renderer_surface.as_mut().unwrap();
        let (new_swapchain, new_images) = renderer_surface
            .swapchain
            .recreate(SwapchainCreateInfo { image_extent: [width, height], ..renderer_surface.swapchain.create_info() })
            .expect("failed to recreate swapchain");

        renderer_surface.swapchain = new_swapchain;

        renderer_surface.framebuffers = window_size_dependent_setup(&new_images, self.render_pass.clone(), &mut self.viewport);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {}
}

fn raw_display_handle(window: WindowHandle) -> RawDisplayHandle {
    todo!()
}

fn raw_window_handle(window: WindowHandle) -> RawWindowHandle {
    todo!()
}

struct W(WindowHandle);

unsafe impl HasRawDisplayHandle for W {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        #[cfg(target_os = "android")]
        return RawDisplayHandle::Android(AndroidDisplayHandle::empty());
        #[cfg(target_os = "windows")]
        return RawDisplayHandle::Windows(WindowsDisplayHandle::empty());

        #[cfg(target_os = "linux")]
        return {
            let mut d = raw_window_handle::XlibDisplayHandle::empty();
            let c = unsafe { exposed::window::_x11::ThreadContext::current_thread() };
            d.display = c.display as _;
            d.screen = c.screen_id;
            raw_window_handle::RawDisplayHandle::Xlib(d)
        };
    }
}

unsafe impl HasRawWindowHandle for W {
    fn raw_window_handle(&self) -> RawWindowHandle {
        #[cfg(target_os = "android")]
        return RawWindowHandle::AndroidNdk({
            let mut handle = AndroidNdkWindowHandle::empty();
            handle.a_native_window = self.0 .0.native_handle() as _;
            handle
        });
        #[cfg(target_os = "windows")]
        return RawWindowHandle::Win32({
            let mut handle = Win32WindowHandle::empty();
            handle.hwnd = self.0 .0 .0 as _;
            handle.hinstance = unsafe { exposed::window::win32::HINSTANCE } as _;
            handle
        });

        #[cfg(target_os = "linux")]
        return {
            let mut w = raw_window_handle::XlibWindowHandle::empty();
            w.window = self.0 .0 .0;
            raw_window_handle::RawWindowHandle::Xlib(w)
        };
    }
}

unsafe impl Sync for W {}
unsafe impl Send for W {}

fn window_size_dependent_setup(
    images: &[Arc<Image>], render_pass: Arc<RenderPass>, viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(render_pass.clone(), FramebufferCreateInfo { attachments: vec![view], ..Default::default() })
                .unwrap()
        })
        .collect::<Vec<_>>()
}
