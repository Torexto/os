#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

use bootloader_api::BootInfo;

pub struct Kernel;

impl Kernel {
    pub fn panic(info: &PanicInfo) -> ! {
        serial_println!("\nKernel panic: \n{:?}", info);

        loop {
            x86_64::instructions::hlt();
        }
    }

    pub fn boot(boot_info: &'static mut BootInfo) -> ! {
        serial_println!("Booting kernel...");

        serial_println!("Loading cpu structures...");

        serial_println!("Initializing framebuffer");
        if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
            let info = framebuffer.info();
            let buffer = framebuffer.buffer_mut();
            framebuffer::init(buffer, info);
            crate::fb_println!("Hello Hello");
            crate::fb_print!("Hello z Framebuffera!");
        }

        interrupts::init();
        serial_println!("Loaded");

        serial_println!("Hlt loop");
        loop {
            x86_64::instructions::hlt();
        }
    }
}
