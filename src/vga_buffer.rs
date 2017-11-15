use core::ptr::Unique;
use volatile::Volatile;
use core::fmt;
use spin::Mutex;

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
    DarkGrey,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_pos: usize,
    default_color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8, color_code: Option<ColorCode>) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;

                let default_color_code = self.default_color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: color_code.unwrap_or(default_color_code),
                });
                self.col_pos += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe {
            self.buffer.as_mut()
        }
    }

    fn write_str(&mut self, s: &str, color_code: Option<ColorCode>) {
        // call self.write_byte(...) for each byte in str
        s.bytes().for_each(|c| self.write_byte(c, color_code) );
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.default_color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s, None);
        Ok(())
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    col_pos: 0,
    default_color_code: ColorCode::new(Color::White, Color::Black),
    buffer: unsafe {
        Unique::new_unchecked(0xb8000 as *mut _)
    },
});

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}