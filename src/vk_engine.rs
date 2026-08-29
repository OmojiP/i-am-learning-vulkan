use ash::vk::Extent2D as vkExtent2D;
use sdl2::event::{Event, WindowEvent};
use std::time::Duration;

pub struct VulkanEngine {
    pub frame_number: u32,
    pub stop_rendering: bool,
    pub window_extent: vkExtent2D,

    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
}

impl VulkanEngine {
    pub fn new() -> VulkanEngine {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window_extent = vkExtent2D {
            width: 800,
            height: 600,
        };

        let window = video_subsystem
            .window("Vulkan Engine", window_extent.width, window_extent.height)
            .position_centered()
            .vulkan()
            .build()
            .unwrap();

        VulkanEngine {
            frame_number: 0,
            stop_rendering: false,
            window_extent: window_extent,
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            window: window,
        }
    }

    pub fn cleanup(&self) {
        // Cleanup logic here
    }

    pub fn draw(&self) {}

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut b_quit: bool = false;

        // Main loop
        while !b_quit {
            while let Some(event) = event_pump.poll_event() {
                match event {
                    Event::Quit { .. } => {
                        b_quit = true;
                    }
                    Event::Window { win_event, .. } => {
                        // Handle window events
                        match win_event {
                            WindowEvent::Minimized => {
                                self.stop_rendering = true;
                            }
                            WindowEvent::Restored => {
                                self.stop_rendering = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            if self.stop_rendering {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.draw();
        }
    }
}
