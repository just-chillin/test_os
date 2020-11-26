use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin;

const VGA_BUFFER_ADDR: *mut u8 = 0xb8000 as *mut u8;

pub struct VGAWriter {
    n: usize,
}

lazy_static! {
    pub static ref STDOUT: spin::Mutex<VGAWriter> = spin::Mutex::new(VGAWriter::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    STDOUT.lock().write_fmt(args).unwrap();
}

impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.chars() {
            self.write_char(ch).unwrap();
        }
        return Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        if c == '\n' {
            self.newline();
            return Ok(())
        }
        unsafe {
            *VGA_BUFFER_ADDR.add(self.n * 2) = c as u8;
            *VGA_BUFFER_ADDR.add(self.n * 2 + 1) = 0xb;
        }
        self.n += 1;
        return Ok(())
    }
}

impl VGAWriter {
    pub fn new() -> VGAWriter {
        return VGAWriter { n: 0 };
    }

    pub fn newline(&mut self) {
        let col = self.n % 80;
        let padding = 80 - col;
        for _ in 0..padding {
            self.write_char(' ');
        }
    }

    pub fn reset(&mut self) {
        self.n = 0;
    }
}