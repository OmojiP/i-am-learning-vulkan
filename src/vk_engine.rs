use ash::vk::{self};
use sdl2::event::{Event, WindowEvent};
use std::time::Duration;

use crate::vulkan::{self, debug::DebugMessenger, instance::VkInstance};

#[derive(Debug)]
pub enum VkEngineError {
    SdlInit(String),
    SdlVideoInit(String),
    WindowCreation(String),
    VulkanExtension(String),
    VulkanEntry(ash::LoadingError),
    VulkanInstance(ash::vk::Result),
    ValidationLayerNotFound(),
}

pub struct VkEngine {
    pub frame_number: u32,
    pub stop_rendering: bool,
    pub window_extent: vk::Extent2D,

    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,

    physical_device: vulkan::physical_device::PhysicalDevice,

    // フィールドは宣言順に破棄されるので、Vulkanオブジェクトは
    // 作成と逆順（debug_messenger -> instance -> entry）に並べる。
    // entryを先に破棄するとvulkan-1.dllがアンロードされ、
    // その後のdestroy_*呼び出しがアクセス違反になる。
    // Debug Messenger
    debug_messenger: Option<DebugMessenger>,
    instance: VkInstance,
    entry: ash::Entry,
}

impl VkEngine {
    pub fn init() -> Result<VkEngine, VkEngineError> {
        // SDLの初期化
        let sdl_context = sdl2::init().map_err(VkEngineError::SdlInit)?;
        let video_subsystem = sdl_context.video().map_err(VkEngineError::SdlVideoInit)?;

        let window_extent = vk::Extent2D {
            width: 800,
            height: 600,
        };

        let window = video_subsystem
            .window("Vulkan Engine", window_extent.width, window_extent.height)
            .position_centered()
            .vulkan()
            .build()
            .map_err(|err| VkEngineError::WindowCreation(err.to_string()))?;

        let entry = unsafe { ash::Entry::load().map_err(VkEngineError::VulkanEntry)? };

        // create Vulkan instance
        let instance = vulkan::instance::VkInstance::new(&window, &entry)?;

        // setupDebugMessenger
        let debug_messenger = if vulkan::instance::ENABLE_VALIDATION_LAYERS {
            Some(DebugMessenger::new(&entry, &instance.instance)?)
        } else {
            None
        };

        // pick Physical Device
        let physical_device = vulkan::physical_device::PhysicalDevice::new(&instance.instance)?;

        Ok(VkEngine {
            frame_number: 0,
            stop_rendering: false,
            window_extent: window_extent,

            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            window: window,

            physical_device: physical_device,
            debug_messenger: debug_messenger,
            instance: instance,
            entry: entry,
        })
    }

    pub fn draw(&self) {}

    pub fn run(&mut self) -> Result<(), VkEngineError> {
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(|err| VkEngineError::SdlInit(err.to_string()))?;
        let mut b_quit: bool = false;

        // Main loop
        while !b_quit {
            while let Some(event) = event_pump.poll_event() {
                match event {
                    Event::Quit { .. } => {
                        b_quit = true;
                    }
                    Event::Window { win_event, .. } => {
                        // Handle window events
                        match win_event {
                            WindowEvent::Minimized => {
                                self.stop_rendering = true;
                            }
                            WindowEvent::Restored => {
                                self.stop_rendering = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            if self.stop_rendering {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.draw();
        }

        Ok(())
    }
}

// cleanupの代わり
impl Drop for VkEngine {
    fn drop(&mut self) {}
}
