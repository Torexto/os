#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::entry_point;
use os::OS;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    OS::run();
}
