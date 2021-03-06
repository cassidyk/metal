
#![no_std]

#![feature(lang_items)]
#![feature(asm)]
#![feature(global_allocator)]
#![feature(allocator_api)]
#![feature(alloc)]

// External crates
extern crate x86_64;
extern crate rlibc;
extern crate multiboot2;
extern crate alloc;

// Internal crates      
#[macro_use] extern crate vga;
extern crate bump_allocator;
extern crate interrupts;
extern crate memory;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

// use alloc::vec::Vec;


#[no_mangle]
pub extern fn rust_main(multiboot_info_addr: usize) {
    let boot_info = unsafe {
        multiboot2::load(multiboot_info_addr)
    };

    enable_nxe_bit();
    enable_write_protect_bit();

    unsafe {
        // Initialize dumb allocator
        bump_allocator::init(HEAP_START, HEAP_SIZE);
    }

    // Initialize memory handling
    let mut mem_ctrl = memory::init(boot_info, HEAP_START, HEAP_SIZE);
    
    unsafe {
        interrupts::pic_remap();
    }

    // Initialize interrupt handling
    interrupts::init(&mut mem_ctrl);

    

    println!("kernel initialized");
    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe {
        cr0_write(cr0() | Cr0::WRITE_PROTECT);
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\nPANIC at {}:{}\n    MSG: {}", file, line, fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! {
    loop {}
}