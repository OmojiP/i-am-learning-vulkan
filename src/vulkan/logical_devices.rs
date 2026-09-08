use ash::vk;

use crate::{
    vk_engine::VkEngineError,
    vulkan::{self, queue},
};

pub struct LogicalDevice {
    pub handle: ash::Device,
}

impl LogicalDevice {
    pub fn new(
        instance: &vulkan::instance::VkInstance,
        indices: &vulkan::queue_families::QueueFamilyIndices,
        physical_device: &vulkan::physical_device::PhysicalDevice,
    ) -> Result<Self, VkEngineError> {
        if indices.graphics_family.is_none() {
            return Err(VkEngineError::VulkanInstance(
                ash::vk::Result::ERROR_INITIALIZATION_FAILED,
            ));
        }

        let queue_priorities = [1.0];

        let mut queue_create_infos = Vec::new();
        let unique_queue_families = indices.get_unique_queue_families();

        for &queue_family in unique_queue_families.iter() {
            let mut queue_create_info = vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family)
                .queue_priorities(&queue_priorities);
            queue_create_info.queue_count = 1;

            queue_create_infos.push(queue_create_info);
        }

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
