use buffer::{BUFFER_WRITER, ScreenChar, BUFFER_WIDTH, BUFFER_HEIGHT};
use color::{ColorCode, Color};


pub struct Writer {
    col_pos: usize,
    color_code: ColorCode,
}

impl Writer {
    /// Creates a new `Writer` with white text on a black background
    pub fn new_default() -> Self {
        Writer {
            col_pos: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
        }
    }

    pub fn new(color_code: ColorCode) -> Self {
        Writer {
            col_pos: 0,
            color_code: color_code,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;

                BUFFER_WRITER.lock()[(row, col)].write(ScreenChar::new(
                    byte,
                    self.color_code,
                ));
                self.col_pos += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        // call self.write_byte(...) for each byte in str
        s.bytes().for_each(|c| self.write_byte(c) );
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let mut writer = BUFFER_WRITER.lock();
                let character = writer[(row, col)].read();
                writer[(row - 1, col)].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for col in 0..BUFFER_WIDTH {
            BUFFER_WRITER.lock()[(row, col)].write(blank);
        }
    }
}