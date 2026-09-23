use spin::LazyLock;
use x86_64::VirtAddr;
use x86_64::instructions::segmentation::{CS, Segment};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

// IST indices are zero-based in the TSS, while the IDT stores them one-based.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PAGE_FAULT_IST_INDEX: u16 = 1;

const STACK_SIZE: usize = 4096 * 5;

#[repr(align(16))]
struct InterruptStack([u8; STACK_SIZE]);

static DOUBLE_FAULT_STACK: InterruptStack = InterruptStack([0; STACK_SIZE]);
static PAGE_FAULT_STACK: InterruptStack = InterruptStack([0; STACK_SIZE]);

static TSS: LazyLock<TaskStateSegment> = LazyLock::new(|| {
    let mut tss = TaskStateSegment::new();

    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] =
        VirtAddr::from_ptr(&DOUBLE_FAULT_STACK.0) + STACK_SIZE as u64;
    tss.interrupt_stack_table[PAGE_FAULT_IST_INDEX as usize] =
        VirtAddr::from_ptr(&PAGE_FAULT_STACK.0) + STACK_SIZE as u64;

    tss
});

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

static GDT: LazyLock<(GlobalDescriptorTable, Selectors)> = LazyLock::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.append(Descriptor::kernel_code_segment());
    let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));

    (
        gdt,
        Selectors {
            code_selector,
            tss_selector,
        },
    )
});

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
