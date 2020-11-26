use lazy_static::lazy_static;

const APIC_BASE: *mut u16 = FEE00000;


lazy_static! {
static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });
}



pub fn init() {
    let apic = 0x802 as *const usize;
    unsafe {
        for i in 0..10 {
            print!("{}", i);
        }
    }
}