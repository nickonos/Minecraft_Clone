use winit::event::{Event, ElementState, KeyboardInput, WindowEvent, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};
use ash::version::{InstanceV1_0, EntryV1_0, DeviceV1_0};
use std::ffi::CString;
use ash::vk;
use std::ptr;
use winit::window::Window;
use std::os::raw::{c_void, c_char};

use crate::vulkan_engine::utilities::constants::{APPLICATION_VERSION, ENGINE_VERSION, API_VERSION, VALIDATION};
use crate::vulkan_engine::utilities;
use crate::vulkan_engine::utilities::debug::{check_validation_layer_support, populate_debug_messenger_create_info, ValidationInfo};
use crate::vulkan_engine::utilities::structures::{QueueFamilyIndices, SwapChainStruct, SwapChainSupportDetail, SurfaceStruct};
use cgmath::num_traits::clamp;
use crate::vulkan_engine::setup::Setup;
use crate::settings_loader::key_mappings::KeyMappings;
use crate::vulkan_engine::presentation::Presentation;

const WINDOW_TITLE: &'static str = "Minecraft";
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;

pub struct VulkanEngine {
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

impl VulkanEngine {

    pub fn new(window: &winit::window::Window) -> VulkanEngine {
        let vulkan_setup = Setup::new(window);

        let surface_struct = SurfaceStruct{
            surface_loader:    vulkan_setup.surface_loader.clone(),
            surface:vulkan_setup.surface
        };

        let presentation = Presentation::new(
            &surface_struct,
            &vulkan_setup.instance,
            &vulkan_setup.device,
            vulkan_setup.physical_device,
            &vulkan_setup.queue_family_indices
        );

        //create image views

        //create graphics pipeline


        VulkanEngine {
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

            swapchain_loader: presentation.swapchain_loader,
            swapchain: presentation.swapchain,
            _swapchain_images: presentation.swapchain_images,
            _swapchain_format: presentation.swapchain_format,
            _swapchain_extent: presentation.swapchain_extent
        }
    }

    pub fn run(self, event_loop: winit::event_loop::EventLoop<()>, window: winit::window::Window, keymappings : KeyMappings){
        self.main_loop(event_loop, window, keymappings);
    }


    pub fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window{
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window")
    }

    fn draw_frame(&mut self) {
        // Drawing will be here
    }


    fn main_loop(mut self, event_loop: EventLoop<()>, window: Window, keymappings : KeyMappings){
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

impl Drop for VulkanEngine {
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

