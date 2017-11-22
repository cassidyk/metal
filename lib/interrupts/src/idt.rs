
use x86_64::structures::idt::{Idt, ExceptionStackFrame};

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

lazy_static! {
    pub static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        // return `idt`
        idt
    };
}

/*
 * INTERRUPT HANDLERS
 */

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT ({:#x})\n{:#?}", error_code, stack_frame);
    loop {}
}