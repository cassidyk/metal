
use super::align_up;
use core::ptr::Unique;
use core::mem::{self, size_of};
use alloc::allocator::{Layout, AllocErr};

struct Block {
    size: usize,
    next: Option<Unique<Block>>,
}

impl Block {
    pub const fn min_size() -> usize {
        size_of::<usize>() * 2
    }
}

struct BlockList {
    start: Option<Unique<Block>>,
}

impl BlockList {
    /// Creates an empty heap with no memory
    pub const fn empty() -> BlockList {
        BlockList {
            start: None,
        }
    }

    /// Creates a new heap starting at `block_addr` with size of `block_size
    pub unsafe fn new(block_addr: usize, block_size: usize) -> BlockList {
        let ptr = block_addr as *mut Block;
        mem::replace(&mut *ptr, Block {
            size: block_size,
            next: None,
        });

        BlockList {
            start: Some(Unique::new_unchecked(ptr)),
        }
    }

    /// Searches the list for a large-enough block 
    /// (it can hold `layout.size()` bytes with the given `layout.align()`).
    /// 
    /// Uses the "First Fit" technique for allocating memory.
    fn allocate_first_fit(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        // Traverses the linked list of blocks until it finds one that's large enough
        allocate_first_fit(self.start, layout).map(|allocator| {
            if allocator.before_padding > 0 {

            }
        });
    }
}

struct Allocation {
    addr: usize,
    size: usize,
    before_padding: usize,
    after_padding: usize,
}

fn allocate_first_fit(mut previous: &mut Block, layout: Layout) -> Result<Allocation, AllocErr> {
    loop {
        let allocation = previous.next.as_mut().and_then(|current| {
            
        });
    }
}