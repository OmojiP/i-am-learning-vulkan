// arg
use std::env;

mod vk_engine;
use crate::vk_engine::VkEngineError;
use vk_engine::VkEngine;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    // init
    let mut engine: VkEngine = match VkEngine::init() {
        Ok(engine) => engine,
        Err(error) => {
            eprintln!("Failed to initialize VkEngine:");
            eprintln!("{:?}", error);
            return;
        }
    };

    if let Err(error) = engine.run() {
        eprintln!("VkEngine runtime error:");
        eprintln!("{:?}", error)
    }
}
