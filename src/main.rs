#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// Links a version of libc re-written in rust, so that we can allocate memory.
extern crate rlibc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

#[macro_use]
mod vga;
mod interrupts;
mod memes;
mod game;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

entry_point!(main);
fn main(_: &'static BootInfo) -> ! {
    interrupts::idt_init();
    interrupts::pic_init();
    println!("OS Started!");
    loop {}
}
