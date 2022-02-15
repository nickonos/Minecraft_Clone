use ash::vk;
use crate::vulkan_engine::utilities;
use crate::vulkan_engine::utilities::structures::{QueueFamilyIndices, SwapChainStruct, SurfaceStruct, SwapChainSupportDetail};
use cgmath::num_traits::clamp;
use crate::vulkan_engine::utilities::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use std::ptr;
use ash::{Instance, Device};
use ash::vk::PhysicalDevice;
use ash::version::DeviceV1_0;


pub struct Presentation {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain_imageviews: Vec<vk::ImageView>,
    _device: ash::Device
}

impl Presentation{

    pub fn new(
        surface_struct: &SurfaceStruct,
        instance: &Instance,
        device: &Device,
        physical_device: PhysicalDevice,
        queue_family_indices: &QueueFamilyIndices,
        window: &winit::window::Window
    ) -> Presentation{

        let swapchain_struct = Presentation::create_swapchain(
            instance,
            device,
            physical_device,
            window,
            &surface_struct,
            queue_family_indices
        );

        let swapchain_imageviews = Presentation::create_image_views(
            device,
            swapchain_struct.swapchain_format,
            &swapchain_struct.swapchain_images,
        );

        Presentation{
            swapchain_loader: swapchain_struct.swapchain_loader,
            swapchain: swapchain_struct.swapchain,
            swapchain_images: swapchain_struct.swapchain_images,
            swapchain_format: swapchain_struct.swapchain_format,
            swapchain_extent: swapchain_struct.swapchain_extent,
            swapchain_imageviews,
            _device: device.clone(),
        }
    }

    pub fn create_image_views(
        device: &ash::Device,
        surface_format: vk::Format,
        images: &Vec<vk::Image>,
    ) -> Vec<vk::ImageView> {
        let mut swapchain_imageviews = vec![];

        for &image in images.iter(){
            let imageview_create_info = vk::ImageViewCreateInfo{
                s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::ImageViewCreateFlags::empty(),
                view_type: vk::ImageViewType::TYPE_2D,
                format: surface_format,
                components: vk::ComponentMapping{
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                },
                subresource_range: vk::ImageSubresourceRange{
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1
                },
                image,
            };

            let imageview = unsafe{
                device
                    .create_image_view(&imageview_create_info, None)
                    .expect("Failed to create Image View")
            };

            swapchain_imageviews.push(imageview)
        }

        swapchain_imageviews
    }

    pub fn create_swapchain(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: vk::PhysicalDevice,
        window: &winit::window::Window,
        surface_struct: &utilities::structures::SurfaceStruct,
        queue_family: &QueueFamilyIndices
    ) -> utilities::structures::SwapChainStruct{


        let swapchain_support = Presentation::query_swapchain_support(physical_device, surface_struct);

        let surface_format = Presentation::choose_swapchain_format(&swapchain_support.formats);
        let present_mode =
            Presentation::choose_swapchain_present_mode(&swapchain_support.present_modes);
        let extent = Presentation::choose_swapchain_extent(&swapchain_support.capabilities, window);

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
                println!("V-Sync: Disabled");
                return available_present_mode;
            }
        }
        println!("V-Sync: Enabled");
        vk::PresentModeKHR::FIFO
    }

    fn choose_swapchain_extent(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        window : &winit::window::Window
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::max_value() {
            capabilities.current_extent
        } else {

            let window_size = window.inner_size();
            vk::Extent2D {
                width: clamp(
                    window_size.width as u32,
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: clamp(
                    window_size.height as u32,
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            }
        }
    }
}