use ash::vk;
use glam::Vec3;
use imgui::{Context, dear_imgui_version};

fn main() {
    println!("Vulkan version: {:?}", vk::make_api_version(0, 1, 3, 0));
    imgui_test();
    glam_test();
}

fn imgui_test() {
    let mut imgui = Context::create();

    imgui.set_ini_filename(None);

    println!("Dear ImGui version: {}", dear_imgui_version());
}

fn glam_test() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    println!("glam is installed: {:?}", v);
}
