#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod interrupts;
pub mod memory;
pub mod serial;

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
        }

        fb_println!("Hello World!");
        fb_println!("It did not crash!");

        interrupts::init();

        serial_println!("Loaded");

        serial_println!("Hlt loop");
        loop {
            serial_println!("loop");
            x86_64::instructions::hlt();
        }
    }
}
