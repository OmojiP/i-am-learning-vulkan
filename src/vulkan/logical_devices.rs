use ash::vk;

use crate::vk_engine::VkEngineError;

pub struct LogicalDevice {
    pub device: ash::Device,
}

impl LogicalDevice {
    pub fn new(
        instance: &ash::Instance,
        physical_device: &ash::vk::PhysicalDevice,
    ) -> Result<Self, VkEngineError> {
        let indices = crate::vulkan::queue_families::QueueFamilyIndices::find_queue_families(
            instance,
            physical_device,
        );

        if indices.graphics_family.is_none() {
            return Err(VkEngineError::VulkanInstance(
                ash::vk::Result::ERROR_INITIALIZATION_FAILED,
            ));
        }

        let queue_priorities = [1.0];
        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(indices.graphics_family.unwrap())
            .queue_priorities(&queue_priorities);
        let queue_create_infos = [queue_create_info];

        let device_features = vk::PhysicalDeviceFeatures::default();

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features);

        unsafe {
            let logical_device = instance
                .create_device(*physical_device, &device_create_info, None)
                .map_err(VkEngineError::VulkanInstance)?;

            Ok(Self {
                device: logical_device,
            })
        }
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
