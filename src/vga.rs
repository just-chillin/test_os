
use core::fmt;
const VGA_BUFFER_ADDR: *mut u8 = 0xb8000 as *mut u8;

pub struct VGAWriter {
    n: usize,
}

impl fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.chars() {
            self.write_char(ch).unwrap();
        }
        return Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
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
}