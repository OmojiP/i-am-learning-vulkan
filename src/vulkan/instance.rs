use crate::{
    vk_engine::{VkEngine, VkEngineError},
    vulkan::debug,
};

pub const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);
pub const VALIDATION_LAYER_NAME: &std::ffi::CStr = c"VK_LAYER_KHRONOS_validation";

pub struct VkInstance {
    pub instance: ash::Instance,
}

impl VkInstance {
    pub fn new(window: &sdl2::video::Window, entry: &ash::Entry) -> Result<Self, VkEngineError> {
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
            if !Self::check_validation_layer_support(entry)? {
                return Err(VkEngineError::ValidationLayerNotFound());
            }
            create_info = create_info.enabled_layer_names(&validation_layer_names)
        }

        // 拡張機能リストを設定
        let extension_names_i8: Vec<*const i8> = Self::get_required_extensions(window)?;
        create_info = create_info.enabled_extension_names(&extension_names_i8);

        let mut debug_create_info = debug::DebugMessenger::create_info();
        if ENABLE_VALIDATION_LAYERS {
            create_info = create_info.push_next(&mut debug_create_info);
        }

        // VkInstanceを作成
        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(VkEngineError::VulkanInstance)?
        };

        Ok(Self { instance })
    }

    fn check_validation_layer_support(entry: &ash::Entry) -> Result<bool, VkEngineError> {
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
    fn get_required_extensions(
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
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
