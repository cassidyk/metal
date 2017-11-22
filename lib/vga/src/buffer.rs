use spin::Mutex;
use core::ptr::Unique;
use core::ops::{Index, IndexMut};
use volatile::Volatile;

use color::ColorCode;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub static BUFFER_WRITER: Mutex<BufferWriter> = Mutex::new(BufferWriter::new(0xb8000));

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub const fn new(ascii_char: u8, color_code: ColorCode) -> Self {
        ScreenChar {
            ascii_char,
            color_code,
        }
    }
}

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct BufferWriter {
    ptr: Unique<Buffer>,
}

impl BufferWriter {
    pub const fn new(ptr: usize) -> Self {
        BufferWriter {
            ptr: unsafe {
                Unique::new_unchecked(ptr as *mut _)
            },
        }
    }

    fn buffer_mut(&mut self) -> &mut Buffer {
        unsafe {
            self.ptr.as_mut()
        }
    }

    fn buffer(&self) -> &Buffer {
        unsafe {
            self.ptr.as_ref()
        }
    }
}

impl Index<(usize, usize)> for BufferWriter {
    type Output = Volatile<ScreenChar>;

    fn index(&self, tuple: (usize, usize)) -> &Volatile<ScreenChar> {
        &self.buffer().chars[tuple.0][tuple.1]
    }
}

impl IndexMut<(usize, usize)> for BufferWriter {
    fn index_mut(&mut self, tuple: (usize, usize)) -> &mut Volatile<ScreenChar> {
        &mut self.buffer_mut().chars[tuple.0][tuple.1]
    }
}