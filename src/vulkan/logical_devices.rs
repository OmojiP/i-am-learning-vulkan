use ash::vk;

use crate::{
    vk_engine::VkEngineError,
    vulkan::{self},
};

pub struct LogicalDevice {
    pub handle: ash::Device,
    pub queue_families: vulkan::queue_families::QueueFamilyIndices,
}

impl LogicalDevice {
    pub fn new(
        instance: &vulkan::instance::VkInstance,
        physical_device: &vulkan::physical_device::PhysicalDevice,
    ) -> Result<Self, VkEngineError> {
        let indices = vulkan::queue_families::QueueFamilyIndices::find_queue_families(
            &instance.handle,
            &physical_device.handle,
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
                .handle
                .create_device(physical_device.handle, &device_create_info, None)
                .map_err(VkEngineError::VulkanInstance)?;

            Ok(Self {
                handle: logical_device,
                queue_families: indices,
            })
        }
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        unsafe {
            self.handle.destroy_device(None);
        }
    }
}
