#![no_std]
#![no_main]
#![feature(fmt_as_str)]
#![feature(core_intrinsics)]

// Links a version of libc re-written in rust, so that we can allocate memory.
extern crate rlibc;

use core::panic::PanicInfo;
use core::fmt::Write;
use core::arch::x86_64::{_rdrand32_step};

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn random() -> u32 {
    let mut result: u32 = 0;
    unsafe {
        _rdrand32_step(&mut result);
    }
    return result;
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = vga::VGAWriter::new();
    write!(vga, "yooooo rust is awesome").unwrap();


    write!(vga, "{}", random()).unwrap();
    loop {}
}
