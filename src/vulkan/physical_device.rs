use ash::vk;

use crate::vk_engine::VkEngineError;

pub struct PhysicalDevice {
    handle: Option<ash::vk::PhysicalDevice>,
}

impl PhysicalDevice {
    pub fn new(instance: &ash::Instance) -> Result<Self, VkEngineError> {
        unsafe {
            instance
                .enumerate_physical_devices()
                .map_err(VkEngineError::VulkanInstance)
                .and_then(|devices| {
                    for device in devices.iter() {
                        if Self::is_device_suitable(instance, device) {
                            return Ok(Self {
                                handle: Some(*device),
                            });
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
        let queue_family_indices = Self::find_queue_families(instance, device);
        return queue_family_indices.is_complete();
    }

    // 必要なキューが使用可能かどうかを確認
    fn find_queue_families(
        instance: &ash::Instance,
        device: &vk::PhysicalDevice,
    ) -> QueueFamilyIndices {
        unsafe {
            for (index, queue_family) in instance
                .get_physical_device_queue_family_properties(*device)
                .iter()
                .enumerate()
            {
                if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    return QueueFamilyIndices {
                        graphics_family: Some(index as u32),
                    };
                }
            }
        }

        QueueFamilyIndices {
            graphics_family: None,
        }
    }
}

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}
