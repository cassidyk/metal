
use super::paging::{PageIter, Page, ActivePageTable, WRITABLE};
use super::{PAGE_SIZE, FrameAllocator};

pub struct Stack {
    top: usize,
    bottom: usize,
}

impl Stack {
    fn new(top: usize, bottom: usize) -> Self {
        assert!(top > bottom);
        Stack {
            top: top,
            bottom: bottom,
        }
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn bottom(&self) -> usize {
        self.bottom
    }
}

pub struct StackAllocator {
    range: PageIter,
}

impl StackAllocator {
    pub fn new(page_range: PageIter) -> Self {
        StackAllocator {
            range: page_range,
        }
    }

    pub fn alloc_stack<A>(&mut self, active_table: &mut ActivePageTable,
                        frame_allocator: &mut A, size_in_pages: usize) -> Option<Stack>
        where A: FrameAllocator
    {
        if size_in_pages == 0 {
            // a 0-sized stack makes no sense
            return None; 
        }

        // clone the range since we only want to change it on success
        let mut range = self.range.clone();

        // try to allocate the stack pages and a guard page
        let guard_page = range.next();
        let stack_start = range.next();
        let stack_end = match size_in_pages {
            1 => stack_start,
            _ => {
                // Choose the (size_in_pages-2)th element, since
                // index starts at 0 and we've already allocated a start page
                range.nth(size_in_pages - 2)
            }
        };

        match (guard_page, stack_start, stack_end) {
            (Some(_), Some(start), Some(end)) => {
                // success!
                // write back updated range
                self.range = range;

                // map pages to physical frames
                for page in Page::range_inclusive(start, end) {
                    active_table.map(page, WRITABLE, frame_allocator);
                }

                // create a new stack
                let top_of_stack = end.start_address() + PAGE_SIZE;
                Some(Stack::new(top_of_stack, start.start_address()))
            },
            _ => {
                // not enough pages
                None
            },
        }
    }
}

