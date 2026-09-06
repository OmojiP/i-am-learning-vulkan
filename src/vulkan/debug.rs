use ash::vk;

use crate::vk_engine::VkEngineError;

pub struct DebugMessenger {
    utils: ash::ext::debug_utils::Instance,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugMessenger {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Result<Self, VkEngineError> {
        let utils = ash::ext::debug_utils::Instance::new(entry, instance);

        let create_info = Self::create_info();

        let messenger = unsafe {
            utils
                .create_debug_utils_messenger(&create_info, None)
                .map_err(VkEngineError::VulkanInstance)?
        };

        Ok(Self { utils, messenger })
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

    pub fn create_info<'a>() -> vk::DebugUtilsMessengerCreateInfoEXT<'a> {
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
}

impl Drop for DebugMessenger {
    fn drop(&mut self) {
        unsafe {
            self.utils
                .destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}
