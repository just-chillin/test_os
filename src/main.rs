#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// Links a version of libc re-written in rust, so that we can allocate memory.
extern crate rlibc;

use core::panic::PanicInfo;

#[macro_use]
mod vga;
mod interrupts;
mod memes;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}



fn main() {
    println!("Started!!");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::idt_init();
    interrupts::pic_init();

    main();
    loop {}
}
