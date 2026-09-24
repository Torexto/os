pub mod gdt;
pub mod idt;

pub fn init() {
    gdt::init();
    idt::init();
    unsafe {
        idt::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}
