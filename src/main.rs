#[macro_use]
extern crate ash;

mod vulkan_app;
mod utilities;
mod vulkan_presentation;
mod vulkan_graphics_pipeline;
mod vulkan_setup;

use winit::event_loop::EventLoop;

fn main(){
    let event_loop = EventLoop::new();
    let window = vulkan_app::VulkanApp::init_window(&event_loop);

    vulkan_app::VulkanApp::new(&window).run(event_loop, window);
}