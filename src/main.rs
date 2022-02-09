#[macro_use]
extern crate ash;
extern crate winit;
extern crate image;
extern crate winapi;
extern crate serde;
extern crate bincode;
extern crate cgmath;


mod vulkan_app;
mod utilities;
mod vulkan_presentation;
mod vulkan_graphics_pipeline;
mod vulkan_setup;
mod settings_loader;

use winit::event_loop::EventLoop;
use settings_loader::key_mappings::KeyMappings;
use winit::event::VirtualKeyCode;
use winapi::_core::convert::TryFrom;


fn main(){
    let keymappings = match KeyMappings::read_from_file(){
        None => {
            let default = KeyMappings::default();
            default.write_to_file();
            default
        }
        Some(keymappings) => keymappings
    };


    let event_loop = EventLoop::new();
    let window = vulkan_app::VulkanApp::init_window(&event_loop);

    vulkan_app::VulkanApp::new(&window).run(event_loop, window, keymappings);
}