use std::collections::HashSet;

use ash::vk;

use crate::{
    vk_engine::VkEngineError,
    vulkan::{self, surface},
};

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    // 必要なキューが使用可能かどうかを確認
    pub fn find_queue_families(
        instance: &ash::Instance,
        surface: &surface::Surface,
        device: &vk::PhysicalDevice,
    ) -> QueueFamilyIndices {
        let mut graphics_index: Option<u32> = None;
        let mut present_index: Option<u32> = None;

        unsafe {
            for (index, queue_family) in instance
                .get_physical_device_queue_family_properties(*device)
                .iter()
                .enumerate()
            {
                // GRAPHICSキューが使用可能かどうかを確認
                if graphics_index.is_none()
                    && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                {
                    graphics_index = Some(index as u32);
                }

                // PRESENTキューが使用可能かどうかを確認
                if present_index.is_none() {
                    let present_support = surface
                        .surface_loader
                        .get_physical_device_surface_support(*device, index as u32, surface.handle);
                    if present_support.is_ok() && present_support.unwrap() {
                        present_index = Some(index as u32);
                    }
                }

                // 両方のキューが見つかった場合はループを終了
                if graphics_index.is_some() && present_index.is_some() {
                    break;
                }
            }
        }

        QueueFamilyIndices {
            graphics_family: graphics_index,
            present_family: present_index,
        }
    }

    // 重複なしのキューインデックスを取得
    pub fn get_unique_queue_families(&self) -> HashSet<u32> {
        let mut unique_indices = HashSet::new();
        if let Some(graphics_index) = self.graphics_family {
            unique_indices.insert(graphics_index);
        }
        if let Some(present_index) = self.present_family {
            unique_indices.insert(present_index);
        }
        unique_indices
    }
}
