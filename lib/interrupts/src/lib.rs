#![no_std]

#![feature(abi_x86_interrupt)]

#[macro_use] extern crate vga;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;
extern crate io;
extern crate memory;
extern crate x86_64;
extern crate bit_field;
extern crate spin;

use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use spin::Once;

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

mod idt;
mod gdt;
mod pic;

pub use pic::remap as pic_remap;

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

pub fn init(mem_controller: &mut MemoryController) {
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    let double_fault_stack = mem_controller.alloc_stack(1)
        .expect("could not allocate double-fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[idt::DOUBLE_FAULT_IST_INDEX] = VirtualAddress(
            double_fault_stack.top()
        );
        tss
    });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);

    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));

        gdt
    });

    gdt.load();

    unsafe {
        // reload code segment register
        set_cs(code_selector);
        // load TSS
        load_tss(tss_selector);
    }

    idt::IDT.load();
    
}