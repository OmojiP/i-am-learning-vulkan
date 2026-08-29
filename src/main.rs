// arg
use std::env;

mod vk_engine;
use vk_engine::VulkanEngine;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    // init
    let mut engine: VulkanEngine = VulkanEngine::new();

    engine.run();

    engine.cleanup();

    Ok(())
}
