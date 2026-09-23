#![no_std]

mod framebuffer;
mod interrupts;
mod memory;
mod serial;
mod vga_buffer;

use bootloader_api::BootInfo;
use framebuffer::{Color, FrameBufferWriter};

pub struct Kernel;

impl Kernel {
    pub fn run(boot_info: &'static mut BootInfo) -> ! {
        if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
            let info = framebuffer.info();
            let buffer = framebuffer.buffer_mut();

            let mut writer = FrameBufferWriter::new(buffer, info);

            let white = Color {
                r: 255,
                g: 255,
                b: 255,
            };
            let green = Color {
                r: 0,
                g: 255,
                b: 100,
            };

            writer.write_string("Hello Hello\n", &white);
            writer.write_string("Hello z Framebuffera!", &green);
        }

        loop {}
    }
}
