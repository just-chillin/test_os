use core::arch::x86_64;

pub fn cpuid() {
    let result;
    unsafe {
        result = x86_64::__cpuid(0x80000001)
    }
    println!("{:b}", result.edx);
}