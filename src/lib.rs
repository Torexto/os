#![no_std]

pub mod vga_buffer;
pub mod interrupts;
pub mod serial;
pub mod memory;

pub struct OS;

static HELLO: &[u8] = b"Hello World!";

impl OS {
    pub fn run() -> ! {
        let vga_buffer = 0xb8000 as *mut u8;

        for (i, &byte) in HELLO.iter().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            }
        }

        loop {}
    }
}