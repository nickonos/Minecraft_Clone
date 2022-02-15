use winit::event::{Event, ElementState, KeyboardInput, WindowEvent, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};
use ash::version::{InstanceV1_0, EntryV1_0, DeviceV1_0};
use std::ffi::CString;
use ash::vk;
use std::ptr;
use winit::window::Window;
use std::os::raw::{c_void, c_char};

use crate::vulkan_engine::utilities::constants::{APPLICATION_VERSION, ENGINE_VERSION, API_VERSION, VALIDATION, MAX_FRAMES_IN_FLIGHT};
use crate::vulkan_engine::utilities;
use crate::vulkan_engine::utilities::debug::{check_validation_layer_support, populate_debug_messenger_create_info, ValidationInfo};
use crate::vulkan_engine::utilities::structures::{QueueFamilyIndices, SwapChainStruct, SwapChainSupportDetail, SurfaceStruct, SyncObjects};
use cgmath::num_traits::clamp;
use crate::vulkan_engine::setup::Setup;
use crate::settings_loader::key_mappings::KeyMappings;
use crate::vulkan_engine::presentation::Presentation;
use crate::vulkan_engine::graphics_pipeline::GraphicsPipeline;
use crate::vulkan_engine::buffers::Buffers;

const WINDOW_TITLE: &'static str = "Minecraft";
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;

pub struct VulkanEngine {
    window: winit::window::Window,

    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    physical_device: vk::PhysicalDevice,
    device: ash::Device,

    queue_family: QueueFamilyIndices,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
    swapchain_imageviews: Vec<vk::ImageView>,
    swapchain_framebuffers: Vec<vk::Framebuffer>,

    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,

    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,

    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: usize,

    is_framebuffer_resized: bool
}

impl VulkanEngine {

    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> VulkanEngine {
        let window = VulkanEngine::init_window(event_loop);

        let vulkan_setup = Setup::new(&window);

        let surface_struct = SurfaceStruct{
            surface_loader:    vulkan_setup.surface_loader.clone(),
            surface:vulkan_setup.surface,
            screen_width: WINDOW_WIDTH,
            screen_height: WINDOW_HEIGHT
        };

        let presentation = Presentation::new(
            &surface_struct,
            &vulkan_setup.instance,
            &vulkan_setup.device,
            vulkan_setup.physical_device,
            &vulkan_setup.queue_family_indices,
            &window
        );

        let graphics_pipeline = GraphicsPipeline::new(
            &vulkan_setup.device,
            presentation.swapchain_format,
            presentation.swapchain_extent
        );

        let buffers = Buffers::new(
            &vulkan_setup.device,
            graphics_pipeline.render_pass,
            &presentation.swapchain_imageviews,
            presentation.swapchain_extent,
            &vulkan_setup.queue_family_indices,
            graphics_pipeline.graphics_pipeline
        );

        let sync_objects = VulkanEngine::create_sync_objects(&vulkan_setup.device);


        VulkanEngine {
            window,

            _entry: vulkan_setup.entry,
            instance: vulkan_setup.instance,
            surface_loader: surface_struct.surface_loader,
            surface: surface_struct.surface,
            debug_utils_loader: vulkan_setup.debug_utils_loader,
            debug_messenger: vulkan_setup.debug_messenger,

            physical_device: vulkan_setup.physical_device,
            device: vulkan_setup.device,

            queue_family: vulkan_setup.queue_family_indices,
            graphics_queue: vulkan_setup.graphics_queue,
            present_queue: vulkan_setup.present_queue,

            swapchain_loader: presentation.swapchain_loader,
            swapchain: presentation.swapchain,
            swapchain_images: presentation.swapchain_images,
            swapchain_format: presentation.swapchain_format,
            swapchain_extent: presentation.swapchain_extent,
            swapchain_imageviews: presentation.swapchain_imageviews,
            swapchain_framebuffers: buffers.framebuffers,

            render_pass: graphics_pipeline.render_pass,
            pipeline_layout: graphics_pipeline.pipeline_layout,
            graphics_pipeline: graphics_pipeline.graphics_pipeline,

            command_pool: buffers.command_pool,
            command_buffers: buffers.command_buffers,

            image_available_semaphores: sync_objects.image_available_semaphores,
            render_finished_semaphores: sync_objects.render_finished_semaphores,
            in_flight_fences: sync_objects.inflight_fences,
            current_frame: 0,

            is_framebuffer_resized: false
        }
    }

    pub fn run(self, event_loop: winit::event_loop::EventLoop<()>, keymappings : KeyMappings){
        self.main_loop(event_loop, keymappings);
    }


    pub fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window{
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window")
    }

    fn draw_frame(&mut self) {
        let wait_fences = [self.in_flight_fences[self.current_frame]];

         unsafe {
            self.device
                .wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failed to wait for Fence!");


        };

        let (image_index, _is_sub_optimal) = unsafe {
            let result = self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    std::u64::MAX,
                    self.image_available_semaphores[self.current_frame],
                    vk::Fence::null(),
                );

            match result {
                Ok(image_index) => image_index,
                Err(vk_result) => match vk_result {
                    vk::Result::ERROR_OUT_OF_DATE_KHR => {
                        self.recreate_swapchain();
                        return;
                    }
                    _ => panic!("Failed to acquire Swap Chain Image!"),
                }
            }
        };

        let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];

        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_buffers[image_index as usize],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        }];

        unsafe {
            self.device
                .reset_fences(&wait_fences)
                .expect("Failed to reset Fence!");

            self.device
                .queue_submit(
                    self.graphics_queue,
                    &submit_infos,
                    self.in_flight_fences[self.current_frame],
                )
                .expect("Failed to execute queue submit.");
        }

        let swapchains = [self.swapchain];

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            p_results: ptr::null_mut(),
        };

        let result =unsafe {
            self.swapchain_loader
                .queue_present(self.present_queue, &present_info)
        };

        let is_resized = match result {
            Ok(_) => self.is_framebuffer_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                _ => panic!("Failed to execute queue present."),
            },
        };
        if is_resized {
            self.is_framebuffer_resized = false;
            self.recreate_swapchain();
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    fn recreate_swapchain(&mut self) {
        // parameters -------------
        let surface_stuff = SurfaceStruct {
            surface_loader: self.surface_loader.clone(),
            surface: self.surface,
            screen_width: WINDOW_WIDTH,
            screen_height: WINDOW_HEIGHT,
        };
        // ------------------------

        unsafe {
            self.device
                .device_wait_idle()
                .expect("Failed to wait device idle!")
        };
        self.cleanup_swapchain();

        let swapchain_stuff = Presentation::create_swapchain(
            &self.instance,
            &self.device,
            self.physical_device,
            &self.window,
            &surface_stuff,
            &self.queue_family,
        );
        self.swapchain_loader = swapchain_stuff.swapchain_loader;
        self.swapchain = swapchain_stuff.swapchain;
        self.swapchain_images = swapchain_stuff.swapchain_images;
        self.swapchain_format = swapchain_stuff.swapchain_format;
        self.swapchain_extent = swapchain_stuff.swapchain_extent;

        self.swapchain_imageviews = Presentation::create_image_views(
            &self.device,
            self.swapchain_format,
            &self.swapchain_images,
        );
        self.render_pass = GraphicsPipeline::create_render_pass(&self.device, self.swapchain_format);
        let (graphics_pipeline, pipeline_layout) = GraphicsPipeline::create_graphics_pipeline(
            &self.device,
            self.render_pass,
            swapchain_stuff.swapchain_extent,
        );
        self.graphics_pipeline = graphics_pipeline;
        self.pipeline_layout = pipeline_layout;

        self.swapchain_framebuffers = Buffers::create_frame_buffers(
            &self.device,
            self.render_pass,
            &self.swapchain_imageviews,
            &self.swapchain_extent,
        );
        self.command_buffers = Buffers::create_command_buffers(
            &self.device,
            self.command_pool,
            self.graphics_pipeline,
            &self.swapchain_framebuffers,
            self.render_pass,
            self.swapchain_extent,
        );
    }

    fn cleanup_swapchain(&self) {
        unsafe {
            self.device
                .free_command_buffers(self.command_pool, &self.command_buffers);
            for &framebuffer in self.swapchain_framebuffers.iter() {
                self.device.destroy_framebuffer(framebuffer, None);
            }
            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);
            for &image_view in self.swapchain_imageviews.iter() {
                self.device.destroy_image_view(image_view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }

    fn create_sync_objects(device: &ash::Device) -> SyncObjects {
        let mut sync_objects = SyncObjects {
            image_available_semaphores: vec![],
            render_finished_semaphores: vec![],
            inflight_fences: vec![],
        };

        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                let image_available_semaphore = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let render_finished_semaphore = device
                    .create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let inflight_fence = device
                    .create_fence(&fence_create_info, None)
                    .expect("Failed to create Fence Object!");

                sync_objects
                    .image_available_semaphores
                    .push(image_available_semaphore);
                sync_objects
                    .render_finished_semaphores
                    .push(render_finished_semaphore);
                sync_objects.inflight_fences.push(inflight_fence);
            }
        }

        sync_objects
    }

    fn main_loop(mut self, event_loop: EventLoop<()>, keymappings : KeyMappings){
        event_loop.run(move |event, _, control_flow| {

            //let escape : VirtualKeyCode = vulkan_engine.utilities::tools::keycode_from_i8(keymappings.menu)
                //expect("No escape Key mapping found");

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(escape), ElementState::Pressed) => {
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                | Event::MainEventsCleared => {
                   self.window.request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                },
                | Event::LoopDestroyed => {
                    unsafe {
                        self.device.device_wait_idle()
                            .expect("failed to wait device idle")
                    };
                }
                _ => (),
            }
        })
    }
}

impl Drop for VulkanEngine {
    fn drop(&mut self){
        unsafe{

            for &available_semaphore in self.image_available_semaphores.iter(){
                self.device.destroy_semaphore(available_semaphore, None);
            }

            for &finished_semaphore in self.render_finished_semaphores.iter(){
                self.device.destroy_semaphore(finished_semaphore, None);
            }

            for &fence in self.in_flight_fences.iter(){
                self.device.destroy_fence(fence, None)
            }

            self.device.destroy_command_pool(self.command_pool, None);

            for &framebuffer in self.swapchain_framebuffers.iter(){
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);

            for &imageview in self.swapchain_imageviews.iter(){
                self.device.destroy_image_view(imageview, None)
            }

            self.swapchain_loader.destroy_swapchain(self.swapchain, None);

            self.device.destroy_device(None);

            self.surface_loader.destroy_surface(self.surface, None);

            if VALIDATION.is_enable{
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None)
            }
            self.instance.destroy_instance(None)
        }
    }
}

