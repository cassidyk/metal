
use super::{ActivePageTable, VirtualAddress, Page, Frame, FrameAllocator};
use super::table::{Table, Level1};

pub struct TemporaryPage {
    page: Page,
    allocator: TinyAllocator,
}

impl TemporaryPage {
    pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage
        where A: FrameAllocator
    {
        TemporaryPage {
            page: page,
            allocator: TinyAllocator::new(allocator),
        }
    }

    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
        use super::entry::WRITABLE;

        assert!(active_table.translate_page(self.page).is_none(),
            "Temporary page is already mapped");
        
        active_table.map_to(self.page, frame, WRITABLE, &mut self.allocator);
        self.page.start_address()
    }

    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator);
    }

    /// Maps the temporary page to the given table frame in the active table.
    /// Returns a reference to the now mapped table.
    pub fn map_table_frame(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> &mut Table<Level1> {
        unsafe {
            &mut *(self.map(frame, active_table) as *mut Table<Level1>)
        }
    }
}

/// `TinyAllocator` can only hold three frames
struct TinyAllocator([Option<Frame>; 3]);

impl TinyAllocator {
    fn new<A>(allocator: &mut A) -> TinyAllocator 
        where A: FrameAllocator
    {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }
}

impl FrameAllocator for TinyAllocator {
    /// Searches for available `Frame` and returns it
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    /// Searches for empty spot and puts `Frame` there
    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            match *frame_option {
                Some(_) => {},
                None => {
                    *frame_option = Some(frame);
                    return;
                }
            }
        }
        panic!("`TinyAllocator` can hold only 3 frames");
    }
}