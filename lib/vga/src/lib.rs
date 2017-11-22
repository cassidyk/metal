

#![no_std]

#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(unique)]


extern crate spin;
extern crate volatile;

use core::fmt;

mod writer;
mod buffer;
mod color;

// re-export `Writer` struct
pub use writer::Writer;


/*****************
 * For a later date
 *****************/

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Writer::new_default();
    writer.write_fmt(args).unwrap();
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}