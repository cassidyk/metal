#![no_std]

#![feature(abi_x86_interrupt)]

#[macro_use] extern crate vga;
#[macro_use] extern crate lazy_static;
extern crate x86_64;

use x86_64::structures::idt::{Idt, ExceptionStackFrame};

/// Interrupt handling

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT ({:#x})\n{:#?}", error_code, stack_frame);
    loop {}
}