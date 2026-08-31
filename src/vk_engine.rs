use ash::vk::{self};
use sdl2::event::{Event, WindowEvent};
use std::time::Duration;

#[derive(Debug)]
pub enum VkEngineError {
    SdlInit(String),
    SdlVideoInit(String),
    WindowCreation(String),
    VulkanExtension(String),
    VulkanEntry(ash::LoadingError),
    VulkanInstance(ash::vk::Result),
}

pub struct VkEngine {
    pub frame_number: u32,
    pub stop_rendering: bool,
    pub window_extent: vk::Extent2D,

    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,

    entry: ash::Entry,
    instance: ash::Instance,
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

        let instance = VkEngine::create_instance(&window, &entry)?;

        Ok(VkEngine {
            frame_number: 0,
            stop_rendering: false,
            window_extent: window_extent,
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            window: window,
            entry: entry,
            instance: instance,
        })
    }

    pub fn create_instance(
        window: &sdl2::video::Window,
        entry: &ash::Entry,
    ) -> Result<ash::Instance, VkEngineError> {
        let mut application_info = ash::vk::ApplicationInfo::default();
        application_info.s_type = ash::vk::StructureType::APPLICATION_INFO;
        application_info.p_application_name = "Hello Triangle\0".as_ptr() as *const i8;
        application_info.application_version = ash::vk::make_api_version(0, 1, 0, 0);
        application_info.p_engine_name = "No Engine\0".as_ptr() as *const i8;
        application_info.engine_version = ash::vk::make_api_version(0, 1, 0, 0);
        application_info.api_version = ash::vk::API_VERSION_1_3;

        let mut create_info = ash::vk::InstanceCreateInfo::default();
        create_info.s_type = ash::vk::StructureType::INSTANCE_CREATE_INFO;
        create_info.p_application_info = &application_info;

        let extension_names = window
            .vulkan_instance_extensions()
            .map_err(|err| VkEngineError::VulkanExtension(err.to_string()))?;
        let extension_names_i8: Vec<*const i8> = extension_names
            .iter()
            .map(|name| name.as_ptr() as *const i8)
            .collect();
        create_info = create_info.enabled_extension_names(&extension_names_i8);

        // あとで
        create_info.enabled_layer_count = 0;

        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(VkEngineError::VulkanInstance)
        };

        instance
    }

    pub fn cleanup(&self) {
        // Cleanup logic here

        // instanceは自分でDropするので不要
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
