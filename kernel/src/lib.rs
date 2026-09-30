#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod framebuffer;
pub mod interrupts;
pub mod memory;
pub mod serial;

use core::panic::PanicInfo;

use bootloader_api::BootInfo;

use crate::memory::paging::BootInfoFrameAllocator;

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub struct Kernel;

impl Kernel {
    pub fn panic(info: &PanicInfo) -> ! {
        serial_println!("\nKernel panic: \n{:?}", info);

        hlt_loop()
    }

    pub fn boot(boot_info: &'static mut BootInfo) -> ! {
        serial_println!("Booting kernel...");

        serial_println!("Loading CPU structures...");
        interrupts::init();
        serial_println!("CPU structures loaded");

        serial_println!("Initializing framebuffer...");
        if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
            let info = framebuffer.info();
            let buffer = framebuffer.buffer_mut();
            framebuffer::init(buffer, info);
        }
        serial_println!("Framebuffer intialized");

        fb_println!("Hello World!");

        serial_println!("Mapping pages...");
        let mut mapper = memory::paging::mapper(boot_info.physical_memory_offset.as_ref());
        let mut frame_allocator =
            unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
        serial_println!("Pages mapped");

        serial_println!("Initializing Heap...");
        let _ = memory::allocators::heap::init(&mut mapper, &mut frame_allocator);
        serial_println!("Heap initialized...");

        serial_println!("Hlt loop");
        hlt_loop()
    }
}
