#![feature(const_fn)]
#![feature(allocator_api)]
#![feature(alloc)]
#![feature(global_allocator)]

#![no_std]

use alloc::heap::{Alloc, AllocErr, Layout};
use spin::Mutex;

extern crate alloc;
extern crate spin;


struct LockedHeap {
    heap: Mutex<Heap>,
}

#[global_allocator]
static GLOBAL_ALLOC: LockedHeap = LockedHeap::empty();

pub unsafe fn init(start: usize, size: usize) {
    GLOBAL_ALLOC.init(start, size);
}

impl LockedHeap {
    /// Creates an empty heap
    pub const fn empty() -> LockedHeap {
        LockedHeap {
            heap: Mutex::new(Heap::empty()),
        }
    }

    /// Initializes the heap
    unsafe fn init(&self, start: usize, size: usize) {
        self.heap.lock().init(start, size);
    }
}

/// The interface used for all allocations of heap structures
unsafe impl<'a> Alloc for &'a LockedHeap {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        self.heap.lock().allocate(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.heap.lock().dealloc(ptr, layout);
    }
}


/// Fixed size heap
struct Heap {
    start: usize,
    end: usize,
    next: usize,
}

impl Heap {
    /// Creates an empty `Heap`
    pub const fn empty() -> Heap {
        Heap {
            start: 0,
            end: 0,
            next: 0,
        }
    }

    /// Initializes `Heap` with given `start` and `size`
    unsafe fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.next = start;
    }

    /// Allocates a chunk of `size` with the given `align`ment
    unsafe fn allocate(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let alloc_start = align_up(self.next, layout.align());
        let alloc_end = alloc_start.saturating_add(layout.size());

        if alloc_end <= self.end {
            self.next = alloc_end;
            Ok(alloc_start as *mut u8)
        } else {
            Err(AllocErr::Exhausted{
                request: layout
            })
        }
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        // leaky
    }
}

/// Align downwards.
/// Returns the greatest x with alignment `align` so that x <= addr.
/// The alignment must be a power of 2
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of two");
    }
}

/// Align upwards.
/// Returns the smallest x with alignment `align so that x >= addr.
/// The alignment must be a power of 2
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}