#![no_std]
#![no_main]

#[allow(unused)]
use bootloader::entry_point;
use core::panic::PanicInfo;
use os::OS;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    OS::run();
}
