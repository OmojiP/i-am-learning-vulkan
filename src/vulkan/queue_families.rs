use ash::vk;

use crate::{
    vk_engine::VkEngineError,
    vulkan::{self},
};

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }

    // 必要なキューが使用可能かどうかを確認
    pub fn find_queue_families(
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
