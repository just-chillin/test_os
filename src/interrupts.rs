use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::memes;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(
        unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }
    );


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(fault_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn idt_init() {
    IDT.load();
    println!("Idt initialized!");
}

pub fn pic_init() {
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

// Handlers
extern "x86-interrupt"
fn fault_handler(stack: &mut InterruptStackFrame) {
    println!("AAAAAA SOMETHING BROKEEEEE\n{:#?}", stack);
}

extern "x86-interrupt"
fn double_fault_handler(stack: &mut InterruptStackFrame, code: u64) -> ! {
    panic!("Error: {} {:#?}\n {}", code, stack, memes::HEADING_OUT);
}

extern "x86-interrupt"
fn timer_interrupt_handler(_: &mut InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt"
fn keyboard_interrupt_handler(_: &mut InterruptStackFrame) {
    println!("Keyboard interrupt!");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}