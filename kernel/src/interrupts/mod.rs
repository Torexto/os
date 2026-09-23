pub mod idt;
pub mod gdt;

pub fn init() {
    idt::init();
    gdt::init();
}