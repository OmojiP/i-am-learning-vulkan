use ash::vk;

use crate::{
    vk_engine::VkEngineError,
    vulkan::{self},
};

pub struct Queue {
    pub grahpics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}

impl Queue {
    pub fn new(
        device: &vulkan::logical_devices::LogicalDevice,
        queue_families: &vulkan::queue_families::QueueFamilyIndices,
    ) -> Result<Self, VkEngineError> {
        let graphics_family_index =
            queue_families
                .graphics_family
                .ok_or(VkEngineError::VulkanInstance(
                    ash::vk::Result::ERROR_INITIALIZATION_FAILED,
                ))?;

        let graphics_queue_handle =
            unsafe { device.handle.get_device_queue(graphics_family_index, 0) };

        let present_family_index =
            queue_families
                .present_family
                .ok_or(VkEngineError::VulkanInstance(
                    ash::vk::Result::ERROR_INITIALIZATION_FAILED,
                ))?;

        let present_queue_handle =
            unsafe { device.handle.get_device_queue(present_family_index, 0) };

        Ok(Self {
            grahpics_queue: graphics_queue_handle,
            present_queue: present_queue_handle,
        })
    }
}
