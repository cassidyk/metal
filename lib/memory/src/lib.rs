#![no_std]

#![feature(unique)]


#[macro_use] extern crate vga;

#[macro_use] extern crate bitflags;
#[macro_use] extern crate once;
extern crate multiboot2;
extern crate x86_64;

pub use area_frame_allocator::AreaFrameAllocator;
pub use paging::remap_the_kernel;
pub use stack_allocator::Stack;

use paging::{PhysicalAddress, Page};

use multiboot2::BootInformation;


mod area_frame_allocator;
mod paging;
mod stack_allocator;


pub const PAGE_SIZE: usize = 4096;

pub struct MemoryController {
    active_table: paging::ActivePageTable,
    frame_allocator: AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut active_table,
            ref mut frame_allocator,
            ref mut stack_allocator,
        } = self;

        stack_allocator.alloc_stack(active_table, frame_allocator, size_in_pages)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(addr: usize) -> Frame {
        Frame {
            number: addr / PAGE_SIZE,
        }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame {
        Frame {
            number: self.number,
        }
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start,
            end,
        }
    }
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub fn init(boot_info: &BootInformation, heap_start: usize, heap_size: usize) -> MemoryController {
    assert_has_not_been_called!("memory::init must be called only once");

    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr)
        .min()
        .unwrap();
    
    let kernel_end = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr)
        .max()
        .unwrap();

    let multiboot_start = boot_info.start_address();
    let multiboot_end = boot_info.end_address();

    // println!("kernel_start: {:#x}, kernel_end: {:#x}", kernel_start, kernel_end);
    // println!("multiboot_start: {:#x}, multiboot_end: {:#x}", multiboot_start, multiboot_end);

    let mut frame_allocator = area_frame_allocator::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize,
        multiboot_start as usize, multiboot_end as usize,
        memory_map_tag.memory_areas()
    );

    // here it goes
    let mut active_table = paging::remap_the_kernel(&mut frame_allocator, boot_info);

    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_start + heap_size - 1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::WRITABLE, &mut frame_allocator);
    }

    let stack_allocator = {
        let start = heap_end_page + 1;
        let end = start + 100;
        let range = Page::range_inclusive(start, end);
        stack_allocator::StackAllocator::new(range)
    };

    MemoryController {
        active_table: active_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}