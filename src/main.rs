#[macro_use]
extern crate ash;

mod vulkan_app;
mod utilities;

fn main(){
    vulkan_app::VulkanApp::new().run();
}