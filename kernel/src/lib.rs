#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

use bootloader_api::BootInfo;
use framebuffer::{Color, FrameBufferWriter};

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

        interrupts::gdt::init();
        interrupts::idt::init();

        serial_println!("Loaded");

        #[allow(unconditional_recursion)]
        #[inline(never)]
        fn stack_overflow() {
            let mut stack_marker = 0_u8;
            stack_overflow();
            // Keep work after the recursive call so optimized builds cannot turn
            // this demonstration into a tail-call loop.
            unsafe { core::ptr::write_volatile(&mut stack_marker, 1) };
        }

        stack_overflow();

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
