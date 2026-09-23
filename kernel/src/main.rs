#![no_std]
#![no_main]

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{BootInfo, entry_point};
use core::panic::PanicInfo;
use kernel::Kernel;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Kernel::panic(info)
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    Kernel::boot(boot_info);
}

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);
