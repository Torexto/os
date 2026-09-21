#![no_std]
#![no_main]

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::OS;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    OS::run();
}

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

//#[unsafe(no_mangle)]
//pub extern "C" fn _start() -> ! {
//    OS::run();
//}
