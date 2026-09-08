use ash::vk::{self, Handle};

use crate::{
    vk_engine::VkEngineError,
    vulkan::{self},
};

pub struct Surface {
    pub handle: vk::SurfaceKHR,
    pub surface_loader: ash::khr::surface::Instance,
}

impl Surface {
    pub fn new(
        entry: &ash::Entry,
        instance: &vulkan::instance::VkInstance,
        window: &sdl2::video::Window,
    ) -> Result<Self, VkEngineError> {
        let surface_handle = Self::create_surface(&instance.handle, window)?;
        let surface_loader = ash::khr::surface::Instance::new(entry, &instance.handle);

        Ok(Self {
            handle: surface_handle,
            surface_loader: surface_loader,
        })
    }

    fn create_surface(
        instance: &ash::Instance,
        window: &sdl2::video::Window,
    ) -> Result<vk::SurfaceKHR, VkEngineError> {
        // sdl2::sys::VkSurfaceKHR::

        // ashのvkInstanceのハンドルをsdl2のVkInstanceに変換する
        let vulkan_instance_handle: sdl2::sys::VkInstance =
            instance.handle().as_raw() as sdl2::sys::VkInstance;

        let raw_surface = window
            .vulkan_create_surface(vulkan_instance_handle)
            .map_err(|err| VkEngineError::SdlCreateWindow(err))?;

        let surface = ash::vk::SurfaceKHR::from_raw(raw_surface);
        Ok(surface)
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.handle, None);
        }
    }
}
