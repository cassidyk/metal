#![feature(lang_items)]
#![feature(unique)]
#![feature(const_unique_new)]
#![feature(const_fn)]

#![no_std]

extern crate volatile;
extern crate rlibc;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;
mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_info_addr: usize) {
    let boot_info = unsafe {
        multiboot2::load(multiboot_info_addr)
    };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    
    println!("Memory areas:");
    memory_map_tag.memory_areas().for_each(|area| {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    });

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");
    
    println!("Kernel sections:");
    elf_sections_tag.sections().for_each(|section| {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
    });

    let kernel_start = elf_sections_tag
        .sections()
        .map(|s| s.addr)
        .min()
        .unwrap();
    
    let kernel_end = elf_sections_tag
        .sections()
        .map(|s| s.addr)
        .max()
        .unwrap();

    let multiboot_start = multiboot_info_addr;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start, kernel_end);
    println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize,
        multiboot_start as usize, multiboot_end as usize,
        memory_map_tag.memory_areas()
    );

    use memory::FrameAllocator;

    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }

    loop {}
}


#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:\n    {}", file, line, fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() -> ! {
    loop {}
}