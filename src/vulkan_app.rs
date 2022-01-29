use winit::event::{Event, ElementState, KeyboardInput, WindowEvent, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};
use ash::version::{InstanceV1_0, EntryV1_0, DeviceV1_0};
use std::ffi::CString;
use ash::vk;
use std::ptr;
use winit::window::Window;
use std::os::raw::{c_void, c_char};

use crate::utilities::constants::{APPLICATION_VERSION, ENGINE_VERSION, API_VERSION, VALIDATION};
use crate::utilities;
use crate::utilities::debug::{check_validation_layer_support, populate_debug_messenger_create_info, ValidationInfo};
use crate::utilities::structures::{QueueFamilyIndices, SwapChainStruct, SwapChainSupportDetail, SurfaceStruct};
use cgmath::num_traits::clamp;
use crate::vulkan_setup::VulkanSetup;

const WINDOW_TITLE: &'static str = "Minecraft";
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;

pub struct VulkanApp{
    _entry: ash::Entry,
    instance: ash::Instance,
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    _physical_device: vk::PhysicalDevice,
    device: ash::Device,

    _graphics_queue: vk::Queue,
    _present_queue: vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    _swapchain_images: Vec<vk::Image>,
    _swapchain_format: vk::Format,
    _swapchain_extent: vk::Extent2D,
}

impl VulkanApp{

    pub fn new(window: &winit::window::Window) -> VulkanApp{
        let vulkan_setup = VulkanSetup::new(window);

        let surface_struct = SurfaceStruct{
            surface_loader:    vulkan_setup.surface_loader.clone(),
            surface:vulkan_setup.surface
        };

        let swapchain_struct = VulkanApp::create_swapchain(
            &vulkan_setup.instance,
            &vulkan_setup.device,
            vulkan_setup.physical_device,
            &surface_struct,
            &vulkan_setup.queue_family_indices
        );


        //create image views

        //create graphics pipeline


        VulkanApp{
            _entry: vulkan_setup.entry.clone(),
            instance: vulkan_setup.instance.clone(),
            surface_loader: surface_struct.surface_loader,
            surface: surface_struct.surface,
            debug_utils_loader: vulkan_setup.debug_utils_loader.clone(),
            debug_messenger: vulkan_setup.debug_messenger,

            _physical_device: vulkan_setup.physical_device,
            device: vulkan_setup.device.clone(),

            _graphics_queue: vulkan_setup.graphics_queue,
            _present_queue: vulkan_setup.present_queue,

            swapchain_loader: swapchain_struct.swapchain_loader,
            swapchain: swapchain_struct.swapchain,
            _swapchain_images: swapchain_struct.swapchain_images,
            _swapchain_format: swapchain_struct.swapchain_format,
            _swapchain_extent: swapchain_struct.swapchain_extent
        }
    }

    pub fn run(self, event_loop: winit::event_loop::EventLoop<()>, window: winit::window::Window){
        self.main_loop(event_loop, window);
    }


    pub fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window{
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window")
    }

    fn create_swapchain(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        surface_struct: &utilities::structures::SurfaceStruct,
        queue_family: &QueueFamilyIndices
    ) -> utilities::structures::SwapChainStruct{


        let swapchain_support = VulkanApp::query_swapchain_support(physical_device, surface_struct);

        let surface_format = VulkanApp::choose_swapchain_format(&swapchain_support.formats);
        let present_mode =
            VulkanApp::choose_swapchain_present_mode(&swapchain_support.present_modes);
        let extent = VulkanApp::choose_swapchain_extent(&swapchain_support.capabilities);

        let image_count = swapchain_support.capabilities.min_image_count + 1;
        let image_count = if swapchain_support.capabilities.max_image_count > 0 {
            image_count.min(swapchain_support.capabilities.max_image_count)
        } else {
            image_count
        };

        let (image_sharing_mode, queue_family_index_count, queue_family_indices) =
            if queue_family.graphics_family != queue_family.present_family {
                (
                    vk::SharingMode::EXCLUSIVE,
                    2,
                    vec![
                        queue_family.graphics_family.unwrap(),
                        queue_family.present_family.unwrap(),
                    ],
                )
            } else {
                (vk::SharingMode::EXCLUSIVE, 0, vec![])
            };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: surface_struct.surface,
            min_image_count: image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            queue_family_index_count,
            pre_transform: swapchain_support.capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
            image_array_layers: 1,
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, device);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Failed to create Swapchain!")
        };

        let swapchain_images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .expect("Failed to get Swapchain Images.")
        };

        SwapChainStruct {
            swapchain_loader,
            swapchain,
            swapchain_format: surface_format.format,
            swapchain_extent: extent,
            swapchain_images,
        }
    }

    fn query_swapchain_support(
        physical_device: vk::PhysicalDevice,
        surface_stuff: &SurfaceStruct,
    ) -> SwapChainSupportDetail {
        unsafe {
            let capabilities = surface_stuff
                .surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface_stuff.surface)
                .expect("Failed to query for surface capabilities.");
            let formats = surface_stuff
                .surface_loader
                .get_physical_device_surface_formats(physical_device, surface_stuff.surface)
                .expect("Failed to query for surface formats.");
            let present_modes = surface_stuff
                .surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface_stuff.surface)
                .expect("Failed to query for surface present mode.");

            SwapChainSupportDetail {
                capabilities,
                formats,
                present_modes,
            }
        }
    }

    fn choose_swapchain_format(
        available_formats: &Vec<vk::SurfaceFormatKHR>,
    ) -> vk::SurfaceFormatKHR {
        for available_format in available_formats {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return available_format.clone();
            }
        }

        return available_formats.first().unwrap().clone();
    }

    fn choose_swapchain_present_mode(
        available_present_modes: &Vec<vk::PresentModeKHR>,
    ) -> vk::PresentModeKHR {
        for &available_present_mode in available_present_modes.iter() {
            if available_present_mode == vk::PresentModeKHR::MAILBOX {
                println!("present Mode: MAILBOX");
                return available_present_mode;
            }
        }
        println!("present Mode: FIFO");
        vk::PresentModeKHR::FIFO
    }

    fn choose_swapchain_extent(capabilities: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::max_value() {
            capabilities.current_extent
        } else {
            vk::Extent2D {
                width: clamp(
                    WINDOW_WIDTH,
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: clamp(
                    WINDOW_HEIGHT,
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            }
        }
    }

    fn draw_frame(&mut self) {
        // Drawing will be here
    }


    fn main_loop(mut self, event_loop: EventLoop<()>, window: Window){
        event_loop.run(move |event, _, control_flow| {

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
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
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
                    window.request_redraw();
                },
                | Event::RedrawRequested(_window_id) => {
                    self.draw_frame();
                },
                _ => (),
            }
        })
    }
}

impl Drop for VulkanApp{
    fn drop(&mut self){
        unsafe{
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

