#![no_std]
#![no_main]

// Links a version of libc re-written in rust, so that we can allocate memory.
extern crate rlibc;

use core::panic::PanicInfo;

#[macro_use]
mod vga;
mod lapic;
mod x86;
mod interrupt;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("yooooo rust is awesome");
    println!("println works too!");
    x86::cpuid();
    // lapic::init();
    // write!(vga, "yooooo rust is awesome").unwrap();
    loop {}
}
