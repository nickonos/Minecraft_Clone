#[macro_use]
extern crate ash;
extern crate winit;
extern crate image;
extern crate winapi;
extern crate serde;
extern crate bincode;
extern crate cgmath;

mod vulkan_engine;
mod settings_loader;

use winit::event_loop::EventLoop;
use settings_loader::key_mappings::KeyMappings;
use vulkan_engine::vulkan_engine::VulkanEngine;


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
    let window = VulkanEngine::init_window(&event_loop);

    VulkanEngine::new(&window).run(event_loop, window, keymappings);
}