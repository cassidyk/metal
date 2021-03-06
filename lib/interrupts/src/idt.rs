
use x86_64::structures::idt::{Idt, ExceptionStackFrame};
use io::{port_out, port_in};

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

lazy_static! {
    pub static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        idt.interrupts[0].set_handler_fn(general_purpose_interrupt_stub);

        idt.interrupts[1].set_handler_fn(keyboard_interrupt_handler);
        
        // return `idt`
        idt
    };
}

fn ack_int() {
    unsafe {
        port_out(0x20, 0x20 as u16);
    }
}

/*
 * INTERRUPT HANDLERS
 */

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: INVALID TSS ({:#x})\n{:#?}", error_code, stack_frame);
    loop {}
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT ({:#x})\n{:#?}", error_code, stack_frame);
    loop {}
}

extern "x86-interrupt" fn general_purpose_interrupt_stub(_stack_frame: &mut ExceptionStackFrame) {
    // println!("An interrupt happened");
    ack_int();
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    println!("KEYBOARD INTERRUPT");
    let _ = unsafe {
        port_in::<u16>(0x60)
    };
    ack_int();
}