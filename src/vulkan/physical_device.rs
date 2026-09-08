use ash::vk;

use crate::{
    vk_engine::VkEngineError,
    vulkan::{self},
};

pub struct PhysicalDevice {
    pub handle: ash::vk::PhysicalDevice,
}

impl PhysicalDevice {
    pub fn new(instance: &vulkan::instance::VkInstance) -> Result<Self, VkEngineError> {
        unsafe {
            instance
                .handle
                .enumerate_physical_devices()
                .map_err(VkEngineError::VulkanInstance)
                .and_then(|devices| {
                    for device in devices.iter() {
                        if Self::is_device_suitable(&instance.handle, device) {
                            return Ok(Self { handle: *device });
                        }
                    }
                    Err(VkEngineError::VulkanInstance(
                        ash::vk::Result::ERROR_INITIALIZATION_FAILED,
                    ))
                })
        }
    }

    fn is_device_suitable(instance: &ash::Instance, device: &vk::PhysicalDevice) -> bool {
        // デバイス選択の基準に使用できる要素
        unsafe {
            let device_properties = instance.get_physical_device_properties(*device);
            let device_features = instance.get_physical_device_features(*device);
        }

        // GRAPHICSキューが使用可能かどうかを確認
        let queue_family_indices =
            vulkan::queue_families::QueueFamilyIndices::find_queue_families(instance, device);
        return queue_family_indices.is_complete();
    }
}
