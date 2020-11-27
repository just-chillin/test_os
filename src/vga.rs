use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin;

pub struct VGAWriter {
    row: usize,
    col: usize,
}

lazy_static! {
    static ref STDOUT: spin::Mutex<VGAWriter> = spin::Mutex::new(VGAWriter::new());
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
        return Ok(());
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let vga_buffer = unsafe {
            (0xb8000 as *mut [[u16; 80]; 35]).as_mut().unwrap()
        };
        if c == '\n' {
            self.newline();
            return Ok(());
        }
        let color = (0xb as u16) << 8;
        vga_buffer[self.row][self.col] = c as u16 | color;
        if self.col >= 80 {
            self.newline();
        } else {
            self.col += 1;
        }
        return Ok(());
    }
}

impl VGAWriter {
    pub fn new() -> VGAWriter {
        return VGAWriter { row: 0, col: 0 };
    }

    pub fn newline(&mut self) {
        self.col = 0;
        self.row += 1;
        self.row = self.row.clamp(0, 80);
    }
}