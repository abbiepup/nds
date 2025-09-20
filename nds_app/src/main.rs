#![no_std]
#![no_main]

use core::ffi::c_int;
use core::hint::black_box;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
extern "C" fn main() -> c_int {
    let a = black_box(5);
    let b = black_box(2);
    a / b
}

#[cold]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
