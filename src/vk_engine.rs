use ash::vk::{self};
use sdl2::event::{Event, WindowEvent};
use std::time::Duration;

const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);
const VALIDATION_LAYER_NAME: &std::ffi::CStr = c"VK_LAYER_KHRONOS_validation";

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

    entry: ash::Entry,
    instance: ash::Instance,

    // Debug Messenger
    debug_utils: Option<ash::ext::debug_utils::Instance>,
    debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
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
        // TODO: debugCallBack setupDebugMessenger()
        let (debug_utils, debug_messenger) = if ENABLE_VALIDATION_LAYERS {
            let (loader, messenger) = VkEngine::setup_debug_messenger(&entry, &instance)?;
            (Some(loader), Some(messenger))
        } else {
            (None, None)
        };

        Ok(VkEngine {
            frame_number: 0,
            stop_rendering: false,
            window_extent: window_extent,

            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            window: window,

            entry: entry,
            instance: instance,

            debug_utils: debug_utils,
            debug_messenger: debug_messenger,
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

        // 検証レイヤーの設定
        let validation_layer_names = [VALIDATION_LAYER_NAME.as_ptr()];
        if ENABLE_VALIDATION_LAYERS {
            if !VkEngine::check_validation_layer_support(entry)? {
                return Err(VkEngineError::ValidationLayerNotFound());
            }
            create_info = create_info.enabled_layer_names(&validation_layer_names)
        }

        // 拡張機能リストを設定
        let extension_names_i8: Vec<*const i8> = VkEngine::get_required_extensions(window)?;
        create_info = create_info.enabled_extension_names(&extension_names_i8);

        let mut debug_create_info = VkEngine::debug_messenger_create_info();
        if ENABLE_VALIDATION_LAYERS {
            create_info = create_info.push_next(&mut debug_create_info);
        }

        // VkInstanceを作成
        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(VkEngineError::VulkanInstance)
        };

        instance
    }

    pub fn check_validation_layer_support(entry: &ash::Entry) -> Result<bool, VkEngineError> {
        let available_layers = unsafe {
            entry
                .enumerate_instance_layer_properties()
                .map_err(VkEngineError::VulkanInstance)?
        };

        return Ok(available_layers.iter().any(|layer| {
            layer
                .layer_name_as_c_str()
                .map(|name| name == VALIDATION_LAYER_NAME)
                .unwrap_or(false)
        }));
    }

    /// 拡張機能リストを取得
    pub fn get_required_extensions(
        window: &sdl2::video::Window,
    ) -> Result<Vec<*const i8>, VkEngineError> {
        let mut extension_names = window
            .vulkan_instance_extensions()
            .map_err(|err| VkEngineError::VulkanExtension(err.to_string()))?;
        if ENABLE_VALIDATION_LAYERS {
            // デバッグメッセンジャー拡張機能を追加
            extension_names.push(
                ash::vk::EXT_DEBUG_UTILS_NAME
                    .to_str()
                    .map_err(|err| VkEngineError::VulkanExtension(err.to_string()))?,
            );
        }
        let extension_names_i8: Vec<*const i8> = extension_names
            .iter()
            .map(|name| name.as_ptr() as *const i8)
            .collect();

        Ok(extension_names_i8)
    }

    unsafe extern "system" fn debug_callback(
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        p_user_data: *mut std::ffi::c_void,
    ) -> vk::Bool32 {
        let message = if p_callback_data.is_null() {
            "<null>"
        } else {
            unsafe {
                std::ffi::CStr::from_ptr((*p_callback_data).p_message)
                    .to_str()
                    .unwrap_or("<invalid utf8>")
            }
        };

        println!(
            "[Vulkan] severity={:?}, type={:?}\n{}",
            message_severity, message_type, message
        );

        vk::FALSE
    }

    fn debug_messenger_create_info<'a>() -> vk::DebugUtilsMessengerCreateInfoEXT<'a> {
        ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(Self::debug_callback))
    }

    fn setup_debug_messenger(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> Result<(ash::ext::debug_utils::Instance, vk::DebugUtilsMessengerEXT), VkEngineError> {
        let debug_utils = ash::ext::debug_utils::Instance::new(entry, instance);

        let create_info = VkEngine::debug_messenger_create_info();

        let debug_messenger = unsafe {
            debug_utils
                .create_debug_utils_messenger(&create_info, None)
                .map_err(VkEngineError::VulkanInstance)?
        };

        Ok((debug_utils, debug_messenger))
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
    fn drop(&mut self) {
        unsafe {
            if let (Some(debug_utils), Some(debug_messenger)) =
                (&self.debug_utils, self.debug_messenger)
            {
                debug_utils.destroy_debug_utils_messenger(debug_messenger, None);
            }

            self.instance.destroy_instance(None);
        }
    }
}
